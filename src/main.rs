use aoc_2024::{day1, day2, day3, day4, day5, day6, day7, read_input};
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    day: u8,
    #[arg(short, long)]
    part: u8,
}

fn main() {
    let args = Args::parse();
    let input = read_input(args.day);
    let solution = match (args.day, args.part) {
        (1, 1) => day1::part_1(&input),
        (1, 2) => day1::part_2(&input),
        (2, 1) => day2::part_1(&input),
        (2, 2) => day2::part_2(&input),
        (3, 1) => day3::part_1(&input),
        (3, 2) => day3::part_2(&input),
        (4, 1) => day4::part_1(&input),
        (4, 2) => day4::part_2(&input),
        (5, 1) => day5::part_1(&input),
        (5, 2) => day5::part_2(&input),
        (6, 1) => day6::part_1(&input),
        (6, 2) => day6::part_2(&input),
        (7, 1) => day7::part_1(&input),
        (7, 2) => day7::part_2(&input),
        _ => panic!("Day/part not implemented"),
    };

    println!("{solution}");
}
