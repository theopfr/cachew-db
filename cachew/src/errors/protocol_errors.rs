use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub enum ProtocolErrorType {
    EmptyRequest,
    StartMarkerNotFound(String),
    EndMarkerNotFound(String)
}


#[derive(Debug)]
pub struct ProtocolError {
    pub error_type: ProtocolErrorType
}


impl Error for ProtocolError {}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ProtocolError ");
        match &self.error_type {
            ProtocolErrorType::EmptyRequest => writeln!(f, "'emptyRequest': Can't process empty request."),
            ProtocolErrorType::StartMarkerNotFound(expected_marker) => writeln!(f, "'startMarkerNotFound': Expected request to start with '{}'.", expected_marker),
            ProtocolErrorType::EndMarkerNotFound(expected_marker) => writeln!(f, "'endMarkerNotFound': Expected request to end with '{}'.", expected_marker.replace('\n', "\\n")),
        }
    }
}

#[macro_export]
macro_rules! protocol_error {
    ($err_type:expr) => {
        return Err(
            (Box::new($crate::errors::protocol_errors::ProtocolError {
                error_type: $err_type,
            }) as Box<dyn std::error::Error>).to_string()
        )
    };
}