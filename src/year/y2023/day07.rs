use crate::aoc::{Day, InputParser};
use std::cmp::Ordering;
use std::collections::HashMap;

#[repr(u8)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum HandType {
    Five = 0,
    Four = 1,
    FullHouse = 2,
    Three = 3,
    TwoPair = 4,
    OnePair = 5,
    HighCard = 6,
}

#[derive(Debug)]
struct Segment {
    best_hand: HandType,
    values: Vec<u32>,
    bid: usize,
}

pub struct Day7;

fn update_groupings(groupings: &mut HashMap<char, usize>, c: char) {
    if groupings.contains_key(&c) {
        groupings.insert(c, groupings.get(&c).unwrap() + 1);
    } else {
        groupings.insert(c, 1);
    }
}

fn count_winnings(mut segments: Vec<Segment>) -> usize {
    segments.sort_by(|subject, object| {
        if subject.best_hand == object.best_hand {
            for (subj_x, obj_x) in subject.values.iter().zip(object.values.iter()) {
                if subj_x > obj_x {
                    return Ordering::Greater;
                } else if obj_x > subj_x {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        } else if subject.best_hand > object.best_hand {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    segments
        .iter()
        .enumerate()
        .map(|(idx, segment)| segment.bid * (idx + 1))
        .sum()
}

impl Day for Day7 {
    type InputType = &'static str;

    fn part1(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<String> {
        let segments = input
            .lines()
            .map(|line| {
                let mut lines = line.split(" ");

                let mut groupings = HashMap::<char, usize>::new();
                let mut values = vec![];

                let hand = lines.next().unwrap();

                for c in hand.chars() {
                    update_groupings(&mut groupings, c);
                    values.push(match c {
                        'A' => 14,
                        'K' => 13,
                        'Q' => 12,
                        'J' => 11,
                        'T' => 10,
                        x if x.is_ascii_digit() => x.to_digit(10).unwrap(),
                        _ => panic!(),
                    });
                }

                let bid = lines.next().unwrap().parse().unwrap();

                let grouping_length = groupings.len();

                let hand_type = if grouping_length == 1 {
                    HandType::Five
                } else if grouping_length == 2 {
                    let pick = groupings.iter().next().unwrap().1;
                    if *pick == 1 || *pick == 4 {
                        HandType::Four
                    } else {
                        HandType::FullHouse
                    }
                } else if grouping_length == 3 {
                    let max_count = groupings.iter().map(|x| *x.1).max().unwrap();
                    if max_count == 3 {
                        HandType::Three
                    } else {
                        HandType::TwoPair
                    }
                } else if grouping_length == 4 {
                    HandType::OnePair
                } else {
                    HandType::HighCard
                };

                Segment {
                    best_hand: hand_type,
                    values,
                    bid,
                }
            })
            .collect::<Vec<Segment>>();

        let solution = count_winnings(segments);
        Ok(format!("{}", solution))
    }

    fn part2(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<String> {
        let segments = input
            .lines()
            .map(|line| {
                let mut lines = line.split(" ");

                let mut jokers = 0;
                let mut groupings = HashMap::<char, usize>::new();
                let mut values = vec![];

                let hand = lines.next().unwrap();

                for c in hand.chars() {
                    if c != 'J' {
                        update_groupings(&mut groupings, c);
                    } else {
                        jokers += 1;
                    }
                    values.push(match c {
                        'A' => 14,
                        'K' => 13,
                        'Q' => 12,
                        'J' => 1,
                        'T' => 10,
                        x if x.is_ascii_digit() => x.to_digit(10).unwrap(),
                        _ => panic!(),
                    });
                }

                let bid = lines.next().unwrap().parse().unwrap();

                let max_count = groupings.iter().map(|item| *item.1).max().unwrap_or(0);

                let hand_type = match (jokers, max_count, jokers + max_count) {
                    (_, _, 5) => HandType::Five,
                    (_, _, 4) => HandType::Four,
                    (x, y, 3) => {
                        if x == 1 && groupings.iter().all(|x| *x.1 == 2) {
                            HandType::FullHouse
                        } else if x == 2 {
                            HandType::Three
                        } else if x == 0 && groupings.iter().find(|x| *x.1 == 2).is_some() {
                            HandType::FullHouse
                        } else {
                            HandType::Three
                        }
                    }
                    (0, _, 2) => {
                        if groupings.len() == 3 {
                            HandType::TwoPair
                        } else {
                            HandType::OnePair
                        }
                    }
                    (1, _, 2) => HandType::OnePair,
                    _ => HandType::HighCard,
                };

                Segment {
                    best_hand: hand_type,
                    values,
                    bid,
                }
            })
            .collect::<Vec<Segment>>();

        let solution = count_winnings(segments);
        Ok(format!("{}", solution))
    }
}
