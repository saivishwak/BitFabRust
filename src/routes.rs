use hyper::{Body, Response};
use router::{Methods, Router};
use std::fs;
use std::path::PathBuf;

pub fn configure(router: &mut Router) {
    router.add_handler(
        String::from(Methods::GET.to_string() + "/"),
        |_| async move {
            let contents = fs::read_to_string(PathBuf::from("./static/hello.html"));
            Response::new(Body::from(contents.unwrap()))
        },
    );

    router.add_handler(
        String::from(Methods::GET.to_string() + "/hello"),
        |_| async move { Response::new(Body::from("Hello path")) },
    );
}
