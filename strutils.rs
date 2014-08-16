
use std::vec::Vec;

fn to_hex_digits(s: &str) -> Result<Vec<u8>, &'static str> {
    let mut result = Vec::new();
    for c in s.chars() {
        match c.to_digit(16) {
            Some(x) => result.push(x as u8),
            None => return Err("Could not match hex")
        }
    }
    Ok(result)
}

fn glue(v : &[u8]) -> Vec<(u8, u8)> {
    let evens = v.iter().enumerate().filter_map(|(i, &x)| if i % 2 == 0 { Some(x) } else { None });
    let odds = v.iter().enumerate().filter_map(|(i, &x)| if i % 2 == 0 { None } else { Some(x) });
    evens.zip(odds).collect()
}

pub fn to_hex(s : &str) -> Result<Vec<u8>, &'static str> {
    let mut result = Vec::new();
    match to_hex_digits(s) {
        Ok(x) => for &(a, b) in glue(x.as_slice()).iter() {
            result.push(a << 4 | b);
        },
        Err(x) => return Err(x)
    }
    Ok(result)
}

pub fn get_score(x: u8) -> uint {
    if !x.is_ascii() {
        0
    } else {
        match x.to_ascii().to_lowercase().to_char() {
            ' ' => 10,
            'a' => 8,
            'b' => 1,
            'c' => 3,
            'd' => 4,
            'e' => 13,
            'f' => 2,
            'g' => 2,
            'h' => 6,
            'i' => 7,
            'j' => 0,
            'k' => 1,
            'l' => 4,
            'm' => 2,
            'n' => 7,
            'o' => 8,
            'p' => 2,
            'q' => 0,
            'r' => 6,
            's' => 6,
            't' => 9,
            'u' => 3,
            'v' => 1,
            'w' => 2,
            'x' => 0,
            'y' => 2,
            'z' => 0,
            _ => 0
        }
    }
}

pub fn get_score_for(x: &[u8]) -> uint {
    x.iter().map(|x| get_score(*x)).fold(0, |a, b| a + b)
}

pub fn to_string(a: &[u8]) -> String {
    let chars = a.iter().map(|x| x.to_ascii().to_char());
    FromIterator::from_iter(chars)
}

