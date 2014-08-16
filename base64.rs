
use std::vec::Vec;
use std::char;

pub type U8b64 = u8;

pub fn to_base64_string(v : Vec<U8b64>) -> String {
    let mut result = String::new();
    for i in v.iter() {
        if *i < 26 {
            let x = (i + 'A'.to_ascii().to_byte()).to_ascii().to_char();
            result.push_char(x);
        } else if *i < 52 {
            let x = (i - 26 + 'a'.to_ascii().to_byte()).to_ascii().to_char();
            result.push_char(x);
        } else {
            let digit = char::from_digit((i - 52) as uint, 10);
            match digit {
                Some(x) => result.push_char(x),
                None => fail!("Badly converted Base64")
            }
        }
    }
    result
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
        assert!(rem == (rem & 0x3f));
        result.push(rem);
    }

    result
}

