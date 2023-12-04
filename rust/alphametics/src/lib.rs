use std::{collections::{HashMap, BTreeMap, BTreeSet}, iter::once};

fn parse(input: &str) -> (Vec<&str>, &str) {
    let (left, right) = input.split_once("==").unwrap();
    let right = right.trim();
    let left: Vec<&str> = left.split(" + ").collect();
    (left, right)
}

fn to_number(addend: &str, letters: &BTreeMap<char, u32>) -> u32 {
    let mut num: String = "".to_string();
    for c in addend.chars() {
        num.push_str(letters[&c].to_string().as_str());
    }
    num.parse::<u32>().unwrap()
}

fn compute(left: &Vec<String>, right: &String, letters: &BTreeMap<char, u32>) -> bool {
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

struct CharNumberIter<'a> {
    index: &'a mut BTreeMap<char, u32>,
}

struct CharNumberIterHack {
    index: BTreeMap<char, u32>,
}

impl CharNumberIterHack {
    fn new(letters: &BTreeSet<char>) -> CharNumberIterHack {
        let index = letters.iter().map(|c| (*c, 0)).collect();
        Self { index }
    }

    fn sweep<'a>(&'a mut self) -> CharNumberIter<'a> {
        CharNumberIter  { index: &mut self.index }
    }
}

impl<'a> Iterator for CharNumberIter<'a> {
    type Item = &'a BTreeMap<char, u32>;

    fn next(&mut self) -> Option<&'a BTreeMap<char, u32>> {
        let len = self.index.len();
        for (i, (_, ii)) in self.index.iter_mut().enumerate() {
            *ii = *ii + 1u32;
            if *ii == 10 {
                *ii = 0u32;
                if i == len {
                    return None;
                }
            } else {
                break;
            }
        }
        let unique = self.index.iter().map(|(_, v)| *v).collect::<BTreeSet<u32>>();
        if unique.len() != self.index.len() {
            return CharNumberIter::next(self)
        }
        Some(&self.index)
    }
}

fn last_letter_constraints(left: &Vec<&str>, right: &str) -> BTreeMap<char, BTreeSet<u32>>{
    let left_last: Vec<char> = left.iter().map(|s| s.chars().last().unwrap()).collect();
    let right: char = right.chars().last().unwrap();
    let chars: BTreeSet<char> = left_last.iter().copied().chain(once(right)).collect();
    let left_last: Vec<String> = left_last.iter().map(|c| c.to_string()).collect();
    let right: String = right.to_string();

    let mut ret: BTreeMap<char, BTreeSet<u32>> = BTreeMap::new();
    let mut iter = CharNumberIterHack::new(&chars);

    for chars in iter.sweep() {
        if compute(&left_last, &right, chars) {
            for (c, n) in chars {
                ret.entry(*c).and_modify(|set| {set.insert(*n);});
            }
        }
    }

    todo!()
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {

    todo!("Solve the alphametic {input:?}")
}
