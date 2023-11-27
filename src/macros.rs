#[macro_export]
macro_rules! impl_year {
    ($year:ident: [ $($mod_path:ident::$day:ident),* ]) => {
        use $crate::aoc::Day;

        $(
        mod $mod_path;
        use $mod_path::$day;
        )*

        pub struct $year;

        impl $crate::aoc::Year for $year {
            fn solve(
                day: $crate::switch::DaySwitch,
                part: $crate::switch::Part,
                input: String,
            ) -> anyhow::Result<()> {
                #[allow(unreachable_patterns)]
                match day {
                    $($crate::switch::DaySwitch::$day => $day::solve_raw(&input, part),)*
                    _ => unimplemented!(),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! day {
    ($day:ident {
        input<$life:lifetime>: $input_type:ty,

        part1($part1_input_ident:ident) $part1_executor:block

        part2($part2_input_ident:ident) $part2_executor:block
    }) => {
        pub struct $day;

        impl $crate::aoc::Day for $day {
            type InputType<$life> = $input_type;

            fn part1($part1_input_ident: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<()> {
                $part1_executor
                Ok(())
            }

            fn part2($part2_input_ident: <Self::InputType as InputParser>::ResolvedType<'_>) -> anyhow::Result<()> {
                $part2_executor
                Ok(())
            }
        }
    };
}
