#![allow(unused)]

use std::collections::{BTreeMap, BTreeSet};

use crate::Solution;
use regex::Regex;

// major idea behind the data structures: I like to avoid special cases for negative
// values or "out of bounds" checks
// - use signed values for rows and colums
// - also parse the symbol locations and do not perform lookups in the input arrays
#[derive(Debug)]
struct Number {
    num: u32,
    row: i32,
    start: i32,
    end: i32,
}
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
struct Symbol(i32, i32);

pub fn solve(input: &str) -> Solution {
    let (numbers, symbols, mut gears) = parse(input);

    // first puzzle
    let sol1: u32 = numbers
        .iter()
        .filter(|n| check_adjacent_symbol(n, &symbols, &mut gears))
        .map(|n| n.num)
        .sum();
    assert_eq!(sol1, 525911);

    //second puzzle
    let sol2: u32 = gears
        .into_values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum();
    assert_eq!(sol2, 75805607);

    Solution {
        one: sol1.to_string(),
        two: sol2.to_string(),
    }
}

fn parse(input: &str) -> (Vec<Number>, BTreeSet<Symbol>, BTreeMap<Symbol, Vec<u32>>) {
    let mut symbols: BTreeSet<Symbol> = BTreeSet::new();
    let mut gears: BTreeMap<Symbol, Vec<u32>> = BTreeMap::new();
    let mut numbers: Vec<Number> = Vec::new();
    let re_numbers = Regex::new(r"[0-9]+").unwrap();
    let re_symbols = Regex::new(r"[^0-9.]").unwrap();

    for (row, line) in input.split_terminator('\n').enumerate() {
        // numbers
        for m in re_numbers.find_iter(line) {
            numbers.push(Number {
                num: m.as_str().parse::<u32>().unwrap(),
                row: row as i32,
                start: m.start() as i32,
                end: m.end() as i32 - 1, // m.end() is the offset after the match
            });
        }
        // symbols
        for m in re_symbols.find_iter(line) {
            let sym = Symbol(row as i32, m.start() as i32);
            symbols.insert(sym.clone());
            if m.as_str().starts_with('*') {
                gears.insert(sym, Vec::new());
            }
        }
    }
    (numbers, symbols, gears)
}

fn check_adjacent_symbol(
    num: &Number,
    symbols: &BTreeSet<Symbol>,
    gears: &mut BTreeMap<Symbol, Vec<u32>>,
) -> bool {
    let mut retval = false;
    // top and bottom row
    for y in num.start - 1..=num.end + 1 {
        let sym_above = Symbol(num.row - 1, y);
        let sym_below = Symbol(num.row + 1, y);
        if (symbols.contains(&sym_above) || symbols.contains(&sym_below)) {
            retval = true;
        }
        if let Some(nums) = gears.get_mut(&sym_above) {
            nums.push(num.num);
        }
        if let Some(nums) = gears.get_mut(&sym_below) {
            nums.push(num.num);
        }
    }
    // left and right
    let sym_left = Symbol(num.row, num.start - 1);
    let sym_right = Symbol(num.row, num.end + 1);
    if (symbols.contains(&sym_left) || symbols.contains(&sym_right)) {
        retval = true;
    }
    if let Some(nums) = gears.get_mut(&sym_left) {
        nums.push(num.num);
    }
    if let Some(nums) = gears.get_mut(&sym_right) {
        nums.push(num.num);
    }
    retval
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let (numbers, symbols, mut gears) = parse(input);
        let sol1: u32 = numbers
            .iter()
            .filter(|n| check_adjacent_symbol(n, &symbols, &mut gears))
            .map(|n| n.num)
            .sum();
        assert_eq!(sol1, 4361);

        // gears has been updated as a side effect of the check_adjacent_symbol calls
        let sol2: u32 = gears
            .into_values()
            .filter(|v| v.len() == 2)
            .map(|v| v[0] * v[1])
            .sum();
        assert_eq!(sol2, 467835);
    }
}
