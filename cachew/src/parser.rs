use regex::Regex;

use crate::schemas::{QueryRequest, KeyValuePair, ValueType, DatabaseType};
use crate::{parser_error};
use crate::errors::parser_errors::{ParserErrorType};

/// Parses a string expected to be consist of two ordered keys seperated by space.
/// 
/// # Arguments:
/// * `query_keys`: A string containing the two keys.
/// 
/// # Returns:
/// `None` if the string didn't contain two valid keys or the left key is smaller than the right one.
/// Else, a vector of length two containing the keys.
fn parse_ranged_keys(query_keys: &str) -> Result<Vec<&str>, String> {    
    let tokens: Vec<&str> = split_whitespaces(query_keys);
    if tokens.len() != 2 {
        return parser_error!(ParserErrorType::InvalidRange(tokens.len()));
    }

    Ok(tokens)
}

/// Parses a string expected to be consist of many keys (>1) seperated by space.
/// 
/// # Arguments:
/// * `query_keys`: A string containing the keys.
/// 
/// # Returns:
/// `None` if the string didn't contain valid keys.
/// Else, a vector containing the keys.
fn parse_many_keys(query_keys: &str) -> Result<Vec<&str>, String> {    
    let tokens: Vec<&str> = split_whitespaces(query_keys);

    Ok(tokens)
}

/// Parses the parameters of a GET query.
/// 
/// # Arguments:
/// * `query`: A string containing the parameters of the query, e.g if the query was "GET key" or "GET RANGE a b" the the parameters are everything after "GET ".
/// 
/// # Returns:
/// An instance of `QueryRequest`, variants: GET, GET_RANGE, GET_MANY or ERROR (if the parse failed).
fn parse_get(query: &str) -> Result<QueryRequest, String> {
    if query.contains(',') || query.contains('/') {
        return parser_error!(ParserErrorType::UnexpectedCharacter);
    }

    if query.starts_with("RANGE ") {
        match parse_ranged_keys(query.strip_prefix("RANGE ").unwrap()) {
            Ok(range_keys) => return Ok(QueryRequest::GET_RANGE { key_lower: range_keys[0].to_owned(), key_upper: range_keys[1].to_owned() }),
            Err(error) => return Err(error),
        }
    }

    if query.starts_with("MANY ") {
        match parse_many_keys(query.strip_prefix("MANY ").unwrap()) {
            Ok(keys) => return Ok(QueryRequest::GET_MANY(keys)),
            Err(error) => return Err(error),
        }
    }

    if split_whitespaces(query).len() > 1 {
        return parser_error!(ParserErrorType::UnexpectedCharacter);
    }

    Ok(QueryRequest::GET(query.to_owned()))
}


/// Parses the parameters of a DEL query.
/// 
/// # Arguments:
/// * `query`: A string containing the parameters of the query, e.g if the query was "DEL key" or "DEL RANGE a b" the the parameters are everything after "DEL ".
/// 
/// # Returns:
/// An instance of `QueryRequest`, variants: DEL, DEL_RANGE, DEL_MANY or ERROR (if the parse failed).
fn parse_del(query: &str) -> Result<QueryRequest, String> {
    if query.contains(',') || query.contains('/') {
        return parser_error!(ParserErrorType::UnexpectedCharacter);
    }


    if query.starts_with("RANGE ") {
        match parse_ranged_keys(query.strip_prefix("RANGE ").unwrap()) {
            Ok(range_keys) => return Ok(QueryRequest::DEL_RANGE { key_lower: range_keys[0].to_owned(), key_upper: range_keys[1].to_owned() }),
            Err(error) => return Err(error),
        }
    }

    if query.starts_with("MANY ") {
        match parse_many_keys(query.strip_prefix("MANY ").unwrap()) {
            Ok(keys) => return Ok(QueryRequest::DEL_MANY(keys)),
            Err(error) => return Err(error),
        }
    }

    if split_whitespaces(query).len() > 1 {
        return parser_error!(ParserErrorType::UnexpectedCharacter);
    }

    Ok(QueryRequest::DEL(query.to_owned()))
}



