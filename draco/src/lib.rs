use draco::{
    html::{button, div},
    App, Mailbox, Node,
};
use wasm_bindgen::prelude::*;

#[derive(Default)]
pub struct Model {
    count: i32,
}

pub enum Message {
    Increment,
    Decrement,
    Reset,
}

impl App for Model {
    type Message = Message;

    fn update(&mut self, _: &Mailbox<Message>, message: Self::Message) {
        use self::Message::*;
        match message {
            Increment => self.count += 1,
            Decrement => self.count -= 1,
            Reset => self.count = 0,
        }
    }

    fn render(&self) -> Node<Self::Message> {
        div()
            .push(button().push("-").on("click", |_| Message::Decrement))
            .push(self.count)
            .push(button().push("+").on("click", |_| Message::Increment))
            .push(button().push("Reset").on("click", |_| Message::Reset))
            .into()
    }
}

#[wasm_bindgen]
pub fn start() {
    draco::start(
        Model::default(),
        draco::select("#main").expect("main").into(),
    );
}
