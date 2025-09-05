pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut count = 0;
    let mut result = 1;

    while count < n {
        result += 2;
        if is_prime(result) {
            count += 1
        }
    }

    result
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let limit = (n as f32).sqrt() as u32 + 1;
    for i in (3..limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }

    true
}
