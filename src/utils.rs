use std::fs;
use std::path;

// simple file read, just panics if something goes wrong
pub fn input(file :&str) -> String {
    fs::read_to_string(file).unwrap_or("reading input file went wrong".to_string())
}

// return lines
pub fn lines(file :&str) -> Vec<String> {
    input(file).split('\n').map(|x| x.to_owned()).collect()
}

