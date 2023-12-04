use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();
    let mut sorted_word: Vec<char> = lower_word.chars().collect();
    sorted_word.sort_unstable();

    let mut ret = HashSet::new();
    for element in possible_anagrams {
        let lower_elem = element.to_lowercase();
        let mut sorted_elem: Vec<char> = lower_elem.chars().collect();
        sorted_elem.sort_unstable();
        if sorted_word == sorted_elem && lower_word != lower_elem{
            ret.insert(*element);
        }
    }
    ret
}
