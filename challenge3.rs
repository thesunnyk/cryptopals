
mod strutils;
mod xor;

use std::borrow::Borrow;

fn main() {
    let xstr = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let x = match strutils::to_hex(xstr) {
        Ok(x) => x,
        Err(x) => panic!(x)
    };
    let xors = xor::get_max_xors(x.borrow());

    let &(val, _) = xors.get(0).expect("Empty vector");

    let xored = xor::xor_byte(x.borrow(), val);
    println!("val: {} -> {}", val, strutils::to_string(xored.borrow()));
    assert_eq!(val, 88);
}

