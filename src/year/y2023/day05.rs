use crate::aoc::{Day, InputParser};
use std::ops::Range;

#[derive(Debug)]
pub struct HeatRange {
    source_start: usize,
    dest_start: usize,
    length: usize,
    source_range: Range<usize>,
}

#[derive(Debug)]
pub enum HeatRangeParseResult {
    Surrounded,
    LowHigh { low: usize, high: usize },
    Low(usize),
    High(usize),
}

impl HeatRange {
    pub fn new(source_start: usize, dest_start: usize, length: usize) -> Self {
        Self {
            source_start,
            dest_start,
            length,
            source_range: source_start..(source_start + length),
        }
    }

    pub fn contains_part_of(&self, input: &Range<usize>) -> bool {
        if input.start <= self.source_start {
            if input.end <= self.source_start {
                return false;
            }
            true
        } else if input.start >= self.source_range.end {
            false
        } else {
            true
        }
    }

    pub fn map_range(&self, input: &Range<usize>) -> (HeatRangeParseResult, Range<usize>) {
        if self.source_start >= input.start {
            if input.end > self.source_range.end {
                (
                    HeatRangeParseResult::LowHigh {
                        low: self.source_start,
                        high: self.source_range.end,
                    },
                    self.dest_start..(self.dest_start + self.length),
                )
            } else {
                (
                    HeatRangeParseResult::Low(self.source_start),
                    self.dest_start..(self.dest_start + (input.end - self.source_start)),
                )
            }
        } else {
            if input.end > self.source_range.end {
                (
                    HeatRangeParseResult::High(self.source_range.end),
                    (self.dest_start + (input.start - self.source_start))
                        ..(self.dest_start + self.length),
                )
            } else {
                (
                    HeatRangeParseResult::Surrounded,
                    (self.dest_start + (input.start - self.source_start))
                        ..(self.dest_start + (input.end - self.source_start)),
                )
            }
        }
    }

    pub fn contains(&self, input: usize) -> bool {
        self.source_range.contains(&input)
    }

    pub fn map_unchecked(&self, input: usize) -> usize {
        self.dest_start + (input - self.source_start)
    }
}

#[derive(Debug)]
pub struct HeatRangeMap {
    ranges: Vec<HeatRange>,
}

impl HeatRangeMap {
    pub fn translate_input(&self, input: usize) -> usize {
        let range = self.ranges.iter().find(|range| range.contains(input));
        range
            .map(|range| range.map_unchecked(input))
            .unwrap_or(input)
    }

    pub fn translate_range_input(&self, input: &Range<usize>) -> Vec<Range<usize>> {
        let range = self
            .ranges
            .iter()
            .find(|range| range.contains_part_of(&input));

        if let Some(value) = range {
            let (retaining, range) = value.map_range(&input);

            match retaining {
                HeatRangeParseResult::Surrounded => {
                    vec![range]
                }
                HeatRangeParseResult::LowHigh {
                    high: new_start,
                    low: new_end,
                } => vec![
                    vec![range],
                    (self.translate_range_input(&(input.start..new_end))),
                    (self.translate_range_input(&(new_start..input.end))),
                ]
                .concat(),
                HeatRangeParseResult::Low(new_end) => vec![
                    vec![range],
                    (self.translate_range_input(&(input.start..new_end))),
                ]
                .concat(),
                HeatRangeParseResult::High(new_start) => vec![
                    vec![range],
                    (self.translate_range_input(&(new_start..input.end))),
                ]
                .concat(),
            }
        } else {
            vec![input.start..input.end]
        }
    }
}

#[derive(Debug)]
pub struct SeedInput {
    seeds: Vec<usize>,
    heat_maps: Vec<HeatRangeMap>,
}

impl InputParser for SeedInput {
    type ResolvedType<'a> = SeedInput;

    fn parse_input(input: &str) -> Self::ResolvedType<'_> {
        let mut lines = input.lines();

        let unparsed_seeds = lines.next().unwrap();
        let unparsed_seeds = unparsed_seeds.split(":").skip(1).next().unwrap();
        let parsed_seeds = unparsed_seeds
            .trim()
            .split(" ")
            .map(str::parse)
            .map(Result::unwrap)
            .collect::<Vec<usize>>();

        let mut lines = lines.skip(2);

        let mut heat_maps = vec![];
        let mut ranges = vec![];
        while let Some(value) = lines.next() {
            if value.is_empty() {
                continue;
            }

            if let Some(c) = value.chars().next() {
                if !c.is_digit(10) {
                    heat_maps.push(HeatRangeMap { ranges });
                    ranges = vec![];
                    continue;
                }
            }

            let mut values = value.split(" ").map(str::parse).map(Result::unwrap);

            if let (Some(dest_start), Some(source_start), Some(length)) =
                (values.next(), values.next(), values.next())
            {
                ranges.push(HeatRange::new(source_start, dest_start, length));
            }
        }

        heat_maps.push(HeatRangeMap { ranges });

        Self {
            seeds: parsed_seeds,
            heat_maps,
        }
    }
}

pub struct Day5;

impl Day for Day5 {
    type InputType = SeedInput;

    fn part1(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<String> {
        let solution = input
            .seeds
            .iter()
            .map(|seed| {
                let mut value = *seed;
                for heat_map in &input.heat_maps {
                    value = heat_map.translate_input(value);
                }
                value
            })
            .min()
            .unwrap();
        Ok(format!("{}", solution))
    }

    fn part2(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<String> {
        let solution = input
            .seeds
            .iter()
            .array_chunks()
            .map(|[v1, v2]| {
                let mut ranges = vec![*v1..(*v1 + *v2)];

                for heat_map in &input.heat_maps {
                    ranges = ranges
                        .iter()
                        .map(|range| heat_map.translate_range_input(range))
                        .flatten()
                        .filter(|r| r.start != r.end)
                        .collect();
                }

                ranges.iter().map(|x| x.start).min().unwrap()
            })
            .min()
            .unwrap();
        Ok(format!("{}", solution))
    }
}
