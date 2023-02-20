pub use crate::input::*;
use itertools::Itertools;
use lazy_static::lazy_static;
// use regex::Regex;
use serde_scan::scan;

pub mod year2020;
pub mod year2022;

pub fn get_solution(year: u32, day: u32, part: u32) -> fn(&[u8]) -> usize {
    match (year, day, part) {
        (2022, 11, 1) => year2022::day11::part1,
        (2022, 11, 2) => year2022::day11::part2,
        _ => todo!(),
    }
}
