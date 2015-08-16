#[macro_use] extern crate rustful;

use rustful::{Server, Context, Response, TreeRouter, Handler};

fn say_hello(_: Context, res: Response) {
  res.send("Hello World!");
}

struct HandlerFn(fn(Context, Response));

impl Handler for HandlerFn {
  fn handle_request(&self, context: Context, res: Response) {
    self.0(context, res);
  }
}

fn main() {
  Server {
    host: 3000.into(),
    handlers: insert_routes!{
      TreeRouter::new() => { Get: HandlerFn(say_hello) }
    },
    ..Server::default()
  }.run().unwrap();
}
