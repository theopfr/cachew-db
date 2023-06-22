use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub enum ParserErrorType {
    InvalidRange(usize),
    UnexpectedCharacter(String),
    InvalidKeyValuePair(usize),
    UnknownQueryOperation(String)
}

#[derive(Debug)]
pub struct ParserError {
    pub error_type: ParserErrorType
}


impl Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParserError ");
        match &self.error_type {
            ParserErrorType::InvalidRange(num) => writeln!(f, "'invalidRange': Expected two keys got {}.", num),
            ParserErrorType::UnexpectedCharacter(char) => writeln!(f, "'unexpectedCharacter': '{}' is not allowed in keys.", char),
            ParserErrorType::InvalidKeyValuePair(num) => writeln!(f, "'invalidKeyValuePair': Expected two parameters (key and value), found {}.", num),
            ParserErrorType::UnknownQueryOperation(op) => writeln!(f, "'unknownQueryOperation': Query '{}' not recognized.", op),
        }
    }
}

#[macro_export]
macro_rules! parser_error {
    ($err_type:expr) => {
        Err(
            (Box::new($crate::errors::parser_errors::ParserError {
                error_type: $err_type,
            }) as Box<dyn std::error::Error>).to_string()
        )
    };
}