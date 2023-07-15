use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, tcp::ReadHalf}
};

use crate::{protocol_error, schemas::DatabaseType};
use crate::query_parser::parser;
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

    Ok(())
}



pub async fn server() {
    let listener = TcpListener::bind("localhost:8080").await.expect("Failed to start CachewDB server!");

    loop {
        let (mut socket, _address) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let (socket_reader, mut socket_writer) = socket.split();
            
            let mut reader: BufReader<ReadHalf> = BufReader::new(socket_reader);
            let mut line: String = String::new();

            loop {
                line.clear();
                let _byte_amount = reader.read_line(&mut line).await.unwrap();
                
                // check if the incoming message followed the protocol specification
                match check_protocol(&line) {
                    Ok(_) => { }
                    Err(error) => {
                        socket_writer.write_all(error.to_string().as_bytes()).await.unwrap();
                        break;
                    }
                }

                // extract the raw database request form the message and parse it
                let request: &str = line.strip_prefix(REQUEST_START_MARKER).unwrap().strip_suffix(REQUEST_END_MARKER).unwrap().trim();
                let query = parser::parse(request, &DatabaseType::Str);
                
                match query {
                    Ok(_) => {
                        todo!("Query database and return response");
                        //socket_writer.write_all("success\n".to_string().as_bytes()).await.unwrap();
                    }
                    Err(error) => { 
                        socket_writer.write_all(error.to_string().as_bytes()).await.unwrap();
                    }
                }
            }
        });
    }
}

