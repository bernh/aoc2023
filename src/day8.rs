#![allow(unused)]

use std::collections::{BTreeMap, HashSet};

use crate::Solution;
use prime_factorization::Factorization;

pub fn solve(input: &str) -> Solution {
    // first puzzle
    let (directions, network) = parse(input);
    let sol1 = find_way("AAA", directions, &network);
    assert_eq!(sol1, 21409);

    //second puzzle
    let sol2 = find_way_2_alternative(directions, &network);
    assert_eq!(sol2, 21165830176709);

    Solution {
        one: sol1.to_string(),
        two: sol2.to_string(),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Node<'a> {
    location: &'a str,
    left: &'a str,
    right: &'a str,
}

fn parse(input: &str) -> (&str, BTreeMap<&str, Node>) {
    let (directions, network_str) = input.split_once("\n\n").unwrap();
    let mut network = BTreeMap::new();
    for l in network_str.lines() {
        network.insert(
            &l[0..3],
            Node {
                location: &l[0..3],
                left: &l[7..10],
                right: &l[12..15],
            },
        );
    }
    (directions, network)
}

fn find_way(start: &str, directions: &str, network: &BTreeMap<&str, Node>) -> u32 {
    let mut steps = 0;
    let mut dirs = directions.chars().cycle();
    let mut node = network.get(start).unwrap();
    loop {
        let dir = dirs.next().unwrap();
        node = network
            .get(if dir == 'L' { node.left } else { node.right })
            .unwrap();
        steps += 1;
        if node.location.ends_with("Z") {
            break;
        }
    }
    steps
}

fn find_way_2(directions: &str, network: &BTreeMap<&str, Node>) -> u64 {
    let mut steps = 0;
    let mut dirs = directions.chars().cycle();
    let mut nodes: Vec<&Node> = network
        .values()
        .filter(|v| v.location.ends_with("A"))
        .collect();
    loop {
        let dir = dirs.next().unwrap();
        let mut next_nodes: Vec<&Node> = Vec::new();
        for node in nodes.iter() {
            next_nodes.push(
                network
                    .get(if dir == 'L' { node.left } else { node.right })
                    .unwrap(),
            );
        }
        nodes = next_nodes;
        steps += 1;
        if nodes.iter().all(|n| n.location.ends_with("Z")) {
            break;
        }
    }
    steps
}

fn find_way_2_alternative(directions: &str, network: &BTreeMap<&str, Node>) -> u64 {
    // slightly cheating here - took some spoilers (-> least common multiplier)
    // but this was not clear from the instructions (IMO)
    let mut steps = 0;
    let mut nodes: Vec<&Node> = network
        .values()
        .filter(|v| v.location.ends_with("A"))
        .collect();

    let individual_steps: Vec<u32> = nodes
        .iter()
        .map(|n| find_way(n.location, directions, network))
        .collect();

    let mut prime_factors: HashSet<u64> = HashSet::new();
    for s in individual_steps {
        let facts = Factorization::<u64>::run(s as u64);
        for f in facts.factors {
            prime_factors.insert(f);
        }
    }
    prime_factors.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let (directions, network) = parse(input);
        let sol1 = find_way("AAA", directions, &network);
        assert_eq!(sol1, 2);
    }

    #[test]
    fn example_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let (directions, network) = parse(input);
        let sol1 = find_way("AAA", directions, &network);
        assert_eq!(sol1, 6);
    }

    #[test]
    fn example_3() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let (directions, network) = parse(input);
        let sol2 = find_way_2_alternative(directions, &network);
        assert_eq!(sol2, 6);
    }
}
