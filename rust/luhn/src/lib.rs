/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code_stripped = code.replace(" ", "");
    if code_stripped.len() <= 1 {
        return false;
    }
    let chars: Vec<char> = code_stripped.chars().collect();
    let digits_result: Vec<Option<u32>> = chars.into_iter().map(|c| c.to_digit(10)).collect();
    let len_digits_result = digits_result.len();
    let mut digits: Vec<u32> = digits_result.into_iter().flatten().collect();
    if len_digits_result != digits.len() {
        return false;
    }
    let len = digits.len();
    for i in (0..(len - 1)).step_by(2) {
        let digit_doubled = digits[len - 2 - i] * 2;
        if digit_doubled > 9 {
            digits[len - 2 - i] = digit_doubled - 9;
        } else {
            digits[len - 2 - i] = digit_doubled
        }
    }
    let sum: u32 = digits.iter().sum();
    let divisible_by_10 = sum % 10;
    divisible_by_10 == 0
}
