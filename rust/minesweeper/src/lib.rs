use std::char::from_digit;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let fast_minefield: Vec<Vec<u8>> = minefield.iter().map(|l| l.chars().map(|c| if c == '*' {1} else {0}).collect()).collect();
    let mut ret: Vec<String> = Vec::with_capacity(minefield.len());
    for y in 0..fast_minefield.len() {
        let mut str:String = String::new();
        for x in 0..fast_minefield[y].len() {
            if fast_minefield[y][x] == 0 {
                let mut e = 0u32;
                for dy in [-1i64, 0i64, 1i64] {
                    for dx in [-1i64, 0i64, 1i64] {
                        let x_ = x as i32 + dx as i32;
                        let y_ = y as i32 + dy as i32;
                        if x_ >= 0 || x_ <= fast_minefield[y].len() as i32 || y_ <= 0 || y_ >= fast_minefield.len() as i32 {
                            match fast_minefield.get(y_ as usize).map(|l| l.get(x_ as usize)).flatten() {
                                Some(v) => {e += *v as u32}
                                None => {}
                            }
                        }
                    }
                }
                if e == 0 {
                    str.push(' ');
                } else {
                    str.push(from_digit(e, 10).unwrap());
                }
            } else {
                str.push('*');
            }
        }
        ret.push(str)
    }
    ret
}
