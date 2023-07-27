mod parser;
mod server;
mod schemas;
mod database;
mod response;
mod state;
mod cli;

#[macro_use]
mod errors;

use state::State;
use cli::arguments::{CachewDbArgs, get_cachew_db_args};
use log::{info};


#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    info!("Starting up CachewDB database.");

    let cachew_db_args: CachewDbArgs = get_cachew_db_args();

    info!("Initializing b-tree storage of type '{}'.", cachew_db_args.database_type);
    let state: State = State::new(cachew_db_args.database_type, cachew_db_args.password);

    server::serve(state, &cachew_db_args.host, &cachew_db_args.port).await;
}
