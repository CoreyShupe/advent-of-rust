use std::fmt::Formatter;

pub enum YearSwitch {
    Y2015,
    Y2023,
}

impl std::fmt::Display for YearSwitch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            YearSwitch::Y2015 => write!(f, "2015"),
            YearSwitch::Y2023 => write!(f, "2023"),
        }
    }
}

pub enum DaySwitch {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

impl std::fmt::Display for DaySwitch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DaySwitch::Day1 => write!(f, "day1"),
            DaySwitch::Day2 => write!(f, "day2"),
            DaySwitch::Day3 => write!(f, "day3"),
            DaySwitch::Day4 => write!(f, "day4"),
            DaySwitch::Day5 => write!(f, "day5"),
            DaySwitch::Day6 => write!(f, "day6"),
            DaySwitch::Day7 => write!(f, "day7"),
            DaySwitch::Day8 => write!(f, "day8"),
            DaySwitch::Day9 => write!(f, "day9"),
            DaySwitch::Day10 => write!(f, "day10"),
            DaySwitch::Day11 => write!(f, "day11"),
            DaySwitch::Day12 => write!(f, "day12"),
            DaySwitch::Day13 => write!(f, "day13"),
            DaySwitch::Day14 => write!(f, "day14"),
            DaySwitch::Day15 => write!(f, "day15"),
            DaySwitch::Day16 => write!(f, "day16"),
            DaySwitch::Day17 => write!(f, "day17"),
            DaySwitch::Day18 => write!(f, "day18"),
            DaySwitch::Day19 => write!(f, "day19"),
            DaySwitch::Day20 => write!(f, "day20"),
            DaySwitch::Day21 => write!(f, "day21"),
            DaySwitch::Day22 => write!(f, "day22"),
            DaySwitch::Day23 => write!(f, "day23"),
            DaySwitch::Day24 => write!(f, "day24"),
            DaySwitch::Day25 => write!(f, "day25"),
        }
    }
}

pub enum Part {
    Part1,
    Part2,
}
