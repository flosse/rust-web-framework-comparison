extern crate hyper;

use hyper::rt::{self, Future};
use hyper::service::service_fn_ok;
use hyper::{Body, Method, Request, Response, Server, StatusCode};

fn hello(req: Request<Body>) -> Response<Body>{
match (req.method(), req.uri().path()) {
            (&Method::GET, "/") => Response::new(Body::from("Hello World!")),
            _ => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap(),
        }
}

fn main() {
    let server = Server::bind(&([127, 0, 0, 1], 3000).into())
        .serve(|| service_fn_ok(hello))
        .map_err(|e| eprintln!("server error: {}", e));

    rt::run(server);
}
