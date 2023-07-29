
use colored::Colorize;
use std::io::{self, Write};
use std::error::Error;


pub fn print_response(response: String) {
    println!("\r{}", response);
}

pub fn prompt_command() {
    print!("{} ", "cachew>".bold());
    io::stdout().flush().expect("Failed to flush stdout");
}


pub fn print_warn(message: String) {
    println!("{} {}", "WARN".bold().yellow(), message);
}

pub fn print_info(message: String) {
    println!("{} {}", "INFO".bold().green(), message);
}

pub fn print_error(message: String, error: String) {
    println!("{} {} Error: {}", "ERROR".bold().red(), message, error);
}
