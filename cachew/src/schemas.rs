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
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ValueType {
    Str(String),
    Int(i32),
    Float(f32),
}


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
}

pub enum DatabaseType {
    Str,
    Int,
    Float
}

impl fmt::Display for DatabaseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
        DatabaseType::Str => write!(f, "STR"),
        DatabaseType::Int => write!(f, "INT"),
        DatabaseType::Float => write!(f, "FLOAT"),
       }
    }
}
