use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut sorted_word: Vec<char> = word.to_lowercase().collect();
    sorted_word.sort_unstable();

    let mut ret = HashSet::new();
    for element in possible_anagrams {
        let mut sorted_elem: Vec<char> = element.chars().collect();
        sorted_elem.sort_unstable();
        if
        ret.insert(*element);
    }
    ret
}
