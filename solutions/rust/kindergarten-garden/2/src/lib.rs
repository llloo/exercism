const CHILDREN: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let position = CHILDREN.iter().position(|&name| name == student).unwrap() * 2;
    diagram
        .lines()
        .flat_map(|line| {
            line[position..position + 2].chars().map(|c| match c {
                'G' => "grass",
                'C' => "clover",
                'R' => "radishes",
                'V' => "violets",
                _ => "",
            })
        })
        .collect()
}
