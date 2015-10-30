#[macro_use] extern crate rustful;

use rustful::{Server, Context, Response, TreeRouter};

fn say_hello(_: Context, res: Response) {
  res.send("Hello World!");
}

fn main() {
  Server {
    host: 3000.into(),
    handlers: insert_routes!{
      TreeRouter::new() => { Get: say_hello }
    },
    ..Server::default()
  }.run().unwrap();
}
