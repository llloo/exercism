pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split([' ', '-', '_'])
        .filter(|word| !word.is_empty())
        .flat_map(|word| {
            word.chars().take(1).chain(
                word.chars()
                    .skip_while(|c| c.is_uppercase())
                    .filter(|c| c.is_uppercase()),
            )
        })
        .collect::<String>()
        .to_ascii_uppercase()
}
