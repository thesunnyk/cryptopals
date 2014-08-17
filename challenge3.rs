
mod strutils;
mod xor;

fn main() {
    let xstr = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let x = match strutils::to_hex(xstr) {
        Ok(x) => x,
        Err(x) => fail!(x)
    };
    let xors = xor::get_max_xors(x.as_slice());

    let &(val, _) = xors.get(0);

    let xored = xor::xor_byte(x.as_slice(), val);
    println!("val: {} -> {}", val, strutils::to_string(xored.as_slice()));
    assert_eq!(val, 88);
}

