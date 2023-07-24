use std::{str::FromStr, fmt};

use serde::{Serialize, Deserialize};



#[derive(Debug, PartialEq)]
pub struct KeyValuePair {
    pub key: String,
    pub value: ValueType
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
    AUTH(String),
    CLEAR,
    LEN,
    PING
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ValueType {
    Str(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    Json(String)
}

/*impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueType::Str(value) => write!(f, "{}", value),
            ValueType::Int(value) => write!(f, "{}", value),
            ValueType::Float(value) => write!(f, "{}", value),
            ValueType::Bool(value) => write!(f, "{}", value),
            ValueType::Json(value) => write!(f, "{}", value),
        }
    }
}*/


#[derive(Debug, PartialEq)]
pub enum QueryResponseType {
    GET_OK(ValueType),
    GET_RANGE_OK(Vec<ValueType>),
    GET_MANY_OK(Vec<ValueType>),
    DEL_OK,
    DEL_RANGE_OK,
    DEL_MANY_OK,
    SET_OK,
    SET_MANY_OK,
    AUTH_OK,
    CLEAR_OK,
    LEN_OK(usize),
    PING_OK
}


#[derive(Copy, Clone)]
pub enum DatabaseType {
    Str,
    Int,
    Float,
    Bool,
    Json
}

impl fmt::Display for DatabaseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatabaseType::Str => write!(f, "STR"),
            DatabaseType::Int => write!(f, "INT"),
            DatabaseType::Float => write!(f, "FLOAT"),
            DatabaseType::Bool => write!(f, "BOOL"),
            DatabaseType::Json => write!(f, "JSON"),
        }
    }
}
