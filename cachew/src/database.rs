use std::collections::BTreeMap;
use std::str::FromStr;
use bincode::{serialize, deserialize};
use std::ops::Bound::Included;
use std::sync::{Arc, Mutex};

use crate::schemas::{KeyValuePair, ValueType, QueryResponseType, QueryRequest};
use crate::{database_error};
use crate::errors::database_errors::{DatabaseErrorType};

use std::marker::{Send, Copy};
use std::fmt::{Debug, Display};



pub struct Database {
    storage: BTreeMap<String, Vec<u8>>
}


impl Database {
    
    pub fn new() -> Self {
        let storage: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        Self {
            storage
        }
    }

    /// Gets a value by its key.
    /// 
    /// # Arguments:
    /// * `key`: The query key.
    /// 
    /// # Returns:
    /// Either the queried value in a GET_OK enum or an error.
    pub fn get(&self, key: &str) -> Result<QueryResponseType, String> {
        if let Some(serialized_value) = self.storage.get(key) {
            let deserialized_value: ValueType = deserialize(serialized_value).unwrap();
            return Ok(QueryResponseType::GET_OK(deserialized_value));
        }
    
        database_error!(DatabaseErrorType::KeyNotFound(key.to_string()))
    }

    /// Gets values from a range of keys.
    /// 
    /// # Arguments:
    /// * `key_lower`: The lower query key.
    /// * `key_upper`: The upper query key.
    /// 
    /// # Returns:
    /// Either the queried values in a GET_RANGE_OK enum or an error.
    pub fn get_range(&self, key_lower: String, key_upper: String) -> Result<QueryResponseType, String> {
        if key_lower > key_upper {
            return database_error!(DatabaseErrorType::InvalidRangeOrder);
        }
    
        let mut values: Vec<ValueType> = Vec::new();
        let range = self.storage.range((Included(key_lower), Included(key_upper)));
        for (_, value) in range {
            values.push(deserialize(value).unwrap());
        }
    
        Ok(QueryResponseType::GET_RANGE_OK(values))
    }

    /// Gets values from a list of keys.
    /// 
    /// # Arguments:
    /// * `keys`: The a vector of multiple keys.
    /// 
    /// # Returns:
    /// Either the queried values in a GET_MANY_OK enum or an error.
    pub fn get_many(&self, keys: Vec<&str>) -> Result<QueryResponseType, String> {
        let mut values: Vec<ValueType> = Vec::new();
        for key in keys {
            if let Some(serialized_value) = self.storage.get(key) {
                let deserialized_value: ValueType = deserialize(serialized_value).unwrap();
                values.push(deserialized_value);
            }
            else {
                return database_error!(DatabaseErrorType::KeyNotFound(key.to_string()));
            }
        }

        Ok(QueryResponseType::GET_MANY_OK(values))
    }

    /// Deletes a value by its key.
    /// 
    /// # Arguments:
    /// * `key`: The query key.
    /// 
    /// # Returns:
    /// Either a GET_DEL enum on deletion or an error.
    pub fn del(&mut self, key: &str) -> Result<QueryResponseType, String> {
        let _ = self.storage.remove(key);
        Ok(QueryResponseType::DEL_OK)
    }

    /// Deletes values by a range of keys.
    /// 
    /// # Arguments:
    /// * `key_lower`: The lower query key.
    /// * `key_upper`: The upper query key.
    /// 
    /// # Returns:
    /// Either a DEL_RANGE_OK enum on deletion or an error.
    pub fn del_range(&mut self, key_lower: String, key_upper: String) -> Result<QueryResponseType, String> {
        if key_lower > key_upper {
            return database_error!(DatabaseErrorType::InvalidRangeOrder);
        }
    
        let keys_to_remove: Vec<String> = self.storage
            .range((Included(key_lower), Included(key_upper)))
            .map(|(key, _)| key.clone())
            .collect();
    
        for key in keys_to_remove {
            self.storage.remove(&key);
        }
    
        Ok(QueryResponseType::DEL_RANGE_OK)
    }

