use clap::Parser;


#[derive(Parser, Debug)] 
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(long = "host")]
    pub host: String,

    #[arg(long = "port")]
    pub port: String,

    #[arg(long = "password")]
    pub password: Option<String>,
}


pub fn get_cli_arguments() -> CliArgs {
    CliArgs::parse()
}