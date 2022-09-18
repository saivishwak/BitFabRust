mod cli;

use cli::Cli;

mod routes;

#[tokio::main]
async fn main() {
    Cli::init().await;
}