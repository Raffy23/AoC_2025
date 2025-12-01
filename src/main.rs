use std::collections::HashSet;

use aoc_2025::{Part, day01, utils::read_input};
use clap::{Parser, command};

/// Simple runner for AoC 2025
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct RunnerArgs {
    /// The day that should be run (1 - 12)
    #[arg(short, long)]
    day: u8,

    /// Part that should be run (1,2)
    #[arg(short, long)]
    part: u8,
}

fn main() {
    let args = RunnerArgs::parse();
    run_day(args.day, args.part);
}

fn run_day(day: u8, part: u8) {
    let separate_input_files: HashSet<u8> = HashSet::with_capacity(12);
    {
        // separate_input_files.insert(1);
    }

    let input = read_input(
        day,
        separate_input_files.get(&day).map(|_| Part::from(part)),
    )
    .expect("unable to read input file");

    print!("Day {:0>2} Part {}: ", day, part);

    match (day, part) {
        (1, 1) => println!("{:?}", day01::solve1(input.as_str())),
        (1, 2) => println!("{:?}", day01::solve2(input.as_str())),
        _ => todo!(),
    }
}
