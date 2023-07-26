mod parser;
mod server;
mod schemas;
mod database;
mod response;
mod state;
mod cli;

#[macro_use]
mod errors;

use log::{info};
use schemas::{DatabaseType};
use state::State;



#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    info!("Starting up CachewDB database.");

    let cli_args: cli::Args = cli::get_cli_args();
    let database_type: DatabaseType = cli::get_database_type(cli_args.database_type);

    info!("Initializing b-tree storage of type '{}'.", database_type);
    let state: State = State::new(database_type, "pwd".to_string());

    server::serve(state).await;
}
