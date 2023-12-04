use crate::aoc::{Day, InputParser};
use std::collections::HashSet;

pub struct Day4;

#[derive(Debug, Clone)]
pub struct Card {
    comparable: HashSet<u32>,
    numbers: Vec<u32>,
}

pub struct Deck;

impl InputParser for Deck {
    type ResolvedType<'a> = Vec<Card>;

    fn parse_input(input: &str) -> Self::ResolvedType<'_> {
        input
            .lines()
            .map(|card| {
                let parts = card
                    .split(":")
                    .skip(1)
                    .next()
                    .unwrap()
                    .split("|")
                    .map(|x| {
                        let mut values = Vec::new();
                        let mut chunker = x.chars();
                        while let Ok([a, b, c]) = chunker.next_chunk() {
                            values.push(if b != ' ' {
                                format!("{}{}", b, c).parse().unwrap()
                            } else {
                                c.to_digit(10).unwrap()
                            })
                        }
                        values
                    })
                    .collect::<Vec<Vec<u32>>>();

                let mut parts = parts.into_iter();
                let [comparable, numbers] = [parts.next().unwrap(), parts.next().unwrap()];
                let comparable = HashSet::from_iter(comparable.into_iter());

                Card {
                    comparable,
                    numbers,
                }
            })
            .collect()
    }
}

impl Day for Day4 {
    type InputType = Deck;

    fn part1(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<String> {
        let solution: u32 = input
            .into_iter()
            .map(|card| {
                card.numbers
                    .iter()
                    .filter(|x| card.comparable.contains(x))
                    .count()
            })
            .map(|x| match x {
                0 => 0,
                1 => 1,
                x => 1 << x - 1,
            })
            .sum();
        Ok(format!("{}", solution))
    }

    fn part2(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<String> {
        let mut mapped_cards: Vec<usize> = vec![1; input.len()];
        for (idx, card) in input.iter().enumerate() {
            let matches = card
                .numbers
                .iter()
                .filter(|x| card.comparable.contains(x))
                .count();

            let mul = mapped_cards[idx];

            for i in 1..=matches {
                mapped_cards[idx + i] += mul;
            }
        }
        let solution: usize = mapped_cards.iter().sum();
        Ok(format!("{}", solution))
    }
}
