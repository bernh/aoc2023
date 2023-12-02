// use clap::{arg, Parser};
use std::fs;
use std::time::Instant;

fn main() {
    // let cli = Cli::parse();
    let now = Instant::now();
    let solutions: aoc2023::Solution = aoc2023::solve(2, input("inputs/day2.txt"));
    let elapsed = now.elapsed();
    println!(
        "Solutions for day 0 are: {} and {}",
        solutions.one, solutions.two
    );
    println!("Time elapsed: {:.2?}", elapsed)
}

// simple but fragile file read, just panics if something goes wrong
pub fn input(file: &str) -> String {
    fs::read_to_string(file).unwrap_or("reading input file went wrong".to_string())
}
