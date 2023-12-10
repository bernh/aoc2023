#![allow(unused)]

use crate::Solution;
use PipeType::*;
use Tile::*;

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    start: Coord,
    size: (usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum PipeType {
    NS,
    EW,
    NE,
    NW,
    SE,
    SW,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
enum Tile {
    Start,
    Ground,
    Pipe(P),
}

#[derive(Debug)]
struct P {
    t: PipeType,
    north: bool,
    south: bool,
    east: bool,
    west: bool,
}

fn parse_char(c: char) -> Tile {
    match c {
        '|' => Pipe(P {
            t: NS,
            north: true,
            south: true,
            east: false,
            west: false,
        }),
        '-' => Pipe(P {
            t: EW,
            north: false,
            south: false,
            east: true,
            west: true,
        }),
        'L' => Pipe(P {
            t: NE,
            north: true,
            south: false,
            east: true,
            west: false,
        }),
        'J' => Pipe(P {
            t: NW,
            north: true,
            south: false,
            east: false,
            west: true,
        }),
        'F' => Pipe(P {
            t: SE,
            north: false,
            south: true,
            east: true,
            west: false,
        }),
        '7' => Pipe(P {
            t: SW,
            north: false,
            south: true,
            east: false,
            west: true,
        }),
        '.' => Ground,
        'S' => Start,
        _ => panic!("Invalid input"),
    }
}

impl Map {
    fn from_str(input: &str) -> Self {
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        let mut start: Coord = Coord { x: 0, y: 0 };

        for (x, line) in input.lines().enumerate() {
            let mut tiles_line = Vec::new();

            for (y, char) in line.chars().enumerate() {
                let t = parse_char(char);
                match t {
                    Start => start = Coord { x, y },
                    _ => (),
                }
                tiles_line.push(t);
            }
            tiles.push(tiles_line);
        }
        let size = (tiles.len(), tiles[0].len());
        Map { tiles, start, size }
    }

    // return the first connected neigbour we find. Dir is the *incoming*
    // direction from the view of the new tile
    // XXX: no index checking! assume that start is not on the border of the map
    fn start_neigbour(&self) -> (Coord, Dir) {
        // check north
        let mut c: Coord = Coord {
            x: self.start.x - 1,
            y: self.start.y,
        };
        if let Pipe(p) = &self.tiles[c.x][c.y] {
            if p.south {
                return (c, Dir::South);
            }
        }
        // check south
        c = Coord {
            x: self.start.x - 1,
            y: self.start.y,
        };
        if let Pipe(p) = &self.tiles[c.x][c.y] {
            if p.north {
                return (c, Dir::North);
            }
        }
        // if north and south are not connected we know that east and west are.
        // just return the east tile
        c = Coord {
            x: self.start.x,
            y: self.start.y + 1,
        };
        (c, Dir::West)
    }

    // follow pipe and return next coordinate. Dir is the *incoming* direction
    // for this next tile
    fn next_tile(&self, t: Coord, incoming: Dir) -> (Coord, Dir) {
        if let Pipe(pipe) = &self.tiles[t.x][t.y] {
            match pipe.t {
                NS => {
                    if incoming == Dir::North {
                        (Coord { x: t.x + 1, y: t.y }, Dir::North)
                    } else {
                        (Coord { x: t.x - 1, y: t.y }, Dir::South)
                    }
                }
                EW => {
                    if incoming == Dir::East {
                        (Coord { x: t.x, y: t.y - 1 }, Dir::East)
                    } else {
                        (Coord { x: t.x, y: t.y + 1 }, Dir::West)
                    }
                }
                NE => {
                    if incoming == Dir::North {
                        (Coord { x: t.x, y: t.y + 1 }, Dir::West)
                    } else {
                        (Coord { x: t.x - 1, y: t.y }, Dir::South)
                    }
                }
                NW => {
                    if incoming == Dir::North {
                        (Coord { x: t.x, y: t.y - 1 }, Dir::East)
                    } else {
                        (Coord { x: t.x - 1, y: t.y }, Dir::South)
                    }
                }
                SE => {
                    if incoming == Dir::South {
                        (Coord { x: t.x, y: t.y + 1 }, Dir::West)
                    } else {
                        (Coord { x: t.x + 1, y: t.y }, Dir::North)
                    }
                }
                SW => {
                    if incoming == Dir::South {
                        (Coord { x: t.x, y: t.y - 1 }, Dir::East)
                    } else {
                        (Coord { x: t.x + 1, y: t.y }, Dir::North)
                    }
                }
            }
        } else {
            panic!("WTF");
        }
    }

    fn traverse(&self) -> Vec<Coord> {
        let mut path = Vec::new();
        path.push(Coord {
            x: self.start.x,
            y: self.start.y,
        });
        let (mut n, mut dir) = self.start_neigbour();
        path.push(n);
        loop {
            (n, dir) = self.next_tile(n, dir);
            if n == self.start {
                break;
            }
            path.push(n);
        }
        path
    }
}

pub fn solve(input: &str) -> Solution {
    let lines: Vec<&str> = input.split_terminator('\n').collect();

    // first puzzle
    let m = Map::from_str(input);
    let path = m.traverse();
    let sol1 = path.len() / 2;
    assert_eq!(sol1, 6806);

    //second puzzle
    let sol2: u32 = 0;

    Solution {
        one: sol1.to_string(),
        two: sol2.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";

        let m = Map::from_str(input);
        let path = m.traverse();
        assert_eq!(path.len(), 8);
    }

    #[test]
    fn example_2() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

        let m = Map::from_str(input);
        let path = m.traverse();
        assert_eq!(path.len() / 2, 8);
    }
}
