use serde::{Serialize, Deserialize};
use std::fmt::{self, Write};


use crate::{schemas::{ValueType, QueryResponseType, DatabaseType}, database};


pub struct QueryResponse { }

impl QueryResponse {

    const CASP_PREFIX: &str = "CASP";
    const CASP_SUFFIX: &str = "\n";
    const CASP_OK_INDENTIFIER: &str = "OK";
    const CASP_ERROR_INDENTIFIER: &str = "ERROR";
    const CASP_WARN_INDENTIFIER: &str = "WARN";

    fn build_ok_response(query_identifier: String, content: Option<String>, database_type: Option<&DatabaseType>) -> String {
        match (content, database_type) {
            (None, None) => format!("{}/{}/{}/{}", Self::CASP_PREFIX, Self::CASP_OK_INDENTIFIER, query_identifier, Self::CASP_SUFFIX),
            (Some(content), None) => format!("{}/{}/{}/{}/{}", Self::CASP_PREFIX, Self::CASP_OK_INDENTIFIER, query_identifier, content, Self::CASP_SUFFIX),
            (Some(content), Some(database_type)) => format!("{}/{}/{}/{}/{}/{}", Self::CASP_PREFIX, Self::CASP_OK_INDENTIFIER, database_type, query_identifier, content, Self::CASP_SUFFIX),
            (None, Some(database_type)) => format!("{}/{}/{}/{}/{}", Self::CASP_PREFIX, Self::CASP_OK_INDENTIFIER, database_type, query_identifier, Self::CASP_SUFFIX),
        }
    }

    fn handle_value_types(value: &ValueType) -> String {
        match value {
            ValueType::Str(value) => format!("\"{}\"", value),
            ValueType::Int(value) => format!("{}", value),
            ValueType::Float(value) => format!("{}", value),
            ValueType::Bool(value) => format!("{}", value),
            ValueType::Json(value) => value.to_string(),
        }
    }

    pub fn ok(response: QueryResponseType, database_type: &DatabaseType) -> String {
        match response {
            QueryResponseType::GET_OK(value) => {
                Self::build_ok_response("GET".to_string(), Some(Self::handle_value_types(&value)), Some(database_type))
            },
            QueryResponseType::GET_RANGE_OK(values) => {
                let mut content: String = String::new();
                for (idx, value) in values.iter().enumerate() {
                    write!(&mut content, "{}", Self::handle_value_types(value)).expect("");
                    if idx < values.len() - 1 {
                        write!(&mut content, ",").expect("");
                    }
                }
                Self::build_ok_response("GET RANGE".to_string(), Some(content), Some(database_type))

            },
            QueryResponseType::GET_MANY_OK(values) => {
                let mut content: String = String::new();
                for (idx, value) in values.iter().enumerate() {
                    write!(&mut content, "{}", Self::handle_value_types(value)).expect("");
                    if idx < values.len() - 1 {
                        write!(&mut content, ",").expect("");
                    }
                }
                Self::build_ok_response("GET MANY".to_string(), Some(content), Some(database_type))
            },
            QueryResponseType::DEL_OK => {
                Self::build_ok_response("DEL".to_string(), None, None)
            },
            QueryResponseType::DEL_RANGE_OK => {
                Self::build_ok_response("DEL RANGE".to_string(), None, None)
            },
            QueryResponseType::DEL_MANY_OK => {
                Self::build_ok_response("DEL MANY".to_string(), None, None)
            },
            QueryResponseType::SET_OK => {
                Self::build_ok_response("SET".to_string(), None, None)
            },
            QueryResponseType::SET_MANY_OK => {
                Self::build_ok_response("SET MANY".to_string(), None, None)
            },
            QueryResponseType::AUTH_OK => {
                Self::build_ok_response("AUTH".to_string(), None, None)
            }
            QueryResponseType::CLEAR_OK => {
                Self::build_ok_response("CLEAR".to_string(), None, None)
            },
            QueryResponseType::LEN_OK(length) => {
                Self::build_ok_response("LEN".to_string(), Some(length.to_string()), None)
            },
            QueryResponseType::PING_OK => {
                Self::build_ok_response("PING".to_string(), Some("PONG".to_string()), None)
            },
            QueryResponseType::EXISTS_OK(exists) => {
                Self::build_ok_response("EXISTS".to_string(), Some(exists.to_string()), None)
            }
            QueryResponseType::SHUTDOWN_OK => {
                Self::build_ok_response("SHUTDOWN".to_string(), None, None)
            }
        }
    }

    pub fn error(error: &str) -> String {
        format!("{}/{}/{}/{}", Self::CASP_PREFIX, Self::CASP_ERROR_INDENTIFIER, error, Self::CASP_SUFFIX)
    }

