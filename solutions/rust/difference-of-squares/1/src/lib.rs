pub fn square_of_sum(n: u32) -> u32 {
    let n = (1..=n).fold(0, |x, y| x + y);
    n * n
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).fold(0, |x, y| x + y * y)
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
