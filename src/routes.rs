use hyper::{Body, Response};
use router::{Methods, Router};
use std::path::PathBuf;
use tokio::fs::read_to_string;

pub fn configure(router: &mut Router) {
    router.add_handler(
        String::from(Methods::GET.to_string() + "/"),
        |_| async move {
            let contents = read_to_string(PathBuf::from("./static/hello.html")).await;
            Response::new(Body::from(contents.unwrap()))
        },
    );

    router.add_handler(
        String::from(Methods::GET.to_string() + "/hello"),
        |_| async move { Response::new(Body::from("Hello path")) },
    );
}
