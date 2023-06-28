use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    for possible_anagram in possible_anagrams {
        if is_anagram(word, possible_anagram) {
            anagrams.insert(*possible_anagram);
        }
    }
    anagrams
}

fn is_anagram<'a>(word: &str, possible_anagram: &str) -> bool {
    if word == possible_anagram {
        return false
    }
    let mut word_chars = word.to_lowercase().chars().collect::<Vec<char>>();
    let mut possible_anagram_chars =possible_anagram.chars().collect::<Vec<char>>();
    possible_anagram_chars[0] = possible_anagram_chars[0].to_lowercase().next().unwrap();
    word_chars.sort();
    possible_anagram_chars.sort();
    word_chars == possible_anagram_chars
}