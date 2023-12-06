use crate::aoc::{Day, InputParser};

pub struct Day6;

#[derive(Debug)]
pub struct Race {
    pub time: usize,
    pub distance: usize,
}

impl Race {
    pub fn winners(&self) -> usize {
        for time in 1..self.time {
            if time * (self.time - time) > self.distance {
                return if self.time % 2 == 0 {
                    (((self.time / 2) - time) * 2) + 1
                } else {
                    (time..(self.time - time) + 1).len()
                };
            }
        }
        panic!()
    }
}

impl Day for Day6 {
    type InputType = &'static str;

    fn part1(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<String> {
        let lines = input.lines();
        let [times, distances] = lines
            .map(|x| {
                x.split(":")
                    .skip(1)
                    .next()
                    .unwrap()
                    .trim()
                    .split(" ")
                    .filter(|x| !x.is_empty())
                    .map(str::parse)
                    .map(Result::unwrap)
            })
            .next_chunk()
            .unwrap();
        let races = times
            .zip(distances)
            .map(|(time, distance)| Race { time, distance })
            .collect::<Vec<Race>>();

        let solution = races.iter().map(Race::winners).fold(1, |acc, v| acc * v);
        Ok(format!("{}", solution))
    }

    fn part2(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<String> {
        let [time, distance] = input
            .lines()
            .map(|x| {
                x.split(":")
                    .skip(1)
                    .next()
                    .unwrap()
                    .replace(" ", "")
                    .parse::<usize>()
                    .unwrap()
            })
            .next_chunk()
            .unwrap();
        let race = Race { time, distance };
        let solution = race.winners();
        Ok(format!("{}", solution))
    }
}
