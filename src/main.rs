mod cli;

use cli::Cli;

mod routes;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    Cli::init().await;
}
