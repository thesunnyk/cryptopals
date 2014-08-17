
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

pub fn get_max_xors(x: &[u8]) -> Vec<(u8, uint)> {
    let mut data : Vec<(u8, uint)> = range_inclusive(0 as u8, 255 as u8)
        .map(|a| (a, get_xor_score(x, a))).collect();
    data.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    data
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

fn split_blocks(v : &[u8], n : uint) -> Vec<u8> {
    v.iter().enumerate().filter_map(|(i, &x)| if (i / n) % 2 == 0 { Some(x) } else { None }).collect()
}

pub fn key_len_score(x: Vec<u8>, y: uint) -> uint {
    use std::cmp::min;
    let first = split_blocks(x.as_slice(), y);
    let second = split_blocks(x.slice(y, x.len()), y);

    let minlen = min(first.len(), second.len());
    // Meh. *1000 should be accurate enough.
    hamming_distance(first.slice(0, minlen), second.slice(0, minlen)) * 1000 / first.len()
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

