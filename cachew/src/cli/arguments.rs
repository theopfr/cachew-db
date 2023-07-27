use clap::Parser;
use crate::schemas::{DatabaseType};
use std::env::{self, VarError};
use log::{info, warn, error};

use crate::cli::validators::{validate_database_type, validate_password};

/// Stores the CLI arguments for starting the CachewDB server.
/// 
/// # Fields:
/// * Optional: `database_type`: The value type the b-tree map will store.
/// * Optional: `password`: The password needed to communitcate with the CachewDB server.
/// * Optional: `host`: The address which the CachewDB server is hostet on.
/// * Optional: `port`: The port on which the CachewDB server is accessible.
#[derive(Parser, Debug)] 
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(short = 't', long = "db-type")]
    pub database_type: Option<String>,

    #[arg(short = 'p', long = "password")]
    pub password: Option<String>,

    #[arg(long = "host")]
    pub host: Option<String>,

    #[arg(long = "port")]
    pub port: Option<String>,
}

/// Stores initial arguments needed for starting a CachewDB instance.
/// 
/// # Fields:
/// * `database_type`: The value type the b-tree map will store.
/// * `password`: The password needed to communitcate with the CachewDB server.
/// * `host`: The address which the CachewDB server is hostet on.
/// * `port`: The port on which the CachewDB server is accessible.
pub struct CachewDbArgs {
    pub database_type: DatabaseType,
    pub password: String,
    pub host: String,
    pub port: String
}

/// Gets the initial arguments needed for starting a CachewDB instance from CLI flags or ENV variables.
/// 
/// # Returns:
/// An CachewDbArgs instance storing the database-type, password, host and port.
pub fn get_cachew_db_args() -> CachewDbArgs {
    let cli_args = CliArgs::parse();

    CachewDbArgs {
        database_type: get_argument::<DatabaseType>(cli_args.database_type, "CACHEW_DB_TYPE", validate_database_type, None),
        password: get_argument::<String>(cli_args.password, "CACHEW_DB_PASSWORD", validate_password, None),
        host: get_argument::<String>(cli_args.host, "CACHEW_DB_HOST", |x| x, Some("127.0.0.1".to_string())),
        port: get_argument::<String>(cli_args.port, "CACHEW_DB_PORT", |x| x, Some("8080".to_string())),
    }
}

/// Gets an argument provided by the user using the CLI flag or ENV variable.
/// The generic ``T`` is the type of the provided argument after parsing.
/// 
/// # Arguments:
/// * `cli_argument`: The argument provided by the CLI flag (will be None if none was provided).
/// * `env_var`: The name of the environment variable that can possibly store the argument.
/// * `validator`: A function to validate the argument provided by the user
///
/// # Returns:
/// Returns the valdiated argument provided by the user.
/// If the argument is invalid or none was provided, the program panics.
pub fn get_argument<T>(cli_argument: Option<String>, env_var: &str, validator: fn (String) -> T, default: Option<T>) -> T {    
    match cli_argument {
        Some(cli_argument) => validator(cli_argument),
        None => {
            let env: Result<String, VarError> = env::var(env_var);
            match env {
                Ok(env_argument) => validator(env_argument),
                Err(_) => {

                    match default {
                        Some(default) => default,
                        None => {
                            let error_message: String = format!("Environment variable '{}' is not set and no according flag was provided.", env_var);
                            error!("{}", error_message);
                            panic!("{}", error_message);
                        }
                    }
                }
            }
        }
    }   
}



#[cfg(test)]
mod tests {
    use super::*;

    // TODO test 'get_argument'
    
}