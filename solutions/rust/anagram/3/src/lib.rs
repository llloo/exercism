use std::collections::HashSet;

fn sorted_chars(s: &str) -> Vec<char> {
    let mut chars: Vec<char> = s.to_lowercase().chars().collect();
    chars.sort_unstable();
    chars
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let target_word = sorted_chars(&word_lower);

    possible_anagrams
        .iter()
        .copied()
        .filter(|candidate| {
            let canditate_lower = candidate.to_lowercase();
            canditate_lower != word_lower && sorted_chars(&canditate_lower) == target_word
        })
        .collect()
}
