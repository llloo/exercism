use std::str::FromStr;

pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 || len > digits.len() {
        return Vec::new();
    }

    let mut result = Vec::new();
    for i in 0..digits.len() - len + 1 {
        result.push(String::from_str(&digits[i..i + len]).unwrap());
    }

    result
}
