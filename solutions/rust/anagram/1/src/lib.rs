use std::collections::HashSet;

fn normalise(word: &str) -> Vec<char> {
    let mut normalised_word: Vec<char> = word.to_lowercase().chars().collect();
    normalised_word.sort_unstable();
    normalised_word
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let normalised_word = normalise(word);

    possible_anagrams
        .iter()
        .copied()
        .filter(|a| a.to_lowercase() != word.to_lowercase())
        .filter(|a| normalise(a) == normalised_word)
        .collect()
}
