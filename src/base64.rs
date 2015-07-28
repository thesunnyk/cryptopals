
use std::vec::Vec;
use std::char;
use std::iter::FromIterator;

pub type U8b64 = u8;

fn to_base64_char(i : U8b64) -> char {
    if i < 26 {
        (i + 'A' as u8) as char
    } else if i < 52 {
        (i - 26 + 'a' as u8) as char
    } else if i < 62 {
        let digit = char::from_digit((i - 52) as u32, 10);
        match digit {
            Some(x) => x,
            None => panic!("Digit couldn't be turned into a character")
        }
    } else {
        match i {
            62 => '+',
            63 => '/',
            _ => panic!("Invalid base64")
        }
    }
}

pub fn to_base64_string(v : Vec<U8b64>) -> String {
    FromIterator::from_iter(v.iter().map(|&x| to_base64_char(x)))
}

pub fn to_base64_unpacked(v : Vec<u8>) -> Vec<U8b64> {
    let mut result = Vec::new();
    let mut rem: u8 = 0;
    let mut offset = 2;
    for i in v.iter() {
        let comp = rem << (8 - offset) | ((*i as u8) >> offset);
        result.push(comp & 0x3f);
        rem = i & ((1 << offset) - 1);
        offset += 2;
        if offset == 8 {
            result.push(rem & 0x3f);
            offset = 2;
            rem = 0;
        }
    }
    if offset > 2 { // Remainder
        assert_eq!(rem, (rem & 0x3f));
        result.push(rem);
    }

    result
}