fn parse_set_value(value_query_parameter: &str, database_type: &DatabaseType) -> Result<ValueType, String> {
    match database_type {
        DatabaseType::Str => {
            if !(value_query_parameter.starts_with('"') && value_query_parameter.ends_with('"') ){
                return parser_error!(ParserErrorType::StringQuotesNotFound)
            }
            let value = value_query_parameter.strip_prefix('"').unwrap().strip_suffix('"').unwrap();

            let parsed_value: String = match value.parse::<String>() {
                Ok(parsed) => parsed,
                Err(_) => return parser_error!(ParserErrorType::WrongValueType(database_type.to_string()))
            };
            Ok(ValueType::Str(parsed_value))
        }
        DatabaseType::Int => {
            let parsed_value: i32 = match value_query_parameter.parse::<i32>() {
                Ok(parsed) => parsed,
                Err(_) => return parser_error!(ParserErrorType::WrongValueType(database_type.to_string()))
            };
            Ok(ValueType::Int(parsed_value))
        },
        DatabaseType::Float => {
            let parsed_value: f32 = match value_query_parameter.parse::<f32>() {
                Ok(parsed) => parsed,
                Err(_) => return parser_error!(ParserErrorType::WrongValueType(database_type.to_string()))
            };
            Ok(ValueType::Float(parsed_value))
        },
        DatabaseType::Bool => {
            let parsed_value: bool = match value_query_parameter.parse::<bool>() {
                Ok(parsed) => parsed,
                Err(_) => return parser_error!(ParserErrorType::WrongValueType(database_type.to_string()))
            };
            Ok(ValueType::Bool(parsed_value))
        },
        DatabaseType::Json => {
            if !(value_query_parameter.starts_with('"') && value_query_parameter.ends_with('"') ){
                return parser_error!(ParserErrorType::StringQuotesNotFound)
            }
            let value = value_query_parameter.strip_prefix('"').unwrap().strip_suffix('"').unwrap();

            let parsed_value: String = match value.parse::<String>() {
                Ok(parsed) => parsed,
                Err(_) => return parser_error!(ParserErrorType::WrongValueType(database_type.to_string()))
            };
            Ok(ValueType::Json(parsed_value))
        }
    }

}


fn parse_set<'a>(query: &'a str, database_type: &DatabaseType) -> Result<QueryRequest<'a>, String> {
    if query.starts_with("MANY ") {
        let pattern = Regex::new(r"\s*,\s*").unwrap();
        let key_value_pairs: Vec<String> = pattern.split(query.strip_prefix("MANY ").unwrap()).map(|s| s.trim().to_owned()).collect();

        let mut parsed_pairs: Vec<KeyValuePair> = vec![];
        for pair in key_value_pairs {
            let parameters: Vec<&str> = split_whitespaces(&pair);
            if parameters.len() != 2 {
                return parser_error!(ParserErrorType::InvalidKeyValuePair(parameters.len()));
            }

            if parameters[0].contains('/') || parameters[0].contains(',') {
                return parser_error!(ParserErrorType::UnexpectedCharacter);
            }

            let parsed_value: Result<ValueType, String> = parse_set_value(parameters[1], database_type);
            match parsed_value {
                Ok(value) => parsed_pairs.push(KeyValuePair { key: parameters[0].to_owned(), value}),
                Err(err) => return Err(err),
            }
        }

        return Ok(QueryRequest::SET_MANY(parsed_pairs));
    }

    // check if the query consists just of a key and value
    let parameters: Vec<&str> = split_whitespaces(query);
    if parameters.len() != 2 {
        return parser_error!(ParserErrorType::InvalidKeyValuePair(parameters.len()));
    }

    if parameters[0].contains('/') || parameters[0].contains(',') {
        return parser_error!(ParserErrorType::UnexpectedCharacter);
    }

    // parse value into the right value type
    let parsed_value: Result<ValueType, String> = parse_set_value(parameters[1], database_type);
    match parsed_value {
        Ok(value) => Ok(QueryRequest::SET(KeyValuePair { key: parameters[0].to_owned(), value})),
        Err(err) => Err(err),
    }
}


fn parse_auth(password: &str) -> Result<QueryRequest, String> {
    if password.contains(' ') {
        return parser_error!(ParserErrorType::WrongAuthentication);
    }

    return Ok(QueryRequest::AUTH(password.to_owned()));
}


fn parse_exists(query: &str) -> Result<QueryRequest, String> {
    if query.contains(',') || query.contains('/') {
        return parser_error!(ParserErrorType::UnexpectedCharacter);
    }
    if split_whitespaces(query).len() > 1 {
        return parser_error!(ParserErrorType::UnexpectedCharacter);
    }

    return Ok(QueryRequest::EXISTS(query.to_owned()));
}


fn parse_single_command<'a>(request: &'a str, expected_command: &'a str, query_request: QueryRequest<'a>) -> Result<QueryRequest<'a>, String> {
    if request.len() > expected_command.len() {
        return parser_error!(ParserErrorType::UnexpectedParameters(expected_command.to_string()));
    }
    Ok(query_request)
}


