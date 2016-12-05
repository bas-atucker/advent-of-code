#![feature(slice_patterns)]

extern crate md5;
use std::collections::BTreeMap;

const INPUT: &'static str = "wtnhxymk";

fn main() {
    let input: String = String::from(INPUT);
    let mut result: BTreeMap<u8, String> = BTreeMap::new();
    let mut i: i32 = 0;
    
    while result.len() < 8 {
        let curr = format!("{}{}", &input, i);
        let digest = md5::compute(curr.as_bytes());

        if digest[0] == 0 && digest[1] == 0 && (digest[2] & 0xF0)==0 {
            let pos = digest[2] & 0x0F;

            if pos < 8 && !result.contains_key(&pos) {
                result.insert(pos, format!("{0:x}", digest[3] >> 4));
            }
        };

        i += 1;
    }

    for (_, val) in result {
        print!("{}", val);
    }
}
