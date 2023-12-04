pub fn abbreviate(phrase: &str) -> String {
    let mut white_space = true;
    let mut uppercase = false;
    let mut ret = String::new();
    for c in phrase.chars() {
        if (white_space || (!uppercase && c.is_uppercase())) && c.is_alphabetic() {
            ret.push_str(c.to_uppercase().to_string().as_str());
        }
        white_space = c.is_whitespace() || c == '_' || c == '-';
        uppercase = c.is_uppercase();
    }
    ret
}
