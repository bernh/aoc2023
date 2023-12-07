// use clap::{arg, Parser};
use clap::{arg, Parser};
use std::fs;
use std::time::Instant;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // day
    #[arg(short, long, value_name = "DAY")]
    day: Option<usize>,

    // run all days
    #[arg(short, long)]
    all: bool,
}

fn main() {
    let cli = Cli::parse();

    if let Some(day) = cli.day {
        solve(day);
    } else {
        // default
        solve(7);
    }
}

fn solve(day: usize) {
    let now = Instant::now();
    let solutions: aoc2023::Solution =
        aoc2023::solve(day, &input(&format!("inputs/day{}.txt", day)));
    let elapsed = now.elapsed();
    println!(
        "Day {} solutions are: {} and {} ({:.2?} elapsed)",
        day, solutions.one, solutions.two, elapsed
    );
}

// simple but fragile file read, just panics if something goes wrong
fn input(file: &str) -> String {
    fs::read_to_string(file).unwrap_or("reading input file went wrong".to_string())
}
