// this is an implementation for AoC 2022 day 4 to get familar with the
// structure of the puzzels

#![allow(dead_code)]
#![allow(unused_imports)]

mod utils;

use env_logger::Env;
use log::{debug, info, warn};
use regex::Regex;
use std::time::Instant;

// use regex::Regex;
#[derive(Debug, PartialEq)]
struct Pair {
    start_1: u32,
    end_1: u32,
    start_2: u32,
    end_2: u32,
}

fn parse(pair_descr: String) -> Pair {
    let re = Regex::new(r"(?P<s1>[0-9]+)-(?P<e1>[0-9]+),(?P<s2>[0-9]+)-(?P<e2>[0-9]+)").unwrap();
    let caps = re.captures(&pair_descr).unwrap();
    Pair {
        start_1: caps["s1"].parse().unwrap(),
        end_1: caps["e1"].parse().unwrap(),
        start_2: caps["s2"].parse().unwrap(),
        end_2: caps["e2"].parse().unwrap(),
    }
}

fn overlaps(p: &Pair) -> bool {
    (p.start_1 <= p.start_2) && (p.end_1 >= p.end_2)
        || (p.start_2 <= p.start_1) && (p.end_2 >= p.end_1)
}

fn overlaps_partial(p: &Pair) -> bool {
    ((p.start_2 >= p.start_1) && (p.start_2 <= p.end_1))
        || ((p.start_1 >= p.start_2) && (p.start_1 <= p.end_2))
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let start = Instant::now();

    let input = utils::lines("inputs/sample.txt");
    let pairs: Vec<Pair> = input.into_iter().map(|x| parse(x)).collect();

    // first part
    let overlapping_pairs: Vec<&Pair> = pairs.iter().filter(|p| overlaps(p)).collect();
    info!("Solution 1: {}", overlapping_pairs.len());

    // second part
    let partially_overlapping_pairs: Vec<&Pair> =
        pairs.iter().filter(|p| overlaps_partial(p)).collect();
    info!("Solution 2: {}", partially_overlapping_pairs.len());

    info!("Elapsed time: {:.2?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pair() {
        assert_eq!(
            parse("3-4,4-9".to_owned()),
            Pair {
                start_1: 3,
                end_1: 4,
                start_2: 4,
                end_2: 9
            }
        );
    }

    #[test]
    fn solve_sample() {
        let mut input = Vec::new();
        input.push("2-4,6-8".to_owned());
        input.push("2-3,4-5".to_owned());
        input.push("5-7,7-9".to_owned());
        input.push("2-8,3-7".to_owned());
        input.push("6-6,4-6".to_owned());
        input.push("2-6,4-8".to_owned());

        let pairs: Vec<Pair> = input.into_iter().map(|x| parse(x)).collect();

        // first part
        let overlapping_pairs: Vec<&Pair> = pairs.iter().filter(|p| overlaps(p)).collect();
        assert_eq!(overlapping_pairs.len(), 2);

        // second part
        let partially_overlapping_pairs: Vec<&Pair> =
            pairs.iter().filter(|p| overlaps_partial(p)).collect();
        assert_eq!(partially_overlapping_pairs.len(), 4);
    }
}
