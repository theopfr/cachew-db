use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub enum AuthenticationErrorType {
    NotAuthenticated,
    AuthenticationFailed,
    //AlreadyAuthenticated
}


#[derive(Debug)]
pub struct AuthenticationError {
    pub error_type: AuthenticationErrorType
}


impl Error for AuthenticationError {}

impl fmt::Display for AuthenticationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AuthenticationError ");
        match &self.error_type {
            AuthenticationErrorType::NotAuthenticated => write!(f, "'notAuthenticated': Please authenticate before executing queries."),
            AuthenticationErrorType::AuthenticationFailed => write!(f, "'authenticationFailed': Wrong password."),
            //AuthenticationErrorType::AlreadyAuthenticated => write!(f, "'alreadyAuthenticated': You are already authenticated."),
        }
    }
}

#[macro_export]
macro_rules! auth_error {
    ($err_type:expr) => {
        return Err(
            (Box::new($crate::errors::authentication_errors::AuthenticationError {
                error_type: $err_type,
            }) as Box<dyn std::error::Error>).to_string()
        )
    };
}

