pub fn egg_count(display_value: u32) -> usize {
    format!("{:b}", display_value)
        .chars()
        .filter(|&x| x == '1')
        .count()
}
