#![allow(unused)]

use crate::Solution;
use itertools::Itertools;
use Pixel::*;

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Debug)]
enum Pixel {
    Space,
    Galaxy,
}

struct Image {
    pixels: Vec<Vec<Pixel>>,
}

pub fn solve(input: &str) -> Solution {
    // first puzzle
    let img = Image::parse(input);
    let sol1: usize = img.galaxy_distances(2).iter().sum();
    assert_eq!(sol1, 9965032);

    //second puzzle
    let sol2: usize = img.galaxy_distances(1000000).iter().sum();
    assert_eq!(sol2, 550358864332);

    Solution {
        one: sol1.to_string(),
        two: sol2.to_string(),
    }
}

impl Image {
    fn parse(input: &str) -> Self {
        Image {
            pixels: input
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|c| match c {
                            '.' => Space,
                            '#' => Galaxy,
                            _ => panic!("invalid input"),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn galaxies(&self) -> Vec<Coord> {
        self.pixels
            .iter()
            .enumerate()
            .flat_map(|(x, l)| {
                l.iter()
                    .enumerate()
                    .filter_map(|(y, pixel)| {
                        if *pixel == Galaxy {
                            Some(Coord { x, y })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Coord>>()
            })
            .collect()
    }

    fn empty_lines(&self, galaxies: &Vec<Coord>) -> Vec<usize> {
        (0..self.pixels.len())
            .filter_map(|x| {
                if galaxies.iter().any(|g| g.x == x) {
                    None
                } else {
                    Some(x)
                }
            })
            .collect()
    }

    fn empty_columns(&self, galaxies: &Vec<Coord>) -> Vec<usize> {
        (0..self.pixels[0].len())
            .filter_map(|y| {
                if galaxies.iter().any(|g| g.y == y) {
                    None
                } else {
                    Some(y)
                }
            })
            .collect()
    }

    fn galaxy_distances(&self, expansion_factor: usize) -> Vec<usize> {
        fn distance(
            a: &Coord,
            b: &Coord,
            empty_lines: &Vec<usize>,
            empty_colums: &Vec<usize>,
            expansion_factor: usize,
        ) -> usize {
            let mut distance: usize =
                (a.x as i32 - b.x as i32).abs() as usize + (a.y as i32 - b.y as i32).abs() as usize;
            let lines_range = if (a.x < b.x) {
                (a.x..=b.x)
            } else {
                (b.x..=a.x)
            };
            let columns_range = if (a.y < b.y) {
                (a.y..=b.y)
            } else {
                (b.y..=a.y)
            };
            distance
                + (empty_lines
                    .iter()
                    .filter(|x| lines_range.contains(x))
                    .count())
                    * (expansion_factor - 1)
                + (empty_colums
                    .iter()
                    .filter(|y| columns_range.contains(y))
                    .count())
                    * (expansion_factor - 1)
        }

        let galaxies = self.galaxies();
        let empty_lines = self.empty_lines(&galaxies);
        let empty_columns = self.empty_columns(&galaxies);
        galaxies
            .iter()
            .tuple_combinations()
            .map(|(a, b)| distance(a, b, &empty_lines, &empty_columns, expansion_factor))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let img = Image::parse(input);
        let sol1: usize = img.galaxy_distances(2).iter().sum();
        assert_eq!(sol1, 374);
    }
}
