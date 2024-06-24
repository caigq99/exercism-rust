use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowered_word = word.to_lowercase();
    let sorted_word = sort_chars(&lowered_word);
    possible_anagrams
        .iter()
        .filter(|&&possible_anagram| {
            let lowered_possible_anagram = possible_anagram.to_lowercase();
            let sorted_possible_anagram = sort_chars(&lowered_possible_anagram);
            lowered_possible_anagram != lowered_word && sorted_possible_anagram == sorted_word
        })
        .copied()
        .collect()
}

fn sort_chars(word: &str) -> String {
    let mut word: Vec<char> = word.chars().collect();
    word.sort();
    word.into_iter().collect()
}
