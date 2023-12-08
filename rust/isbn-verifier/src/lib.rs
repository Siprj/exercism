/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let letters: Vec<char> = isbn.chars().filter(|c| c != &'-').collect();
    if letters.len() != 10 || (letters[0..9]).iter().any(|d| !d.is_ascii_digit()) || (!letters[9].is_ascii_digit() && letters[9] != 'X') {
        return  false;
    }
    letters.iter().rev().zip(1..).map(|(l, m)| {
        l.to_digit(10).unwrap_or(10) * m
    }).sum::<u32>() % 11 == 0
}

