use crate::aoc::{Day, InputParser};

pub struct Day3;

pub enum Symbol {
    Gear(Position),
    NonGear(Position),
}

impl Symbol {
    pub fn is_gear(&self) -> bool {
        match self {
            Self::Gear(_) => true,
            Self::NonGear(_) => false,
        }
    }

    pub fn pos(&self) -> &Position {
        match self {
            Self::Gear(ref pos) => pos,
            Self::NonGear(ref pos) => pos,
        }
    }
}

pub struct Position {
    x: usize,
    y: usize,
}

pub struct OrderNumber {
    positions: Vec<usize>,
    y_order: usize,
    value: usize,
}

impl OrderNumber {
    pub fn is_adjacent_to(&self, symbol: &Position) -> bool {
        match symbol.y as isize - self.y_order as isize {
            0 =>
            // symbol is next to the position
            {
                self.positions
                    .iter()
                    .find(|position| {
                        **position + 1 == symbol.x
                            || (**position != 0 && **position - 1 == symbol.x)
                    })
                    .is_some()
            }
            -1 | 1 =>
            // symbol is below or above the position
            {
                self.positions
                    .iter()
                    .find(|position| {
                        **position == symbol.x
                            || **position + 1 == symbol.x
                            || (**position != 0 && **position - 1 == symbol.x)
                    })
                    .is_some()
            }
            _ => false,
        }
    }

    pub fn check_against(&self, symbols: &Vec<Symbol>) -> bool {
        symbols
            .iter()
            .find(|symbol| self.is_adjacent_to(symbol.pos()))
            .is_some()
    }
}

#[derive(Default)]
pub struct PartTable {
    order_numbers: Vec<OrderNumber>,
    symbols: Vec<Symbol>,
}

pub struct PartTableParser;
impl InputParser for PartTableParser {
    type ResolvedType<'a> = PartTable;

    fn parse_input(input: &str) -> Self::ResolvedType<'_> {
        let mut part_table = PartTable::default();

        let mut y_marker = 0;
        for x in input.lines() {
            let mut x_marker = 0;
            let mut number_builder: Option<String> = None;

            fn pop_number_builder(
                part_table: &mut PartTable,
                input: Option<String>,
                x_marker: usize,
                y_marker: usize,
            ) {
                if let Some((initial, Ok(number_out))) =
                    input.map(|string| (string.clone(), string.parse::<usize>()))
                {
                    part_table.order_numbers.push(OrderNumber {
                        positions: (1..=initial.len())
                            .into_iter()
                            .map(|x| x_marker - x)
                            .collect(),
                        y_order: y_marker,
                        value: number_out,
                    })
                }
            }

            for c in x.chars() {
                if c.is_ascii_digit() {
                    number_builder = match &number_builder {
                        Some(current) => Some(format!("{}{}", current, c)),
                        None => Some(format!("{}", c)),
                    };

                    x_marker += 1;
                    continue;
                }

                pop_number_builder(&mut part_table, number_builder.take(), x_marker, y_marker);

                if c != '.' {
                    let pos = Position {
                        x: x_marker,
                        y: y_marker,
                    };
                    if c == '*' {
                        part_table.symbols.push(Symbol::Gear(pos));
                    } else {
                        part_table.symbols.push(Symbol::NonGear(pos));
                    }
                }

                x_marker += 1;
            }

            pop_number_builder(&mut part_table, number_builder.take(), x_marker, y_marker);
            y_marker += 1;
        }

        part_table
    }
}

impl Day for Day3 {
    type InputType = PartTableParser;

    fn part1(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<String> {
        let solution: usize = input
            .order_numbers
            .iter()
            .filter(|order_number| order_number.check_against(&input.symbols))
            .map(|order_number| order_number.value)
            .sum();
        Ok(format!("{}", solution))
    }

    fn part2(input: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<String> {
        let solution: usize = input
            .symbols
            .iter()
            .filter(|gear| gear.is_gear())
            .map(|gear| {
                input
                    .order_numbers
                    .iter()
                    .filter(|order| order.is_adjacent_to(gear.pos()))
                    .map(|order| order.value)
                    .collect::<Vec<usize>>()
            })
            .filter(|gear_associations| gear_associations.len() == 2)
            .map(|gear_associations| gear_associations[0] * gear_associations[1])
            .sum();
        Ok(format!("{}", solution))
    }
}
