#![allow(unused)]

use std::iter::zip;

use crate::Solution;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct Race {
    time: u64,
    record: u64,
}

pub fn solve(input: &str) -> Solution {
    let lines: Vec<&str> = input.split_terminator('\n').collect();

    // first puzzle
    let races = parse_1(&lines);
    let sol1 = races
        .iter()
        .map(|r| num_calc_winning_times(r))
        .fold(1, |acc, x| acc * x);
    assert_eq!(sol1, 1624896);

    //second puzzle
    let races = parse_2(&lines);
    let sol2 = num_calc_winning_times(&races[0]);
    assert_eq!(sol2, 32583852);

    Solution {
        one: sol1.to_string(),
        two: sol2.to_string(),
    }
}

fn parse_1(lines: &Vec<&str>) -> Vec<Race> {
    // for this day it would be faster to manually enter the input ;-)
    let times: Vec<_> = lines[0].split_ascii_whitespace().collect();
    let records: Vec<_> = lines[1].split_ascii_whitespace().collect();
    zip(&times[1..], &records[1..])
        .map(|(t, r): (&&str, &&str)| Race {
            time: t.parse::<u64>().unwrap(),
            record: r.parse::<u64>().unwrap(),
        })
        .collect()
}

fn parse_2(lines: &Vec<&str>) -> Vec<Race> {
    // re-use parse_1 code
    let times: Vec<_> = lines[0].split_ascii_whitespace().collect();
    let records: Vec<_> = lines[1].split_ascii_whitespace().collect();
    // concat
    let time = times[1..]
        .iter()
        .map(|s| s.to_owned())
        .fold("".to_owned(), |acc, s| acc + s)
        .parse::<u64>()
        .unwrap();
    let record = records[1..]
        .iter()
        .map(|s| s.to_owned())
        .fold("".to_owned(), |acc, s| acc + s)
        .parse::<u64>()
        .unwrap();
    vec![Race { time, record }]
}

fn num_calc_winning_times(race: &Race) -> u64 {
    // t = t_1 + t_2                        total time is t_1 (holding) + t_2 (racing)
    // s = v * t_2
    // s = t_1 * t_2                        with v = t_1
    // s = t_1 * (t - t_2)                  t_1 is what we are looking for
    // t_1 = (t/2) +- sqrt( (t/2)^2 - s)    solving the quadratic equation

    let first_part = race.time as f64 / 2.0;
    let sqrt_part = ((race.time as f64 / 2.0).powf(2.0) - race.record as f64).sqrt();

    let (lower, upper) = (
        (first_part - sqrt_part + 0.0001).ceil() as u64, // f64::EPSILON is not working?
        (first_part + sqrt_part - 0.0001).floor() as u64, //
    );
    upper - lower + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let lines = vec!["Time:      7  15   30", "Distance:  9  40  200"];
        let races = parse_1(&lines);
        assert_eq!(
            Race {
                time: 15,
                record: 40
            },
            races[1]
        );
        assert_eq!(races.len(), 3);
        assert_eq!(num_calc_winning_times(&races[0]), 4);
        assert_eq!(num_calc_winning_times(&races[1]), 8);
        assert_eq!(num_calc_winning_times(&races[2]), 9);
        let sol1 = races
            .iter()
            .map(|r| num_calc_winning_times(r))
            .fold(1, |acc, x| acc * x);
        assert_eq!(sol1, 288);
    }

    #[test]
    fn example_2() {
        let lines = vec!["Time:      7  15   30", "Distance:  9  40  200"];
        let races = parse_2(&lines);
        let sol2 = num_calc_winning_times(&races[0]);
        assert_eq!(sol2, 71503);
    }
}
