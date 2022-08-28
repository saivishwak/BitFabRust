mod cli;

use cli::Cli;

#[tokio::main]
async fn main() {
    Cli::init().await;
}
