use clap::{Parser, Subcommand};
use http_core::{Server};

#[derive(Subcommand)]
pub enum Commands {
    Start {
        #[clap(short, long)]
        address: String,
        
        #[clap(short, long)]
        port: u16
    }
}

#[derive(Parser)]
#[clap(name = "BitFab")]
#[clap(author = "BitFab")]
#[clap(version = "1.0")]
#[clap(about = "Distributed Computing Platform", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>
}

impl Cli {
    pub async fn init(){
        let cmd: Cli = self::Cli::parse();
        match & cmd.command {
            Some(Commands::Start { address, port }) => {
                let server:Server = Server::new(address.to_string(), *port);
                let k = server.start().await;
                println!("{}", server.get_addr_string());
            }
            None => {}
        }
    }
}