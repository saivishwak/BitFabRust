use clap::Parser;
use std::sync::Arc;

//Submod files
mod commands;
use super::routes;
mod types;

#[derive(Parser)]
#[clap(name = "BitFab")]
#[clap(author = "BitFab")]
#[clap(version = "1.0")]
#[clap(about = "Distributed Computing Platform", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<types::Commands>,
}

impl Cli {
    pub async fn init() {
        let cmd: Cli = self::Cli::parse();
        match &cmd.command {
            Some(types::Commands::Start { address, port }) => {
                commands::start(address.clone(), port.clone()).await;
            }
            None => {}
        }
    }
}