    /// Deletes values by a list of keys.
    /// 
    /// # Arguments:
    /// * `store`: The BTreeMap storing the ordered key-value pairs.
    /// * `keys`: The a vector of multiple keys.
    /// 
    /// # Returns:
    /// A DEL_MANY_OK enum.
    pub fn del_many(&mut self, keys: Vec<&str>) -> Result<QueryResponseType, String> {
        for key in keys {
            let _ = self.storage.remove(key);
        }

        Ok(QueryResponseType::DEL_MANY_OK)
    }

    /// Inserts a new key value pair.
    /// 
    /// # Arguments:
    /// * `key`: The new key.
    /// * `value`: The value.
    /// 
    /// # Returns:
    /// A SET_OK enum.
    pub fn set(&mut self, key: &str, value: ValueType) -> Result<QueryResponseType, String> {
        self.storage.insert(key.to_owned(), serialize(&value).unwrap());
        Ok(QueryResponseType::SET_OK)
    }

    /// Inserts multiple key value pairs.
    /// 
    /// # Arguments:
    /// * `key_value_pairs`: A vector of `key_value_pairs`.
    /// 
    /// # Returns:
    /// A SET_OK enum.
    pub fn set_many(&mut self, key_value_pairs: Vec<KeyValuePair>) -> Result<QueryResponseType, String> {
        for pair in key_value_pairs {
            self.storage.insert(pair.key.to_owned(), serialize(&pair.value).unwrap());
        }
        Ok(QueryResponseType::SET_MANY_OK)
    }

    pub fn execute_query(&mut self, query: QueryRequest) -> Result<QueryResponseType, String> {
        match query {
            QueryRequest::GET(key) => self.get(&key),
            QueryRequest::GET_RANGE { key_lower, key_upper } => self.get_range(key_lower, key_upper),
            QueryRequest::GET_MANY(keys) => self.get_many(keys),
            QueryRequest::DEL(key) => self.del(&key),
            QueryRequest::DEL_RANGE { key_lower, key_upper } => self.del_range(key_lower, key_upper),
            QueryRequest::DEL_MANY(keys) => self.del_many(keys),
            QueryRequest::SET(key_value_pair) => self.set(&key_value_pair.key, key_value_pair.value),
            QueryRequest::SET_MANY(key_value_pairs) => self.set_many(key_value_pairs),
        }
    }

}






#[cfg(test)]
mod tests {
    use crate::database;
    use super::*;

    #[test]
    fn test_get() {
        let mut database: database::Database = database::Database::new();

        let _ = database.set("key1", ValueType::Str("val1".to_string()));
        let response = database.get("key1");

        assert_eq!(response, Ok(database::QueryResponseType::GET_OK(ValueType::Str("val1".to_string()))));
    }

    #[test]
    fn test_get_fail() {
        let mut database: database::Database = database::Database::new();
        let _ = database.set("key0", ValueType::Int(23));
        let response = database.get("key2");

        assert_eq!(response, database_error!(DatabaseErrorType::KeyNotFound("key2".to_string())));
    }

    #[test]
    fn test_get_int() {
        let mut database: database::Database = database::Database::new();
        let _ = database.set("key", ValueType::Int(23));
        let response = database.get("key");

        assert_eq!(response, Ok(database::QueryResponseType::GET_OK(ValueType::Int(23))));
    }

    #[test]
    fn test_get_many() {
        let mut database: database::Database = database::Database::new();
        for i in 0..5 {
            let _ = database.set(&format!("key{}", i), ValueType::Str(format!("val{}", i)));
        }
        let response = database.get_many(vec!["key1", "key3", "key4"]);

        assert_eq!(response, Ok(QueryResponseType::GET_MANY_OK(vec![
            ValueType::Str("val1".to_string()), 
            ValueType::Str("val3".to_string()),
            ValueType::Str("val4".to_string())
        ])));
    }

