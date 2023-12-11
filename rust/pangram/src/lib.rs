use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let set: HashSet<char> = sentence.to_lowercase().chars().collect();
    ('a'..='z').all(|c| set.contains(&c))
}
