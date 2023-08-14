use std::cmp::min;
use std::collections::HashMap;
use std::thread;

type FrequencyLetter = HashMap<char, usize>;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input = input.join("");
    let mut frequency_map: FrequencyLetter = HashMap::new();

    if input.is_empty() {
        return frequency_map;
    }

    let mut churn = input.chars();
    let real_worker_count = min(input.len(), worker_count);
    let mut thread_pool = Vec::with_capacity(real_worker_count);
    let mut work_length = (input.len() / real_worker_count).max(1);
    if work_length * real_worker_count < input.len() {
        work_length += 1;
    }

    for _ in 0..real_worker_count {
        let chunk = churn.by_ref().take(work_length).collect::<String>();
        let thread = thread::spawn(move || worker_thread(chunk));
        thread_pool.push(thread);
    }
    // "https://stackoverflow.com/questions/50282619/is-it-possible-to-share-a-hashmap-between-threads-without-locking-the-entire-has"
    for t in thread_pool {
        let result = t.join().expect("Thread panicked");
        for (key, value) in result.iter() {
            *frequency_map.entry(*key).or_default() += value;
        }
    }
    frequency_map
}
fn worker_thread(chunk: String) -> FrequencyLetter {
    let mut frequency_map: FrequencyLetter = HashMap::new();
    for char in chunk.chars().filter(|c| c.is_alphabetic()) {
        if let Some(c) = char.to_lowercase().next() {
            *frequency_map.entry(c).or_default() += 1;
        }
    }
    frequency_map
}
