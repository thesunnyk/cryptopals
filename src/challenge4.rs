
mod fileutils;
mod xor;
mod strutils;

use std::borrow::Borrow;

fn max_by<F>(iter: &[Vec<u8>], func: F) -> Option<Vec<u8>> where F: Fn(&[u8]) -> u64 {
        let mut maxScore = 0 as u64;
        let mut maxVal: Option<&Vec<u8>> = None;
        for elt in iter {
            let curScore = func(elt);
            if (curScore > maxScore) {
                maxScore = curScore;
                maxVal = Some(elt);
            }
        }
        maxVal.map(|x| x.clone())
    }

fn main() {
    let lines_str = fileutils::read_file("challenge_4.txt".to_string());
    let lines : Vec<Vec<u8>> = lines_str.iter().map(|x| match strutils::to_hex(x.borrow()) {
        Ok(x) => x,
        Err(x) => panic!(x)
    }).collect();
    let max_xors = lines.iter().map(|x| {
        let xors = xor::get_max_xors(x.borrow());
        let &(a, _) = xors.get(0).expect("Could not get xor data");
        a
    });
    let mut xored: Vec<Vec<u8>> = lines.iter().zip(max_xors).map(|(l, s)| xor::xor_byte(l.borrow(), s)).collect();
    let max: Vec<u8> = match max_by(xored.borrow(), strutils::get_score_for) {
        Some(x) => x,
        None => panic!("Could not get max score")
    };

    let result = strutils::to_string(max.borrow());
    println!("val: {}", result);
    assert_eq!(result, "Now that the party is jumping\n".to_string());
}

