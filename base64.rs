// Package ID
#![ crate_id = "cryptopals#0.1" ]

// Additional metadata attributes
#![ desc = "base64 converter" ]
#![ license = "GPL" ]
#![ comment = "http://cryptopals.com/sets/1/challenges/1/" ]

// Specify the output type
#![ crate_type = "lib" ]

// extern crate stuff
extern crate core;

// Mods
mod base64 {
    use std::vec::Vec;
    use std::char;
    use core::char::Char;

    // Naked functions for now

    pub fn to_hex(s : &str) -> Result<Vec<u8>, &'static str> {
        let mut result = Vec::new();
        for c in s.chars() {
            match c.to_digit(16) {
                Some(x) => result.push(x as u8),
                None => return Err("Could not match hex")
            }
        }
        Ok(result)
    }

    type U8b64 = u8;

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
        let mut offset = 0;
        for i in v.iter() {
            let comp = rem | (i >> offset);
            result.push(comp & 0x3f);
            offset += 2;
            rem = i << (8 - offset);
            if (offset == 8) {
                result.push(rem & 0x3f);
                offset = 0;
            }
        }
        if offset > 0 { // Remainder
            assert!(rem == (rem & 0x3f));
            result.push(rem);
        }

        result
    }

    #[test]
    fn test_to_base64() {
        let data_str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        match to_hex(data_str) {
            Ok(x) => {
                let data_b64 = to_base64_unpacked(x);
                let data_str = to_base64_string(data_b64);
                println!("Result: {}", data_str)
                assert!(data_str == expected.to_string());
            },
            Err(y) => {
                fail!(y);
            }
        }

    }
}

