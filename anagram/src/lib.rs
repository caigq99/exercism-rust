use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    let lowered_word = word.to_lowercase();
    let sorted_word = sort_chars(&lowered_word);
    for &possible_anagram in possible_anagrams {
        let lowered_possible_anagram = possible_anagram.to_lowercase();
        let sorted_possible_anagram = sort_chars(&lowered_possible_anagram);
        if lowered_possible_anagram != lowered_word && sorted_possible_anagram == sorted_word {
            result.insert(possible_anagram);
        }
    }
    result
}

fn sort_chars(word: &str) -> String {
    let mut word: Vec<char> = word.chars().collect();
    word.sort();
    word.into_iter().collect()
}
