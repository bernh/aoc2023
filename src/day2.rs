use crate::Solution;
use regex::{self, Regex};

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    blue: u32,
    red: u32,
    green: u32,
}

pub fn solve(input: &str) -> Solution {
    let lines: Vec<&str> = input.split_terminator('\n').collect();

    // first puzzle
    let sol1: u32 = lines
        .iter()
        .map(|l| parse_game(l))
        .filter(game_possible)
        .map(|g| g.id)
        .sum();
    assert_eq!(sol1, 2545);

    //second puzzle
    let sol2: u32 = lines.iter().map(|l| parse_game(l)).map(|g| power(&g)).sum();
    assert_eq!(sol2, 78111);

    Solution {
        one: sol1.to_string(),
        two: sol2.to_string(),
    }
}

fn parse_game(game: &str) -> Game {
    let mut blue = 0;
    let mut red = 0;
    let mut green = 0;

    let re = Regex::new(r"Game (?P<id>[0-9]+): (?<games>.*)").unwrap();
    let caps = re.captures(game).unwrap();
    let id = &caps["id"].parse::<u32>().unwrap();
    let games = &caps["games"];

    for game in games.split(';') {
        let re = Regex::new(r"(?P<num>[0-9]+) (?P<color>[a-z]+)").unwrap();
        for cap in re.captures_iter(game) {
            let num = cap["num"].parse::<u32>().unwrap();
            match &cap["color"] {
                "green" => green = if num > green { num } else { green },
                "red" => red = if num > red { num } else { red },
                "blue" => blue = if num > blue { num } else { blue },
                _ => panic!("WTF?"),
            }
        }
    }

    Game {
        id: *id,
        blue,
        red,
        green,
    }
}

fn game_possible(game: &Game) -> bool {
    game.red <= 12 && game.green <= 13 && game.blue <= 14
}

fn power(game: &Game) -> u32 {
    game.red * game.green * game.blue
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game() {
        assert_eq!(
            parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Game {
                id: 1,
                blue: 6,
                red: 4,
                green: 2
            }
        )
    }

    #[test]
    fn example_1() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let s: u32 = input
            .into_iter()
            .map(parse_game)
            .filter(game_possible)
            .map(|g| g.id)
            .sum();
        assert_eq!(s, 8);
    }

    #[test]
    fn example_2() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let s: u32 = input.into_iter().map(parse_game).map(|g| power(&g)).sum();
        assert_eq!(s, 2286);
    }
}
