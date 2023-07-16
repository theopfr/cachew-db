use std::sync::{Arc};
use tokio::sync::{Mutex, MutexGuard};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream, tcp::ReadHalf}
};

use crate::response::QueryResponse;
use crate::{protocol_error, schemas::DatabaseType, database::Database};
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



async fn run(mut socket: TcpStream, db_clone: Arc<Mutex<Database>>, dataset_type: DatabaseType) {
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
                socket_writer.write_all(QueryResponse::error(&error).to_string().as_bytes()).await.unwrap();
                break;
            }
        }

        // extract the raw database request form the message and parse it
        let request: &str = line.strip_prefix(REQUEST_START_MARKER).unwrap().strip_suffix(REQUEST_END_MARKER).unwrap().trim();
        let query = parser::parse(request, &dataset_type);
        
        let mut db_lock = db_clone.lock().await;
        match query {
            Ok(query) => {
                match db_lock.execute_query(query) {
                    Ok(result) => {
                        socket_writer.write_all(QueryResponse::ok(result, &dataset_type).to_string().as_bytes()).await.unwrap();                            
                    }
                    Err(error) => {
                        socket_writer.write_all(QueryResponse::error(&error).to_string().as_bytes()).await.unwrap();                            
                    }
                }
            }
            Err(error) => { 
                socket_writer.write_all(error.to_string().as_bytes()).await.unwrap();
            }
        }
    }
}



pub async fn serve(db: Database, database_type: DatabaseType) {
    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to start CachewDB server!");
    println!("Started server...");
    let db: Arc<Mutex<Database>> = Arc::new(Mutex::new(db));

    loop {
        let (socket, _address) = listener.accept().await.unwrap();
        println!("accepted new client");

        let db_clone = Arc::clone(&db);
        tokio::spawn(run(socket, db_clone, database_type));
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_protocol() {
        let request_set = check_protocol("CASP/SET key value/\n");
        assert_eq!(request_set, Ok(()));
    }

    #[test]
    fn test_check_protocol_fail() {
        let protocol_validity = check_protocol("");
        assert_eq!(protocol_validity.unwrap_err(), "ProtocolError 'emptyRequest': Can't process empty request.");

        let protocol_validity = check_protocol("CAS/SET key value/\n");
        assert_eq!(protocol_validity.unwrap_err(), format!("ProtocolError 'startMarkerNotFound': Expected request to start with '{}'.", REQUEST_START_MARKER));

        let protocol_validity = check_protocol("CASP/SET key value");
        assert_eq!(protocol_validity.unwrap_err(), format!("ProtocolError 'endMarkerNotFound': Expected request to end with '{}'.", REQUEST_END_MARKER.replace('\n', "\\n")));
    }
}