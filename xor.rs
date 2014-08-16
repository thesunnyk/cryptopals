
use std::iter::Repeat;
use std::iter::range_inclusive;
use strutils;

pub fn xor(x: &[u8], y: &[u8]) -> Vec<u8> {
    x.iter().zip(y.iter()).map(|(i, j)| { *i ^ *j }).collect()
}

pub fn xor_byte(x: &[u8], a: u8) -> Vec<u8> {
    let rep : Vec<u8> = Repeat::new(a).take(x.len()).collect();
    xor(x, rep.as_slice())
}

pub fn get_xor_score(x: &[u8], a: u8) -> uint {
    strutils::get_score_for(xor_byte(x, a).as_slice())
}

pub fn find_max_xor(x: &[u8]) -> Option<u8> {
    range_inclusive(0 as u8, 255 as u8).max_by(|&a| get_xor_score(x, a))
}

#[test]
fn test_score() {
    assert!(strutils::get_score('e'.to_ascii().to_byte()) == 13);
    assert!(strutils::get_score('s'.to_ascii().to_byte()) == 6);
    let helloscore = 6 + 13 + 4 + 4 + 8 + 0;
    let helloascii = "helloq".to_string().into_ascii();
    let hellostr : Vec<u8> = helloascii.iter().map(|x| x.to_byte()).collect();
    assert!(strutils::get_score_for(hellostr.as_slice()) == helloscore);
}

