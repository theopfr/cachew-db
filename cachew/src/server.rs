use std::{sync::{Arc}, net::SocketAddr};
use tokio::{sync::{Mutex, broadcast}, signal::unix::{signal, SignalKind}};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream, tcp::ReadHalf}
};
use log::{info, warn, error};

use crate::{response::QueryResponse, state::State, schemas::QueryRequest};
use crate::{protocol_error};
use crate::parser;
use crate::errors::protocol_errors::{ProtocolErrorType};

const REQUEST_START_MARKER: &str = "CASP/";
const REQUEST_END_MARKER: &str = "/\n";


fn check_protocol(request: &str) -> Result<(), String> {
    if request.is_empty() || request == "\n" {
        protocol_error!(ProtocolErrorType::EmptyRequest)
    }
    else if !(request.len() >= REQUEST_START_MARKER.len() && &request[..REQUEST_START_MARKER.len()] == REQUEST_START_MARKER) {
        protocol_error!(ProtocolErrorType::StartMarkerNotFound(REQUEST_START_MARKER.to_string()))
    }

    else if &request[request.len() - REQUEST_END_MARKER.len()..] != REQUEST_END_MARKER {
        protocol_error!(ProtocolErrorType::EndMarkerNotFound(REQUEST_END_MARKER.to_string()))
    }

    else if request.len() <= (REQUEST_START_MARKER.len() + REQUEST_END_MARKER.len()) {
        protocol_error!(ProtocolErrorType::EndMarkerNotFound(REQUEST_END_MARKER.to_string()))
    }

    Ok(())
}



async fn handle_client(mut socket: TcpStream, address: SocketAddr, state_clone: Arc<Mutex<State>>, mut shutdown_rx: broadcast::Receiver<()>) {
    let (socket_reader, mut socket_writer) = socket.split();
            
    let mut reader: BufReader<ReadHalf> = BufReader::new(socket_reader);
    let mut line: String = String::new();

    loop {
        tokio::select! {
            _ = shutdown_rx.recv() => {
                let _ = socket_writer.write_all(QueryResponse::warn("SHUTDOWN").to_string().as_bytes()).await;
                break;
            }
            byte_amount = reader.read_line(&mut line) => {                
                let mut state_lock = state_clone.lock().await;

                if byte_amount.unwrap() == 0 {
                    warn!("Connection closed. ({})", address);
                    state_lock.deauthenticate(&address.to_string());
                    return;
                }

                info!("Incoming request: {:?}.", &line);

                // check if the incoming message followed the protocol specification
                match check_protocol(&line) {
                    Ok(_) => { }
                    Err(error) => {
                        error!("Invalid request. Request didn't follow CASP specification.");
                        socket_writer.write_all(QueryResponse::error(&error).to_string().as_bytes()).await.unwrap();
                        break;
                    }
                }

                // extract the raw database request form the message and parse it
                let request: &str = line.strip_prefix(REQUEST_START_MARKER).unwrap().strip_suffix(REQUEST_END_MARKER).unwrap().trim();
                let query = parser::parse(request, &state_lock.database_type);
                
                match query {
                    Ok(query) => {
                        // handle shutdown on request
                        if let QueryRequest::SHUTDOWN = query {
                            warn!("Received shutdown request. Shutting down gracefully...");

                            // notify all client handlers to send shutdown message to their client
                            state_lock.signal_shutdown().await;

                            // send OK response to client who intiated shutdown
                            socket_writer.write_all(QueryResponse::ok(crate::schemas::QueryResponseType::SHUTDOWN_OK, &state_lock.database_type).to_string().as_bytes()).await.unwrap();                            

                            // TODO: add persistance here if needed.

                            info!("Graceful shutdown completed.");
                            std::process::exit(0);
                        }

                        match state_lock.execute_request(&address.to_string(), query) {
                            Ok(result) => {
                                info!("Successfully executed request.");
                                socket_writer.write_all(QueryResponse::ok(result, &state_lock.database_type).to_string().as_bytes()).await.unwrap();                            
                            }
                            Err(error) => {
                                error!("Failed to execute request. Error: {:?}.", &error);
                                socket_writer.write_all(QueryResponse::error(&error).to_string().as_bytes()).await.unwrap();                            
                            }
                        }
                    }
                    Err(error) => {
                        error!("Failed to parse request. Error: {:?}.", &error);
                        socket_writer.write_all(QueryResponse::error(&error).to_string().as_bytes()).await.unwrap();                            
                    }
                }

                line.clear();
            }
        }
    }
}



pub async fn serve(state: State) {
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "8080";

    let listener = TcpListener::bind(format!("{}:{}", HOST, PORT)).await;

    match listener {
        Ok(listener) => {
            info!("Started CachewDB server. Listening on {}:{}.", HOST, PORT);
            let state: Arc<Mutex<State>> = Arc::new(Mutex::new(state));
            let state_clone = Arc::clone(&state);

            let mut signal = signal(SignalKind::interrupt()).expect("Failed to create SIGINT signal handler.");
            tokio::spawn(async move {
                signal.recv().await;

                println!();
                warn!("Received SIGINT signal (Ctrl+C). Shutting down gracefully...");

                // notify all client handlers to send shutdown message to their client
                state_clone.lock().await.signal_shutdown().await;

                // TODO: add persistance here if needed.

                info!("Graceful shutdown completed.");
                std::process::exit(0);
            });
            
            loop {
                let (socket, address) = listener.accept().await.unwrap();
                info!("Accepted new client ({}).", address);

                let state_clone = Arc::clone(&state);
                let shutdown_rx_clone =  state_clone.lock().await.subscribe_shutdown();
                tokio::spawn(handle_client(socket, address, state_clone, shutdown_rx_clone));
            }
        }
        Err(error) => {
            error!("Failed to start CachewDB server! Error: {}", error);
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_protocol() {
        let request_set = check_protocol("CASP/SET key value/\n");
        assert_eq!(request_set, Ok(()));

        let protocol_validity = check_protocol("");
        assert_eq!(protocol_validity.unwrap_err(), "ProtocolError 'emptyRequest': Can't process empty request.");

        let protocol_validity = check_protocol("CAS/SET key value/\n");
        assert_eq!(protocol_validity.unwrap_err(), format!("ProtocolError 'startMarkerNotFound': Expected request to start with '{}'.", REQUEST_START_MARKER));

        let protocol_validity = check_protocol("CASP/SET key value");
        assert_eq!(protocol_validity.unwrap_err(), format!("ProtocolError 'endMarkerNotFound': Expected request to end with '{}'.", REQUEST_END_MARKER.replace('\n', "\\n")));
    }

}