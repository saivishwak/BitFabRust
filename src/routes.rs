use hyper::{Body, Response};
use router::{Methods, Router};
use std::path::Path;
use std::fs;

pub fn configure(router: &mut Router) {
    router.add_handler(
        String::from(Methods::GET.to_string() + "/"),
        |_| -> Response<Body> {
                let contents = fs::read_to_string(Path::new(
                    "/home/vishwak/Desktop/Test/bitfab/static/hello.html",
                ));
                Response::new(Body::from(contents.unwrap()))
        },
    );

    router.add_handler(
        String::from(Methods::GET.to_string() + "/hello"),
        |_| -> Response<Body> { Response::new(Body::from("Hello path")) },
    );
}
