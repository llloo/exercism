pub fn factors(mut n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    let mut factor = 2;

    while n > 1 {
        while n % factor == 0 {
            result.push(factor);
            n /= factor;
        }
        factor += 1;
    }

    result
}
