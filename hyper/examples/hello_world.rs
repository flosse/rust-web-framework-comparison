extern crate hyper;

use hyper::Get;
use hyper::server::{Server, Request, Response};
use hyper::uri::RequestUri::AbsolutePath;

fn hello(req: Request, mut res: Response) {
  match req.uri {
    AbsolutePath(ref path) => match (&req.method, &path[..]) {
      (&Get, "/") => {
        res.send(b"Hello World!").unwrap();
        return;
      },
      _ => {
        *res.status_mut() = hyper::NotFound;
        return;
      }
    },
    _ => {
      return;
    }
  };
}

fn main() {
  Server::http("127.0.0.1:3000")
    .unwrap()
    .handle(hello)
    .unwrap();
}
