pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| !c.is_alphabetic() && c != '\'')
        .filter(|w| !w.is_empty())
        .filter_map(|word| {
            let up_letters = word
                .chars()
                .filter(|c| c.is_ascii_uppercase())
                .collect::<String>();
            if !up_letters.is_empty() && !word.chars().all(|c: char| c.is_ascii_uppercase()) {
                Some(up_letters)
            } else {
                word.chars()
                    .next()
                    .map(|c| c.to_ascii_uppercase().to_string())
            }
        })
        .map(|c| c.to_ascii_uppercase())
        .collect()
}
