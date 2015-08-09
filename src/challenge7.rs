
extern crate openssl;
extern crate rustc_serialize;

mod fileutils;

use openssl::crypto::symm;
use std::borrow::Borrow;
use rustc_serialize::base64::FromBase64;

fn main() {
    let key = "YELLOW SUBMARINE".to_string().into_bytes();
    let lines_str = fileutils::read_file("challenge_7.txt".to_string());
    let lines: String = lines_str.iter().fold("".to_string(), |x, y| x + y);
    let data: Vec<u8> = match lines.from_base64() {
        Err(x) => panic!(x),
        Ok(x) => x
    };

    let crypter = symm::Crypter::new(symm::Type::AES_128_ECB);
    crypter.init(symm::Mode::Decrypt, key.borrow(), []);

    let mut first = crypter.update(data.borrow());
    first.extend(crypter.finalize());

    let ans = match String::from_utf8(first) {
        Err(x) => panic!(x),
        Ok(x) => x
    };

    println!("message is: {}", ans);

}
