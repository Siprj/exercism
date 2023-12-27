#![feature(iter_intersperse)]
use std::collections::HashSet;

pub fn translate(input: &str) -> String {
    let vowels: HashSet<char> = HashSet::from(['a', 'e', 'i', 'o', 'u']);

    let mut out: Vec<String> = vec![];
    for word in input.split_whitespace() {
        let mut new_word = String::new();
        if vowels.contains(&word.chars().next().unwrap())
            || word.starts_with("xr")
            || word.starts_with("yt")
        {
            new_word.push_str(word);
            new_word.push_str("ay");
        } else {
            let mut found_consonant = false;
            let consonant_cluster: String = word
                .chars()
                .take_while(|c| {
                    let ret = !(vowels.contains(c) || c == &'y' && found_consonant);
                    found_consonant = true;
                    ret
                })
                .collect();
            println!("consonant_cluster: {:?}", consonant_cluster);
            if word[consonant_cluster.len()..].starts_with('u') && consonant_cluster.ends_with('q')
            {
                new_word.push_str(&word[consonant_cluster.len() + 1..]);
                new_word.push_str(&consonant_cluster);
                new_word.push('u');
            } else if consonant_cluster.len() != 1 && consonant_cluster.ends_with('y') {
                new_word.push_str(&word[consonant_cluster.len() - 1..]);
                new_word.push_str(&consonant_cluster[0..&consonant_cluster.len() - 1]);
            } else {
                new_word.push_str(&word[consonant_cluster.len()..]);
                new_word.push_str(&consonant_cluster);
            }
            new_word.push_str("ay");
        }
        out.push(new_word);
    }
    out.iter()
        .intersperse(&String::from(" "))
        .cloned()
        .collect::<String>()
}
