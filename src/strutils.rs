
use std::vec::Vec;
use std::iter::FromIterator;
use std::ascii::AsciiExt;
use std::ffi::CString;
use std::borrow::Borrow;

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
        Ok(x) => for &(a, b) in glue(x.borrow()).iter() {
            result.push(a << 4 | b);
        },
        Err(x) => return Err(x)
    }
    Ok(result)
}

pub fn skip_n(v : &[u8], n : usize) -> Vec<u8> {
    v.iter().enumerate().filter_map(|(i, &x)| if i % n == 0 { Some(x) } else { None }).collect()
}

pub fn get_score(x: u8) -> u64 {
    if !(x as char).is_ascii() {
        0
    } else {
        match (x as char).to_ascii_lowercase() {
            ' ' => 100,
            'a' => 82,
            'b' => 15,
            'c' => 28,
            'd' => 43,
            'e' => 130,
            'f' => 22,
            'g' => 20,
            'h' => 61,
            'i' => 70,
            'j' => 2,
            'k' => 8,
            'l' => 40,
            'm' => 24,
            'n' => 67,
            'o' => 75,
            'p' => 19,
            'q' => 1,
            'r' => 60,
            's' => 63,
            't' => 90,
            'u' => 28,
            'v' => 10,
            'w' => 24,
            'x' => 2,
            'y' => 20,
            'z' => 1,
            _ => 0
        }
    }
}

pub fn get_score_for(x: &[u8]) -> u64 {
    x.iter().map(|x| get_score(*x)).fold(0, |a, b| a + b)
}

pub fn to_string(a: &[u8]) -> String {
    let chars = a.iter().map(|x| x.clone() as char);
    FromIterator::from_iter(chars)
}

pub fn from_ascii_string(a: String) -> Vec<u8> {
    CString::new(a).unwrap().as_bytes().iter().map(|x| x.clone()).collect()
}

