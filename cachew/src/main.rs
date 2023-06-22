mod query_parser;
mod store;
mod server;
mod schemas;

#[macro_use]
//mod errors;
mod errors;

use std::collections::BTreeMap;
use schemas::{QueryRequest, QueryResponse, ValueType};
use query_parser::{parser};


fn execute_query(storage: &mut BTreeMap<String, Vec<u8>>, query: QueryRequest) -> Result<QueryResponse, String> {
    match query {
        QueryRequest::GET(key) => store::get(storage, &key),
        QueryRequest::GET_RANGE { key_lower, key_upper } => store::get_range(storage, key_lower, key_upper),
        QueryRequest::GET_MANY(keys) => store::get_many(storage, keys),
        QueryRequest::DEL(key) => store::del(storage, &key),
        QueryRequest::DEL_RANGE { key_lower, key_upper } => store::del_range(storage, key_lower, key_upper),
        QueryRequest::DEL_MANY(keys) => store::del_many(storage, keys),
        QueryRequest::SET(key_value_pair) => store::set(storage, &key_value_pair.key, ValueType::Str(key_value_pair.value)),
        QueryRequest::SET_MANY(key_value_pairs) => store::set_many(storage, key_value_pairs),
    }
}

#[tokio::main]
async fn main() {

    server::server().await;

    /*let mut storage: BTreeMap<String, Vec<u8>> = BTreeMap::new();

    let query1: Result<QueryRequest<'_>, String> = parser::parse("SET MANY key1 val1, key2 val2, key3 val3");
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

    let query2: Result<QueryRequest<'_>, String> = parser::parse("GET MANY key2 key3 key100");
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
    }*/
    
}
