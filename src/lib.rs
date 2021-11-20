use num_bigint::BigUint;
use std::env;

/// Calculates factorial of a number
///
/// ```
/// use factorial::factorial;
/// use num_bigint::BigUint;
///
/// assert_eq!(factorial(0), BigUint::from(1_u32));
/// assert_eq!(factorial(1), BigUint::from(1_u32));
/// assert_eq!(factorial(2), BigUint::from(2_u32));
/// assert_eq!(factorial(3), BigUint::from(6_u32));
/// assert_eq!(factorial(10), BigUint::from(3628800_u32));
/// ```
pub fn factorial(num: u128) -> BigUint {
    (2..=num).product()
}

pub fn parse_num(mut args: env::Args) -> Result<u128, &'static str> {
    args.next();
    let error_string = "factorial: missing number operand";

    match args.next() {
        Some(arg) => match arg.parse() {
            Ok(num) => Ok(num),
            _ => Err(error_string),
        },
        _ => Err(error_string),
    }
}
