mod cli;
//mod interface;
mod parser;

use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncBufReadExt, AsyncWriteExt, WriteHalf, BufReader};
use tokio::task;
use tokio::net::{TcpStream, tcp::ReadHalf};
use std::io;

use cli::*;
use parser::{ParsedResponse, ResponseStatus, parse_response};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli_arguments = get_cli_arguments();
    let host_address = format!("{}:{}", cli_arguments.host, cli_arguments.port);

    // connect to CachewDB server
    let mut stream = TcpStream::connect(&host_address).await.map_err(|error| {
        print_error(&format!("Failed to connect to server {}. Error: {}", &host_address, error))
    }).unwrap();

    let (reader, mut writer) = stream.split(); 
    let mut reader: BufReader<ReadHalf> = BufReader::new(reader);

    print_info(&format!("Connected to server {}.", &host_address));

    // string buffer to store responses
    let mut line: String = String::new();

    // make an initial authentication request
    if let Some(password) = cli_arguments.password {
        let auth_request: String = format!("AUTH {}", password);
        let _ = writer.write_all(build_request(&auth_request).as_bytes()).await;

        let _ = reader.read_line(&mut line).await;
        let parsed_response: Result<ParsedResponse, String> = parse_response(&line);
        match parsed_response {
            Ok(response) => {
                if response.status == ResponseStatus::OK {
                    print_info("Authentication successful.");
                }
                else {
                    print_error(&format!("Authentication failed. Error: {}", response.value.unwrap()));
                    std::process::exit(0);
                }
            },
            Err(error) => {
                print_parser_error(&error);
                std::process::exit(0);
            }
        }
    }
    else {
        print_warn("Not authenticated. You won't be able to make requests.");
    }
    
    println!();

    line = String::new();
    // string buffer to store user input
    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin());
    let mut input = String::new();

    loop {
        prompt_command();

        input = String::new();

        tokio::select! {
            _ = reader.read_line(&mut line) => {

                let parsed_response: Result<ParsedResponse, String> = parse_response(&line);
                match &parsed_response {
                    Ok(response) => {
                        print_response(&response);

                        if let Some(command) = &response.command {
                            if command == "SHUTDOWN" {
                                print_warn("Disconnecting.");
                                std::process::exit(0);
                            }
                        }
                    },
                    Err(error) => {
                        print_parser_error(&error);
                    }
                }

                line.clear();
            }
            _ = stdin.read_line(&mut input) => {
                if input.len() > 1 {
                    let cli_command: CliCommand = get_cli_command(&input);

                    match cli_command {
                        CliCommand::DatabaseRequest(request) => {
                            let casp_request = build_request(&request);
                            let _ = writer.write_all(casp_request.as_bytes()).await;
                        }
                        CliCommand::Exit => {
                            print_warn("Stopping client.");
                            std::process::exit(0);
                        }
                        CliCommand::Help => {
                            //print_info("Help on its way.");
                            print_help();
                        }
                        CliCommand::Unknown(command) => {
                            print_error(&format!("Unknown meta command '{}'. Type '$help' to see available commands.", command));
                        }
                    }
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