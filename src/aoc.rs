use std::str::{Chars, Lines};

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
        let input = Self::InputType::parse_input(input);
        match part {
            Part::Part1 => Self::part1(input),
            Part::Part2 => Self::part2(input),
        }
    }

    fn part1(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<()>;

    fn part2(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<()>;
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
