//! An example application for working with [Gotham](https://gotham.rs).
//!
//! For an example of working with Gotham please see our full
//! [example application](https://github.com/gotham-rs/example-app) which
//! includes usage of the Gotham Router and other advanced Gotham components.

extern crate futures;
extern crate hyper;
extern crate gotham;
extern crate mime;

use hyper::server::Http;
use hyper::{Request, Response, StatusCode};

use gotham::http::response::create_response;
use gotham::state::State;
use gotham::handler::NewHandlerService;

pub fn say_hello(state: State, _req: Request) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((
            String::from("Hello World!").into_bytes(),
            mime::TEXT_PLAIN,
        )),
    );

    (state, res)
}

pub fn main() {
    let addr = "127.0.0.1:7878".parse().unwrap();

    let server = Http::new()
        .bind(&addr, NewHandlerService::new(|| Ok(say_hello)))
        .unwrap();

    println!("Listening on http://{}", server.local_addr().unwrap());

    server.run().unwrap();
}
