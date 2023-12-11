/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let val: Vec<char> = value.to_string().chars().collect();
        let val_rev: Vec<char> = val.iter().rev().copied().collect();
        if val == val_rev {
            Some(Palindrome(value))
        }
        else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

fn find_min(min: u64, max: u64) -> Option<Palindrome>{
    for n in (min*min)..=(max*max) {
        let pal = Palindrome::new(n);
        if pal.is_some() {
            if (min..=max).any(|f| n % f == 0 && (n / f) <= max && (n/f) >= min){
                return pal;
            }
        }
    }
    None
}

fn find_max(min: u64, max: u64) -> Option<Palindrome>{
    for n in ((min*min)..=(max*max)).rev() {
        let pal = Palindrome::new(n);
        if pal.is_some() {
            if (min..=max).rev().any(|f| n % f == 0 && (n / f) <= max && (n/f) >= min){
                return pal;
            }
        }
    }
    None
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let left = find_min(min, max);
    left.and_then(|left| find_max(min, max).map(|right| (left, right)))
}
