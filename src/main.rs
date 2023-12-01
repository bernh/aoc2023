// use clap::{arg, Parser};
use std::fs;

fn main() {
    // let cli = Cli::parse();
    let solutions: aoc2023::Solution = aoc2023::solve(0, input("inputs/day0.txt"));
    println!(
        "Solutions for day 0 are: {} and {}",
        solutions.one, solutions.two
    );
}

// simple but fragile file read, just panics if something goes wrong
pub fn input(file: &str) -> String {
    fs::read_to_string(file).unwrap_or("reading input file went wrong".to_string())
}
