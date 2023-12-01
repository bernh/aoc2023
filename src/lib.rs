// declare modules: example + one per day
mod day0;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(getter_with_clone)]
pub struct Solution {
    pub one: String,
    pub two: String,
}

pub fn solve(day: usize, input: String) -> Solution {
    match day {
        // 0 is special for the sample day (day 4 of 2022)
        0 => day0::solve(input),
        _ => unimplemented!("Day {} not implemented", day),
    }
}

#[wasm_bindgen]
pub fn wasm_solve(_day: usize, input: String) -> String {
    let solution = day0::solve(input);
    format!("Solutions are: {} and {}", solution.one, solution.two)
}
