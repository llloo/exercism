pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut count = 0u64;
    let mut m = n;

    while m != 1 {
        if m % 2 == 0 {
            m /= 2;
        } else {
            m = m * 3 + 1;
        }
        count += 1;
    }

    Some(count)
}
