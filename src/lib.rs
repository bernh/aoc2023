// declare modules: example + one per day
mod day0;
mod day1;
mod day2;
mod day3;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct Solution {
    pub one: String,
    pub two: String,
}

pub fn solve(day: usize, input: String) -> Solution {
    match day {
        0 => day0::solve(input), // 0 is for the sample day (day 4 of 2022)
        1 => day1::solve(input),
        2 => day2::solve(input),
        3 => day3::solve(input),
        _ => unimplemented!("Day {} not implemented", day),
    }
}

#[wasm_bindgen]
pub fn wasm_solve(_day: usize, input: String) -> String {
    let solution = day0::solve(input);
    format!("Solutions are: {} and {}", solution.one, solution.two)
}
