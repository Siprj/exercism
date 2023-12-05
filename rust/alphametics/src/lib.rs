use std::{
    collections::{BTreeSet, HashMap},
    iter::once,
};

use itertools::Itertools;

fn parse(input: &str) -> (Vec<&str>, &str) {
    let (left, right) = input.split_once("==").unwrap();
    let right = right.trim();
    let left: Vec<&str> = left.split(" + ").map(|s| s.trim()).collect();
    (left, right)
}

fn compute_weights(left: &[&str], right: &str) -> HashMap<char, i64> {
    let mut weights: HashMap<char, i64> = HashMap::new();
    for w in left {
        for (i, c) in w.chars().rev().enumerate() {
            weights
                .entry(c)
                .and_modify(|v| *v += 10i64.pow(i as u32))
                .or_insert(10i64.pow(i as u32));
        }
    }
    for (i, c) in right.chars().rev().enumerate() {
        weights
            .entry(c)
            .and_modify(|v| *v -= 10i64.pow(i as u32))
            .or_insert(-(10i64.pow(i as u32)));
    }
    weights
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (left, right) = parse(input);
    let weights = compute_weights(&left, right);
    let first_letters: BTreeSet<char> = left
        .iter()
        .map(|l| l.chars().next().unwrap())
        .chain(once(right.chars().next().unwrap()))
        .collect();

    let len = weights.keys().len();
    if len > 10 {
        return None;
    }

    for values in (0u8..10).permutations(len) {
        if values
            .iter()
            .zip(weights.keys())
            .any(|(n, c)| n == &0 && first_letters.contains(c))
        {
            continue;
        }

        let sum: i64 = values
            .iter()
            .zip(weights.values())
            .map(|(n, w)| (*n as i64) * w)
            .sum();
        if sum == 0 {
            return Some(
                weights
                    .iter()
                    .zip(values.iter())
                    .map(|((k, _), v)| (*k, *v))
                    .collect(),
            );
        }
    }
    None
}
