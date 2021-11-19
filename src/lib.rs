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
/// fn to_big_uint(num: u32) -> BigUint {
///     BigUint::from(num)
/// }
///
/// assert_eq!(factorial(0), to_big_uint(1));
/// assert_eq!(factorial(1), to_big_uint(1));
/// assert_eq!(factorial(10), to_big_uint(3628800));
/// ```
pub fn factorial(x: u128) -> BigUint {
    if x == 0 {
        return BigUint::from(1 as u8);
    }
    let mut n = BigUint::from(1 as u8);
    for i in 2..=x {
        n *= BigUint::from(i);
    }
    n
}
