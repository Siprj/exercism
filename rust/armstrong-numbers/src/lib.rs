pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num.to_string();
    digits.chars().map(|v| v.to_digit(10).unwrap().pow(digits.len() as u32)).fold(Some(0), |acc, v| acc.and_then(|a: u32| a.checked_add(v))) == Some(num)
}
