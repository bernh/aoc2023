mod utils;

use regex::Regex;

// use regex::Regex;
#[derive(Debug, PartialEq)]
struct Pair {
    start_1: u32,
    end_1 : u32,
    start_2 : u32,
    end_2: u32,
}

fn parse(pair_descr: String) -> Pair {
    let re = Regex::new(r"(?P<s1>[0-9]+)-(?P<e1>[0-9]+),(?P<s2>[0-9]+)-(?P<e2>[0-9]+)").unwrap();
    let caps = re.captures(&pair_descr).unwrap();
    Pair {start_1:caps["s1"].parse().unwrap(),
         end_1:caps["e1"].parse().unwrap(),
         start_2:caps["s2"].parse().unwrap(),
         end_2:caps["e2"].parse().unwrap(),
    }
}


fn main() {
    let input = utils::lines("inputs/sample.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_1() {
        assert_eq!(parse("3-4,4-9".to_owned()), Pair {start_1:3, end_1:4, start_2:4, end_2:9});
        
    }
}


