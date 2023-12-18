#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn aliquot_sum(n: u64) -> u64 {
    let mut sum = 0;
    for i in 1..=(n / 2) {
        if n % i == 0 {
            sum += i;
        }
    }
    sum
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    let sum = aliquot_sum(num);
    println!("sum: {}", sum);
    match sum.cmp(&num) {
        std::cmp::Ordering::Less => Some(Classification::Deficient),
        std::cmp::Ordering::Equal => Some(Classification::Perfect),
        std::cmp::Ordering::Greater => Some(Classification::Abundant),
    }
}
