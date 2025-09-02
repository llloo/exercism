const OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];
pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return Vec::new();
    }

    garden
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.bytes()
                .enumerate()
                .map(|(y, cell)| {
                    if cell == b'*' {
                        '*'
                    } else {
                        count_neighbors(garden, x, y)
                    }
                })
                .collect()
        })
        .collect()
}

fn count_neighbors(garden: &[&str], x: usize, y: usize) -> char {
    let rows = garden.len();
    let cols = garden[0].len();

    match OFFSETS
        .iter()
        .map(|&(dx, dy)| (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)))
        .filter(|&(x, y)| x < rows && y < cols && garden[x].as_bytes()[y] == b'*')
        .count()
    {
        0 => ' ',
        n => (n as u8 + b'0') as char,
    }
}
