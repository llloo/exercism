use std::collections::VecDeque;

pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return Vec::new();
    }
    let directions = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let rows = garden.len();
    let cols = garden[0].len();

    let mut result: Vec<Vec<char>> = garden.iter().map(|row| row.chars().collect()).collect();

    for i in 0..rows {
        for j in 0..cols {
            if garden[i].as_bytes()[j] == b' ' {
                let mut count = 0;

                for &(dx, dy) in &directions {
                    let ni = i as i32 + dx;
                    let nj = j as i32 + dy;

                    if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 {
                        let ni = ni as usize;
                        let nj = nj as usize;

                        if garden[ni].as_bytes()[nj] == b'*' {
                            count += 1;
                        }
                    }
                }

                if count > 0 {
                    result[i][j] = std::char::from_digit(count, 10).unwrap();
                }
            }
        }
    }

    result
        .into_iter()
        .map(|chars| chars.into_iter().collect())
        .collect()
}
