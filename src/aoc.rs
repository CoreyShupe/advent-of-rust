use std::str::{Chars, Lines};
use std::time::SystemTime;

use crate::switch::{DaySwitch, Part};

pub trait Year {
    fn solve(day: DaySwitch, part: Part, input: String) -> anyhow::Result<()>;
}

pub trait InputParser {
    type ResolvedType<'a>;

    fn parse_input(input: &str) -> Self::ResolvedType<'_>;
}

pub trait Day {
    type InputType: InputParser;

    fn solve_raw(input: &str, part: Part) -> anyhow::Result<()> {
        let start = SystemTime::now();
        let input = Self::InputType::parse_input(input);
        let input_break = SystemTime::now();
        let result = match part {
            Part::Part1 => Self::part1(input),
            Part::Part2 => Self::part2(input),
        }?;

        if result.is_empty() {
            return Ok(());
        }

        let result_break = SystemTime::now();
        println!(
            "Part {} Results:",
            match part {
                Part::Part1 => 1,
                Part::Part2 => 2,
            }
        );
        println!("\tSolution:\t{}", result);
        let input_break_time = input_break.duration_since(start)?;
        let result_break_time = result_break.duration_since(input_break)?;
        let total_time = result_break.duration_since(start)?;
        println!(
            "\tBreaks: \t[{}\u{03BC}/{}\u{03BC}/{}\u{03BC}]",
            input_break_time.as_micros(),
            result_break_time.as_micros(),
            total_time.as_micros()
        );
        Ok(())
    }

    fn part1(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<String>;

    fn part2(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<String>;
}

// input implementations

impl InputParser for Chars<'static> {
    type ResolvedType<'a> = Chars<'a>;

    fn parse_input(input: &str) -> Self::ResolvedType<'_> {
        input.chars()
    }
}

impl InputParser for &'static str {
    type ResolvedType<'a> = &'a str;

    fn parse_input(input: &str) -> Self::ResolvedType<'_> {
        input
    }
}

impl InputParser for Lines<'static> {
    type ResolvedType<'a> = Lines<'a>;

    fn parse_input(input: &str) -> Self::ResolvedType<'_> {
        input.lines()
    }
}
