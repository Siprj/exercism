const LIST: [(u32, &str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(n: u32) -> String {
    let ret: String = LIST
        .iter()
        .filter_map(|(d, s)| if n % d == 0 { Some(s) } else { None })
        .fold(String::new(), |mut acc, v| {
            acc.push_str(v);
            acc
        });
    if ret.len() == 0 {
        return n.to_string();
    }
    ret
}
