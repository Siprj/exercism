pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::with_capacity(n as usize);
    (2..)
        .filter(|i| {
            if primes.iter().any(|p| i % p == 0) {
                false
            } else {
                primes.push(*i);
                true
            }
        })
        .nth(n as usize)
        .unwrap()
}
