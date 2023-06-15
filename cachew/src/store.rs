use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use bincode::{serialize, deserialize};
use std::ops::Bound::Included;

use crate::casp_parser::parser;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ValueType {
    Str(String),
    Int(i32),
    Float(f32),
    IntList(Vec<i32>),
    StrList(Vec<String>)
}


#[derive(Debug, PartialEq)]
pub enum QueryResponse {
    GET_OK(ValueType),
    GET_RANGE_OK(Vec<ValueType>),
    GET_MANY_OK(Vec<ValueType>),
    DEL_OK,
    DEL_RANGE_OK,
    DEL_MANY_OK,
    SET_OK,
    SET_MANY_OK,
    ERROR(String)
}

/// Gets a value by its key.
/// 
/// # Arguments:
/// * `store`: The BTreeMap storing the ordered key-value pairs.
/// * `key`: The query key.
/// 
/// # Returns:
/// Either the queried value in a GET_OK enum or an error.
pub fn get(store: &mut BTreeMap<String, Vec<u8>>, key: &str) -> Result<QueryResponse, String> {
    if let Some(serialized_value) = store.get(key) {
        let deserialized_value: ValueType = deserialize(serialized_value).unwrap();
        return Ok(QueryResponse::GET_OK(deserialized_value));
    }

    Err(format!("keyNotFound: Couldn't find key '{}'.", key))
}

/// Gets values from a range of keys.
/// 
/// # Arguments:
/// * `store`: The BTreeMap storing the ordered key-value pairs.
/// * `key_lower`: The lower query key.
/// * `key_upper`: The upper query key.
/// 
/// # Returns:
/// Either the queried values in a GET_RANGE_OK enum or an error.
pub fn get_range(store: &mut BTreeMap<String, Vec<u8>>, key_lower: String, key_upper: String) -> Result<QueryResponse, String> {
    if key_lower > key_upper {
        return Err("invalidRange: Lower key is bigger than upper key.".to_owned());
    }

    let mut values: Vec<ValueType> = Vec::new();
    let range = store.range((Included(key_lower), Included(key_upper)));
    for (_, value) in range {
        values.push(deserialize(value).unwrap());
    }

    Ok(QueryResponse::GET_RANGE_OK(values))
}

/// Gets values from a list of keys.
/// 
/// # Arguments:
/// * `store`: The BTreeMap storing the ordered key-value pairs.
/// * `keys`: The a vector of multiple keys.
/// 
/// # Returns:
/// Either the queried values in a GET_MANY_OK enum or an error.
pub fn get_many(store: &mut BTreeMap<String, Vec<u8>>, keys: Vec<&str>) -> Result<QueryResponse, String> {
    let mut values: Vec<ValueType> = Vec::new();
    for key in keys {
        if let Some(serialized_value) = store.get(key) {
            let deserialized_value: ValueType = deserialize(serialized_value).unwrap();
            values.push(deserialized_value);
        }
        else {
            return Err(format!("keyNotFound: Couldn't find key '{}'.", key));
        }
    }

    Ok(QueryResponse::GET_MANY_OK(values))
}

/// Deletes a value by its key.
/// 
/// # Arguments:
/// * `store`: The BTreeMap storing the ordered key-value pairs.
/// * `key`: The query key.
/// 
/// # Returns:
/// Either a GET_DEL enum on deletion or an error.
pub fn del(store: &mut BTreeMap<String, Vec<u8>>, key: &str) -> Result<QueryResponse, String> {
    let _ = store.remove(key);
    Ok(QueryResponse::DEL_OK)
}

/// Deletes values by a range of keys.
/// 
/// # Arguments:
/// * `store`: The BTreeMap storing the ordered key-value pairs.
/// * `key_lower`: The lower query key.
/// * `key_upper`: The upper query key.
/// 
/// # Returns:
/// Either a DEL_RANGE_OK enum on deletion or an error.
pub fn del_range(store: &mut BTreeMap<String, Vec<u8>>, key_lower: String, key_upper: String) -> Result<QueryResponse, String> {
    if key_lower > key_upper {
        return Err("invalidRange: Lower key is bigger than upper key.".to_owned());
    }

    let keys_to_remove: Vec<String> = store
        .range((Included(key_lower), Included(key_upper)))
        .map(|(key, _)| key.clone())
        .collect();

    for key in keys_to_remove {
        store.remove(&key);
    }

    Ok(QueryResponse::DEL_RANGE_OK)
}

/// Deletes values by a list of keys.
/// 
/// # Arguments:
/// * `store`: The BTreeMap storing the ordered key-value pairs.
/// * `keys`: The a vector of multiple keys.
/// 
/// # Returns:
/// A DEL_MANY_OK enum.
pub fn del_many(store: &mut BTreeMap<String, Vec<u8>>, keys: Vec<&str>) -> Result<QueryResponse, String> {
    for key in keys {
        let _ = store.remove(key);
    }

    Ok(QueryResponse::DEL_MANY_OK)
}

/// Inserts a new key value pair.
/// 
/// # Arguments:
/// * `key`: The new key.
/// * `value`: The value.
/// 
/// # Returns:
/// A SET_OK enum.
pub fn set(store: &mut BTreeMap<String, Vec<u8>>, key: &str, value: ValueType) -> Result<QueryResponse, String> {
    store.insert(key.to_owned(), serialize(&value).unwrap());
    Ok(QueryResponse::SET_OK)
}

