mod query_parser;
mod server;
mod schemas;
mod database;
mod response;

#[macro_use]
mod errors;

use database::Database;
use schemas::{DatabaseType};


#[tokio::main]
async fn main() {

    let db: Database = Database::new();
    let database_type = DatabaseType::Str;

    server::serve(db, database_type).await;

    /*let query1: Result<QueryRequest<'_>, String> = parser::parse("SET MANY key1 \"habba habbe\", key2 \"jabba jabba\", key3 \"babba babba\"", &database_type);
    match query1 {
        Ok(query) => {
            match db.execute_query(query) {
                Ok(result) => println!("{:?}", QueryResponse::ok(result, &database_type)),
                Err(error) => println!("{:?}", QueryResponse::error(&error))
            }
        }
        Err(error) => { 
            println!("{:?}", QueryResponse::error(&error)) 
        }
    }

    let query2: Result<QueryRequest<'_>, String> = parser::parse("GET MANY key1 key2 key4", &database_type);
    match query2 {
        Ok(query) => {
            match db.execute_query(query) {
                Ok(result) => println!("{:?}", QueryResponse::ok(result, &database_type)),
                Err(error) => println!("{:?}", QueryResponse::error(&error))
            }
        }
        Err(error) => { 
            println!("{:?}", QueryResponse::error(&error)) 
        }
    }*/
}
