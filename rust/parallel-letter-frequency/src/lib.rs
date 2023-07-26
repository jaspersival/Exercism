use std::collections::{HashMap, HashSet};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut frequency_letters = HashMap::new();

    for word in input {
        for char in word.to_lowercase().chars() {
            let count = frequency_letters.entry(char).or_insert(0);
            *count += 1;
        }
    }
    frequency_letters
}
