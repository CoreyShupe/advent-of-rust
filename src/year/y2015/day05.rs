use crate::aoc::{Day, InputParser};
use std::collections::HashSet;
use std::str::Lines;

pub struct Day5;

impl Day for Day5 {
    type InputType = Lines<'static>;

    fn part1(input: Lines) -> anyhow::Result<()> {
        let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);

        let mut nice_lines = 0;

        for line in input {
            let mut vowels = 0;
            let mut double_letter_flag = false;

            let mut chars = line.chars();
            let prev: char = chars.next().unwrap();
            for c in chars {

            }
        }

        let solution = "";
        println!("Solution: {}", solution);
        Ok(())
    }

    fn part2(input: Lines) -> anyhow::Result<()> {
        let solution = "";
        println!("Solution: {}", solution);
        Ok(())
    }
}
