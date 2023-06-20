use std::env;

fn hello(name: &str) {
    println!("Hello, {}", name); // {} is a placeholder for the value of name. you can chain multiple {} to print multiple values
}

fn main() {
    let mut args = env::args(); // mut is needed here because args() returns an iterator and calling nth() on an iterator consumes it
    let arg1 = args.nth(1).unwrap(); // need to unwrap because nth() returns an Option and we don't want to deal with none values for now
    hello(&arg1); // arg1 is of type String and needs to be borrowed to be passed to hello() because hello() expects a &str type
    hello("jojo") // sting literals are of type &str and can be passed to hello() without borrowing
}
