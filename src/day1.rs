use crate::Solution;
use regex::Regex;

pub fn solve(input: &str) -> Solution {
    let lines: Vec<&str> = input.split_terminator('\n').collect();

    // first puzzle
    let sol1: u32 = lines.iter().map(|v| get_cal_value(v)).sum();
    assert_eq!(sol1, 53080); // known solution

    //second puzzle
    let sol2: u32 = lines.iter().map(|v| get_cal_value_real(v)).sum();
    assert_eq!(sol2, 78111);

    Solution {
        one: sol1.to_string(),
        two: sol2.to_string(),
    }
}

fn get_cal_value(value_str: &str) -> u32 {
    let re = Regex::new(r"[0-9]").unwrap();
    let digits: Vec<&str> = re.find_iter(&value_str).map(|m| m.as_str()).collect();
    assert!(digits.len() > 0);
    digits[0].parse::<u32>().unwrap() * 10 + digits[digits.len() - 1].parse::<u32>().unwrap()
}

fn get_cal_value_real(value_str: &str) -> u32 {
    fn to_u32(d: &str) -> u32 {
        match d {
            "1" => 1,
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            "9" => 9,
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => panic!("invalid digit string: {}", d),
        }
    }

    let re = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let re_last = Regex::new(r".*([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let first = re.find(&value_str).unwrap().as_str();
    let last = &re_last.captures(&value_str).unwrap()[1];

    to_u32(first) * 10 + to_u32(last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_value() {
        assert_eq!(get_cal_value(&"1abc2"), 12);
        assert_eq!(get_cal_value(&"pqr3stu8vwx"), 38);
        assert_eq!(get_cal_value(&"a1b2c3d4e5f"), 15);
    }

    #[test]
    fn solve_example1() {
        let lines = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

        let sol: u32 = lines.iter().map(|v| get_cal_value(v)).sum();
        assert_eq!(sol, 142);
        // should be the same result with second parsing method
        let sol: u32 = lines.iter().map(|v| get_cal_value_real(v)).sum();
        assert_eq!(sol, 142);
    }

    #[test]
    fn test_single_value_2() {
        assert_eq!(
            get_cal_value_real(&"eight9fhstbssrplmdlncmmqqnklb39ninejz".to_owned()),
            89
        );
        assert_eq!(get_cal_value_real(&"52three"), 53);
        assert_eq!(get_cal_value_real(&"nine"), 99);
        // this is a crucial test! greedy regex matching finds a two!
        assert_eq!(get_cal_value_real(&"1twone"), 11);
    }

    #[test]
    fn solve_example2() {
        let lines = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];

        let sol: u32 = lines.iter().map(|v| get_cal_value_real(v)).sum();
        assert_eq!(sol, 281);
    }
}
