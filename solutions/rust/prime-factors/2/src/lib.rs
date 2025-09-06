pub fn factors(mut n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();

    while n % 2 == 0 {
        result.push(2);
        n /= 2;
    }

    let mut factor = 3u64;
    while n > 1 {
        while n % factor == 0 {
            result.push(factor);
            n /= factor;
        }
        factor += 2;
    }

    result
}
