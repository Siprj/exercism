use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    iter::once,
    ops::RangeInclusive,
};

use itertools::Itertools;

fn parse(input: &str) -> (Vec<&str>, &str) {
    let (left, right) = input.split_once("==").unwrap();
    let right = right.trim();
    let left: Vec<&str> = left.split(" + ").map(|s| s.trim()).collect();
    (left, right)
}

fn map_string(s: &str, map: &BTreeMap<char, char>) -> u64 {
    let string: String = s.to_string();
    let mut new_string = String::new();
    for c in string.chars() {
        if let Some(d) = map.get(&c) {
            new_string.push(*d);
        }
    }
    new_string.parse().unwrap()
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (left, right) = parse(input);
    let first_letters: BTreeSet<char> = left
        .iter()
        .map(|l| l.chars().next().unwrap())
        .chain(once(right.chars().next().unwrap()))
        .collect();
    let unique_letters: BTreeSet<char> = left
        .iter()
        .flat_map(|l| l.chars())
        .chain(right.chars())
        .collect();
    let unique_letters: Vec<&char> = unique_letters.iter().collect();
    let first_letters: BTreeSet<usize> = first_letters
        .iter()
        .map(|c1| unique_letters.iter().position(|c2| *c1 == **c2).unwrap())
        .collect();

    if unique_letters.len() > 10 {
        return None;
    }

    let range: RangeInclusive<char> = '0'..='9';

    let mut iteration_count = 0;
    for values in range.permutations(unique_letters.len()) {
        iteration_count += 1;
        if values
            .iter()
            .enumerate()
            .any(|(i, c)| c == &'0' && first_letters.contains(&i))
        {
            continue;
        }

        let map: BTreeMap<char, char> = unique_letters
            .iter()
            .copied()
            .copied()
            .zip(values)
            .collect();
        let left: u64 = left.iter().map(|s| map_string(s, &map)).sum();
        let right: u64 = map_string(right, &map);
        if left == right {
            return Some(map.iter().map(|(&k, &v)| (k,v.to_digit(10).unwrap() as u8)).collect());
        }
    }
    None
}
