use std::{collections::HashMap, thread::JoinHandle, sync::Arc, cmp::min};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.len() == 0 {
        return HashMap::new();
    }

    let mut threads: Vec<JoinHandle<HashMap<char, usize>>> = Vec::with_capacity(worker_count);
    let shared_data: Arc<Vec<String>> = Arc::new(input.iter().map(|l| (*l).into()).collect());
    let real_worker_count = min(input.len(), worker_count);

    let mut chunk_size: usize = input.len() / real_worker_count;
    if chunk_size * real_worker_count < input.len() { chunk_size += 1};
    let mut start = 0;

    for _ in 0..real_worker_count {
        let arc_data = Arc::clone(&shared_data);
        threads.push(
            std::thread::spawn(move ||{
                let mut chars: HashMap<char, usize> = HashMap::new();
                let stop = min(start + chunk_size, arc_data.len());
            for line in arc_data[start..stop].iter() {
                for char in line.to_lowercase().chars().filter(|c| c.is_alphabetic()) {
                    chars.entry(char).and_modify(|count| *count += 1).or_insert(1);
                }
            }
            chars
        }));
        start += chunk_size;
    }

    let mut ret = HashMap::new();
    for thread in threads {
        for (k, v) in thread.join().unwrap().iter() {
            ret.entry(*k).and_modify(|count| *count += *v).or_insert(*v);
        }
    }
    ret
}
