#![allow(unused_variables)]
#![feature(iter_advance_by)]
#![feature(iter_map_windows)]
#![feature(iter_next_chunk)]
#![feature(iter_array_chunks)]

use crate::aoc::Year;
use crate::switch::{DaySwitch, Part, YearSwitch};
use std::path::PathBuf;

mod aoc;
pub mod macros;
mod switch;

pub mod year {
    pub mod y2015;
    pub mod y2023;
}

fn main() {
    _main_helper(YearSwitch::Y2023, DaySwitch::Day5);
}

fn _main_helper(year: YearSwitch, day: DaySwitch) {
    solve(YearSwitch::Y2023, day, Part::Part1);
    solve(YearSwitch::Y2023, day, Part::Part2);
}

pub fn solve(year: YearSwitch, day: DaySwitch, part: Part) {
    let input = PathBuf::from(format!("input/{}/{}", year, day));
    let input = std::fs::read_to_string(input).expect("Failed to read input file");

    if let Err(err) = match year {
        YearSwitch::Y2015 => year::y2015::Y2015::solve(day, part, input),
        YearSwitch::Y2023 => year::y2023::Y2023::solve(day, part, input),
    } {
        eprintln!("Error: {}", err);
    }
}
