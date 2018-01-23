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
* `cargo install cargo-web`
* Add 