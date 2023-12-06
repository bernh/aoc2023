#![allow(unused)]

use crate::Solution;
use regex::Regex;

pub fn solve(input: &str) -> Solution {
    let (seeds, maps) = parse(input);
    // first puzzle
    let sol1 = find_minimum(&seeds.seeds, &maps);

    //second puzzle
    let mut min: u64 = std::u64::MAX;
    // TODO use rayon to parallaize search!
    for chunk in seeds.seeds.chunks(2) {
        let start = chunk[0];
        let len = chunk[1];
        println!("processing chunk with {} seeds", len);
        let chunk_minimum = find_minimum(&(start..(start + len)).into_iter().collect(), &maps);
        if chunk_minimum < min {
            min = chunk_minimum
        }
    }
    let sol2 = min;
    assert_eq!(sol2, 15290096);

    Solution {
        one: sol1.to_string(),
        two: sol2.to_string(),
    }
}

#[derive(Debug)]
struct Seeds {
    seeds: Vec<u64>,
}

#[derive(Debug)]
struct Mappings {
    dst_start: u64,
    src_start: u64,
    len: u64,
}

#[derive(Debug)]
struct Map<Mappings> {
    maps: Vec<Mappings>,
}

impl Map<Mappings> {
    fn map_num(self: &Self, s: u64) -> u64 {
        for m in self.maps.iter() {
            if s >= m.src_start && s < (m.src_start + m.len) {
                return (s - m.src_start) + m.dst_start;
            }
        }
        s // fallback, map to input
    }
}

fn parse(input: &str) -> (Seeds, Vec<Map<Mappings>>) {
    let re_seeds = Regex::new(r"seeds: (.*)\n\n").unwrap();
    let re_maps = vec![
        Regex::new(r"seed\-to\-soil map:\n((\d.*\n)+)").unwrap(),
        Regex::new(r"soil\-to\-fertilizer map:\n((\d.*\n)+)").unwrap(),
        Regex::new(r"fertilizer\-to\-water map:\n((\d.*\n)+)").unwrap(),
        Regex::new(r"water\-to\-light map:\n((\d.*\n)+)").unwrap(),
        Regex::new(r"light\-to\-temperature map:\n((\d.*\n)+)").unwrap(),
        Regex::new(r"temperature\-to\-humidity map:\n((\d.*\n)+)").unwrap(),
        Regex::new(r"humidity\-to\-location map:\n((\d.*\n)+)").unwrap(),
    ];

    let seeds = Seeds {
        seeds: re_seeds
            .captures(input)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .split(' ')
            .map(|x| x.parse::<u64>().unwrap())
            .collect(),
    };

    let mut maps: Vec<Map<Mappings>> = Vec::new();
    for re_map in re_maps {
        let mut map_: Map<Mappings> = Map { maps: Vec::new() };
        for line in re_map
            .captures(input)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .split('\n')
            .filter(|x| !x.is_empty())
        {
            let n: Vec<u64> = line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
            map_.maps.push(Mappings {
                dst_start: n[0],
                src_start: n[1],
                len: n[2],
            });
        }
        maps.push(map_);
    }
    (seeds, maps)
}

fn find_minimum(seeds: &Vec<u64>, maps: &Vec<Map<Mappings>>) -> u64 {
    seeds
        .iter()
        .map(|s| {
            maps[6].map_num(maps[5].map_num(
                maps[4].map_num(
                    maps[3].map_num(maps[2].map_num(maps[1].map_num(maps[0].map_num(*s)))),
                ),
            ))
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_mapping() {
        let m = Map {
            maps: vec![
                Mappings {
                    dst_start: 50,
                    src_start: 98,
                    len: 2,
                },
                Mappings {
                    dst_start: 52,
                    src_start: 50,
                    len: 48,
                },
            ],
        };
        assert_eq!(m.map_num(0), 0);
        assert_eq!(m.map_num(1), 1);
        assert_eq!(m.map_num(48), 48);
        assert_eq!(m.map_num(49), 49);
        assert_eq!(m.map_num(50), 52);
        assert_eq!(m.map_num(51), 53);
        assert_eq!(m.map_num(96), 98);
        assert_eq!(m.map_num(97), 99);
        assert_eq!(m.map_num(98), 50);
        assert_eq!(m.map_num(99), 51);
    }

    #[test]
    fn example() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

        let (seeds, maps) = parse(input);
        let sol1 = find_minimum(&seeds.seeds, &maps);
        assert_eq!(sol1, 35);

        // puzzle 2
        let mut min: u64 = std::u64::MAX;
        for chunk in seeds.seeds.chunks(2) {
            let start = chunk[0];
            let len = chunk[1];
            let chunk_minimum = find_minimum(&(start..(start + len)).into_iter().collect(), &maps);
            if chunk_minimum < min {
                min = chunk_minimum
            }
        }
        let sol2 = min;
        assert_eq!(sol2, 46);
    }
}
