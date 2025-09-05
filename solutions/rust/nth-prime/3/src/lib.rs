pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut cache: Vec<u32> = vec![2];

    (3..).step_by(2)
        .filter(|candidate| {
            if cache.iter().take_while(|&&p| (p as u64) * (p as u64) <= (*candidate as u64)).all(|prime| candidate % prime != 0) {
                cache.push(*candidate);
                true
            } else {
                false
            }
        })
        .nth((n - 1) as usize)
        .unwrap()
}
