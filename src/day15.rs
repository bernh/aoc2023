#![allow(unused)]

use crate::Solution;
use regex::Regex;

pub fn solve(input: &str) -> Solution {
    // first puzzle
    let sol1: u32 = input.trim_end().split(',').into_iter().map(hash_str).sum();
    assert_eq!(sol1, 509167);

    //second puzzle
    let sol2: u32 = 0;

    Solution {
        one: sol1.to_string(),
        two: sol2.to_string(),
    }
}

fn hash_str(input: &str) -> u32 {
    let mut hash = 0;
    let mut byte_buffer: [u8; 1] = [0; 1];
    for x in input.chars() {
        assert!(x.is_ascii());
        // Determine the ASCII code for the current character of the string.
        x.encode_utf8(&mut byte_buffer); // would panic if it does not fit into single byte buffer
                                         // Increase the current value by the ASCII code you just determined.
        hash += byte_buffer[0] as u32;
        // Set the current value to itself multiplied by 17.
        hash *= 17;
        // Set the current value to the remainder of dividing itself by 256.
        hash = hash % 256;
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(hash_str("HASH"), 52);
    }

    #[test]
    fn example_2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let sol1: u32 = input.split(',').into_iter().map(hash_str).sum();
    }
}