    #[test]
    fn test_get_many_fail() {
        let mut database: database::Database = database::Database::new();
        for i in 0..5 {
            let _ = database.set(&format!("key{}", i), ValueType::Str(format!("val{}", i)));
        }
        let response = database.get_many(vec!["key4", "key5"]);

        assert_eq!(response, database_error!(DatabaseErrorType::KeyNotFound("key5".to_string())));
    }

    #[test]
    fn test_get_range() {
        let mut database: database::Database = database::Database::new();
        for i in 0..5 {
            let _ = database.set(&format!("key{}", i), ValueType::Float(0.75f32));
        }
        let response = database.get_range("key2".to_owned(), "key5".to_owned());

        assert_eq!(response, Ok(QueryResponseType::GET_RANGE_OK(vec![
            ValueType::Float(0.75f32), 
            ValueType::Float(0.75f32),
            ValueType::Float(0.75f32)
        ])))
    }

    #[test]
    fn test_get_range_fail() {
        let mut database: database::Database = database::Database::new();

        for i in 0..5 {
            let _ = database.set(&format!("key{}", i), ValueType::Int(i));
        }
        let response = database.get_range("key5".to_owned(), "key2".to_owned());

        assert_eq!(response, database_error!(DatabaseErrorType::InvalidRangeOrder));
    }

    #[test]
    fn test_get_range_empty() {
        let mut database: database::Database = database::Database::new();
        let _ = database.set("key1", ValueType::Str("val1".to_owned()));
        let response = database.get_range("a".to_owned(), "b".to_owned());

        assert_eq!(response, Ok(QueryResponseType::GET_RANGE_OK(vec![])));
    }

    #[test]
    fn test_del() {
        let mut database: database::Database = database::Database::new();
        let _ = database.set("key", ValueType::Float(-9.99f32));
        let _ = database.del("key");

        let response = database.get("key");
        assert_eq!(response, database_error!(DatabaseErrorType::KeyNotFound("key".to_string())));
    }
    
    #[test]
    fn test_del_range() {
        let mut database: database::Database = database::Database::new();
        for i in 0..5 {
            let _ = database.set(&format!("key{}", i), ValueType::Str(format!("val{}", i)));
        }
        let _ = database.del_range("key2".to_owned(), "key5".to_owned());
        let response = database.get("key3");

        assert_eq!(response, database_error!(DatabaseErrorType::KeyNotFound("key3".to_string())));
    }

    #[test]
    fn test_del_many() {
        let mut database: database::Database = database::Database::new();
        for i in 0..5 {
            let _ = database.set(&format!("key{}", i), ValueType::Int(i));
        }
        let _ = database.del_many(vec!["key1", "key4"]);
        let response = database.get("key1");

        assert_eq!(response, database_error!(DatabaseErrorType::KeyNotFound("key1".to_string())));
    }

    #[test]
    fn test_set() {
        let mut database: database::Database = database::Database::new();
        let _ = database.set("key", ValueType::Str("val".to_string()));

        let response = database.get("key");
        assert_eq!(response, Ok(database::QueryResponseType::GET_OK(ValueType::Str("val".to_string()))));
    }

    #[test]
    fn test_set_many() {
        let mut database: database::Database = database::Database::new();
        let _ = database.set_many(vec![
            KeyValuePair { key: "key0".to_owned(), value: ValueType::Int(1) },
            KeyValuePair { key: "key1".to_owned(), value: ValueType::Int(2) },
            KeyValuePair { key: "key2".to_owned(), value: ValueType::Int(3) },
            KeyValuePair { key: "key3".to_owned(), value: ValueType::Int(4) },
        ]);

        let response = database.get("key2");
        assert_eq!(response, Ok(database::QueryResponseType::GET_OK(ValueType::Int(3))));
    }
}
