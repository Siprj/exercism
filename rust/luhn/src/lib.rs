/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let ret = code.chars().filter(|c| !c.is_ascii_whitespace()).rev().try_fold((0u32, 0u32), |(acc, index), c|{
        let mut d = c.to_digit(10)?;
        if index % 2 == 1 {
            d = d * 2;
            if d > 9 { d = d - 9 }
        }
        Some((acc + d, index + 1))
    });
    match ret {
        None => false,
        Some((acc, len)) => acc % 10 == 0 && len > 1,
    }
}
