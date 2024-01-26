fn luhn(cc_number: &str) -> bool {
    let mut sum = 0;
    let mut is_even = false;
    for digit in cc_number.chars().rev() {
        if let Some(d) = digit.to_digit(10) {
            sum += if is_even {
                match d * 2 {
                    n if n > 9 => n - 9,
                    n => n,
                }
            } else {
                d
            };
            is_even = !is_even;
        } else if !digit.is_whitespace() {
            return false;
        }
    }
    sum % 10 == 0 && sum > 0
}
fn main() {
    println!("{}", luhn("1935421"));
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_number() {
    assert!(!luhn(""));
    assert!(!luhn("  "));
    assert!(!luhn("   "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
    assert!(!luhn(" 0"));
    assert!(!luhn("0 "));
    assert!(!luhn(" 0 "));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(!luhn("00"));
    assert!(!luhn("0 "));
    assert!(!luhn(" 00"));
    assert!(!luhn(" 00 "));
    assert!(!luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"))
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("7992 7398 7133 1123"))
}
