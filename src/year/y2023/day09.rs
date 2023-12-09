use crate::aoc::{Day, InputParser};

pub fn splice_iterator<I: Iterator<Item = isize>>(mut iter: I) -> (isize, Vec<isize>, isize) {
    let size_hint = iter.size_hint();
    let size_hint = size_hint.1.unwrap_or(size_hint.0);
    let mut next_vec = vec![0; size_hint - 1];
    let mut last_value = iter.next().unwrap();
    let initial = last_value;

    for (idx, next) in iter.enumerate() {
        next_vec[idx] = next - last_value;
        last_value = next
    }

    (last_value, next_vec, initial)
}

pub struct PathsParser;

impl InputParser for PathsParser {
    type ResolvedType<'a> = Vec<Vec<isize>>;

    fn parse_input(input: &str) -> Self::ResolvedType<'_> {
        input
            .lines()
            .map(|x| x.split(" ").map(str::parse).map(Result::unwrap).collect())
            .collect()
    }
}

pub struct Day9;

impl Day for Day9 {
    type InputType = PathsParser;

    fn part1(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<String> {
        let mut solution = 0;
        for path in input {
            let mut values: Vec<isize> = vec![];
            let mut next_path = path;
            loop {
                let (remainder, future, _) = splice_iterator(next_path.into_iter());

                values.push(remainder);

                if future.iter().all(|x| *x == 0) {
                    break;
                }

                next_path = future;
            }

            solution += values.into_iter().rev().fold(0, |acc, value| acc + value);
        }

        Ok(format!("{}", solution))
    }

    fn part2(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<String> {
        let mut solution = 0;
        for path in input {
            let mut values: Vec<isize> = vec![];
            let mut next_path = path;
            loop {
                let (_, future, initial) = splice_iterator(next_path.into_iter());

                values.push(initial);

                if future.iter().all(|x| *x == 0) {
                    break;
                }

                next_path = future;
            }

            solution += values.into_iter().rev().fold(0, |acc, value| value - acc);
        }

        Ok(format!("{}", solution))
    }
}
