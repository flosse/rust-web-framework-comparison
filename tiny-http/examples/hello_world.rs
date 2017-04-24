extern crate tiny_http;

use tiny_http::{Server, Response, StatusCode};

fn main() {
  let server = Server::http("0.0.0.0:3000").unwrap();

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
