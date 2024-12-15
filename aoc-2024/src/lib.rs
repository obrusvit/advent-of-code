pub trait Solution {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

pub mod utils {
    use std::fs;
    use std::path::Path;

    pub fn read_input(day: u8) -> String {
        let input_path = format!("data/day{:02}.txt", day);
        fs::read_to_string(Path::new(&input_path))
            .unwrap_or_else(|_| panic!("Error reading input file for day {}", day))
    }

    pub fn parse_numbers_from_string(input: &str) -> Vec<i32> {
        input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()
    }

    pub fn parse_numbers_from_string_comma(input: &str) -> Vec<i32> {
        input.split(",").map(|s| s.parse().unwrap()).collect()
    }
}

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;

pub fn get_solver(day: u8) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Box::new(day01::Day01)),
        2 => Some(Box::new(day02::Day02)),
        3 => Some(Box::new(day03::Day03)),
        4 => Some(Box::new(day04::Day04)),
        5 => Some(Box::new(day05::Day05)),
        6 => Some(Box::new(day06::Day06)),
        7 => Some(Box::new(day07::Day07)),
        8 => Some(Box::new(day08::Day08)),
        9 => Some(Box::new(day09::Day09)),
        10 => Some(Box::new(day10::Day10)),
        11 => Some(Box::new(day11::Day11)),
        12 => Some(Box::new(day12::Day12)),
        13 => Some(Box::new(day13::Day13)),
        _ => None,
    }
}
