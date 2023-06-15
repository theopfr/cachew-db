use regex::Regex;


use crate::casp_parser::string_utils::{split_whitespaces};


#[derive(Debug, PartialEq)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String
}



#[derive(Debug, PartialEq)]
pub enum QueryRequest<'a> {
    GET(String),
    SET(KeyValuePair),
    SET_MANY(Vec<KeyValuePair>),
    GET_RANGE { key_lower: String, key_upper: String},
    GET_MANY(Vec<&'a str>),
    DEL(String),
    DEL_RANGE { key_lower: String, key_upper: String},
    DEL_MANY(Vec<&'a str>),
    ERROR(String)
}



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
        return Err(format!("invalidRange: Expected two keys got {}.", tokens.len()));
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
    if query.contains(',') {
        return Err("unexpectedCharacter: Character ',' is not allowed in keys.".to_owned());//QueryRequest::ERROR("invalidGetQuery".to_string());
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
        return Err("unexpectedCharacter: Character ' ' is not allowed in keys.".to_owned());//QueryRequest::ERROR("invalidGetQuery".to_string());
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
    if query.contains(',') {
        return Err("unexpectedCharacter: Character ',' is not allowed in keys.".to_owned());//QueryRequest::ERROR("invalidGetQuery".to_string());
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
        return Err("unexpectedCharacter: Character ' ' is not allowed in keys.".to_owned());//QueryRequest::ERROR("invalidGetQuery".to_string());
    }

    Ok(QueryRequest::DEL(query.to_owned()))
}


fn parse_set(query: &str) -> Result<QueryRequest, String> {
    if query.starts_with("MANY ") {
        let pattern = Regex::new(r"\s*,\s*").unwrap();
        let key_value_pairs: Vec<String> = pattern.split(query.strip_prefix("MANY ").unwrap()).map(|s| s.trim().to_owned()).collect();

        let mut parsed_pairs: Vec<KeyValuePair> = vec![];
        for pair in key_value_pairs {
            let parameters: Vec<&str> = split_whitespaces(&pair);
            if parameters.len() != 2 {
                return Err(format!("invalidKeyValuePair: Expected two parameters (key and value), found {}.", parameters.len()));
            }
            parsed_pairs.push(KeyValuePair { key: parameters[0].to_owned(), value: parameters[1].to_owned()})
        }

        return Ok(QueryRequest::SET_MANY(parsed_pairs));
    }

    let parameters: Vec<&str> = split_whitespaces(query);
    if parameters.len() != 2 {
        return Err(format!("invalidKeyValuePair: Expected two parameters (key and value), found {}.", parameters.len()));
    }

    Ok(QueryRequest::SET(KeyValuePair { key: parameters[0].to_owned(), value: parameters[1].to_owned()}))
}



/// Parses a query string into a QueryRequest.
/// 
/// # Arguments:
/// * `query`: The query, e.g "GET key0".
/// 
/// # Returns:
/// An instance of `QueryRequest`, variants: GET, GET_RANGE, GET_MANY, SET, DEL, DEL_RANGE, DEL_MANY, or ERROR (if the parse failed).
pub fn parse(query: &str) -> Result<QueryRequest, String> {
    if query.starts_with("GET ") {
        return parse_get(query.strip_prefix("GET ").unwrap());
    }
    else if query.starts_with("DEL ") {
        return parse_del(query.strip_prefix("DEL ").unwrap());
    }
    else if query.starts_with("SET ") {
        return parse_set(query.strip_prefix("SET ").unwrap());
    }

    Err("invalidQueryOperation".to_owned())
}




#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    // Unit tests for the `parse_get` function:

    #[test]
    fn test_parse_get_key() {
        let get_query = parse_get("key");
        assert_eq!(get_query, Ok(QueryRequest::GET("key".to_string())))
    }

    #[test]
    fn test_parse_get_key_string() {
        let get_query = parse_get("\"key one\"");
        assert_eq!(get_query, Ok(QueryRequest::GET("\"key one\"".to_string())))
    }

    #[test]
    fn test_parse_get_key_fail() {
        let get_query = parse_get("key0 key1");
        assert_eq!(get_query, Err("unexpectedCharacter: Character ' ' is not allowed in keys.".to_owned()))
    }

    #[test]
    fn test_parse_get_range_keys() {
        let get_query = parse_get("RANGE key0 key1");
        assert_eq!(get_query, Ok(QueryRequest::GET_RANGE { key_lower: "key0".to_string(), key_upper: "key1".to_string() }))
    }

    #[test]
    fn test_parse_get_many_keys() {
        let get_query = parse_get("MANY key0 key1 key2");
        assert_eq!(get_query, Ok(QueryRequest::GET_MANY(vec!["key0", "key1", "key2"])))
    }

    // Unit tests for the `parse_ranged_keys` function:

    #[test]
    fn test_parse_ranged_keys() {
        let range_keys = parse_ranged_keys("key0 key1");
        assert_eq!(range_keys, Ok(vec!["key0", "key1"]))
    }

    #[test]
    fn test_parse_ranged_keys_fail() {
        let range_keys = parse_ranged_keys("key0 key1 key2");
        assert_eq!(range_keys, Err("invalidRange: Expected two keys got 3.".to_owned()))
    }

    // Unit tests for the `parse_many_keys` function:

    #[test]
    fn test_parse_many_keys() {
        let range_keys = parse_many_keys("key0 key1 02=2?%3");
        assert_eq!(range_keys, Ok(vec!["key0", "key1", "02=2?%3"]))
    }

    // Unit tests for the `parse_del` function:
    #[test]
    fn test_parse_del_key() {
        let del_query = parse_del("key");
        assert_eq!(del_query, Ok(QueryRequest::DEL("key".to_string())))
    }

    #[test]
    fn test_parse_del_key_string() {
        let del_query = parse_del("\"key one\"");
        assert_eq!(del_query, Ok(QueryRequest::DEL("\"key one\"".to_string())))
    }

    #[test]
    fn test_parse_del_key_fail() {
        let del_query = parse_del("key0 key1");
        assert_eq!(del_query, Err("unexpectedCharacter: Character ' ' is not allowed in keys.".to_owned()))
    }

    #[test]
    fn test_parse_del_range_keys() {
        let del_query = parse_del("RANGE key0 key1");
        assert_eq!(del_query, Ok(QueryRequest::DEL_RANGE { key_lower: "key0".to_string(), key_upper: "key1".to_string() }))
    }

    #[test]
    fn test_parse_del_many_keys() {
        let del_query = parse_del("MANY key0 key1 key2");
        assert_eq!(del_query, Ok(QueryRequest::DEL_MANY(vec!["key0", "key1", "key2"])))
    }

    // Unit tests for the `set` function:
    #[test]
    fn test_parse_set_parameters() {
        let set_query = parse_set("key value");
        assert_eq!(set_query, Ok(QueryRequest::SET(KeyValuePair { key: "key".to_owned(), value: "value".to_owned() })))
    }

    #[test]
    fn test_parse_set_parameters_fail() {
        let set_query = parse_set("key val0 val1");
        assert_eq!(set_query, Err("invalidKeyValuePair: Expected two parameters (key and value), found 3.".to_owned()))
    }

    #[test]
    fn test_parse_set_many_parameters() {
        let get_query = parse_set("MANY key0 val0, key1 val1 ,   key2 val2,key3 val3");
        assert_eq!(get_query, Ok(QueryRequest::SET_MANY(vec![
            KeyValuePair { key: "key0".to_owned(), value: "val0".to_owned() },
            KeyValuePair { key: "key1".to_owned(), value: "val1".to_owned() },
            KeyValuePair { key: "key2".to_owned(), value: "val2".to_owned() },
            KeyValuePair { key: "key3".to_owned(), value: "val3".to_owned() },
        ])))
    }

    #[test]
    fn test_parse_set_many_parameters_fail() {
        let set_query = parse_set("MANY key0 val0, key1,");
        assert_eq!(set_query, Err("invalidKeyValuePair: Expected two parameters (key and value), found 1.".to_owned()))
    }

    // Unit tests for the `parse` function:

    #[test]
    fn test_parse_get() {
        let get_query = parse("GET key");
        assert_eq!(get_query, Ok(QueryRequest::GET("key".to_string())))
    }

    #[test]
    fn test_parse_get_range() {
        let get_query = parse("GET RANGE key0 key1");
        assert_eq!(get_query, Ok(QueryRequest::GET_RANGE { key_lower: "key0".to_string(), key_upper: "key1".to_string() }))
    }

    #[test]
    fn test_parse_get_many() {
        let get_query = parse("GET MANY key0 key1 key2");
        assert_eq!(get_query, Ok(QueryRequest::GET_MANY(vec!["key0", "key1", "key2"])))
    }

    #[test]
    fn test_parse_del() {
        let del_query = parse("DEL key");
        assert_eq!(del_query, Ok(QueryRequest::DEL("key".to_string())))
    }

    #[test]
    fn test_parse_del_range() {
        let del_query = parse("DEL RANGE key0 key1");
        assert_eq!(del_query, Ok(QueryRequest::DEL_RANGE { key_lower: "key0".to_string(), key_upper: "key1".to_string() }))
    }

    #[test]
    fn test_parse_del_many() {
        let del_query = parse("DEL MANY key0 key1 key2");
        assert_eq!(del_query, Ok(QueryRequest::DEL_MANY(vec!["key0", "key1", "key2"])))
    }

    #[test]
    fn test_parse_set() {
        let set_query = parse("SET key0 val1");
        assert_eq!(set_query, Ok(QueryRequest::SET(KeyValuePair { key: "key0".to_owned(), value: "val1".to_owned() } )))
    }

    #[test]
    fn test_parse_set_many() {
        let set_query = parse("SET MANY key0 val0, key1 val1");
        assert_eq!(set_query, Ok(QueryRequest::SET_MANY(vec![
            KeyValuePair { key: "key0".to_owned(), value: "val0".to_owned() },
            KeyValuePair { key: "key1".to_owned(), value: "val1".to_owned() },
        ])))
    }
    
}