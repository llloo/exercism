use std::collections::HashMap;
use std::collections::HashSet;

fn char_count_map(s: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in s.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    map
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    let lowercase_word = word.to_lowercase();
    let target_char_map = char_count_map(&lowercase_word);

    for &s in possible_anagrams {
        if s.to_lowercase() == lowercase_word {
            continue;
        }
        let s_lower = s.to_lowercase();
        if target_char_map == char_count_map(&s_lower) {
            result.insert(s);
        }
    }
    result
}
