#![allow(unused)]

// solution for this day is overly complicated! Finding reflections on each
// line or row and finding commong reflections afterwards is not necessary.
// Just check for two lines next to each other that are equal

use std::{collections::HashSet, iter::zip};

use crate::Solution;
use regex::Regex;

pub fn solve(input: &str) -> Solution {
    let lines: Vec<&str> = input.lines().collect();
    let patterns: Vec<_> = input.split("\n\n").map(|p| parse(p)).collect();

    // first puzzle
    let sol1: u32 = patterns.iter().map(|p| reflection_score(p)).sum();
    assert_eq!(sol1, 27505);

    //second puzzle
    let sol2: u32 = 0;

    Solution {
        one: sol1.to_string(),
        two: sol2.to_string(),
    }
}

#[derive(Debug)]
struct Pattern {
    tiles: Vec<Vec<char>>,
}

fn parse(pattern: &str) -> Pattern {
    Pattern {
        tiles: pattern
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect(),
    }
}

fn reflection_score(p: &Pattern) -> u32 {
    let line_indices: Vec<HashSet<usize>> = p.tiles.iter().map(|l| find_reflections(l)).collect();
    let row_indices: Vec<HashSet<usize>> = (0..p.tiles[0].len())
        .map(|i| {
            (0..p.tiles.len())
                .map(|r| p.tiles[r][i])
                .collect::<Vec<char>>()
        })
        .map(|s| find_reflections(&s))
        .collect();

    if let Some(index) = find_common_reflection(&line_indices) {
        return index as u32;
    }
    if let Some(index) = find_common_reflection(&row_indices) {
        return index as u32 * 100;
    }
    // dbg!(p, &line_indices, &row_indices);
    panic!("no reflection found!");
}

fn find_common_reflection(indices: &Vec<HashSet<usize>>) -> Option<usize> {
    // find the index that is a reflection in all lines
    let mut intersection: HashSet<usize> = HashSet::new();

    let common_index: Vec<usize> = indices[0]
        .clone()
        .into_iter()
        .filter(|c| indices[1..].iter().all(|s| s.contains(c)))
        .collect();
    if common_index.len() > 0 {
        assert_eq!(1, common_index.len());
        Some(common_index[0])
    } else {
        None
    }
}

fn find_reflections(seq: &[char]) -> HashSet<usize> {
    // assumption is that there may be multiple reflections. A line starting with
    // two equal characters would already be a reflection!
    let mut indices: HashSet<usize> = HashSet::new();
    for i in 1..seq.len() {
        if seq[0..i]
            .iter()
            .rev()
            .zip(seq[i..].iter())
            .all(|(a, b)| *a == *b)
        {
            indices.insert(i);
        }
    }

    /*
    for i in 1..seq.len() {
        dbg!(seq[0..i]
            .iter()
            .rev()
            .zip(seq[i..].iter())
            // .all(|(a, b)| *a == *b)
            .collect::<Vec<_>>());
    }
            */

    // dbg!(seq, &indices);
    indices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines() {
        let line = "##..#.##";
        let refl = find_reflections(&line.chars().collect::<Vec<char>>());
        dbg!(refl);
    }

    #[test]
    #[ignore]
    fn example_1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let patterns: Vec<_> = input.split("\n\n").map(|p| parse(p)).collect();
        let sol1: u32 = patterns.iter().map(|p| reflection_score(p)).sum();
        assert_eq!(sol1, 405);
    }
}

/*

.#.#....#.#.###
...#....#...#..
....####....#..
.#.######.#..##
###..##..###...
.#...##...#.###
..##....##.....
.#...##...#..##
..##....##..#..
.##########....
##.#.##.#.##...
....####.....##
##...##...##.##
##.#.##.#.##.##
...#....#...###
##...##...#####
....#..........

*/
