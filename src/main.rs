#![feature(
    str_split_once,
    map_into_keys_values
)]
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

aoc_main::main! {
    year 2020;
    day1 : input_generator => solve_part1_slow, solve_part2_slow, solve_part1_set, solve_part2_set;
    day2 : input_generator => part_1, part_2, part_2_bytes;
    day3 : input_generator => part_1, part_2;
    day4 : input_generator => part_1, part_2;
    day5 : input_generator => part_1;
}
