
use std::iter::repeat;
use std::ops::Index;
use std::borrow::Borrow;

use strutils;

struct Repeatedly<T> {
    items: Vec<T>,
    offset: usize
}

impl<T: Clone> Iterator for Repeatedly<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let curOffset = self.offset;
        self.offset = (self.offset + 1) % self.items.len();
        self.items.get(curOffset).map(|x| x.clone())
    }
}

impl<T> Repeatedly<T> {
    fn new(x: Vec<T>) -> Repeatedly<T> { Repeatedly { items: x, offset: 0 } }
}

pub fn xor(x: &[u8], y: &[u8]) -> Vec<u8> {
    x.iter().zip(y.iter()).map(|(i, j)| { *i ^ *j }).collect()
}

pub fn xor_byte(x: &[u8], a: u8) -> Vec<u8> {
    let rep : Vec<u8> = repeat(a).take(x.len()).collect();
    xor(x, rep.borrow())
}

pub fn xor_repeat(x: &[u8], a: Vec<u8>) -> Vec<u8> {
    let rep : Vec<u8> = Repeatedly::new(a).take(x.len()).collect();
    xor(x, rep.borrow())
}

pub fn get_xor_score(x: &[u8], a: u8) -> u64 {
    strutils::get_score_for(xor_byte(x, a).borrow())
}

pub fn get_max_xors(x: &[u8]) -> Vec<(u8, u64)> {
    let mut data : Vec<(u8, u64)> = (0..256).map(|a| (a as u8, get_xor_score(x, a as u8))).collect();
    data.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    data
}

pub fn popcount(x : u8) -> u64 {
    let twocount = (x & 0x55) + ((x >> 1) & 0x55);
    let fourcount = (twocount & 0x33) + ((twocount >> 2) & 0x33);
    ((fourcount & 0x0f) + ((fourcount >> 4) & 0x0f)) as u64
}

pub fn hamming_distance(x: &[u8], y: &[u8]) -> usize {
    assert_eq!(x.len(), y.len());
    xor(x, y).iter().map(|&x| popcount(x) as usize).fold(0, |x, y| x + y)
}

fn split_blocks(v : &[u8], n : usize) -> Vec<u8> {
    v.iter().enumerate().filter_map(|(i, &x)| if ((i as u64) / (n as u64)) % 2 == 0 { Some(x) } else { None }).collect()
}

pub fn key_len_score(x: Vec<u8>, y: usize) -> usize {
    use std::cmp::min;
    let first = split_blocks(x.borrow(), y);
    let second = split_blocks(x.index(y..x.len()), y);

    let minlen = min(first.len(), second.len());
    // Meh. *1000 should be accurate enough.
    hamming_distance(first.index(0 as usize..minlen), second.index(0 as usize..minlen)) * 1000 / first.len()
}

#[test]
fn test_hamming() {
    let ham1 = strutils::from_ascii_string("this is a test".to_string());
    let ham2 = strutils::from_ascii_string("wokka wokka!!!".to_string());
    let distance = xor::hamming_distance(ham1.borrow(), ham2.borrow());
    assert_eq!(distance, 37);
}

#[test]
fn test_score() {
    assert_eq!(strutils::get_score('e'.to_ascii().to_byte()), 13);
    assert_eq!(strutils::get_score('s'.to_ascii().to_byte()), 6);
    let helloscore = 6 + 13 + 4 + 4 + 8 + 0;
    let helloascii = "helloq".to_string().into_ascii();
    let hellostr : Vec<u8> = helloascii.iter().map(|x| x.to_byte()).collect();
    assert_eq!(strutils::get_score_for(hellostr.borrow()), helloscore);
}

