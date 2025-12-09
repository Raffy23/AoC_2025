use std::collections::HashSet;

use aoc_2025::{Part, day01, day02, day03, day04, day05, day06, day07, day08, day09, utils::read_input};
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
        (2, 1) => println!("{:?}", day02::solve1(input.as_str())),
        (2, 2) => println!("{:?}", day02::solve2(input.as_str())),
        (3, 1) => println!("{:?}", day03::solve1(input.as_str())),
        (3, 2) => println!("{:?}", day03::solve2(input.as_str())),
        (4, 1) => println!("{:?}", day04::solve1(input.as_str())),
        (4, 2) => println!("{:?}", day04::solve2(input.as_str())),
        (5, 1) => println!("{:?}", day05::solve1(input.as_str())),
        (5, 2) => println!("{:?}", day05::solve2(input.as_str())),
        (6, 1) => println!("{:?}", day06::solve1(input.as_str())),
        (6, 2) => println!("{:?}", day06::solve2(input.as_str())),
        (7, 1) => println!("{:?}", day07::solve1(input.as_str())),
        (7, 2) => println!("{:?}", day07::solve2(input.as_str())),
        (8, 1) => println!("{:?}", day08::solve1(input.as_str())),
        (8, 2) => println!("{:?}", day08::solve2(input.as_str())),
        (9, 1) => println!("{:?}", day09::solve1(input.as_str())),
        (9, 2) => println!("{:?}", day09::solve2(input.as_str())),
        _ => todo!(),
    }
}
