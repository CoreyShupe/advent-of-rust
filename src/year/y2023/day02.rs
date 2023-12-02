use std::cmp::max;

use crate::aoc::{Day, InputParser};

pub struct Day2;

pub struct GamesParser;

impl InputParser for GamesParser {
    type ResolvedType<'a> = Vec<Game>;

    fn parse_input(input: &str) -> Self::ResolvedType<'_> {
        input
            .lines()
            .enumerate()
            .map(|(idx, line)| {
                let mut rounds: Vec<[usize; 3]> = vec![];
                let mut current_round: Option<[usize; 3]> = None;

                let char_iter = line.chars();
                let mut char_iter = char_iter.skip_while(|c| *c != ':').skip(2).enumerate();

                let mut latest_number = None;
                while let Some((index, next)) = char_iter.next() {
                    if next.is_ascii_digit() {
                        // this is always followed by a space
                        latest_number = match latest_number {
                            Some(old) => Some(format!("{}{}", old, next)),
                            None => Some(format!("{}", next)),
                        };
                    } else {
                        let (last_position, marker) = char_iter.next().unwrap();
                        let value = latest_number.take().unwrap().parse().unwrap();
                        match marker {
                            'r' => {
                                if let Some(ref mut current_round) = current_round.as_mut() {
                                    current_round[0] = value;
                                } else {
                                    current_round = Some([value, 0, 0]);
                                }
                            }
                            'g' => {
                                if let Some(ref mut current_round) = current_round.as_mut() {
                                    current_round[1] = value;
                                } else {
                                    current_round = Some([0, value, 0]);
                                }
                            }
                            'b' => {
                                if let Some(ref mut current_round) = current_round.as_mut() {
                                    current_round[2] = value;
                                } else {
                                    current_round = Some([0, 0, value]);
                                }
                            }
                            _ => unreachable!(),
                        };

                        let splitter = char_iter.find(|(_, x)| *x == ',' || *x == ';');
                        match splitter {
                            Some((position, ',')) => {
                                char_iter.advance_by(1).unwrap();
                                continue;
                            }
                            Some((position, ';')) => {
                                if let Some(round) = current_round.take() {
                                    rounds.push(round);
                                }
                                char_iter.advance_by(1).unwrap();
                                continue;
                            }
                            None => {
                                if let Some(round) = current_round.take() {
                                    rounds.push(round);
                                }
                                break;
                            }
                            _ => panic!(),
                        }
                    }
                }

                Game {
                    game_id: idx + 1,
                    rounds,
                }
            })
            .collect()
    }
}

pub struct Game {
    game_id: usize,
    rounds: Vec<[usize; 3]>,
}

pub mod part1 {}

impl Day for Day2 {
    type InputType = GamesParser;

    fn part1(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<()> {
        let solution: usize = input
            .iter()
            .filter(|game| {
                game.rounds
                    .iter()
                    .all(|round| round[0] <= 12 && round[1] <= 13 && round[2] <= 14)
            })
            .map(|game| game.game_id)
            .sum();
        println!("Solution: {}", solution);
        Ok(())
    }

    fn part2(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<()> {
        let solution: usize = input
            .iter()
            .map(|game| {
                let maxes = game.rounds.iter().fold([0, 0, 0], |mut acc, round| {
                    acc[0] = max(acc[0], round[0]);
                    acc[1] = max(acc[1], round[1]);
                    acc[2] = max(acc[2], round[2]);
                    acc
                });
                maxes[0] * maxes[1] * maxes[2]
            })
            .sum();
        println!("Solution: {}", solution);
        Ok(())
    }
}
