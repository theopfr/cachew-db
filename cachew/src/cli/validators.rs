use log::{info, warn, error};
use regex::Regex;

use crate::schemas::{DatabaseType};


/// Validates the database-type provided by the user.
/// 
/// # Arguments:
/// * `database_type_arg`: The database-type provided by the user.
/// 
/// # Returns:
/// Returns a DatabaseType instance based on the the database-type the user provided.
/// If the database-type is invalid, the program panics.
pub fn validate_database_type(database_type_arg: String) -> DatabaseType {
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

/// Validates the password strength provided by the user using regex.
/// Criteria: At least 8 characters, 1 uppercase letter, 1 lowercase letter, 1 special character, one digit.
/// 
/// # Arguments:
/// * `password`: The password provided by the user.
/// 
/// # Returns:
/// Returns the same password if validation passes. If not, the program panics.
pub fn validate_password(password: String) -> String {
    let mut is_strong: bool = true;

    // TODO: replace with regex, but it's hard since look-arounds and look-behinds are not supported.

    // check if password is under 8 chars
    if password.len() < 8 {
        is_strong = false;
    }

    // check if it contains at least one uppercase letter
    if !password.chars().any(|c| c.is_uppercase()) {
        is_strong = false;
    }

    // check if it contains at least one lowercase letter
    if !password.chars().any(|c| c.is_lowercase()) {
        is_strong = false;
    }

    // check if it contains at least one digit
    if !password.chars().any(|c| c.is_ascii_digit()) {
        is_strong = false;
    }

    // check if it contains at least one special character (non-alphanumeric)
    if password.chars().all(|c| c.is_alphanumeric()) {
        is_strong = false;
    }

    if !is_strong {
        let error_message: String = "Password too weak.".to_string();
        error!("{}", error_message);
        panic!("{}", error_message);
    }
    password
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
    fn test_validate_wrong_database_type() {
        validate_database_type("WOOL".to_string());
    }

    #[test]
    fn test_validate_correct_password() {
        assert_eq!(validate_password("Ottffss8%".to_string()), "Ottffss8%".to_string());
    }

    #[test]
    #[should_panic(expected = "Password too weak.")]
    fn test_validate_wrong_password() {
        validate_password("p".to_string());
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