use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;

type FrequencyLetter = HashMap<char, usize>;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input = input
        .iter()
        .map(|x| String::from(*x))
        .collect::<Vec<String>>();
    let empty_map: FrequencyLetter = HashMap::new();

    if input.is_empty() {
        return empty_map;
    }

    let chunk_size = (input.len() + worker_count - 1) / worker_count;
    let input_chunked = input
        .chunks(chunk_size)
        .map(|x| x.to_vec())
        .collect::<Vec<_>>();

    let mut frequency_map: FrequencyLetter = HashMap::new();
    // "https://stackoverflow.com/questions/50282619/is-it-possible-to-share-a-hashmap-between-threads-without-locking-the-entire-has"
    let threads: Vec<JoinHandle<FrequencyLetter>> = input_chunked
        .into_iter()
        .map(|input_chunk| thread::spawn(move || worker_thread(input_chunk)))
        .collect();
    for t in threads {
        let result = t.join().expect("Thread panicked");
        for (key, value) in result.iter() {
            *frequency_map.entry(*key).or_default() += value;
        }
    }
    frequency_map
}
fn worker_thread(input_chunk: Vec<String>) -> FrequencyLetter {
    let mut frequency_map: FrequencyLetter = HashMap::new();
    for word in input_chunk {
        for char in word.chars().filter(|c| c.is_alphabetic()) {
            if let Some(c) = char.to_lowercase().next() {
                *frequency_map.entry(c).or_default() += 1;
            }
        }
    }
    frequency_map
}
