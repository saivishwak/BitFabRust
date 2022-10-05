use hyper::{Body, Response};
use router::{Methods, Router};
use std::path::PathBuf;
use tokio::fs::read_to_string;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

pub fn configure(router: &mut Router) {
    router.add_handler(
        String::from(Methods::GET.to_string() + "/"),
        |_, tx: mpsc::Sender<i32>, mut rx: broadcast::Receiver<i32>| async move {
            let contents = read_to_string(PathBuf::from("./static/hello.html")).await;
            let _ = tx.send(100).await;
            while let Ok(x) = rx.recv().await {
                println!("HTTP Channel receive {}", x);
                if x == 1023 {
                    break;
                }
            }
            Response::new(Body::from(contents.unwrap()))
        },
    );

    router.add_handler(
        String::from(Methods::GET.to_string() + "/hello"),
        |_, tx: mpsc::Sender<i32>, mut rx: broadcast::Receiver<i32>| async move {
            let _ = tx.send(100).await;
            let x = rx.recv().await.unwrap();
            Response::new(Body::from(x.to_string()))
        },
    );
}
