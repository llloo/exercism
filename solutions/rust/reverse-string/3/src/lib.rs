pub fn reverse(input: &str) -> String {
    if input.is_empty() {
        return "".to_string();
    }
    let mut s = String::new();
    for c in input.chars() {
        s.insert(0, c);
    }
    s
}
