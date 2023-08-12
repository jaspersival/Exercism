use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

type FrequencyLetters = HashMap<char, Mutex<usize>>;

#[derive(Debug)]
struct ParallelLetterFrequency {
    frequency_letters: Mutex<HashMap<char, usize>>,
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input = input
        .into_iter()
        .map(|x| String::from(*x))
        .collect::<Vec<String>>();
    let parallel_letter_frequency = Arc::new(ParallelLetterFrequency {
        frequency_letters: Mutex::new(HashMap::new()),
    });
    let chunk_size = (input.len() + worker_count - 1) / worker_count;
    let input_chunked = input
        .chunks(chunk_size)
        .map(|x| x.to_vec())
        .collect::<Vec<_>>();

    // "https://stackoverflow.com/questions/50282619/is-it-possible-to-share-a-hashmap-between-threads-without-locking-the-entire-has"
    let threads: Vec<JoinHandle<()>> = input_chunked
        .into_iter()
        .map(|input_chunk| {
            let frequency_letter_clone = Arc::clone(&parallel_letter_frequency);
            thread::spawn(move || worker_thread(input_chunk, frequency_letter_clone))
        })
        .collect();
    for t in threads {
        t.join().expect("Thread panicked");
    }
    Arc::try_unwrap(parallel_letter_frequency)
        .expect("Failed to get map from Arc")
        .frequency_letters
        .into_inner()
        .expect("Failed to get map from Mutex")
}
fn worker_thread(
    input_chunk: Vec<String>,
    parallel_letter_frequency: Arc<ParallelLetterFrequency>,
) {
    for word in input_chunk {
        for char in word.to_lowercase().chars() {
            if char.is_alphabetic() {
                let mut frequency_letters_guard = parallel_letter_frequency
                    .frequency_letters
                    .lock()
                    .expect("Failed to lock Mutex");

                let count = frequency_letters_guard.entry(char).or_insert(0);
                *count += 1;
            }
        }
    }
}
