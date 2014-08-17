
extern crate serialize;

use serialize::base64::FromBase64;
use std::iter;

mod strutils;
mod fileutils;
mod base64;
mod xor;

fn find_candidate(keys : Vec<u8>, data: &[u8]) -> Option<u8> {
    for key in keys.iter() {
        let cleartext_i = xor::xor_byte(data, *key);
        if cleartext_i.iter().all(|x| x.is_ascii()) {
            return Some(*key);
        }
    }
    return None;
}

fn find_key(keysize : uint, data : &[u8]) -> Option<Vec<u8>> {
        let mut key : Vec<u8> = Vec::new();

        // transpose and skip(keysize) to find score for individual block items, then use that to
        // generate the full key
        for i in range(0, keysize) {
            let data_i = strutils::skip_n(data.slice(i, data.len()), keysize);
            let keys : Vec<u8> = xor::get_max_xors(data_i.as_slice())
                .iter().map(|&(x, _)| x).collect();
            match find_candidate(keys, data_i.as_slice()) {
                None => return None,
                Some(x) => key.push(x)
            };
        }
        return Some(key);
}

fn main() {
    let lines_str = fileutils::read_file("challenge_4.txt".to_string());
    let lines : String = lines_str.iter().fold("".to_string(), |x, y| x + *y);
    let data = match lines.as_slice().from_base64() {
        Err(x) => fail!(x),
        Ok(x) => x
    };

    let mut scores : Vec<(uint, uint)> = iter::range_inclusive(2 as uint,40 as uint)
        .map(|x| (x, xor::key_len_score(data.clone(), x))).collect();

    scores.sort_by(|&(_, x), &(_, y)| x.cmp(&y));

    for &(keysize, _) in scores.iter() {
        println!("trying keysize: {}", keysize);

        let key = match find_key(keysize, data.as_slice()) {
            None => continue,
            Some(x) => x
        };

        let cleartext = xor::xor_repeat(data.as_slice(), key);
        if cleartext.iter().all(|x| x.is_ascii()) {
            println!("Cleartext: {}", strutils::to_string(cleartext.as_slice()));
            return;
        }
    }

    println!("Couldn't find a key");
}

