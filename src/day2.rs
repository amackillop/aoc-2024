use nom::{character::complete::space1, multi::separated_list1};

use crate::{parse_input, parse_number};

pub fn part_1(input: &str) -> usize {
    solution(input, floor_is_safe)
}

pub fn part_2(input: &str) -> usize {
    solution(input, |floor| {
        floor_is_safe(floor) || floor_is_safe_less_one_level(floor)
    })
}

/// Given a predicate for safety, count the number of safe floors
fn solution(input: &str, is_safe: impl Fn(&[usize]) -> bool) -> usize {
    input
        .lines()
        .map(parse_floor)
        .filter(|floor| is_safe(floor))
        .count()
}

/// Check if the floor is safe
fn floor_is_safe(floor: &[usize]) -> bool {
    (all_decreasing(floor) || all_increasing(floor)) && greatest_difference(floor) <= 3
}

/// Check if the floor is safe with one level removed
fn floor_is_safe_less_one_level(floor: &[usize]) -> bool {
    for i in 0..floor.len() {
        let mut new_floor = floor.to_vec();
        new_floor.remove(i);
        if floor_is_safe(&new_floor) {
            return true;
        }
    }
    false
}

fn all_decreasing(floor: &[usize]) -> bool {
    floor.windows(2).all(|w| w[0] > w[1])
}

fn all_increasing(floor: &[usize]) -> bool {
    floor.windows(2).all(|w| w[0] < w[1])
}

fn greatest_difference(floor: &[usize]) -> usize {
    floor.windows(2).map(|w| w[0].abs_diff(w[1])).max().unwrap()
}

fn parse_floor(line: &str) -> Vec<usize> {
    parse_input(line, separated_list1(space1, parse_number))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        let input = read_input(2);
        assert_eq!(part_1(&input), 432);
    }

    #[test]
    fn test_part_2() {
        let input = read_input(2);
        assert_eq!(part_2(&input), 488);
    }

    #[bench]
    fn bench_solution(b: &mut Bencher) {
        let input = read_input(2);
        b.iter(|| {
            solution(&input, |floor| {
                floor_is_safe(floor) || floor_is_safe_less_one_level(floor)
            })
        });
    }
}
