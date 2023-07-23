use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub enum ParserErrorType {
    InvalidRange(usize),
    UnexpectedCharacter(String),
    InvalidKeyValuePair(usize),
    UnknownQueryOperation(String),
    WrongValueType(String),
    WrongAuthentication,
    StringQuotesNotFound
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
            ParserErrorType::InvalidRange(num) => write!(f, "'invalidRange': Expected two keys got {}.", num),
            ParserErrorType::UnexpectedCharacter(char) => write!(f, "'unexpectedCharacter': '{}' is not allowed in keys.", char),
            ParserErrorType::InvalidKeyValuePair(num) => write!(f, "'invalidKeyValuePair': Expected two parameters (key and value), found {}.", num),
            ParserErrorType::UnknownQueryOperation(op) => write!(f, "'unknownQueryOperation': Query '{}' not recognized.", op),
            ParserErrorType::WrongValueType(db_type) => write!(f, "'wrongValueType': The value doesn't match the database type '{}'.", db_type),
            ParserErrorType::WrongAuthentication => write!(f, "'wrongAuthentication': Couldn't read password. Expected: 'AUTH <password>'."),
            ParserErrorType::StringQuotesNotFound => write!(f, "'stringQuotesNotFound': Expected double quotes around strings."),
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