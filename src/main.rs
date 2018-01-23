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