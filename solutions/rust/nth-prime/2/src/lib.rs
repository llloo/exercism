pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut cache: Vec<u32> = vec![];

    (2..)
        .filter(|candidate| {
            if cache.iter().all(|prime| candidate % prime != 0) {
                cache.push(*candidate);
                true
            } else {
                false
            }
        })
        .nth(n as usize)
        .unwrap()
}
