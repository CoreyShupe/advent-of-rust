use crate::aoc::{Day, InputParser};

pub struct Day4;

impl Day for Day4 {
    type InputType = &'static str;

    fn part1(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<()> {
        let mut incr = 1;
        loop {
            let test = md5::compute(format!("{}{}", input, incr));
            if format!("{:x}", test).starts_with("00000") {
                break;
            }
            incr += 1;
        }
        println!("Solution: {}", incr);
        Ok(())
    }

    fn part2(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<()> {
        let mut incr = 1;
        loop {
            let test = md5::compute(format!("{}{}", input, incr));
            if test.0[0..=2] == [0, 0, 0] {
                break;
            }
            incr += 1;
        }
        println!("Solution: {}", incr);
        Ok(())
    }
}
