use std::str::Chars;

use crate::aoc::Day;

pub struct Day1;

impl Day for Day1 {
    type InputType = Chars<'static>;

    fn part1(input: Chars) -> anyhow::Result<String> {
        Ok(format!(
            "{}",
            input.fold(0, |acc, c| match c {
                '(' => acc + 1,
                ')' => acc - 1,
                _ => acc,
            })
        ))
    }

    fn part2(input: Chars) -> anyhow::Result<String> {
        let mut acc = 0;
        let mut pointer = 1;
        for c in input {
            acc = match c {
                '(' => acc + 1,
                ')' => acc - 1,
                _ => acc,
            };
            if acc == -1 {
                break;
            }
            pointer += 1;
        }
        Ok(format!("{}", pointer))
    }
}