/// Parses a query string into a QueryRequest.
/// 
/// # Arguments:
/// * `query`: The query, e.g "GET key0".
/// 
/// # Returns:
/// An instance of `QueryRequest`, variants: GET, GET_RANGE, GET_MANY, SET, DEL, DEL_RANGE, DEL_MANY, or ERROR (if the parse failed).
pub fn parse<'a>(request: &'a str, database_type: &DatabaseType) -> Result<QueryRequest<'a>, String> {
    if request.starts_with("GET ") {
        return parse_get(request.strip_prefix("GET ").unwrap());
    }
    else if request.starts_with("DEL ") {
        return parse_del(request.strip_prefix("DEL ").unwrap());
    }
    else if request.starts_with("SET ") {
        return parse_set(request.strip_prefix("SET ").unwrap(), database_type);
    }
    else if request.starts_with("AUTH ") {
        return parse_auth(request.strip_prefix("AUTH ").unwrap());
    }
    else if request.starts_with("CLEAR") {
        return parse_single_command(request, "CLEAR", QueryRequest::CLEAR);
    }
    else if request.starts_with("LEN") {
        return parse_single_command(request, "LEN", QueryRequest::LEN);
    }
    else if request.starts_with("PING") {
        return parse_single_command(request, "PING", QueryRequest::PING);
    }
    else if request.starts_with("EXISTS ") {
        return parse_exists(request.strip_prefix("EXISTS ").unwrap());
    }
    else if request.starts_with("SHUTDOWN") {
        return parse_single_command(request, "SHUTDOWN", QueryRequest::SHUTDOWN);
    }

    parser_error!(ParserErrorType::UnknownQueryOperation(request.to_string()))
}

