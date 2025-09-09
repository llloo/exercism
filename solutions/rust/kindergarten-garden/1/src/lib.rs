pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let i = student
        .chars()
        .next()
        .and_then(|c| Some(c as u8 - b'A'))
        .unwrap() as usize;
    let plant: Vec<&str> = diagram.split("\n").collect();

    let mut result = Vec::new();

    let row_1 = &plant[0][i * 2..(i + 1) * 2];
    let row_2 = &plant[1][i * 2..(i + 1) * 2];

    let plants = format!("{}{}", row_1, row_2);

    for c in plants.chars() {
        match c {
            'G' => result.push("grass"),
            'C' => result.push("clover"),
            'R' => result.push("radishes"),
            'V' => result.push("violets"),
            _ => (),
        }
    }

    result
}
