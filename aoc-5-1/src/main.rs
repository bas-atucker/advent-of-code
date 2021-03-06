#![feature(slice_patterns)]

extern crate md5;

const INPUT: &'static str = "wtnhxymk";

fn main() {
    let mut output: String = String::from("");
    let input: String = String::from(INPUT);

    let mut i: i32 = 0;
    
    while output.len() < 8 {
        let curr = format!("{}{}", &input, i);
        let digest = md5::compute(curr.as_bytes());

        if digest[0] == 0 && digest[1] == 0 && (digest[2] & 0xF0)==0 {
            output += &format!("{0:x}", digest[2] & 0x0F);
            println!("{}", i);
        };

        i += 1;
    }

    println!("{}", output);
}
