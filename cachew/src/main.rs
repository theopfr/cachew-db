mod casp_parser;

use std::collections::BTreeMap;
mod store;
use casp_parser::{parser};


fn get_query(storage: &mut BTreeMap<String, Vec<u8>>, query: parser::QueryRequest) -> store::QueryResponse {
    match query {
        parser::QueryRequest::GET(key) => todo!(),//store::get(storage, &key),
        parser::QueryRequest::GET_RANGE { key_lower, key_upper } => todo!(),
        parser::QueryRequest::GET_MANY(_) => todo!(),
        parser::QueryRequest::DEL(_) => todo!(),
        parser::QueryRequest::DEL_RANGE { key_lower, key_upper } => todo!(),
        parser::QueryRequest::DEL_MANY(_) => todo!(),
        parser::QueryRequest::SET(key_value_pair) => todo!(),//store::set(storage, &key_value_pair.key, store::ValueType::Str(key_value_pair.value)),
        parser::QueryRequest::SET_MANY(key_value_pairs) => todo!(),//store::set_many(storage, key_value_pairs),
        parser::QueryRequest::ERROR(_) => todo!(),
    }
}



fn main() {
    let mut storage: BTreeMap<String, Vec<u8>> = BTreeMap::new();

    let query1: Result<parser::QueryRequest<'_>, String> = parser::parse("SET key1 val1");
    match query1 {
        Ok(query) => {
            //println!("{:?}", get_query(&mut storage, query));
            println!("successfully set")
        }
        Err(error) => { 
            println!("Error! {}", error) 
        }
    }

    let query2: Result<parser::QueryRequest<'_>, String> = parser::parse("GET key1");
    match query2 {
        Ok(query) => {
            //println!("{:?}", get_query(&mut storage, query));
            println!("successfully set")
        }
        Err(error) => { 
            println!("Error! {}", error) 
        }
    }
    
}
