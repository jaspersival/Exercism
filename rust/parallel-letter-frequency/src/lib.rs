use std::collections::{HashMap, HashSet};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut frequency_letters = HashMap::new();

    for word in input {
        for char in word.chars() {
            if !frequency_letters.contains_key(&char) {
                frequency_letters.insert(char, 1);
            } else {
                match frequency_letters.get_mut(&char) {
                    Some(value) => *value += 1,
                    None => (),
                };
            }
        }
    }
    frequency_letters
}
