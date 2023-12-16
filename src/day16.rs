#![allow(unused)]

use crate::Solution;
use regex::Regex;

pub fn solve(input: &str) -> Solution {
    let mut map = Map::parse(input);

    // first puzzle
    let start = Pos { x: 0, y: 0 };
    let dir = E;
    map.trace(&start, &dir);
    let sol1 = map.energized();
    assert_eq!(sol1, 7074);

    //second puzzle
    let sol2 = map.find_max_energized();
    assert_eq!(sol2, 7530);

    Solution {
        one: sol1.to_string(),
        two: sol2.to_string(),
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Debug)]
struct Tile {
    symbol: char,
    energized: bool,
    visited: Vec<Direction>, // if we hit a tile in a certain direction already we don't have to
                             // check it again -> this is needed to break loops
}

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    N,
    E,
    S,
    W,
}

use Direction::*;

impl Map {
    fn parse(input: &str) -> Self {
        Map {
            tiles: input
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|c| Tile {
                            symbol: c,
                            energized: false,
                            visited: Vec::new(),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn next_pos(&self, pos: &Pos, dir: &Direction) -> Option<Pos> {
        match dir {
            N => {
                if pos.x > 0 {
                    Some(Pos {
                        x: pos.x - 1,
                        y: pos.y,
                    })
                } else {
                    None
                }
            }
            E => {
                if pos.y < self.tiles[0].len() - 1 {
                    Some(Pos {
                        x: pos.x,
                        y: pos.y + 1,
                    })
                } else {
                    None
                }
            }
            S => {
                if pos.x < self.tiles.len() - 1 {
                    Some(Pos {
                        x: pos.x + 1,
                        y: pos.y,
                    })
                } else {
                    None
                }
            }
            W => {
                if pos.y > 0 {
                    Some(Pos {
                        x: pos.x,
                        y: pos.y - 1,
                    })
                } else {
                    None
                }
            }
        }
    }

    fn trace(&mut self, pos: &Pos, dir: &Direction) {
        if !self.tiles[pos.x][pos.y].visited.contains(dir) {
            self.tiles[pos.x][pos.y].visited.push(dir.clone());
            self.tiles[pos.x][pos.y].energized = true;
            // dbg!((&pos, &dir, &self.tiles[pos.x][pos.y]));
            match (self.tiles[pos.x][pos.y].symbol) {
                '.' => {
                    if let Some(new_pos) = self.next_pos(pos, dir) {
                        self.trace(&new_pos, dir)
                    }
                }
                '\\' => {
                    let new_dir = match dir {
                        N => W,
                        E => S,
                        S => E,
                        W => N,
                    };
                    if let Some(new_pos) = self.next_pos(pos, &new_dir) {
                        self.trace(&new_pos, &new_dir);
                    }
                }
                '/' => {
                    let new_dir = match dir {
                        N => E,
                        E => N,
                        S => W,
                        W => S,
                    };
                    if let Some(new_pos) = self.next_pos(pos, &new_dir) {
                        self.trace(&new_pos, &new_dir);
                    }
                }
                '-' => match dir {
                    N | S => {
                        if let Some(new_pos) = self.next_pos(pos, &E) {
                            self.trace(&new_pos, &E);
                        }
                        if let Some(new_pos) = self.next_pos(pos, &W) {
                            self.trace(&new_pos, &W);
                        }
                    }
                    E | W => {
                        if let Some(new_pos) = self.next_pos(pos, dir) {
                            self.trace(&new_pos, dir);
                        }
                    }
                },
                '|' => match dir {
                    E | W => {
                        if let Some(new_pos) = self.next_pos(pos, &N) {
                            self.trace(&new_pos, &N);
                        }
                        if let Some(new_pos) = self.next_pos(pos, &S) {
                            self.trace(&new_pos, &S);
                        }
                    }
                    N | S => {
                        if let Some(new_pos) = self.next_pos(pos, dir) {
                            self.trace(&new_pos, dir);
                        }
                    }
                },
                _ => panic!("unexpected symbol: {}", self.tiles[pos.x][pos.y].symbol),
            }
        }
    }

    fn energized(&self) -> usize {
        self.tiles
            .iter()
            .map(|l| l.iter().filter(|t| t.energized).count())
            .sum()
    }

    fn clear(&mut self) {
        for l in self.tiles.iter_mut() {
            for t in l.iter_mut() {
                t.energized = false;
                t.visited = Vec::new();
            }
        }
    }

    // quick and dirty! but it does the job
    fn find_max_energized(&mut self) -> usize {
        let mut max = 0;
        for x in (0..self.tiles.len()) {
            self.trace(&Pos { x, y: 0 }, &E);
            let e = self.energized();
            if e > max {
                max = e;
            };
            self.clear();

            self.trace(
                &Pos {
                    x,
                    y: self.tiles[0].len() - 1,
                },
                &W,
            );
            let e = self.energized();
            if e > max {
                max = e;
            };
            self.clear();
        }

        for y in (0..self.tiles[0].len()) {
            self.trace(&Pos { x: 0, y }, &S);
            let e = self.energized();
            if e > max {
                max = e;
            };
            self.clear();

            self.trace(
                &Pos {
                    x: self.tiles.len() - 1,
                    y,
                },
                &N,
            );
            let e = self.energized();
            if e > max {
                max = e;
            };
            self.clear();
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let mut map = Map::parse(input);
        let start = Pos { x: 0, y: 0 };
        let dir = E;
        map.trace(&start, &dir);
        let sol1 = map.energized();
        // dbg!(map);
        assert_eq!(sol1, 46);
    }

    #[test]
    fn example_2() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let mut map = Map::parse(input);
        let sol2 = map.find_max_energized();
        assert_eq!(sol2, 51);
    }
}
