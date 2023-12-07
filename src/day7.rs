#![allow(unused)]

use std::{cmp::Ordering, collections::BTreeMap, iter::zip};

use crate::Solution;
use regex::Regex;

pub fn solve(input: &str) -> Solution {
    let lines: Vec<&str> = input.split_terminator('\n').collect();

    // first puzzle
    let mut hands: Vec<Hand> = lines.iter().map(|l| Hand::from_str(l)).collect();
    let sol1 = total_winnings(&mut hands);
    assert_eq!(sol1, 250946742);
    // let sol1 = 0;

    //second puzzle
    for hand in hands.iter_mut() {
        hand.adjust_for_puzzle_2();
    }
    let sol2 = total_winnings(&mut hands);
    assert_eq!(sol2, 251824095);

    Solution {
        one: sol1.to_string(),
        two: sol2.to_string(),
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
enum Card {
    Jack2, // only used for puzzle 2
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfaKind,
    FullHouse,
    FourOfaKind,
    FiveOfaKind,
}

impl Card {
    fn from_char(input: char) -> Self {
        match input {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("Invalid card in input!"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    htype: HandType,
    bid: u32,
    map: BTreeMap<Card, i32>, // also save raw input, used for puzzle 2
}

impl Hand {
    fn from_str(input: &str) -> Self {
        let cards: [Card; 5] = input[0..5]
            .chars()
            .map(|c| Card::from_char(c))
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap();
        let bid = input[6..].trim_end().parse::<u32>().unwrap();

        // compute hand type
        let mut map = BTreeMap::from([
            (Card::Ace, 0),
            (Card::King, 0),
            (Card::Queen, 0),
            (Card::Jack, 0),
            (Card::Ten, 0),
            (Card::Nine, 0),
            (Card::Eight, 0),
            (Card::Seven, 0),
            (Card::Six, 0),
            (Card::Five, 0),
            (Card::Four, 0),
            (Card::Three, 0),
            (Card::Two, 0),
        ]);
        for c in cards.iter() {
            if let Some(v) = map.get_mut(c) {
                *v += 1;
            }
        }
        let htype = get_hand_type(&map);
        Hand {
            cards,
            htype,
            bid,
            map,
        }
    }

    fn adjust_for_puzzle_2(&mut self) {
        // remember num of jockers
        let num_jockers: i32 = *self.map.get(&Card::Jack).unwrap();
        // remove Jockers from the map and run htype calculation again
        if let Some(v) = self.map.get_mut(&Card::Jack) {
            *v = 0;
        }
        self.htype = get_hand_type(&self.map);
        // check for each hand type: what can be improved if Jockers are used for something else
        self.htype = match num_jockers {
            0 => self.htype,
            1 => match self.htype {
                HandType::HighCard => HandType::OnePair,
                HandType::OnePair => HandType::ThreeOfaKind, // never go to TwoPairs
                HandType::TwoPair => HandType::FullHouse,
                HandType::ThreeOfaKind => HandType::FourOfaKind,
                HandType::FullHouse => HandType::FourOfaKind,
                _ => HandType::FiveOfaKind,
            },
            2 => match self.htype {
                HandType::HighCard => HandType::ThreeOfaKind,
                HandType::OnePair => HandType::FourOfaKind,
                HandType::TwoPair => HandType::FourOfaKind,
                _ => HandType::FiveOfaKind,
            },
            3 => match self.htype {
                HandType::HighCard => HandType::FourOfaKind,
                _ => HandType::FiveOfaKind,
            },
            4 => HandType::FiveOfaKind,
            5 => HandType::FiveOfaKind,
            _ => panic!("more than 5 Jockers!"),
        };

        // replace Jack with Jocker for fallback card by card comparision
        for card in self.cards.iter_mut() {
            if *card == Card::Jack {
                *card = Card::Jack2;
            }
        }
    }
}

fn get_hand_type(map: &BTreeMap<Card, i32>) -> HandType {
    let pairs = map.values().filter(|&&v| v == 2).count();
    let triples = map.values().filter(|&&v| v == 3).count();
    let quadrupels = map.values().filter(|&&v| v == 4).count();
    let quintuples = map.values().filter(|&&v| v == 5).count();
    match (pairs, triples, quadrupels, quintuples) {
        (0, 0, 0, 0) => HandType::HighCard,
        (1, 0, 0, 0) => HandType::OnePair,
        (2, 0, 0, 0) => HandType::TwoPair,
        (0, 1, 0, 0) => HandType::ThreeOfaKind,
        (1, 1, 0, 0) => HandType::FullHouse,
        (0, 0, 1, 0) => HandType::FourOfaKind,
        (0, 0, 0, 1) => HandType::FiveOfaKind,
        (_, _, _, _) => panic!("impossible hand"),
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.htype != other.htype {
            return self.htype.cmp(&other.htype);
        } else {
            // compare individual cards
            let dif: Vec<(&Card, &Card)> = zip(&self.cards, &other.cards)
                .filter(|(s, o)| s != o)
                .collect();
            return dif[0].0.cmp(&dif[0].1);
        }
    }
}

fn total_winnings(hands: &mut Vec<Hand>) -> u32 {
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index + 1) as u32 * hand.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let lines: Vec<&str> = input.split_terminator('\n').collect();
        let mut hands: Vec<Hand> = lines.iter().map(|l| Hand::from_str(l)).collect();
        let sol1 = total_winnings(&mut hands);
        assert_eq!(sol1, 6440);

        for hand in hands.iter_mut() {
            hand.adjust_for_puzzle_2();
        }
        let sol2 = total_winnings(&mut hands);
        assert_eq!(sol2, 5905);
    }
}
