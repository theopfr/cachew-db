mod query_parser;
mod server;
mod schemas;
mod database;
mod response;

#[macro_use]
//mod errors;
mod errors;

use std::collections::BTreeMap;
use database::Database;
use response::QueryResponse;
use schemas::{QueryRequest, QueryResponseType, DatabaseType};
use query_parser::{parser};


fn execute_query(db: &mut Database, query: QueryRequest) -> Result<QueryResponseType, String> {
    match query {
        QueryRequest::GET(key) => db.get(&key),
        QueryRequest::GET_RANGE { key_lower, key_upper } => db.get_range(key_lower, key_upper),
        QueryRequest::GET_MANY(keys) => db.get_many(keys),
        QueryRequest::DEL(key) => db.del(&key),
        QueryRequest::DEL_RANGE { key_lower, key_upper } => db.del_range(key_lower, key_upper),
        QueryRequest::DEL_MANY(keys) => db.del_many(keys),
        QueryRequest::SET(key_value_pair) => db.set(&key_value_pair.key, key_value_pair.value),
        QueryRequest::SET_MANY(key_value_pairs) => db.set_many(key_value_pairs),
    }
}




#[tokio::main]
async fn main() {

    let mut db: Database = Database::new();
    let database_type = DatabaseType::Float;

    //server::server().await;

    let query1: Result<QueryRequest<'_>, String> = parser::parse("SET MANY key1 0.1, key2 -000.1, key3 10000.11111", &database_type);
    match query1 {
        Ok(query) => {
            match execute_query(&mut db, query) {
                Ok(result) => println!("{:?}", result),
                Err(error) => println!("{:?}", error)
            }
        }
        Err(error) => { 
            println!("{:?}", QueryResponse::error(&error)) 
        }
    }

    let query2: Result<QueryRequest<'_>, String> = parser::parse("GET MANY key1 key2 key3", &database_type);
    match query2 {
        Ok(query) => {
            match execute_query(&mut db, query) {
                Ok(result) => println!("{:?}", QueryResponse::ok(result, database_type)),
                Err(error) => println!("{:?}", QueryResponse::error(&error))
            }
        }
        Err(error) => { 
            println!("{:?}", QueryResponse::error(&error)) 
        }
    }
    
}
