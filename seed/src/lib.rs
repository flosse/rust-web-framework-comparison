#[macro_use]
extern crate seed;

use seed::{dom_types::Style, prelude::*, App};

#[derive(Clone, Copy, Default)]
struct Model {
    count: i32,
}

#[derive(Clone)]
enum Msg {
    Increment,
    Decrement,
}

fn update(msg: Msg, model: Model) -> Update<Model> {
    use self::Msg::*;
    Render(match msg {
        Increment => Model {
            count: model.count + 1,
            ..model
        },
        Decrement => Model {
            count: model.count - 1,
            ..model
        },
    })
}

fn plural(x: i32) -> &'static str {
    if x == 1 {
        ""
    } else {
        "s"
    }
}

fn view(_state: App<Msg, Model>, model: &Model) -> El<Msg> {
    let text = format!("{} click{} so far", model.count, plural(model.count));
    let outer_style: Style = style! {
            "display" => "flex";
            "flex-direction" => "column";
            "text-align" => "center"
    };

    div![
        outer_style,
        h1!["Seed counter example"],
        div![
            style! {
                "color" => if model.count > 4 {"purple"} else {"gray"};
                "border" => "2px solid #004422"; "padding" => 20
            },
            h3![text],
            button![simple_ev("click", Msg::Increment), "+"],
            button![simple_ev("click", Msg::Decrement), "-"],
            // Optionally-displaying an element
            if model.count >= 10 {
                h2![style! {"padding" => 50}, "Nice!"]
            } else {
                seed::empty()
            }
        ],
    ]
}

#[wasm_bindgen]
pub fn render() {
    App::build(Model::default(), update, view).finish().run();
}
