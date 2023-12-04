pub fn factors(mut n: u64) -> Vec<u64> {
    if n < 2 { return vec![] }

    let mut factors = vec![];
    let mut factor = 2;

    loop {
        if n == 1 { break; }

        if n % factor == 0 {
            factors.push(factor);
            n = n / factor;
        } else {
            factor += 1;
        }
    }
    factors
}
