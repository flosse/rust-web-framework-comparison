extern crate tiny_http;

use tiny_http::{Response, StatusCode};

fn main() {
  let server = tiny_http::ServerBuilder::new()
    .with_port(3000)
    .build()
    .unwrap();

  loop {
    match server.recv() {
      Err(_) => break,
      Ok(rq) => {
        if rq.url().to_string() == "/" {
          rq.respond(Response::from_string("Hello World".to_string()));
        } else {
          rq.respond(Response::new_empty(StatusCode(404)));
        }
      }
    }
  }
}
