mod parser;
mod server;
mod schemas;
mod database;
mod response;
mod state;

#[macro_use]
mod errors;

use schemas::{DatabaseType};
use state::State;


#[tokio::main]
async fn main() {

    // TODO handle Clap args and env vars

    let database_type = DatabaseType::Bool;
    let state: State = State::new(database_type, "pwd".to_string());

    server::serve(state).await;
}
