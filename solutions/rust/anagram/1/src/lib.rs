use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();

    let mut target_char_map = HashMap::new();
    for ch in word.to_lowercase().chars() {
        let count = target_char_map.entry(ch).or_insert(0);
        *count += 1;
    }

    for &s in possible_anagrams {
        if s.to_lowercase() == word.to_lowercase() {
            continue;
        } else {
            let mut possible_char_map = HashMap::new();
            for ch in s.to_lowercase().chars() {
                let count = possible_char_map.entry(ch).or_insert(0);
                *count += 1;
            }
            if possible_char_map == target_char_map {
                result.insert(s);
            }
        }
    }
    result
}
