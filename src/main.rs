mod cli;

use cli::Cli;

mod p2p_routes;
mod routes;

#[tokio::main]
async fn main() {
    Cli::init().await;
}
