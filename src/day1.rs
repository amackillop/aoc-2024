use nom::{character::complete::space1, sequence::separated_pair};
use std::collections::HashMap;

use crate::{parse_input, parse_number};

/// Sort the left and right numbers, then sum the differences between each pair
pub fn part_1(input: &str) -> usize {
    let (mut left, mut right) = input.lines().map(parse_left_right).fold(
        (vec![], vec![]),
        |(mut left, mut right), (l, r)| {
            left.push(l);
            right.push(r);
            (left, right)
        },
    );
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| r.abs_diff(*l))
        .sum()
}

/// Multiply each left number by the number of times it appears in the right. Sum the result.
pub fn part_2(input: &str) -> usize {
    let (left, right) = input.lines().map(parse_left_right).fold(
        (vec![], HashMap::new()),
        |(mut left, mut right), (l, r)| {
            left.push(l);
            *right.entry(r).or_insert(0) += 1;
            (left, right)
        },
    );
    left.iter().map(|l| l * right.get(l).unwrap_or(&0)).sum()
}

fn parse_left_right(line: &str) -> (usize, usize) {
    parse_input(line, separated_pair(parse_number, space1, parse_number))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        let input = read_input(1);
        assert_eq!(part_1(&input), 2742123);
    }

    #[test]
    fn test_part_2() {
        let input = read_input(1);
        assert_eq!(part_2(&input), 21328497);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = read_input(1);
        b.iter(|| part_1(&input));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = read_input(1);
        b.iter(|| part_2(&input));
    }
}
