use std::str::Lines;

use crate::aoc::{Day, InputParser};
use crate::year::y2023::day01::part2::capture_value;

pub struct Day1;

pub mod part2 {
    use std::iter::{Peekable, Rev, Skip};
    use std::str::Chars;

    #[derive(Clone)]
    struct Capture {
        value: &'static str,
        output: u32,
    }

    #[derive(Debug)]
    struct ActiveCapture<I: Iterator<Item = char>> {
        value_iterator: Peekable<Skip<I>>,
        output: u32,
    }

    pub enum Output {
        FailedCheck,
        Value(u32),
    }

    impl<I> ActiveCapture<I>
    where
        I: Iterator<Item = char>,
    {
        fn next_value(&mut self, digit: char) -> Option<Output> {
            let next_digit = self.value_iterator.next().unwrap();
            if next_digit == digit {
                if self.value_iterator.peek().is_none() {
                    return Some(Output::Value(self.output));
                }
            } else {
                return Some(Output::FailedCheck);
            }
            return None;
        }
    }

    impl Into<ActiveCapture<Chars<'static>>> for Capture {
        fn into(self) -> ActiveCapture<Chars<'static>> {
            ActiveCapture {
                value_iterator: self.value.chars().skip(1).peekable(),
                output: self.output,
            }
        }
    }

    impl Into<ActiveCapture<Rev<Chars<'static>>>> for Capture {
        fn into(self) -> ActiveCapture<Rev<Chars<'static>>> {
            ActiveCapture {
                value_iterator: self.value.chars().rev().skip(1).peekable(),
                output: self.output,
            }
        }
    }

    macro_rules! capture {
        ($($ident:ident: $value:expr => $output:expr;)*) => {
            $(const $ident: Capture = Capture {
                value: $value,
                output: $output,
            };)*
        }
    }

    capture! {
        CAPTURE_ONE: "one" => 1;
        CAPTURE_TWO: "two" => 2;
        CAPTURE_THREE: "three" => 3;
        CAPTURE_FOUR: "four" => 4;
        CAPTURE_FIVE: "five" => 5;
        CAPTURE_SIX: "six" => 6;
        CAPTURE_SEVEN: "seven" => 7;
        CAPTURE_EIGHT: "eight" => 8;
        CAPTURE_NINE: "nine" => 9;
    }

    pub fn capture_value(full_string: &str) -> u32 {
        fn forward_capture_match<
            B: Iterator<Item = char> + std::fmt::Debug,
            F: Fn(Capture) -> ActiveCapture<B>,
        >(
            into_f: &F,
            c: char,
        ) -> Vec<ActiveCapture<B>> {
            match c {
                'o' => vec![into_f(CAPTURE_ONE.clone())],
                't' => vec![into_f(CAPTURE_TWO.clone()), into_f(CAPTURE_THREE.clone())],
                'f' => vec![into_f(CAPTURE_FOUR.clone()), into_f(CAPTURE_FIVE.clone())],
                's' => vec![into_f(CAPTURE_SIX.clone()), into_f(CAPTURE_SEVEN.clone())],
                'e' => vec![into_f(CAPTURE_EIGHT.clone())],
                'n' => vec![into_f(CAPTURE_NINE.clone())],
                _ => vec![],
            }
        }

        fn backward_capture_match<
            B: Iterator<Item = char> + std::fmt::Debug,
            F: Fn(Capture) -> ActiveCapture<B>,
        >(
            into_f: &F,
            c: char,
        ) -> Vec<ActiveCapture<B>> {
            match c {
                'e' => vec![
                    into_f(CAPTURE_ONE.clone()),
                    into_f(CAPTURE_NINE.clone()),
                    into_f(CAPTURE_THREE.clone()),
                    into_f(CAPTURE_FIVE.clone()),
                ],
                'o' => vec![into_f(CAPTURE_TWO.clone())],
                'r' => vec![into_f(CAPTURE_FOUR.clone())],
                'x' => vec![into_f(CAPTURE_SIX.clone())],
                'n' => vec![into_f(CAPTURE_SEVEN.clone())],
                't' => vec![into_f(CAPTURE_EIGHT.clone())],
                _ => vec![],
            }
        }

        fn capture_value_inner<
            I: Iterator<Item = char>,
            B: Iterator<Item = char> + std::fmt::Debug,
            F: Fn(Capture) -> ActiveCapture<B>,
            F2: Fn(&F, char) -> Vec<ActiveCapture<B>>,
        >(
            capture_function: F2,
            into_f: F,
            value_iterator: I,
        ) -> Option<u32> {
            let mut captures: Vec<ActiveCapture<B>> = vec![];

            enum CaptureResult<B: Iterator<Item = char>> {
                NewCaptures(Vec<ActiveCapture<B>>),
                Complete(u32),
            }

            for item in value_iterator {
                if item.is_ascii_digit() {
                    return item.to_digit(10);
                }

                let new_captures: Vec<ActiveCapture<B>> = capture_function(&into_f, item);

                println!("New captures: {:?}", new_captures);

                captures = match captures.into_iter().fold(
                    CaptureResult::NewCaptures(new_captures),
                    |mut result, mut capture| match &mut result {
                        CaptureResult::NewCaptures(ref mut captures) => {
                            match capture.next_value(item) {
                                None => {
                                    captures.push(capture);
                                    result
                                }
                                Some(Output::Value(value)) => CaptureResult::Complete(value),
                                Some(Output::FailedCheck) => result,
                            }
                        }
                        CaptureResult::Complete(_) => result,
                    },
                ) {
                    CaptureResult::NewCaptures(captures) => captures,
                    CaptureResult::Complete(value) => {
                        return Some(value);
                    }
                }
            }
            return None;
        }

        let forwards = capture_value_inner(
            forward_capture_match,
            Into::<ActiveCapture<Chars<'static>>>::into,
            full_string.chars(),
        )
        .unwrap();
        let backwards = capture_value_inner(
            backward_capture_match,
            Into::<ActiveCapture<Rev<Chars<'static>>>>::into,
            full_string.chars().rev(),
        )
        .unwrap_or(forwards);
        return format!("{}{}", forwards, backwards).parse().unwrap();
    }
}

impl Day for Day1 {
    type InputType = Lines<'static>;

    fn part1(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<()> {
        let solution: u32 = input
            .map(|line| {
                let mut chars = line.chars();
                let f = chars.find(char::is_ascii_digit).unwrap();
                let mut rev = chars.rev();
                let z = rev.find(char::is_ascii_digit).unwrap_or(f);
                return format!("{}{}", f, z).parse::<u32>().unwrap();
            })
            .sum();
        println!("Solution: {}", solution);
        Ok(())
    }

    fn part2(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<()> {
        let solution: u32 = input.map(|line| capture_value(line)).sum();
        println!("Solution: {}", solution);
        Ok(())
    }
}
