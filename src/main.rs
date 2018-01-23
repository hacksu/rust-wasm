#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate yew;

use yew::html::*;

use stdweb::web::*;

struct State {
    v: String
}

enum Msg {
    Change(String),
    None
}

struct Context {}

fn factorial(f: u64) -> u64 {
    if f <= 1 {
        return 1;
    } else {
        return factorial(f-1) * f;
    }
}


fn update(_: &mut Context, state: &mut State, msg: Msg) {
    match msg {
        Msg::Change(v) => state.v = v,
        Msg::None => {}
    }
}


fn view(state: &State) -> Html<Msg> {
    html! {
        <div>
            <input oninput=|e: InputData| Msg::Change(e.value), value=&state.v, />
            <div>
            {if let Ok(v) = state.v.parse::<u64>() {factorial(v).to_string()} else {String::from("")}}
            </div>
        </div>
    }
}



fn main() {
    yew::initialize();
    let mut app = App::new();
    let context = Context {};
    let model = State {
        v: String::from("1"),
    };
    app.mount(context, model, update, view);
yew::run_loop();
}