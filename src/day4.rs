#![allow(unused)]

use core::array;
use std::{error::Error, str::FromStr};

use crate::Solution;
use regex::Regex;

#[derive(Debug)]
struct Card {
    id: u32,
    winning: Vec<u32>,
    actual: Vec<u32>,
    num: u32,
}

impl Card {
    fn score_1(self: &Self) -> u32 {
        let count = self
            .actual
            .iter()
            .map(|a| self.winning.contains(a))
            .filter(|x| *x)
            .count();
        if count == 0 {
            0
        } else {
            2_u32.pow(count as u32 - 1)
        }
    }

    fn score_2(self: &Self) -> u32 {
        self.actual
            .iter()
            .map(|a| self.winning.contains(a))
            .filter(|x| *x)
            .count() as u32
    }
}

#[derive(Debug)]
struct ParseCardError;

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id_nums: Vec<&str> = s.split(':').collect();
        let w_a: Vec<&str> = id_nums[1].split('|').collect();
        Ok(Card {
            id: id_nums[0]
                .split(' ')
                .filter(|p| !p.is_empty())
                .collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap(),
            winning: w_a[0]
                .split(' ')
                .filter(|p| !p.is_empty())
                .map(|n| n.parse::<u32>().unwrap())
                .collect(),
            actual: w_a[1]
                .split(' ')
                .filter(|p| !p.is_empty())
                .map(|n| n.parse::<u32>().unwrap())
                .collect(),
            num: 1,
        })
    }
}

pub fn solve(input: &str) -> Solution {
    let lines: Vec<&str> = input.split_terminator('\n').collect();
    let mut cards: Vec<Card> = lines.iter().map(|l| Card::from_str(l).unwrap()).collect();

    // first puzzle
    let sol1: u32 = cards.iter().map(|c| c.score_1()).sum();
    assert_eq!(sol1, 21213);

    //second puzzle
    win_cards(&mut cards);
    let sol2: u32 = cards.iter().map(|c| c.num).sum();
    assert_eq!(sol2, 8549735);

    Solution {
        one: sol1.to_string(),
        two: sol2.to_string(),
    }
}

fn win_cards(cards: &mut Vec<Card>) {
    for i in (0..cards.len()) {
        for offset in (1..=cards[i].score_2()) {
            cards[i + offset as usize].num += cards[i].num;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_line_1() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::from_str(line).unwrap();
        assert_eq!(card.score_1(), 8);
    }

    #[test]
    fn example_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let sol1: u32 = input
            .split_terminator('\n')
            .map(|l| Card::from_str(l).unwrap())
            .map(|c| c.score_1())
            .sum();
        assert_eq!(sol1, 13);
    }

    #[test]
    fn parse_single_line_2() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::from_str(line).unwrap();
        assert_eq!(card.score_2(), 4);
    }

    #[test]
    fn example_2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let mut cards: Vec<Card> = input
            .split_terminator('\n')
            .map(|l| Card::from_str(l).unwrap())
            .collect();
        win_cards(&mut cards);
        let sol2: u32 = cards.iter().map(|c| c.num).sum();
        assert_eq!(sol2, 30);
    }
}
