
use std::iter::Repeat;
use std::iter::range_inclusive;
use strutils;

struct Repeatedly<T> {
    items: Vec<T>,
    offset: uint
}

impl<T: Clone> Iterator<T> for Repeatedly<T> {
    fn next(&mut self) -> Option<T> {
        let curOffset = self.offset;
        self.offset = (self.offset + 1) % self.items.len();
        Some(self.items.get(curOffset).clone())
    }
}

impl<T> Repeatedly<T> {
    fn new(x: Vec<T>) -> Repeatedly<T> { Repeatedly { items: x, offset: 0 } }
}

pub fn xor(x: &[u8], y: &[u8]) -> Vec<u8> {
    x.iter().zip(y.iter()).map(|(i, j)| { *i ^ *j }).collect()
}

pub fn xor_byte(x: &[u8], a: u8) -> Vec<u8> {
    let rep : Vec<u8> = Repeat::new(a).take(x.len()).collect();
    xor(x, rep.as_slice())
}

pub fn xor_repeat(x: &[u8], a: Vec<u8>) -> Vec<u8> {
    let rep : Vec<u8> = Repeatedly::new(a).take(x.len()).collect();
    xor(x, rep.as_slice())
}

pub fn get_xor_score(x: &[u8], a: u8) -> uint {
    strutils::get_score_for(xor_byte(x, a).as_slice())
}

pub fn find_max_xor(x: &[u8]) -> Option<u8> {
    range_inclusive(0 as u8, 255 as u8).max_by(|&a| get_xor_score(x, a))
}

pub fn popcount(x : u8) -> uint {
    let twocount = (x & 0x55) + ((x >> 1) & 0x55);
    let fourcount = (twocount & 0x33) + ((twocount >> 2) & 0x33);
    ((fourcount & 0x0f) + ((fourcount >> 4) & 0x0f)) as uint
}

pub fn hamming_distance(x: &[u8], y: &[u8]) -> uint {
    assert_eq!(x.len(), y.len());
    xor(x, y).iter().map(|&x| popcount(x)).fold(0, |x, y| x + y)
}

pub fn key_len_score(x: Vec<u8>, y: uint) -> uint {
    assert!(x.len() > y * 2);
    // TODO Don't know why I have to do the funny map :(
    let first : Vec<u8> = x.iter().take(y).map(|&x| x).collect();
    let second : Vec<u8> = x.iter().skip(y).take(y).map(|&x| x).collect();
    // Meh. *1000 should be accurate enough.
    hamming_distance(first.as_slice(), second.as_slice()) * 1000 / first.len()
}

#[test]
fn test_hamming() {
    let ham1 = strutils::from_ascii_string("this is a test".to_string());
    let ham2 = strutils::from_ascii_string("wokka wokka!!!".to_string());
    let distance = xor::hamming_distance(ham1.as_slice(), ham2.as_slice());
    assert_eq!(distance, 37);
}

#[test]
fn test_score() {
    assert_eq!(strutils::get_score('e'.to_ascii().to_byte()), 13);
    assert_eq!(strutils::get_score('s'.to_ascii().to_byte()), 6);
    let helloscore = 6 + 13 + 4 + 4 + 8 + 0;
    let helloascii = "helloq".to_string().into_ascii();
    let hellostr : Vec<u8> = helloascii.iter().map(|x| x.to_byte()).collect();
    assert_eq!(strutils::get_score_for(hellostr.as_slice()), helloscore);
}

