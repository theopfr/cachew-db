mod cli;
mod interface;

use tokio::net::{TcpStream, tcp::ReadHalf};
use std::error::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::io::{self, Write};

use cli::get_cli_arguments;
use interface::*;

fn process_response(response: &str) -> String {
    todo!();
}


fn build_request(input: &str) -> String {
    const CASP_PREFIX: &str = "CASP";
    const CASP_SUFFIX: &str = "\n";

    format!("{}/{}/{}", CASP_PREFIX, input.trim(), CASP_SUFFIX)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let cli_arguments = get_cli_arguments();
    let host_address = format!("{}:{}", cli_arguments.host, cli_arguments.port);

    let mut stream = TcpStream::connect(&host_address).await;

    match stream {
        Ok(stream) => {
            let (reader_stream, mut writer_stream) = stream.into_split(); 
            let mut reader = BufReader::new(reader_stream);

            print_info(format!("Connected to server {}.", &host_address));

            tokio::spawn(async move {
                loop {
                    let mut response = String::new();
                    match reader.read_line(&mut response).await {
                        Ok(0) => { 
                            // ignore for now
                        },
                        Ok(_) => {
                            println!("\r{}\n", response.strip_suffix("\n").unwrap());
                            prompt_command();
                        },
                        Err(error) => {
                            println!("An error occured while trying to read an incoming response. Error {}", error);
                            break;
                        }
                    }
                }
            });

            let mut input_reader = tokio::io::BufReader::new(tokio::io::stdin());
            loop {
                prompt_command();
                
                let mut input = String::new();
                input_reader.read_line(&mut input).await.unwrap();

                if input.len() > 1 {
                    writer_stream.write_all(build_request(&input).as_bytes()).await?;
                    writer_stream.flush().await?;
                }
            }
        },
        Err(error) => {
            print_error(format!("Failed to connect to server {}.", &host_address), format!("{}", error));
        },
    }

    

    Ok(())
}