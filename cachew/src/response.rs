use serde::{Serialize, Deserialize};
use std::fmt::{self, Write};


use crate::schemas::{ValueType, QueryResponseType, DatabaseType};


impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Str(value) => write!(f, "{}", value),
            ValueType::Int(value) => write!(f, "{}", value),
            ValueType::Float(value) => write!(f, "{}", value),
        }
    }
}



const CASP_PREFIX: &str = "CASP";
const CASP_SUFFIX: &str = "\n";
const CASP_OK_INDENTIFIER: &str = "OK";
const CASP_ERROR_INDENTIFIER: &str = "ERROR";

pub struct QueryResponse { }

impl QueryResponse {
    pub fn ok(response: QueryResponseType, database_type: DatabaseType) -> String {
        match response {
            QueryResponseType::GET_OK(value) => {
                match value {
                    ValueType::Str(_) => format!("{}/{}/{}/GET/'{}'/{}", CASP_PREFIX, CASP_OK_INDENTIFIER, database_type, value, CASP_SUFFIX),
                    ValueType::Int(_) => format!("{}/{}/{}/GET/{}/{}", CASP_PREFIX, CASP_OK_INDENTIFIER, database_type, value, CASP_SUFFIX),
                    ValueType::Float(_) => format!("{}/{}/{}/GET/{}/{}", CASP_PREFIX, CASP_OK_INDENTIFIER, database_type, value, CASP_SUFFIX),
                }
            },
            QueryResponseType::GET_RANGE_OK(values) => {
                let mut casp_response: String = String::new();
                write!(&mut casp_response, "{}/{}/{}/GET MANY/", CASP_PREFIX, CASP_OK_INDENTIFIER, database_type).expect("");

                for (idx, value) in values.iter().enumerate() {
                    write!(&mut casp_response, "{value}").expect("");
                    if idx < values.len() - 1 {
                        write!(&mut casp_response, " ").expect("");
                    }
                }
                write!(&mut casp_response, "/{}", CASP_SUFFIX).expect("");

                casp_response
            },
            QueryResponseType::GET_MANY_OK(values) => {
                let mut casp_response: String = String::new();
                write!(&mut casp_response, "{}/{}/{}/GET RANGE/", CASP_PREFIX, CASP_OK_INDENTIFIER, database_type).expect("");

                for (idx, value) in values.iter().enumerate() {
                    write!(&mut casp_response, "{value}").expect("");
                    if idx < values.len() - 1 {
                        write!(&mut casp_response, " ").expect("");
                    }
                }
                write!(&mut casp_response, "/{}", CASP_SUFFIX).expect("");

                casp_response
            },
            QueryResponseType::DEL_OK => {
                format!("{}/{}/{}/{}", CASP_PREFIX, CASP_OK_INDENTIFIER, "DEL", CASP_SUFFIX)
            },
            QueryResponseType::DEL_RANGE_OK => {
                format!("{}/{}/{}/{}", CASP_PREFIX, CASP_OK_INDENTIFIER, "DEL RANGE", CASP_SUFFIX)
            },
            QueryResponseType::DEL_MANY_OK => {
                format!("{}/{}/{}/{}", CASP_PREFIX, CASP_OK_INDENTIFIER, "DEL MANY", CASP_SUFFIX)
            },
            QueryResponseType::SET_OK => {
                format!("{}/{}/{}/{}", CASP_PREFIX, CASP_OK_INDENTIFIER, "SET", CASP_SUFFIX)
            },
            QueryResponseType::SET_MANY_OK => {
                format!("{}/{}/{}/{}", CASP_PREFIX, CASP_OK_INDENTIFIER, "SET MANY", CASP_SUFFIX)
            },
        }
    }

    pub fn error(error: &str) -> String {
        format!("{}/{}/{}/{}", CASP_PREFIX, CASP_ERROR_INDENTIFIER, error, CASP_SUFFIX)
    }
}

