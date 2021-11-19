use num_bigint::BigUint;
use std::env;

pub fn parse_num(mut args: env::Args) -> Result<u128, &'static str> {
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

/// Calculates factorial of a number
///
/// ```
/// use factorial::factorial;
/// use num_bigint::BigUint;
///
/// assert_eq!(factorial(0), BigUint::from(1_u32));
/// assert_eq!(factorial(1), BigUint::from(1_u32));
/// assert_eq!(factorial(10), BigUint::from(3628800_u32));
/// ```
pub fn factorial(num: u128) -> BigUint {
    (2..=num).product()
}
