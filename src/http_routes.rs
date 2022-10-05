use hyper::{Body, Response};
use router::{Methods, Router};
use std::path::PathBuf;
use tokio::fs::read_to_string;
use tokio::sync::mpsc;

pub fn configure(router: &mut Router) {
    router.add_handler(
        String::from(Methods::GET.to_string() + "/"),
        |_, tx: mpsc::Sender<i32>| async move {
            let contents = read_to_string(PathBuf::from("./static/hello.html")).await;
            let _ = tx.send(100).await;
            Response::new(Body::from(contents.unwrap()))
        },
    );

    router.add_handler(
        String::from(Methods::GET.to_string() + "/hello"),
        |_, _: mpsc::Sender<i32>| async move { Response::new(Body::from("Hello path")) },
    );
}
