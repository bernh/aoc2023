#![allow(unused)]

// XXX Incorrect due to problem with the "3 straights in a row"!
//     Problem happens when there are multiple equivalent ways to reach a certain
//     point with the same score. These multiple ways may effect the future due
//     to the straight row checks.
// Solution: keep at least the last 4 steps to reach a certain node as part of the
//     explore list. Don't feel like doing this.

use std::collections::BTreeMap;

use crate::Solution;
use regex::Regex;

pub fn solve(input: &str) -> Solution {
    let lines: Vec<&str> = input.lines().collect();

    // first puzzle
    let mut map = Map::parse(input);
    let goal = Coord(map.height - 1, map.width - 1);
    map.a_star(Coord(0, 0), goal);
    map.print_path();
    let sol1 = map.nodes[goal.0][goal.1].to_score;

    //second puzzle
    let sol2: u32 = 0;

    Solution {
        one: sol1.unwrap().to_string(),
        two: sol2.to_string(),
    }
}

#[derive(Debug)]
struct Map {
    nodes: Vec<Vec<Node>>,
    width: usize,
    height: usize,
}

#[derive(Debug)]
struct Node {
    cost: u32,                // cost according to input map
    to_score: Option<u32>,    // cheapest score found so far
    from_score: Option<u32>,  // cheapest score found so far
    came_from: Option<Coord>, // predecessor for cheapest score so far
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord(usize, usize);

impl Map {
    fn parse(input: &str) -> Self {
        let nodes: Vec<Vec<Node>> = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| Node {
                        cost: c.to_digit(10).unwrap(),
                        to_score: None,
                        from_score: None,
                        came_from: None,
                    })
                    .collect()
            })
            .collect();
        let width = nodes[0].len();
        let height = nodes.len();
        Map {
            nodes,
            width,
            height,
        }
    }

    fn h(&self, c: Coord) -> u32 {
        // heuristic to estimate the cheapest path from c till the destination
        (self.height - c.0) as u32 + (self.width - c.1) as u32
    }

    fn neighbors(&self, c: Coord) -> Vec<Coord> {
        let node = &self.nodes[c.0][c.1];
        let mut neighbors = Vec::new();

        // the 4 in a row check is not needed for c.0 or c.1 <= 2
        let mut previous = Coord(1, 1); // initialize to something not in a row
        let mut pre_previous = Coord(2, 2); // initialize to something not in a row
        let mut pre_pre_previous = Coord(3, 3); // initialize to something not in a row
        let straight_check = (c.0 > 2) || (c.1 > 2);
        if straight_check {
            previous = node.came_from.unwrap();
            pre_previous = self.nodes[previous.0][previous.1].came_from.unwrap();
            pre_pre_previous = self.nodes[pre_previous.0][pre_previous.1]
                .came_from
                .unwrap();
        }

        if (c.1 < (self.width - 1))
            && ((previous.0 != c.0) || (pre_previous.0 != c.0) || (pre_pre_previous.0 != c.0))
        {
            neighbors.push(Coord(c.0, c.1 + 1));
        }
        if (c.1 > 0)
            && ((previous.0 != c.0) || (pre_previous.0 != c.0) || (pre_pre_previous.0 != c.0))
        {
            neighbors.push(Coord(c.0, c.1 - 1));
        }
        if (c.0 > 0)
            && ((previous.1 != c.1) || (pre_previous.1 != c.1) || (pre_pre_previous.1 != c.1))
        {
            neighbors.push(Coord(c.0 - 1, c.1));
        }
        if (c.0 < (self.height - 1))
            && ((previous.1 != c.1) || (pre_previous.1 != c.1) || (pre_pre_previous.1 != c.1))
        {
            neighbors.push(Coord(c.0 + 1, c.1));
        }
        // dbg!(c, previous, pre_previous, pre_pre_previous, &neighbors);
        neighbors
    }

    fn a_star(&mut self, start: Coord, goal: Coord) {
        // The set of discovered nodes that may need to be (re-)expanded.
        // Initially, only the start node is known.
        let mut explore: Vec<Coord> = vec![start];
        self.nodes[start.0][start.1].cost = 0;
        self.nodes[start.0][start.1].to_score = Some(0);
        self.nodes[start.0][start.1].from_score = Some(self.h(start));

        while !explore.is_empty() {
            // next to explore is the node with the lowest score
            explore.sort_unstable_by_key(|c| u32::MAX - self.nodes[c.0][c.1].from_score.unwrap());
            let c = explore.pop().unwrap();
            if c == goal {
                return;
            } else {
                for n in self.neighbors(c) {
                    // dbg!(c, n);
                    let tentative_score =
                        self.nodes[c.0][c.1].to_score.unwrap() + self.nodes[n.0][n.1].cost;
                    if let Some(to_score) = self.nodes[n.0][n.1].to_score {
                        if tentative_score < to_score {
                            // cheaper path found
                            println!("set cheaper score {} for {:?}", tentative_score, n);
                            self.nodes[n.0][n.1].to_score = Some(tentative_score);
                            self.nodes[n.0][n.1].came_from = Some(c);
                            self.nodes[n.0][n.1].from_score = Some(tentative_score + self.h(n));
                            if !explore.contains(&n) {
                                explore.push(n);
                            }
                        }
                    } else {
                        // no to_score yet, set it
                        println!("set score {} for {:?}", tentative_score, n);
                        self.nodes[n.0][n.1].to_score = Some(tentative_score);
                        self.nodes[n.0][n.1].came_from = Some(c);
                        self.nodes[n.0][n.1].from_score = Some(tentative_score + self.h(n));
                        if !explore.contains(&n) {
                            explore.push(n);
                        }
                    }
                }
            }
        }
    }

    fn reconstruct_path(&self, goal: Coord) -> Vec<Coord> {
        let mut node = goal;
        let mut path = vec![goal];
        loop {
            if let Some(previous) = self.nodes[node.0][node.1].came_from {
                path.push(previous);
                node = previous;
            } else {
                break;
            }
        }
        path.into_iter().rev().collect()
    }

    fn print_path(&self) {
        let path = self.reconstruct_path(Coord(self.height - 1, self.width - 1));
        for (x, l) in self.nodes.iter().enumerate() {
            let mut line: String = "".to_owned();
            for (y, c) in l.iter().enumerate() {
                if path.contains(&Coord(x, y)) {
                    line = line.clone() + "#";
                } else {
                    if let Some(s) = self.nodes[x][y].to_score {
                        line = line.clone() + ".";
                    } else {
                        line = line.clone() + " ";
                    }
                }
            }
            println!("{}", line);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let mut map = Map::parse(input);
        let goal = Coord(map.height - 1, map.width - 1);
        map.a_star(Coord(0, 0), goal);
        map.print_path();
        // dbg!(&map);
        let sol1 = map.nodes[goal.0][goal.1].to_score;
        assert_eq!(sol1, Some(102));
    }
}
