#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 { return Err(Error::InvalidInputBase) } else if let Some(max_digit) = number.iter().max().copied() {
        if max_digit > 1 && from_base == 2 {
            return Err(Error::InvalidDigit(max_digit));
        }
        if max_digit == 0 { return Ok(vec![0]) }
    } else if to_base == 0 { return Err(Error::InvalidOutputBase) } else if number.is_empty() { return Ok(vec![0]) }


    let mut total: u32 = 0;
    for (index, item) in number.iter().rev().enumerate() {
        total += item * from_base.pow(index as u32);
    }
    let n = 0;
    let mut result: Vec<u32> = Vec::new();
    result = push_constant_to_result(to_base, &mut total, n, result);
    Ok(result)
}

fn push_constant_to_result(to_base: u32, total: &mut u32, mut n: u32, mut result: Vec<u32>) ->Vec<u32>{
    if *total == 0 {
        if n > 0 {
            n -= 1;
            loop {
                result.push(0);
                if n == 0 { break }
                n -= 1
            }

        }
        return result
    }
    while to_base.pow(n) <= *total {
        n += 1;
    }
    n -= 1;
    let constant = *total / to_base.pow(n);
    *total -= constant * to_base.pow(n);
    result.push(constant);
    push_constant_to_result(to_base, total, n, result)
}

