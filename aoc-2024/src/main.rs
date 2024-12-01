use aoc_2024::{day01, utils, Solution};
use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    days: Vec<u8>,
    #[arg(short, long, default_value_t = 1)]
    part: u8,
}

fn get_solver(day: u8) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Box::new(day01::Day01)),
        _ => None,
    }
}

fn main() {
    let args = Args::parse();
    args.days.iter().for_each(|day| {
        // Read input file
        let input_path = format!("data/day{:02}.txt", day);
        let input = fs::read_to_string(Path::new(&input_path))
            .unwrap_or_else(|_| panic!("Error reading input file for day {}", day));

        // Get and run appropriate solver
        if let Some(solver) = get_solver(*day) {
            let result = match args.part {
                1 => solver.part1(&input),
                2 => solver.part2(&input),
                _ => panic!("Invalid part number"),
            };
            println!("Day {} part {}: {}", day, args.part, result);
        } else {
            println!("Solution for day {} not implemented yet", day);
        }
    });
}