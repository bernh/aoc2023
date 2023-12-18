// declare modules: example + one per day
mod day0;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day15;
mod day16;
mod day17;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct Solution {
    pub one: String,
    pub two: String,
}

pub fn solve(day: usize, input: &str) -> Solution {
    match day {
        0 => day0::solve(input), // 0 is for the sample day (day 4 of 2022)
        1 => day1::solve(input),
        2 => day2::solve(input),
        3 => day3::solve(input),
        4 => day4::solve(input),
        5 => day5::solve(input),
        6 => day6::solve(input),
        7 => day7::solve(input),
        8 => day8::solve(input),
        9 => day9::solve(input),
        10 => day10::solve(input),
        11 => day11::solve(input),
        12 => day12::solve(input),
        13 => day13::solve(input),
        14 => todo!(),
        15 => day15::solve(input),
        16 => day16::solve(input),
        17 => day17::solve(input),
        _ => unimplemented!("Day {} not implemented", day),
    }
}

#[wasm_bindgen]
pub fn wasm_solve(_day: usize, input: &str) -> String {
    let solution = day0::solve(input);
    format!("Solutions are: {} and {}", solution.one, solution.two)
}
