use factorial::{factorial, parse_num};
use std::env;
use std::process;

fn main() {
    let number = parse_num(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    println!("{}", factorial(number))
}
