use crate::aoc::{Day, InputParser};
use std::collections::HashMap;

pub struct Day8;

#[derive(Debug, Clone, Copy)]
pub enum LR {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Mapping {
    instructions: Vec<LR>,
    a_enders: Vec<String>,
    mappings: HashMap<String, (String, String)>,
}

pub fn resolve_mappings(
    lr: LR,
    mappings: &HashMap<String, (String, String)>,
    key: &String,
) -> String {
    match lr {
        LR::Left => mappings.get(key).map(|x| x.0.to_string()).unwrap(),
        LR::Right => mappings.get(key).map(|x| x.1.to_string()).unwrap(),
    }
}

impl InputParser for Mapping {
    type ResolvedType<'a> = Mapping;

    fn parse_input(input: &str) -> Self::ResolvedType<'_> {
        let mut lines = input.lines();
        let instructions = lines
            .next()
            .unwrap()
            .chars()
            .map(|x| match x {
                'L' => LR::Left,
                'R' => LR::Right,
                _ => panic!(),
            })
            .collect();

        let mut mappings = HashMap::new();

        let mut a_enders = vec![];

        for line in lines.skip(1) {
            let mut iter = line.split(" = ");
            let key = iter.next();
            let values = iter
                .next()
                .unwrap()
                .to_string()
                .replace("(", "")
                .replace(")", "");
            let mut values_split = values.split(", ");

            let key = key.unwrap().to_string();
            if key.ends_with("A") {
                a_enders.push(key.clone());
            }

            mappings.insert(
                key,
                (
                    values_split.next().unwrap().to_string(),
                    values_split.next().unwrap().to_string(),
                ),
            );
        }

        Mapping {
            instructions,
            a_enders,
            mappings,
        }
    }
}

impl Day for Day8 {
    type InputType = Mapping;

    fn part1(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<String> {
        let mut cycler = input.instructions.iter().cycle();
        let mappings = input.mappings;
        let mut current: String = format!("AAA");
        let mut counter = 1;

        while let Some(next) = cycler.next() {
            current = resolve_mappings(*next, &mappings, &current);
            if current.eq_ignore_ascii_case("zzz") {
                break;
            }
            counter += 1;
        }

        let solution = counter;
        Ok(format!("{}", solution))
    }

    fn part2(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<String> {
        let mut cycler = input.instructions.iter().cycle();
        let mappings = input.mappings;
        let mut current = input.a_enders;
        let mut counter = 1;

        let mut groups = vec![];

        while let Some(next) = cycler.next() {
            current = current
                .into_iter()
                .map(|key| resolve_mappings(*next, &mappings, &key))
                .collect();

            for (idx, x) in current.iter().enumerate() {
                if x.ends_with("Z") {
                    groups.push(counter);
                }
            }
            current = current.into_iter().filter(|x| !x.ends_with("Z")).collect();

            if current.len() == 0 {
                break;
            }

            counter += 1;
        }

        let mut groups: Vec<usize> = groups.into_iter().map(|x| x as usize).collect();
        groups.sort();

        let mut stepper: usize = groups[0];
        groups = groups.into_iter().skip(1).collect();
        let mut step = stepper;

        loop {
            step += stepper;
            if let Some(v) = groups.iter().find(|v| step % *v == 0) {
                let v = *v;
                groups = groups.into_iter().filter(|x| *x != v).collect();
                stepper = step;
            }
            if groups.len() == 0 {
                break;
            }
        }

        let solution = step;
        Ok(format!("{}", solution))
    }
}
