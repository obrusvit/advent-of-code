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
}

pub mod day01;
pub mod day02;
