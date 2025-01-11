use aoc_2024::{
    day1, day10, day2, day3, day4, day5, day6, day7, day8, day9, read_input
};
use clap::Parser;
use tikv_jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

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
        (8, 1) => day8::part_1(&input),
        (8, 2) => day8::part_2(&input),
        (9, 1) => day9::part_1(&input),
        (9, 2) => day9::part_2(&input),
        (10, 1) => day10::part_1(),
        (10, 2) => day10::part_2(),
        // (11, 1) => day11::part_1(),
        // (11, 2) => day11::part_2(),
        // (12, 1) => day12::part_1(),
        // (12, 2) => day12::part_2(),
        // (13, 1) => day13::part_1(),
        // (13, 2) => day13::part_2(),
        // (14, 1) => day14::part_1(),
        // (14, 2) => day14::part_2(),
        // (15, 1) => day15::part_1(),
        // (15, 2) => day15::part_2(),
        // (16, 1) => day16::part_1(),
        // (16, 2) => day16::part_2(),
        // (17, 1) => day17::part_1(),
        // (17, 2) => day17::part_2(),
        // (18, 1) => day18::part_1(),
        // (18, 2) => day18::part_2(),
        // (19, 1) => day19::part_1(),
        // (19, 2) => day19::part_2(),
        // (20, 1) => day20::part_1(),
        // (20, 2) => day20::part_2(),
        // (21, 1) => day21::part_1(),
        // (21, 2) => day21::part_2(),
        // (22, 1) => day22::part_1(),
        // (22, 2) => day22::part_2(),
        // (23, 1) => day23::part_1(),
        // (23, 2) => day23::part_2(),
        // (24, 1) => day24::part_1(),
        // (24, 2) => day24::part_2(),
        // (25, 1) => day25::part_1(),
        // (25, 2) => day25::part_2(),
        _ => panic!("Day/part not implemented"),
    };

    println!("{solution}");
}