/// Splits a string at its spaces, unless enclosed by quotes.
/// 
/// # Arguments:
/// * `string`: The string to split.
/// 
/// # Returns:
/// A vector containing the parts of the string (with quotes removed).
pub fn split_whitespaces(string: &str) -> Vec<&str>{
    let regex = Regex::new(r#""[^"]+"|\S+"#).unwrap();
    regex.find_iter(string).map(|m| {
        let matched_string: &str = m.as_str();
        matched_string
    }).collect()
}



#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    // Unit tests for the `parse_get` function:

    #[test]
    fn test_parse_get() {
        let get_query = parse_get("key");
        assert_eq!(get_query, Ok(QueryRequest::GET("key".to_string())));

        let get_query = parse_get("\"key one\"");
        assert_eq!(get_query, Ok(QueryRequest::GET("\"key one\"".to_string())));

        let get_query = parse_get("key0 key1");
        assert_eq!(get_query, parser_error!(ParserErrorType::UnexpectedCharacter))
    }

    #[test]
    fn test_parse_get_range() {
        let get_range_query = parse_get("RANGE key0 key1");
        assert_eq!(get_range_query, Ok(QueryRequest::GET_RANGE { key_lower: "key0".to_string(), key_upper: "key1".to_string() }));

        let get_range_query = parse_get("RANGE key0");
        assert_eq!(get_range_query, parser_error!(ParserErrorType::InvalidRange(1)));
    }

    #[test]
    fn test_parse_get_many() {
        let get_query = parse_get("MANY key0 key1 key2");
        assert_eq!(get_query, Ok(QueryRequest::GET_MANY(vec!["key0", "key1", "key2"])));
    }

    // Unit tests for the `parse_ranged_keys` function:

    #[test]
    fn test_parse_ranged_keys() {
        let range_keys = parse_ranged_keys("key0 key1");
        assert_eq!(range_keys, Ok(vec!["key0", "key1"]));

        let range_keys = parse_ranged_keys("key0 key1 key2");
        assert_eq!(range_keys, parser_error!(ParserErrorType::InvalidRange(3)));
    }

    // Unit tests for the `parse_many_keys` function:

    #[test]
    fn test_parse_many_keys() {
        let range_keys = parse_many_keys("key0 key1 02=2?%3");
        assert_eq!(range_keys, Ok(vec!["key0", "key1", "02=2?%3"]));
    }

    // Unit tests for the `parse_del` function:
    #[test]
    fn test_parse_del() {
        let del_query = parse_del("key");
        assert_eq!(del_query, Ok(QueryRequest::DEL("key".to_string())));

        let del_query = parse_del("\"key one\"");
        assert_eq!(del_query, Ok(QueryRequest::DEL("\"key one\"".to_string())));

        let del_query = parse_del("key0 key1");
        assert_eq!(del_query, parser_error!(ParserErrorType::UnexpectedCharacter))
    }

    #[test]
    fn test_parse_del_range() {
        let del_query = parse_del("RANGE key0 key1");
        assert_eq!(del_query, Ok(QueryRequest::DEL_RANGE { key_lower: "key0".to_string(), key_upper: "key1".to_string() }));

        let del_query = parse_del("RANGE key0");
        assert_eq!(del_query, parser_error!(ParserErrorType::InvalidRange(1)));
    }

    #[test]
    fn test_parse_del_many() {
        let del_query = parse_del("MANY key0 key1 key2");
        assert_eq!(del_query, Ok(QueryRequest::DEL_MANY(vec!["key0", "key1", "key2"])));
    }

    // Unit tests for the `parse_set` function:
    #[test]
    fn test_parse_set() {
        let set_query = parse_set("key \"value\"", &DatabaseType::Str);
        assert_eq!(set_query, Ok(QueryRequest::SET(KeyValuePair { key: "key".to_owned(), value: ValueType::Str("value".to_owned()) })));

        let set_query = parse_set("key \"hello world\"", &DatabaseType::Str);
        assert_eq!(set_query, Ok(QueryRequest::SET(KeyValuePair { key: "key".to_owned(), value: ValueType::Str("hello world".to_owned()) })));

        let set_query = parse_set("key 1", &DatabaseType::Int);
        assert_eq!(set_query, Ok(QueryRequest::SET(KeyValuePair { key: "key".to_owned(), value: ValueType::Int(1) })));

        let set_query = parse_set("key 0.95", &DatabaseType::Float);
        assert_eq!(set_query, Ok(QueryRequest::SET(KeyValuePair { key: "key".to_owned(), value: ValueType::Float(0.95) })));

        let set_query = parse_set("key true", &DatabaseType::Bool);
        assert_eq!(set_query, Ok(QueryRequest::SET(KeyValuePair { key: "key".to_owned(), value: ValueType::Bool(true) })));

        let set_query = parse_set("key false", &DatabaseType::Bool);
        assert_eq!(set_query, Ok(QueryRequest::SET(KeyValuePair { key: "key".to_owned(), value: ValueType::Bool(false) })));

        let set_query = parse_set("key \"{key1: 10, key2: 20}\"", &DatabaseType::Json);
        assert_eq!(set_query, Ok(QueryRequest::SET(KeyValuePair { key: "key".to_owned(), value: ValueType::Json("{key1: 10, key2: 20}".to_owned()) })));

        let set_query = parse_set("key \"val0\" \"val1\"", &DatabaseType::Str);
        assert_eq!(set_query, parser_error!(ParserErrorType::InvalidKeyValuePair(3)));

        let set_query = parse_set("key value", &DatabaseType::Str);
        assert_eq!(set_query, parser_error!(ParserErrorType::StringQuotesNotFound));

        let database_type: DatabaseType = DatabaseType::Float;
        let set_query = parse_set("MANY key notAFloat", &database_type);   
        assert_eq!(set_query, parser_error!(ParserErrorType::WrongValueType(database_type.to_string())));
    }

    #[test]
    fn test_parse_set_many() {
        let get_query = parse_set("MANY key0 \"val0\", key1 \"val1\" ,   key2 \"val2\",key3 \"val3\"", &DatabaseType::Str);
        assert_eq!(get_query, Ok(QueryRequest::SET_MANY(vec![
            KeyValuePair { key: "key0".to_owned(), value: ValueType::Str("val0".to_owned()) },
            KeyValuePair { key: "key1".to_owned(), value: ValueType::Str("val1".to_owned()) },
            KeyValuePair { key: "key2".to_owned(), value: ValueType::Str("val2".to_owned()) },
            KeyValuePair { key: "key3".to_owned(), value: ValueType::Str("val3".to_owned()) },
        ])));

        let get_query = parse_set("MANY key0 1, key1 22, key2 -22, key3 1000", &DatabaseType::Int);
        assert_eq!(get_query, Ok(QueryRequest::SET_MANY(vec![
            KeyValuePair { key: "key0".to_owned(), value: ValueType::Int(1) },
            KeyValuePair { key: "key1".to_owned(), value: ValueType::Int(22) },
            KeyValuePair { key: "key2".to_owned(), value: ValueType::Int(-22) },
            KeyValuePair { key: "key3".to_owned(), value: ValueType::Int(1000) },
        ])));

        let set_query = parse_set("MANY key0 \"val0\", key1,", &DatabaseType::Str);   
        assert_eq!(set_query, parser_error!(ParserErrorType::InvalidKeyValuePair(1)));
    }

    // Unit tests for the `parse_auth` function:

    #[test]
    fn test_parse_auth() {
        let auth_request = parse_auth("password123");
        assert_eq!(auth_request, Ok(QueryRequest::AUTH("password123".to_string())));

        let failed_auth_request = parse_auth("pass word 123");
        assert_eq!(failed_auth_request, parser_error!(ParserErrorType::WrongAuthentication));
    }

    // Unit tests for the `parse_single_command` function:

    #[test]
    fn test_parse_single_command() {
        let clear_request = parse_single_command("CLEAR", "CLEAR", QueryRequest::CLEAR);
        assert_eq!(clear_request, Ok(QueryRequest::CLEAR));

        let len_request = parse_single_command("LEN", "LEN", QueryRequest::LEN);
        assert_eq!(len_request, Ok(QueryRequest::LEN));

        let ping_request = parse_single_command("PING", "PING", QueryRequest::PING);
        assert_eq!(ping_request, Ok(QueryRequest::PING));

        let failed_request = parse_single_command("CLEAR NOW", "CLEAR", QueryRequest::PING);
        assert_eq!(failed_request, parser_error!(ParserErrorType::UnexpectedParameters("CLEAR".to_string())));
    }

    // Unit tests for the `parse_exists` function:

    #[test]
    fn test_parse_exists() {
        let exists_request = parse_exists("key");
        assert_eq!(exists_request, Ok(QueryRequest::EXISTS("key".to_string())));

        let failed_exists_request = parse_exists("key1,key2");
        assert_eq!(failed_exists_request, parser_error!(ParserErrorType::UnexpectedCharacter));
    }

    // Unit tests for the `parse` function:

    #[test]
    fn test_parse() {
        let get_query = parse("GET key", &DatabaseType::Str);
        assert_eq!(get_query, Ok(QueryRequest::GET("key".to_string())));

        let get_range_query = parse("GET RANGE key0 key1", &DatabaseType::Int);
        assert_eq!(get_range_query, Ok(QueryRequest::GET_RANGE { key_lower: "key0".to_string(), key_upper: "key1".to_string() }));

        let get_range_query = parse("GET RANGE key0", &DatabaseType::Int);
        assert_eq!(get_range_query, parser_error!(ParserErrorType::InvalidRange(1)));

        let get_many_query = parse("GET MANY key0 key1 key2", &DatabaseType::Float);
        assert_eq!(get_many_query, Ok(QueryRequest::GET_MANY(vec!["key0", "key1", "key2"])));

        let get_query = parse("GET MANY key0, key1, key2", &DatabaseType::Float);
        assert_eq!(get_query, parser_error!(ParserErrorType::UnexpectedCharacter));

        let del_query = parse("DEL key", &DatabaseType::Str);
        assert_eq!(del_query, Ok(QueryRequest::DEL("key".to_string())));

        let del_range_query = parse("DEL RANGE key0 key1", &DatabaseType::Int);
        assert_eq!(del_range_query, Ok(QueryRequest::DEL_RANGE { key_lower: "key0".to_string(), key_upper: "key1".to_string() }));

        let del_many_query = parse("DEL MANY key0 key1 key2", &DatabaseType::Float);
        assert_eq!(del_many_query, Ok(QueryRequest::DEL_MANY(vec!["key0", "key1", "key2"])));

        let set_query = parse("SET key0 \"val1\"", &DatabaseType::Str);
        assert_eq!(set_query, Ok(QueryRequest::SET(KeyValuePair { key: "key0".to_owned(), value: ValueType::Str("val1".to_owned()) } )));

        let set_many_query = parse("SET MANY key0 10, key1 -10", &DatabaseType::Int);
        assert_eq!(set_many_query, Ok(QueryRequest::SET_MANY(vec![
            KeyValuePair { key: "key0".to_owned(), value: ValueType::Int(10) },
            KeyValuePair { key: "key1".to_owned(), value: ValueType::Int(-10) },
        ])));

        let set_query = parse("UNKNOWN key \"val\"", &DatabaseType::Str);
        assert_eq!(set_query, parser_error!(ParserErrorType::UnknownQueryOperation("UNKNOWN key \"val\"".to_string())));
    }

    #[test]
    fn test_split_whitespaces() {
        let split_string: Vec<&str> = split_whitespaces("test test \"in quotes\" test \"in quotes\"");
        assert_eq!(split_string, vec!["test", "test", "\"in quotes\"", "test", "\"in quotes\""]);
    }
}