
extern crate serialize;

use serialize::base64::FromBase64;
use std::iter;

mod strutils;
mod fileutils;
mod base64;
mod xor;

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

    println!("scores: {}", scores);

    let (keysize, _) = *scores.get(0);

    println!("keysize: {}", keysize);

    let mut key : Vec<u8> = Vec::new();

    // transpose and skip(keysize) to find score for individual block items, then use that to
    // generate the full key
    for i in range(0, keysize) {
        let data_i = strutils::skip_n(data.slice(i, data.len()), keysize);
        let key_i = match xor::find_max_xor(data_i.as_slice()) {
            Some(x) => x,
            None => fail!("couldn't find max xor")
        };
        key.push(key_i);
    }

    let cleartext = xor::xor_repeat(data.as_slice(), key);
    println!("Cleartext: {}", strutils::to_string(cleartext.as_slice()));
}

