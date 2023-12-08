use std::collections::BTreeSet;

pub fn check(candidate: &str) -> bool {
    candidate.to_lowercase().chars().filter(|c| c != &' ' && c != &'-').fold((true, BTreeSet::new()),| mut acc, c|{
        acc.0 = acc.1.insert(c) && acc.0;
        acc
    }).0
}
