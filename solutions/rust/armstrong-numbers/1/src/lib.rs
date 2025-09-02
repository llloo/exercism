pub fn is_armstrong_number(num: u32) -> bool {
    let l = num.to_string().len();
    let m = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(l as u32))
        .sum();
    num == m
}
