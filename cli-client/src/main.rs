mod cli;
mod interface;
mod parser;

use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncBufReadExt, AsyncWriteExt, WriteHalf, BufReader};
use tokio::task;
use tokio::net::{TcpStream, tcp::ReadHalf};
use std::io;

use cli::{CliArgs, get_cli_arguments};
use parser::{ParsedResponse, parse_response};
use interface::*;


fn build_request(input: &str) -> String {
    const CASP_PREFIX: &str = "CASP";
    const CASP_SUFFIX: &str = "\n";

    format!("{}/{}/{}", CASP_PREFIX, input.trim(), CASP_SUFFIX)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli_arguments = get_cli_arguments();
    let host_address = format!("{}:{}", cli_arguments.host, cli_arguments.port);

    // connect to CachewDB server
    let mut stream = TcpStream::connect(&host_address).await.map_err(|error| {
        print_error(format!("Failed to connect to server {}.", &host_address), format!("{}", error))
    }).unwrap();

    let (reader, mut writer) = stream.split(); 
    let mut reader: BufReader<ReadHalf> = BufReader::new(reader);

    print_info(format!("Connected to server {}.", &host_address));

    // string buffer to store responses
    let mut line: String = String::new();
    // string buffer to store user input
    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin());

    // make an initial authentication request
    if let Some(password) = cli_arguments.password {
        let auth_request: String = format!("AUTH {}", password);
        let _ = writer.write_all(build_request(&auth_request).as_bytes()).await;

        let _ = reader.read_line(&mut line).await;
        let parsed_response: Result<ParsedResponse, String> = parse_response(&line);

        match parsed_response {
            Ok(response) => {
                if response.is_ok {
                    print_info("Authentication successful.".to_string());
                }
                else {
                    print_error("Authentication failed.".to_string(), response.value.unwrap());
                    std::process::exit(0);
                }
            },
            Err(error) => {
                print_parser_error(&error);
                std::process::exit(0);
            }
        }
    }
    
    println!();

    // string buffer to store responses
    let mut line: String = String::new();
    // string buffer to store user input
    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin());

    loop {
        prompt_command();        
        let mut input = String::new();

        tokio::select! {
            _ = reader.read_line(&mut line) => {
                let parsed_response: Result<ParsedResponse, String> = parse_response(&line);

                match parsed_response {
                    Ok(response) => {
                        print_response(&response);
                        
                        // TODO: exit on shutdown command
                        /*if let Some(command) = &response.command {
                            if command == "SHUTDOWN" {
                                print_warn("Disconnecting.");
                                std::process::exit(0);
                            }
                        }*/
                    },
                    Err(error) => print_parser_error(&error)
                }
            }
            _ = stdin.read_line(&mut input) => {
                if input.len() > 1 {
                    input = build_request(&input);
                    let _ = writer.write_all(input.as_bytes()).await;
                }
            }
        }
    }

    Ok(())
}



/*
// make an initial authentication request
    let auth_request: String = format!("AUTH {}", cli_arguments.password);
    writer.write_all(auth_request.as_bytes()).await;
    let _ = reader.read_line(&mut line).await;
    
    let parsed_response: Result<ParsedResponse, String> = parse_response(&line);
    match parsed_response {
        Ok(response) => print_response(response),
        Err(error) => print_parser_error(&error)
    }
 */