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
* 