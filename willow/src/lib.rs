use std::rc::Rc;
use wasm_bindgen::prelude::*;
use willow::{
    attributes::style,
    cmd,
    events::on_click,
    html::{button, div, h1, text, Html},
    Cmd, Program,
};

#[derive(Debug, Clone, Default)]
pub struct Model {
    counter: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Msg {
    Increment,
    Decrement,
}

fn update(msg: &Msg, model: &mut Model) -> Box<Cmd<Msg>> {
    use self::Msg::*;
    match msg {
        Increment => model.counter += 1,
        Decrement => model.counter -= 1,
    }
    Box::new(cmd::None)
}

fn view(model: &Model) -> Html<Msg> {
    let outer_style = vec![
        style("display", "flex"),
        style("flex-direction", "column"),
        style("text-align", "center"),
    ];
    div(
        &outer_style,
        &[
            h1(&[], &[text("Willow counter example")]),
            div(
                &[
                    style("color", if model.counter > 4 { "purple" } else { "gray" }),
                    style("border", "2px solid #004422"),
                    style("padding", "20px"),
                ],
                &[
                    button(&[on_click(Msg::Increment)], &[text("+")]),
                    div(&[], &[text(&model.counter.to_string())]),
                    button(&[on_click(Msg::Decrement)], &[text("-")]),
                ],
            ),
        ],
    )
}

#[wasm_bindgen]
pub fn render() {
    let program = Rc::new(Program::new(view, update, Model::default()));
    program.start();
}
