#![feature(slice_patterns)]

extern crate crypto;

use std::collections::BTreeMap;
use crypto::md5::Md5;
use crypto::digest::Digest;

const INPUT: &'static str = "wtnhxymk";

fn main() {
    let output: String = String::from("");
    let input: String = String::from(INPUT);

    let mut hasher = Md5::new();
    let i: i32 = 0;
    
    while output.len() < 8 {
        let curr = format!("{}{}", &input, i);
        hasher.reset();
        hasher.input_str(&curr);
        let hash = hasher.result_str();

        match hash.split_at(5) {
            ("00000",_) => {
                output += hash[5]
            },
            _ => {}
        };
        
        i += 1;
    }
}
