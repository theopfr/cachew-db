
use colored::Colorize;
use std::io::{self, Write};
use std::error::Error;
use crate::parser::{ParsedResponse, ResponseStatus};


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

pub fn print_info(message: String) {
    println!("{} {}", "INFO".bold().green(), message);
}

pub fn print_error(message: String, error: String) {
    println!("{} {} Error: {}", "ERROR".bold().red(), message, error);
}
