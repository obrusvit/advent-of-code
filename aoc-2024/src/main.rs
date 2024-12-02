use aoc_2024::{day01, day02, utils, Solution};
use clap::Parser;

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
        2 => Some(Box::new(day02::Day02)),
        _ => None,
    }
}

fn main() {
    let args = Args::parse();
    args.days.iter().for_each(|day| {
        // Read input file
        let input = utils::read_input(*day);

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
