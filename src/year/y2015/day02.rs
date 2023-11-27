use crate::aoc::{Day, InputParser};
use std::cmp::min;

pub struct Day2InputParser;

impl InputParser for Day2InputParser {
    type ResolvedType<'a> = Vec<Dimensions>;

    fn parse_input(input: &str) -> Self::ResolvedType<'_> {
        input
            .lines()
            .map(|line| {
                let mut split = line.split("x");
                Dimensions {
                    length: split
                        .next()
                        .expect("Invalid input.")
                        .parse()
                        .expect("Invalid input."),
                    width: split
                        .next()
                        .expect("Invalid input.")
                        .parse()
                        .expect("Invalid input."),
                    height: split
                        .next()
                        .expect("Invalid input.")
                        .parse()
                        .expect("Invalid input."),
                }
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct Dimensions {
    length: usize,
    width: usize,
    height: usize,
}

pub struct Day2;

impl Day for Day2 {
    type InputType = Day2InputParser;

    fn part1(input: Vec<Dimensions>) -> anyhow::Result<()> {
        let solution = input.iter().fold(0, |acc, dimensions| {
            let mut smallest_area = usize::MAX;
            let mut total_area = 0;

            for [opx, opy] in [
                [dimensions.length, dimensions.width],
                [dimensions.width, dimensions.height],
                [dimensions.height, dimensions.length],
            ] {
                let area = opx * opy;
                smallest_area = min(smallest_area, area);
                total_area += area * 2;
            }
            acc + smallest_area + total_area
        });
        println!("Solution: {}", solution);
        Ok(())
    }

    fn part2(input: Vec<Dimensions>) -> anyhow::Result<()> {
        let solution = input.iter().fold(0, |acc, dimensions| {
            let lowest_2 = match [
                dimensions.height >= dimensions.width,
                dimensions.width >= dimensions.length,
                dimensions.length >= dimensions.height,
            ] {
                [true, true, true] => [dimensions.height, dimensions.width],
                [true, _, false] => [dimensions.width, dimensions.length],
                [_, false, true] => [dimensions.height, dimensions.width],
                [false, true, _] => [dimensions.height, dimensions.length],
                [false, false, false] => unreachable!(),
            };

            let ribbon_length = (lowest_2[0] * 2) + (lowest_2[1] * 2);
            let bow_length = dimensions.length * dimensions.height * dimensions.width;
            acc + bow_length + ribbon_length
        });
        println!("Solution: {}", solution);
        Ok(())
    }
}
