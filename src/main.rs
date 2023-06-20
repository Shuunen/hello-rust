use std::env;

fn hello(name: String) {
    println!("Hello, {}", name);
}

fn main() {
    let arg1: Vec<String> = env::args().collect();
    hello(arg1[1].clone());
}
