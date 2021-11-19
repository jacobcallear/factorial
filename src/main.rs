use num_bigint::BigUint;
use std::env;
use std::process;

fn main() {
    let number = parse_num(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    println!("{}", factorial(number))
}

fn parse_num(mut args: env::Args) -> Result<u128, &'static str> {
    args.next();
    let error_string = "factorial: missing number operand";
    let num: u128 = match args.next() {
        Some(arg) => match arg.parse() {
            Ok(num) => num,
            _ => return Err(error_string),
        },
        _ => return Err(error_string),
    };
    Ok(num)
}

fn factorial(x: u128) -> BigUint {
    if x == 0 {
        return BigUint::from(0 as u8);
    }
    let mut n = BigUint::from(1 as u8);
    for i in 2..=x {
        n *= BigUint::from(i);
    }
    n
}
