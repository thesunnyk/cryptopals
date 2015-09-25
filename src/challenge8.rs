
extern crate openssl;
extern crate rustc_serialize;

mod fileutils;
mod strutils;
mod base64;

use openssl::crypto::symm;
use std::borrow::Borrow;
use rustc_serialize::base64::FromBase64;

fn main() {
    let key = "YELLOW SUBMARINE".to_string().into_bytes();
    let lines_str = fileutils::read_file("challenge_8.txt".to_string());
    let data: Vec<Vec<u8>> = lines_str.iter().map(|x| match x.from_base64() {
        Err(x) => panic!(x),
        Ok(x) => x
    }).collect();

    println!("*** {}", data.len());

    let out = data.iter().filter(|x| repeats(x) > 0);

    for x in out {
        let unpacked = base64::to_base64_unpacked(x.clone());
        println!("***** {}", base64::to_base64_string(unpacked));
    }
}

fn repeats(data: &Vec<u8>) -> u32 {
    let all_chunks = split_chunks(data);

    let mut x = 0;

    for i in 0..all_chunks.len() {
        for j in (i+1)..all_chunks.len() {
            if (compare(&all_chunks[i], &all_chunks[j])) {
                x = x + 1
            }
        }
    }
    x
}

fn split_chunks(data: &Vec<u8>) -> Vec<Vec<u8>> {
    let chunks = data.chunks(4);
    chunks.map(|x| x.iter().map(|y| *y).collect()).collect()
}

fn compare(d1: &[u8], d2: &[u8]) -> bool {
    for i in 0..d1.len() {
        if d1[i] != d2[i] {
            return false
        }
    }
    return true
}
