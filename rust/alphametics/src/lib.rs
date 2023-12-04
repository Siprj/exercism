use std::{collections::{HashMap, BTreeMap, BTreeSet}, iter::once, ops::Range};

fn parse(input: &str) -> (Vec<&str>, &str) {
    let (left, right) = input.split_once("==").unwrap();
    let right = right.trim();
    let left: Vec<&str> = left.split(" + ").collect();
    (left, right)
}

fn to_number(addend: &str, letters: &BTreeMap<char, char>) -> u32 {
    let mut num = "".to_string();
    for c in addend.chars() {
        num.push(letters[&c])
    }
    num.parse::<u32>().unwrap()
}

fn compute(left: &Vec<&str>, right: &str, letters: &BTreeMap<char, char>) -> bool {
    let left_sum: u32 = left.iter().map(|n| to_number(n, letters)).sum();
    let right = to_number(right, letters);
    left_sum == right
}

fn full_vec() -> Vec<u32> {
    let mut vec = Vec::with_capacity(10);
    for n in 0..9 {
        vec.push(n)
    }
    vec
}

fn get_full_letter_space(input: &str) -> BTreeMap<char, Vec<u32>>{
    let chars: BTreeSet<char> = input.chars().collect();
    chars.iter().map(|c| (*c, full_vec())).collect()
}

fn last_letter_constraints(left: &Vec<&str>, right: &str) -> Vec<Vec<(char, u32)>>{
    let left_last: Vec<char> = left.iter().map(|s| s.chars().last().unwrap()).collect();
    let right = right.chars().last().unwrap();
    let chars: BTreeSet<char> = left_last.iter().copied().chain(once(right)).collect();
    let mut iterators: Vec<(char, u32)> = chars.iter().map(|c| (*c, 0)).collect();

    loop {
        let mut index = 0;
        loop {
            iterators[index].1 += 1;
            if (iterators[index].1 > 9) {
                iterators[index].1 = 0;
                index += 1;
            } else {
                break;
            }
        }
        if index == iterators.len() {
            break;
        }
    }
    todo!()
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {

    todo!("Solve the alphametic {input:?}")
}
