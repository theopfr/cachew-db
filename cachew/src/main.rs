mod casp_parser;

use std::collections::BTreeMap;
mod store;
use casp_parser::{parser};


fn execute_query(storage: &mut BTreeMap<String, Vec<u8>>, query: parser::QueryRequest) -> Result<store::QueryResponse, String> {
    match query {
        parser::QueryRequest::GET(key) => store::get(storage, &key),
        parser::QueryRequest::GET_RANGE { key_lower, key_upper } => store::get_range(storage, key_lower, key_upper),
        parser::QueryRequest::GET_MANY(keys) => store::get_many(storage, keys),
        parser::QueryRequest::DEL(key) => store::del(storage, &key),
        parser::QueryRequest::DEL_RANGE { key_lower, key_upper } => store::del_range(storage, key_lower, key_upper),
        parser::QueryRequest::DEL_MANY(keys) => store::del_many(storage, keys),
        parser::QueryRequest::SET(key_value_pair) => store::set(storage, &key_value_pair.key, store::ValueType::Str(key_value_pair.value)),
        parser::QueryRequest::SET_MANY(key_value_pairs) => store::set_many(storage, key_value_pairs),
        parser::QueryRequest::ERROR(error) => Err(error),
    }
}


fn main() {
    let mut storage: BTreeMap<String, Vec<u8>> = BTreeMap::new();

    let query1: Result<parser::QueryRequest<'_>, String> = parser::parse("SET MANY key1 val1, key2 val2, key3 val3");
    match query1 {
        Ok(query) => {
            match execute_query(&mut storage, query) {
                Ok(result) => println!("{:?}", result),
                Err(error) => println!("{:?}", error)
            }
        }
        Err(error) => { 
            println!("Error! {}", error) 
        }
    }

    let query2: Result<parser::QueryRequest<'_>, String> = parser::parse("GET MANY key2 key3 key100");
    match query2 {
        Ok(query) => {
            match execute_query(&mut storage, query) {
                Ok(result) => println!("{:?}", result),
                Err(error) => println!("{:?}", error)
            }
        }
        Err(error) => { 
            println!("Error! {}", error) 
        }
    }
    
}