    pub fn warn(message: &str) -> String {
        format!("{}/{}/{}/{}", Self::CASP_PREFIX, Self::CASP_WARN_INDENTIFIER, message, Self::CASP_SUFFIX)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_string() {
        let response = QueryResponse::ok(
            QueryResponseType::GET_OK(ValueType::Str("value".to_string())),
            &DatabaseType::Str
        );
        assert_eq!(response, "CASP/OK/STR/GET/\"value\"/\n")
    }

    #[test]
    fn test_get_int() {
        let response = QueryResponse::ok(
            QueryResponseType::GET_OK(ValueType::Int(-100)),
            &DatabaseType::Int
        );
        assert_eq!(response, "CASP/OK/INT/GET/-100/\n")
    }

    #[test]
    fn test_get_float() {
        let response = QueryResponse::ok(
            QueryResponseType::GET_OK(ValueType::Float(0.01)),
            &DatabaseType::Float
        );
        assert_eq!(response, "CASP/OK/FLOAT/GET/0.01/\n")
    }

    #[test]
    fn test_get_range_string() {
        let response = QueryResponse::ok(
            QueryResponseType::GET_RANGE_OK(vec![ValueType::Str("value1".to_string()), ValueType::Str("value2".to_string())]),
            &DatabaseType::Str
        );
        assert_eq!(response, "CASP/OK/STR/GET RANGE/\"value1\",\"value2\"/\n")
    }

    #[test]
    fn test_get_range_float() {
        let response = QueryResponse::ok(
            QueryResponseType::GET_RANGE_OK(vec![ValueType::Float(0.01), ValueType::Float(0.02), ValueType::Float(0.03)]),
            &DatabaseType::Float
        );
        assert_eq!(response, "CASP/OK/FLOAT/GET RANGE/0.01,0.02,0.03/\n")
    }

    #[test]
    fn test_get_many_string() {
        let response = QueryResponse::ok(
            QueryResponseType::GET_MANY_OK(vec![ValueType::Str("value1".to_string()), ValueType::Str("value2".to_string())]),
            &DatabaseType::Str
        );
        assert_eq!(response, "CASP/OK/STR/GET MANY/\"value1\",\"value2\"/\n")
    }

    #[test]
    fn test_del() {
        let response = QueryResponse::ok(
            QueryResponseType::DEL_OK,
            &DatabaseType::Float
        );
        assert_eq!(response, "CASP/OK/DEL/\n")
    }

    #[test]
    fn test_del_range() {
        let response = QueryResponse::ok(
            QueryResponseType::DEL_RANGE_OK,
            &DatabaseType::Str
        );
        assert_eq!(response, "CASP/OK/DEL RANGE/\n")
    }

    #[test]
    fn test_del_many() {
        let response = QueryResponse::ok(
            QueryResponseType::DEL_MANY_OK,
            &DatabaseType::Str
        );
        assert_eq!(response, "CASP/OK/DEL MANY/\n")
    }

    #[test]
    fn test_set() {
        let response = QueryResponse::ok(
            QueryResponseType::SET_OK,
            &DatabaseType::Int
        );
        assert_eq!(response, "CASP/OK/SET/\n")
    }

    #[test]
    fn test_set_many() {
        let response = QueryResponse::ok(
            QueryResponseType::SET_MANY_OK,
            &DatabaseType::Str
        );
        assert_eq!(response, "CASP/OK/SET MANY/\n")
    }

    #[test]
    fn test_auth() {
        let response = QueryResponse::ok(
            QueryResponseType::AUTH_OK,
            &DatabaseType::Float
        );
        assert_eq!(response, "CASP/OK/AUTH/\n")
    }

    #[test]
    fn test_clear() {
        let response = QueryResponse::ok(
            QueryResponseType::CLEAR_OK,
            &DatabaseType::Bool
        );
        assert_eq!(response, "CASP/OK/CLEAR/\n")
    }

    #[test]
    fn test_len() {
        let response = QueryResponse::ok(
            QueryResponseType::LEN_OK(3),
            &DatabaseType::Json
        );
        assert_eq!(response, "CASP/OK/LEN/3/\n")
    }

    #[test]
    fn test_ping() {
        let response = QueryResponse::ok(
            QueryResponseType::PING_OK,
            &DatabaseType::Str
        );
        assert_eq!(response, "CASP/OK/PING/PONG/\n")
    }

    #[test]
    fn test_exists() {
        let response = QueryResponse::ok(
            QueryResponseType::EXISTS_OK(true),
            &DatabaseType::Str
        );
        assert_eq!(response, "CASP/OK/EXISTS/true/\n");

        let response = QueryResponse::ok(
            QueryResponseType::EXISTS_OK(false),
            &DatabaseType::Str
        );
        assert_eq!(response, "CASP/OK/EXISTS/false/\n")
    }

    #[test]
    fn test_error() {
        let response = QueryResponse::error("This is an error message.");
        assert_eq!(response, "CASP/ERROR/This is an error message./\n")
    } 
}