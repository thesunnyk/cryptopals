
mod fileutils;
mod xor;
mod strutils;

fn main() {
    let lines_str = fileutils::read_file("challenge_4.txt".to_string());
    let lines : Vec<Vec<u8>> = lines_str.iter().map(|x| match strutils::to_hex(x.as_slice()) {
        Ok(x) => x,
        Err(x) => fail!(x)
    }).collect();
    let max_xors = lines.iter().map(|x| match xor::find_max_xor(x.as_slice()) {
        Some(x) => x,
        None => fail!("Could not find max xor")
    });
    let mut xored = lines.iter().zip(max_xors).map(|(l, s)| xor::xor_byte(l.as_slice(), s));
    let max = match xored.max_by(|x| strutils::get_score_for(x.as_slice())) {
        Some(x) => x,
        None => fail!("Could not get max score")
    };

    let result = strutils::to_string(max.as_slice());
    println!("val: {}", result);
    assert_eq!(result, "Now that the party is jumping\n".to_string());
}

