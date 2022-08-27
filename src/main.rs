mod cli;
use std::future::Future;

use cli::Cli;
use tokio::{main};

#[tokio::main]
async fn main() {
    Cli::init().await;
}