/// Inserts multiple key value pairs.
/// 
/// # Arguments:
/// * `key_value_pairs`: A vector of `key_value_pairs`.
/// 
/// # Returns:
/// A SET_OK enum.
pub fn set_many(store: &mut BTreeMap<String, Vec<u8>>, key_value_pairs: Vec<parser::KeyValuePair>) -> Result<QueryResponse, String> {
    for pair in key_value_pairs {
        store.insert(pair.key.to_owned(), serialize(&ValueType::Str(pair.value)).unwrap());
    }
    Ok(QueryResponse::SET_MANY_OK)
}


#[cfg(test)]
mod tests {
    use crate::store;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get() {
        let mut store: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        let _ = store::set(&mut store, "key1", store::ValueType::Int(2));
        let response = store::get(&mut store, "key1");

        assert_eq!(response, Ok(store::QueryResponse::GET_OK(store::ValueType::Int(2))));
    }

    #[test]
    fn test_get_fail() {
        let mut store: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        let _ = store::set(&mut store, "key0", store::ValueType::Str("val0".to_owned()));
        let response = store::get(&mut store, "key2");

        assert_eq!(response, Err("keyNotFound: Couldn't find key 'key2'.".to_owned()));
    }

    #[test]
    fn test_get_many() {
        let mut store: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        for i in 0..5 {
            let _ = store::set(&mut store, &format!("key{}", i), store::ValueType::Str(format!("val{}", i)));
        }
        println!("{:?}", store);
        let response = store::get_many(&mut store, vec!["key1", "key3", "key4"]);

        assert_eq!(response, Ok(QueryResponse::GET_MANY_OK(vec![
            store::ValueType::Str("val1".to_string()),
            store::ValueType::Str("val3".to_string()),
            store::ValueType::Str("val4".to_string()),
        ])));
    }

    #[test]
    fn test_get_many_fail() {
        let mut store: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        for i in 0..5 {
            let _ = store::set(&mut store, &format!("key{}", i), store::ValueType::Str(format!("val{}", i)));
        }
        let response = store::get_many(&mut store, vec!["key4", "key5"]);

        assert_eq!(response, Err("keyNotFound: Couldn't find key 'key5'.".to_owned()));
    }

    #[test]
    fn test_get_range() {
        let mut store: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        for i in 0..5 {
            let _ = store::set(&mut store, &format!("key{}", i), store::ValueType::Str(format!("val{}", i)));
        }
        let response = store::get_range(&mut store, "key2".to_owned(), "key5".to_owned());

        assert_eq!(response, Ok(QueryResponse::GET_RANGE_OK(vec![
            store::ValueType::Str("val2".to_string()),
            store::ValueType::Str("val3".to_string()),
            store::ValueType::Str("val4".to_string()),
        ])));
    }

    #[test]
    fn test_get_range_fail() {
        let mut store: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        for i in 0..5 {
            let _ = store::set(&mut store, &format!("key{}", i), store::ValueType::Str(format!("val{}", i)));
        }
        let response = store::get_range(&mut store, "key5".to_owned(), "key2".to_owned());

        assert_eq!(response, Err("invalidRange: Lower key is bigger than upper key.".to_owned()));
    }

    #[test]
    fn test_get_range_empty() {
        let mut store: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        let _ = store::set(&mut store, "key1", store::ValueType::Str("val1".to_owned()));
        let response = store::get_range(&mut store, "a".to_owned(), "b".to_owned());

        assert_eq!(response, Ok(QueryResponse::GET_RANGE_OK(vec![])));
    }

    #[test]
    fn test_del() {
        let mut store: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        let _ = store::set(&mut store, "key", store::ValueType::Str("val".to_owned()));
        let _ = store::del(&mut store, "key");

        let response = store::get(&mut store, "key");
        assert_eq!(response, Err("keyNotFound: Couldn't find key 'key'.".to_owned()));
    }
    
    #[test]
    fn test_del_range() {
        let mut store: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        for i in 0..5 {
            let _ = store::set(&mut store, &format!("key{}", i), store::ValueType::Str(format!("val{}", i)));
        }
        let _ = store::del_range(&mut store, "key2".to_owned(), "key5".to_owned());
        let response = store::get(&mut store, "key3");

        assert_eq!(response, Err("keyNotFound: Couldn't find key 'key3'.".to_owned()));
    }

    #[test]
    fn test_del_many() {
        let mut store: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        for i in 0..5 {
            let _ = store::set(&mut store, &format!("key{}", i), store::ValueType::Str(format!("val{}", i)));
        }
        let _ = store::del_many(&mut store, vec!["key1", "key4"]);
        let response = store::get(&mut store, "key1");

        assert_eq!(response, Err("keyNotFound: Couldn't find key 'key1'.".to_owned()));
    }

    #[test]
    fn test_set() {
        let mut store: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        let _ = store::set(&mut store, "key", store::ValueType::Str("val".to_owned()));

        let response = store::get(&mut store, "key");
        assert_eq!(response, Ok(store::QueryResponse::GET_OK(store::ValueType::Str("val".to_string()))));
    }

    #[test]
    fn test_set_many() {
        let mut store: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        let _ = store::set_many(&mut store, vec![
            parser::KeyValuePair { key: "key0".to_owned(), value: "val0".to_owned() },
            parser::KeyValuePair { key: "key1".to_owned(), value: "val1".to_owned() },
            parser::KeyValuePair { key: "key2".to_owned(), value: "val2".to_owned() },
            parser::KeyValuePair { key: "key3".to_owned(), value: "val3".to_owned() },
        ]);

        let response = store::get(&mut store, "key2");
        assert_eq!(response, Ok(store::QueryResponse::GET_OK(store::ValueType::Str("val2".to_string()))));
    }
}