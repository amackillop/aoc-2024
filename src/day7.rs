use std::{
    collections::HashSet,
    fmt::{Debug, Display},
};

use nom::{
    bytes::complete::tag, character::complete::space1, multi::separated_list1, sequence::separated_pair, IResult
};

use crate::{parse_input, parse_number};

#[derive(Debug)]
struct Equation {
    test_value: usize,
    list: Vec<usize>,
}

pub fn part_1(input: &str) -> usize {
    input.lines().map(parse_line).for_each(|equation| {
        println!("{:?}", equation);
    });
    0
}

pub fn part_2(input: &str) -> usize {
    0
}

fn parse_list(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(space1, parse_number)(input)
}

fn parse_line(line: &str) -> Equation {
    let (test_value, list) = parse_input(line, separated_pair(parse_number, tag(": "), parse_list));
    Equation { test_value, list }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        let input = read_input(6);
        // assert_eq!(part_1(&input), 41);
        assert_eq!(part_1(&input), 5101);
    }

    #[test]
    fn test_part_2() {
        let input = read_input(6);
        // assert_eq!(part_2(&input), 6);
        assert_eq!(part_2(&input), 1951);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = read_input(6);
        b.iter(|| part_1(&input));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = read_input(6);
        b.iter(|| part_2(&input));
    }
}
