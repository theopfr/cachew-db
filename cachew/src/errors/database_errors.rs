use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub enum DatabaseErrorType {
    KeyNotFound(String),
    InvalidRangeOrder,
    WrongValueType
}


#[derive(Debug)]
pub struct DatabaseError {
    pub error_type: DatabaseErrorType
}


impl Error for DatabaseError {}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DatabaseError ");
        match &self.error_type {
            DatabaseErrorType::KeyNotFound(key) => write!(f, "'keyNotFound': The key '{}' doesn't exist.", key),
            DatabaseErrorType::InvalidRangeOrder => write!(f, "'invalidRangeOrder': The lower key is bigger than the upper key."),
            DatabaseErrorType::WrongValueType => write!(f, "'wrongValueType': The value doesn't match the database type."),
        }
    }
}

#[macro_export]
macro_rules! database_error {
    ($err_type:expr) => {
        Err(
            (Box::new($crate::errors::database_errors::DatabaseError {
                error_type: $err_type,
            }) as Box<dyn std::error::Error>).to_string()
        )
    };
}
