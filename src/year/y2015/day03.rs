use crate::aoc::{Day, InputParser};
use std::collections::HashSet;
use std::str::Chars;

struct Pointer {
    x: isize,
    y: isize,
}

pub struct Day3;
impl Day for Day3 {
    type InputType = Chars<'static>;

    fn part1(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<()> {
        let mut pointer = Pointer { x: 0, y: 0 };
        let mut visited = HashSet::new();
        visited.insert((0, 0));

        for x in input {
            match x {
                '^' => pointer.y += 1,
                'v' => pointer.y -= 1,
                '>' => pointer.x += 1,
                '<' => pointer.x -= 1,
                _ => (),
            }

            visited.insert((pointer.x, pointer.y));
        }

        let solution = visited.len();
        println!("Solution: {}", solution);
        Ok(())
    }

    fn part2(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<()> {
        let mut pointer = Pointer { x: 0, y: 0 };
        let mut robo_pointer = Pointer { x: 0, y: 0 };
        let mut state = true;

        let mut visited = HashSet::new();
        visited.insert((0, 0));

        for x in input {
            fn visit(pointer: &mut Pointer, x: char, visited: &mut HashSet<(isize, isize)>) {
                match x {
                    '^' => pointer.y += 1,
                    'v' => pointer.y -= 1,
                    '>' => pointer.x += 1,
                    '<' => pointer.x -= 1,
                    _ => (),
                }

                visited.insert((pointer.x, pointer.y));
            }

            if state {
                visit(&mut pointer, x, &mut visited);
            } else {
                visit(&mut robo_pointer, x, &mut visited);
            }

            state = !state;
        }

        let solution = visited.len();
        println!("Solution: {}", solution);
        Ok(())
    }
}
