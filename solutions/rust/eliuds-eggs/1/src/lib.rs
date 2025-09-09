pub fn egg_count(display_value: u32) -> usize {
    let mut n = display_value;

    if n <= 1 {
        return n as usize;
    }

    let mut cnt = 1;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n -= 1;
            n /= 2;
            cnt += 1;
        }
    }
    cnt
}
