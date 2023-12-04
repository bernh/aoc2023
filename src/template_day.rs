#![allow(unused)]

use crate::Solution;
use regex::Regex;

pub fn solve(input: String) -> Solution {
    let lines: Vec<&str> = input.split_terminator('\n').collect();

    // first puzzle
    let sol1: u32 = 0;

    //second puzzle
    let sol2: u32 = 0;

    Solution {
        one: sol1.to_string(),
        two: sol2.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_line_1() {
        unimplemented!()
    }

    #[test]
    fn example_1() {
        unimplemented!()
    }
}
