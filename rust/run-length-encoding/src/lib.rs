pub fn encode(source: &str) -> String {
    let mut ret = String::new();
    let mut source = source.as_bytes();
    while !source.is_empty() {
        let char = source[0];
        let count = source.iter().take_while(|c| c == &&char).count();
        if count == 1 {
            ret.push(char.into());
            source = &source[1..];
        } else {
            ret.push_str(&count.to_string());
            ret.push(char.into());
            source = &source[count..];
        }
    }
    ret
}

pub fn decode(source: &str) -> String {
    let mut ret = String::new();
    let mut source = source.as_bytes();
    while !source.is_empty() {
        if source[0].is_ascii_digit() {
            let pos = source.iter().position(|c| !c.is_ascii_digit()).unwrap();
            let num_str: String = String::from_utf8(source[0..pos].into()).unwrap();
            let num: u32 = num_str.parse().unwrap();
            for _ in 0..num {
                ret.push(source[pos].into());
            }
            source = &source[pos + 1..];
        } else {
            ret.push(source[0].into());
            source = &source[1..];
        }
    }

    ret
}
