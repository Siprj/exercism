use std::iter::zip;

pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new();
    }
    let mut ret = zip(list, &list[1..]).fold(vec![], |mut acc, (w1, w2)| {
        acc.push(format!("For want of a {} the {} was lost.", w1, w2));
        acc
    });
    ret.push(format!("And all for the want of a {}.", list[0]));
    ret.join("\n")
}
