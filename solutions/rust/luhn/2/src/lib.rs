/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.chars().filter(|c| c.is_ascii_digit()).count() < 2 {
        return false;
    }
    if !code
        .chars()
        .filter(|c| !c.is_whitespace())
        .all(|c| c.is_ascii_digit())
    {
        return false;
    }

    let digits = code.chars().filter_map(|c| c.to_digit(10));
    let mut sum = 0;
    let mut double = false;

    for d in digits.into_iter().rev() {
        let mut val = d;
        if double {
            val *= 2;
            if val > 9 {
                val -= 9;
            }
        }
        sum += val;
        double = !double;
    }
    sum % 10 == 0
}
