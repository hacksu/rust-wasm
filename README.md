# Rust WASM

Javascript is great for a lot of things. In particular it's very flexible and easy for the programmer to user, but it's also very challenging to optimize and run quickly a lot of code is already written in other languages

WASM is a binary format that's designed to fix those problems. With it and a bit of work you can compile just about anything into
it. We're be playing with Rust today, because it's one of the easier ones to get working now and because it's just cool (if hard to use sometimes).

## Install

* Install C++ compiler
* Install [Rust](https://www.rust-lang.org/en-US/install.html)
* Set rust to nightly (might be a theme) `rustup default nightly`
* Enable the WASM target `rustup target add wasm32-unknown-unknown`
* `cargo new --bin wasm-maze`
* `cd wasm-maze`
* `cargo install cargo-web`
* Add `rustup target add wasm32-unknown-unknown`

## Make the code

* Open up `Cargo.toml` and add `stdweb = "0.3.0"`
* Open up `src/main.rs`
    #[macro_use]
    extern crate stdweb;

    fn factorial(f: i32) -> i32 {
        if f <= 1 {
            return 1;
        } else {
            return factorial(f-1) * f;
        }
    }


    fn main() {
        stdweb::initialize();
        let f: i32 = factorial(5);

        js! {
            alert(@{f});
        };

        stdweb::event_loop();
    }

## User input

* Now lets handle some input
* This macro lets us not only send values to the javascript, but also functions
    #[macro_use]
    extern crate stdweb;

    use stdweb::web::*;

    fn factorial(f: i32) -> i32 {
        if f <= 1 {
            return 1;
        } else {
            return factorial(f-1) * f;
        }
    }



    fn main() {
        stdweb::initialize();
        document().query_selector("body").unwrap().append_child(&document().create_element("input"));
        let f: i32 = factorial(5);
        
        js! { 
            document.querySelector("input")
                .oninput=(e)=>(@{|s: String|{
                    if let Ok(n) = s.trim().parse::<i32>() {
                        alert(&factorial(n).to_string())
                    }
                }})(e.target.value);
            alert(@{f});
        };




        stdweb::event_loop();
   }

## Yew

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