
mod base64;
mod strutils;
mod xor;

use std::borrow::Borrow;

fn main() {
    let xstr = "1c0111001f010100061a024b53535009181c";
    let ystr = "686974207468652062756c6c277320657965";
    let expectedstr = "746865206b696420646f6e277420706c6179";

    let x = match strutils::to_hex(xstr) {
        Ok(x) => x,
        Err(x) => panic!(x)
    };
    let y = match strutils::to_hex(ystr) {
        Ok(x) => x,
        Err(x) => panic!(x)
    };
    let expected = match strutils::to_hex(expectedstr) {
        Ok(x) => x,
        Err(x) => panic!(x)
    };
    assert_eq!(xor::xor(x.borrow(), y.borrow()), expected);
}

