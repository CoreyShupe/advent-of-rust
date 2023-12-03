use crate::aoc::Day;
use std::collections::HashSet;
use std::str::Lines;

pub struct Day5;

impl Day for Day5 {
    type InputType = Lines<'static>;

    fn part1(input: Lines) -> anyhow::Result<String> {
        let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);

        let mut nice_lines = 0;

        'lines: for line in input {
            let mut double_letter_flag = false;

            let mut chars = line.chars();
            let mut prev: char = chars.next().unwrap();

            let mut vowels_counted = if vowels.contains(&prev) { 1 } else { 0 };

            for c in chars {
                if vowels.contains(&c) {
                    vowels_counted += 1;
                }

                match [prev, c] {
                    ['a', 'b'] => continue 'lines,
                    ['c', 'd'] => continue 'lines,
                    ['p', 'q'] => continue 'lines,
                    ['x', 'y'] => continue 'lines,
                    [x, y] if x == y => double_letter_flag = true,
                    _ => (),
                }

                prev = c;
            }

            if vowels_counted >= 3 && double_letter_flag {
                nice_lines += 1;
            }
        }

        let solution = nice_lines;
        Ok(format!("{}", solution))
    }

    fn part2(input: Lines) -> anyhow::Result<String> {
        let mut nice_lines = 0;

        for line in input {
            let mut patterns = HashSet::new();
            let mut double_pattern_flag = false;
            let mut letter_between_flag = false;

            let mut chars = line.chars();

            let mut prev_prev: char = chars.next().unwrap();
            let mut prev: char = chars.next().unwrap();

            for c in chars {
                if patterns.contains(&(prev, c)) {
                    double_pattern_flag = true;
                }

                if prev_prev == c {
                    letter_between_flag = true;
                }

                if letter_between_flag && double_pattern_flag {
                    break;
                }

                patterns.insert((prev_prev, prev));
                prev_prev = prev;
                prev = c;
            }

            if letter_between_flag && double_pattern_flag {
                nice_lines += 1;
            }
        }

        let solution = nice_lines;
        Ok(format!("{}", solution))
    }
}
