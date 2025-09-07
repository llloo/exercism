pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .into_iter()
        .filter(|x| {
            factors
                .iter()
                .any(|&y| if y == 0 { false } else { x % y == 0 })
        })
        .sum()
}
