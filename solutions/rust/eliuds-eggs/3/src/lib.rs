pub fn egg_count(display_value: u32) -> usize {
    format!("{display_value:b}")
        .chars()
        .filter(|&x| x == '1')
        .count()
}
