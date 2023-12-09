#![allow(unused)]

use crate::Solution;

pub fn solve(input: &str) -> Solution {
    let lines: Vec<&str> = input.lines().collect();

    // first puzzle
    let seqs_1: Vec<_> = lines
        .iter()
        .map(|l| parse(l))
        .map(|s| extrapolate_seq(s))
        .collect();
    let sol1: i32 = seqs_1.iter().map(|x| x.last().unwrap()).sum();
    assert_eq!(sol1, 2098530125);

    //second puzzle
    let seqs_2: Vec<_> = lines
        .iter()
        .map(|l| parse(l))
        .map(|s| s.into_iter().rev().collect())
        .map(|s| extrapolate_seq(s))
        .collect();
    let sol2: i32 = seqs_2.iter().map(|x| x.last().unwrap()).sum();
    assert_eq!(sol2, 1016);

    Solution {
        one: sol1.to_string(),
        two: sol2.to_string(),
    }
}

fn parse(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn derivative_seq(in_seq: &Vec<i32>) -> Vec<i32> {
    in_seq.windows(2).map(|w| w[1] - w[0]).collect()
}

fn extrapolate_seq(in_seq: Vec<i32>) -> Vec<i32> {
    let mut extrapolated = in_seq.clone();
    if in_seq.iter().all(|x| *x == 0) {
        extrapolated.push(0);
    } else {
        extrapolated
            .push(in_seq.last().unwrap() + extrapolate_seq(derivative_seq(&in_seq)).last().unwrap())
    }
    extrapolated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let lines = vec!["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];
        let seqs: Vec<_> = lines
            .iter()
            .map(|l| parse(l))
            .map(|s| extrapolate_seq(s))
            .collect();
        // assert_eq!(*seqs[0].last().unwrap(), 18);
        // assert_eq!(*seqs[1].last().unwrap(), 28);
        // assert_eq!(*seqs[2].last().unwrap(), 68);
        let sol1: i32 = seqs.iter().map(|x| x.last().unwrap()).sum();
        assert_eq!(sol1, 114);
    }
}
