#![feature(iter_intersperse)]
fn base(n: u64) -> String {
    match n {
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        _ => unreachable!(),
    }
}

fn decade_base(n: u64) -> String {
    match n {
        2 => "twenty".to_string(),
        3 => "thirty".to_string(),
        4 => "forty".to_string(),
        5 => "fifty".to_string(),
        6 => "sixty".to_string(),
        7 => "seventy".to_string(),
        8 => "eighty".to_string(),
        9 => "ninety".to_string(),
        _ => unreachable!(),
    }
}

fn decode_1_99(n: u64) -> String {
    match n {
        n if n > 0 && n < 19 => return base(n),
        n if n >= 20 && n < 100 => {
            let mut ret = String::new();
            ret.push_str(&decade_base(n / 10));
            if n % 10 != 0 {
                ret.push('-');
                ret.push_str(&base(n % 10));
            }
            return ret;
        }
        _ => "".to_string(),
    }
}

fn part(n: u64) -> String {
    let mut ret = String::new();
    let hundreds = n / 100;
    if hundreds == 0 {
        println!("asdf");
        ret.push_str(&decode_1_99(n));
    } else {
        ret.push_str(&base(n / 100));
        ret.push_str(" hundred");
        if n % 100 != 0 {
            ret.push(' ');
            ret.push_str(&decode_1_99(n % 100));
        }
    }
    return ret;
}

pub fn encode(mut n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    } else {
        let mut ret: Vec<String> = vec![];
        for suffix in [
            "",
            "thousand",
            "million",
            "billion",
            "trillion",
            "quadrillion",
            "quintillion",
        ] {
            if n % 1000 != 0 {
                let mut val = part(n % 1000);
                if !val.is_empty() && !suffix.is_empty() {
                    val.push(' ');
                }
                val.push_str(suffix);
                ret.push(val);
            }
            if n / 1000 != 0 {
                n = n / 1000;
            } else {
                break;
            }
        }
        return ret.iter().rev().intersperse(&" ".to_string()).fold(
            "".to_string(),
            |mut acc, str| {
                acc.push_str(str);
                acc
            },
        );
    }
}
