use clap::Parser;
use colored::Colorize;
use std::io::{self, Write};
use std::error::Error;
use crate::parser::{ParsedResponse, ResponseStatus};


#[derive(Parser, Debug)] 
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(long = "host")]
    pub host: String,

    #[arg(long = "port")]
    pub port: String,

    #[arg(long = "password")]
    pub password: Option<String>,
}


pub fn get_cli_arguments() -> CliArgs {
    CliArgs::parse()
}


pub enum CliCommand {
    DatabaseRequest(String),
    Exit,
    Help,
    Unknown(String)
}


pub fn get_cli_command(input: &str) -> CliCommand {
    const META_COMMAND_PREFIX: &str = "$";

    if input.starts_with(META_COMMAND_PREFIX) {

        let command: &str = input.strip_prefix(META_COMMAND_PREFIX).unwrap().trim();
        match command {
            "exit" => {
                return CliCommand::Exit;
            }
            "help" => {
                return CliCommand::Help;
            }
            _ => {
                return CliCommand::Unknown(command.to_owned())
            }
        }
    }

    CliCommand::DatabaseRequest(input.to_string())
}



pub fn build_request(input: &str) -> String {
    const CASP_PREFIX: &str = "CASP";
    const CASP_SUFFIX: &str = "\n";

    format!("{}/{}/{}", CASP_PREFIX, input.trim(), CASP_SUFFIX)
}



pub fn print_response(response: &ParsedResponse) {
    match response.status {
        ResponseStatus::OK => {
            match &response.value {
                None => println!("\r{} {} {}\n", "server >>".bright_black(), "OK".green(), response.command.as_ref().unwrap().green()),
                Some(value) => println!("\r{} {} {}: {}\n", "server >>".bright_black(), "OK".green(), response.command.as_ref().unwrap().green(), value),
            }
        },
        ResponseStatus::WARN => {
            println!("\r{} {} {}\n", "server >>".bright_black(), "WARN".yellow(), response.command.as_ref().unwrap().green())
        },
        ResponseStatus::ERROR => {
            match &response.value {
                None => println!("\r{}: Failed to parse response.\n", "ERROR".red()),
                Some(value) => println!("\r{} {} {}\n", "server >>".bright_black(), "ERROR".red(), value),
            }
        }
    }

}

pub fn print_parser_error(error: &str) {
    println!("\r{}: {}\n", "ERROR".red(), error);
}

pub fn prompt_command() {
    print!("{} ", "cachew >>".bold());
    io::stdout().flush().expect("Failed to flush stdout");
}

pub fn print_warn(message: &str) {
    println!("{} {}", "WARN".bold().yellow(), message);
}

pub fn print_info(message: &str) {
    println!("{} {}", "INFO".bold().green(), message);
}

pub fn print_error(message: &str) {
    println!("{} {}", "ERROR".bold().red(), message);
}

pub fn print_help() {

    let meta_commands: Vec<(&str, &str)> = vec![
        ("$exit", "Stops the client."),
        ("$help", "Shows this help text."),
    ];

    let database_commands: Vec<(&str, &str)> = vec![
        ("AUTH", "Authenticating on the server."),
        ("PING", "Checks if server is running (responses with 'PONG' if so)."),
        ("SET <key> <value>", "Inserts a new key value pair."),
        ("SET MANY <key1> <value1>, ...", "Inserts multiple key value pairs."),
        ("GET <key>", "Gets a value by key."),
        ("GET MANY <key1>, ... <keyN>", "Gets multiple values by their key."),
        ("GET RANGE <lower key> <upper key>", "Gets values in a range of keys."),
        ("DEL <key>", "Deletes a value by key."),
        ("DEL MANY <key1>, ... <keyN>", "Deletes multiple values by their key."),
        ("DEL RANGE <lower key> <upper key>", "Deletes values in a range of keys."),
        ("EXISTS <key>", "Checks if a key exists or not."),
        ("LEN", "Returns the amount of entries in the database."),
        ("CLEAR", "Deletes all entries in the database."),
        ("SHUTDOWN", "Shuts down the server."),
    ];

    let mut help_text = String::new();

    help_text.push_str("1. Meta commands (prefixed with '$'):\n");
    for (command, description) in meta_commands {
        help_text.push_str(&format!("   {}: {}\n", command.bold(), description))
    }

    help_text.push_str("\n2. Database commands:\n");
    for (command, description) in database_commands {
        help_text.push_str(&format!("   {}: {}\n", command.bold(), description))
    }
    
    println!("\n{} The following is a list of all executable commands:\n{}", "HELP".bold().green(), help_text);
}