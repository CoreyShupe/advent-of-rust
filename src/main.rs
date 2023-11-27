#![allow(unused_variables)]

use crate::aoc::Year;
use crate::switch::{DaySwitch, Part, YearSwitch};
use std::path::PathBuf;

mod aoc;
pub mod macros;
mod switch;

pub mod year {
    pub mod y2015;
}

macro_rules! solve {
    ($year:ident, $day:ident, $part:ident) => {
        solve(YearSwitch::$year, DaySwitch::$day, Part::$part);
    };
}

fn main() {
    solve!(Y2015, Day5, Part1);
}

pub fn solve(year: YearSwitch, day: DaySwitch, part: Part) {
    let input = PathBuf::from(format!("input/{}/{}", year, day));
    let input = std::fs::read_to_string(input).expect("Failed to read input file");

    if let Err(err) = match year {
        YearSwitch::Y2015 => year::y2015::Y2015::solve(day, part, input),
    } {
        eprintln!("Error: {}", err);
    }
}
