use clap::Parser;
use crate::schemas::{DatabaseType};
use std::env::{self, VarError};
use log::{info, warn, error};


#[derive(Parser, Debug)] 
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short = 't', long = "db-type")]
    pub database_type: Option<String>,
}


pub fn get_cli_args() -> Args {
    Args::parse()
}


fn validate_database_type(database_type_arg: String) -> DatabaseType {
    match database_type_arg.as_str() {
        "STR" => DatabaseType::Str,
        "INT" => DatabaseType::Int,
        "FLOAT" => DatabaseType::Float,
        "BOOL" => DatabaseType::Bool,
        "JSON" => DatabaseType::Json,
        _ => {
            let error_message: String = format!("Invalid database type '{}'. Choose one of: STR, INT, FLOAT, BOOL or JSON.", database_type_arg);
            error!("{}", error_message);
            panic!("{}", error_message);
        }
    }
}


pub fn get_database_type(database_type_arg: Option<String>) -> DatabaseType {
    const DATABASE_TYPE_ENV: &str = "CACHEW_DB_TYPE";
    
    let database_type: DatabaseType;
    match database_type_arg {
        Some(database_type_arg) => {
            database_type = validate_database_type(database_type_arg);
        }
        None => {
            let env: Result<String, VarError> = env::var(DATABASE_TYPE_ENV);
            match env {
                Ok(database_type_var) => {
                    database_type = validate_database_type(database_type_var);
                }
                Err(error) => {
                    let error_message: String = format!("Environment variable '{}' is not set and no 'db-type' flag was provided.", DATABASE_TYPE_ENV);
                    error!("{}", error_message);
                    panic!("{}", error_message);
                }
            }
        }
    }   
    database_type
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_database_type() {
        let db_types: &[&str] = &["STR", "INT", "FLOAT", "BOOL", "JSON"];
        let expected_db_types: &[DatabaseType] = &[DatabaseType::Str, DatabaseType::Int, DatabaseType::Float, DatabaseType::Bool, DatabaseType::Json];

        for (idx, db_type) in db_types.iter().enumerate() {
            assert_eq!(validate_database_type(db_type.to_string()), expected_db_types[idx]);
        }
    }

    #[test]
    #[should_panic(expected = "Invalid database type 'WOOL'. Choose one of: STR, INT, FLOAT, BOOL or JSON.")]
    fn test_failed_database_type() {
        validate_database_type("WOOL".to_string());
    }

    #[test]
    fn test_get_database_type() {
        let database_type: DatabaseType = get_database_type(Some("STR".to_string()));
        assert_eq!(database_type, DatabaseType::Str);

        std::env::set_var("CACHEW_DB_TYPE", "INT");

        let database_type: DatabaseType = get_database_type(Some("STR".to_string()));
        assert_eq!(database_type, DatabaseType::Str);

        let database_type: DatabaseType = get_database_type(None);
        assert_eq!(database_type, DatabaseType::Int);
    }

    /*#[test]
    #[should_panic(expected = "Invalid database type 'WOOL'. Choose one of: STR, INT, FLOAT, BOOL or JSON.")]
    fn test_failed_database_type_env() {
        std::env::set_var("CACHEW_DB_TYPE", "WOOL");
        get_database_type(None);
    }

    #[test]
    #[should_panic(expected = "Environment variable 'CACHEW_DB_TYPE' is not set and no 'db-type' flag was provided.")]
    fn test_no_database_type_env() {
        std::env::remove_var("CACHEW_DB_TYPE");
        get_database_type(None);
    }*/

}