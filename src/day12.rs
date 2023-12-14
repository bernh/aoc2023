#![allow(unused)]

use crate::Solution;
use itertools::Itertools;
use regex::Regex;
use std::iter::zip;

pub fn solve(input: &str) -> Solution {
    let lines: Vec<&str> = input.lines().collect();
    let springs: Vec<_> = lines.iter().map(|l| parse(l)).collect();

    // first puzzle
    let sol1: u32 = springs.iter().map(|s| try_combinations(s)).sum();

    //second puzzle
    let sol2: u32 = 0;

    Solution {
        one: sol1.to_string(),
        two: sol2.to_string(),
    }
}

#[derive(Debug)]
struct Springs {
    cond: Vec<char>,
    unknown: Vec<usize>,
    groups: Vec<usize>,
}

fn parse(line: &str) -> Springs {
    let (cond_str, groups_str) = line.split_ascii_whitespace().collect_tuple().unwrap();
    let cond: Vec<char> = cond_str.chars().collect();
    let unknown = cond
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| if c == '?' { Some(i) } else { None })
        .collect();
    let groups = groups_str
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    Springs {
        cond,
        unknown,
        groups,
    }
}

// check if a condition string (no ?) matches the groups
fn check(cond: &Vec<char>, groups: &Vec<usize>) -> bool {
    let mut in_group: bool = false;
    let mut found_groups: Vec<usize> = Vec::new();
    let mut size = 0;

    for &c in cond.iter() {
        if in_group {
            if c == '#' {
                size += 1;
            } else {
                found_groups.push(size);
                size = 0;
                in_group = false;
            }
        } else {
            if c == '#' {
                in_group = true;
                size += 1;
            } else {
                ()
            }
        }
    }
    if in_group {
        found_groups.push(size);
    }

    if groups.len() == found_groups.len() {
        zip(groups, found_groups).all(|(&a, b)| a == b)
    } else {
        false
    }
}

fn try_combinations(s: &Springs) -> u32 {
    let missing: usize =
        s.groups.iter().sum::<usize>() - s.cond.iter().filter(|x| **x == '#').count();
    // use combinations from itertools to get all possible damaged spring placements
    let mut valid_combinations = 0;

    for replacements in s.unknown.iter().combinations(missing) {
        // create a replacement Vec
        let mut cond = s.cond.to_vec();
        for (i, c) in cond.iter_mut().enumerate() {
            if s.unknown.contains(&i) {
                if replacements.contains(&&i) {
                    *c = '#';
                } else {
                    *c = '.';
                }
            }
        }
        if check(&cond, &s.groups) {
            valid_combinations += 1;
        }
    }
    valid_combinations
}

fn unfold(s: &Springs) -> Springs {
    let mut cond: Vec<char> = Vec::new();
    let mut groups: Vec<usize> = Vec::new();

    for i in (0..5) {
        cond.extend(&s.cond);
        cond.push('?');
        groups.extend(&s.groups);
    }
    _ = cond.pop(); // remove last '?'

    let unknown = cond
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| if c == '?' { Some(i) } else { None })
        .collect();
    Springs {
        cond,
        unknown,
        groups,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1";
        let lines: Vec<_> = input.lines().collect();
        let springs: Vec<_> = lines.iter().map(|l| parse(l)).collect();
        assert!(springs.iter().map(|s| check(&s.cond, &s.groups)).all(|x| x));
    }

    #[test]
    fn example_2() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let lines: Vec<_> = input.lines().collect();
        let springs: Vec<_> = lines.iter().map(|l| parse(l)).collect();
        let sol1: u32 = springs.iter().map(|s| try_combinations(s)).sum();
        assert_eq!(sol1, 21);

        // let springs_2: Vec<_> = springs.iter().map(|s| unfold(s)).collect();
        // let sol2: u32 = springs_2.iter().map(|s| try_combinations(s)).sum();
        // assert_eq!(sol2, 525152);
    }
}
