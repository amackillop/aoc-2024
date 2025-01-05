use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

#[derive(Debug)]
struct Antenna {
    frequency: char,
    row: i32,
    col: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct AntiNode {
    row: i32,
    col: i32,
}

pub fn part_1(input: &str) -> usize {
    let (antennas, max_row, max_col) = parse_antennas(input);
    solution(&antennas, max_row, max_col, compute_antinodes_1)
}

pub fn part_2(input: &str) -> usize {
    let (antennas, max_row, max_col) = parse_antennas(input);
    solution(&antennas, max_row, max_col, |antenna_1, antenna_2| {
        compute_antinodes_2(antenna_1, antenna_2, max_row, max_col)
    })
}

fn solution(
    antennas: &[Antenna],
    max_row: i32,
    max_col: i32,
    antinode_fn: impl Fn(&Antenna, &Antenna) -> Vec<AntiNode>,
) -> usize {
    antennas
        .iter()
        .fold(HashMap::<char, Vec<_>>::new(), |mut acc, antenna| {
            acc.entry(antenna.frequency).or_default().push(antenna);
            acc
        })
        .iter()
        .flat_map(|(_, antennas)| antinodes(antennas, &antinode_fn))
        .filter(|antinode| {
            antinode.row >= 0
                && antinode.col >= 0
                && antinode.row <= max_row
                && antinode.col <= max_col
        })
        .collect::<HashSet<_>>()
        .len()
}

fn antinodes(
    antennas: &[&Antenna],
    antinode_fn: impl Fn(&Antenna, &Antenna) -> Vec<AntiNode>,
) -> Vec<AntiNode> {
    if antennas.len() < 2 {
        return vec![];
    }
    let mut checked = vec![antennas[0]];
    let mut antinodes = vec![];
    for antenna in antennas.iter().skip(1) {
        let new_antinodes = new_antinodes(&checked, antenna, &antinode_fn);
        antinodes.extend(new_antinodes);
        checked.push(antenna);
    }
    antinodes
}

fn new_antinodes(
    antennas: &[&Antenna],
    new_antenna: &Antenna,
    antinode_fn: impl Fn(&Antenna, &Antenna) -> Vec<AntiNode>,
) -> Vec<AntiNode> {
    antennas
        .iter()
        .flat_map(|antenna| antinode_fn(antenna, new_antenna))
        .collect()
}

fn compute_antinodes_1(antenna_1: &Antenna, antenna_2: &Antenna) -> Vec<AntiNode> {
    let row_diff = antenna_2.row - antenna_1.row;
    let col_diff = antenna_2.col - antenna_1.col;
    let antinode_1 = AntiNode {
        row: antenna_1.row - row_diff,
        col: antenna_1.col - col_diff,
    };
    let antinode_2 = AntiNode {
        row: antenna_2.row + row_diff,
        col: antenna_2.col + col_diff,
    };
    [antinode_1, antinode_2].to_vec()
}

fn compute_antinodes_2(
    antenna_1: &Antenna,
    antenna_2: &Antenna,
    max_row: i32,
    max_col: i32,
) -> Vec<AntiNode> {
    let row_diff = antenna_2.row - antenna_1.row;
    let col_diff = antenna_2.col - antenna_1.col;
    let mut antinodes = vec![AntiNode {
        row: antenna_1.row,
        col: antenna_1.col,
    }];
    for i in 1.. {
        let antinode = AntiNode {
            row: antenna_1.row - i * row_diff,
            col: antenna_1.col - i * col_diff,
        };
        if antinode.row < 0 || antinode.col < 0 {
            break;
        }
        antinodes.push(antinode);
    }

    for i in 1.. {
        let antinode = AntiNode {
            row: antenna_1.row + i * row_diff,
            col: antenna_1.col + i * col_diff,
        };
        if antinode.row > max_row || antinode.col > max_col {
            break;
        }
        antinodes.push(antinode);
    }

    antinodes
}

/// Get the antennas from the input as well as the bounds (max_row, max_col) of the grid
fn parse_antennas(input: &str) -> (Vec<Antenna>, i32, i32) {
    let mut antennas = Vec::new();
    let mut max_row = 0;
    let mut max_col = 0;
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c.is_ascii_alphanumeric() {
                antennas.push(Antenna {
                    frequency: c,
                    row: row as i32,
                    col: col as i32,
                });
            }
            max_row = row;
            max_col = col;
        }
    }
    (antennas, max_row as i32, max_col as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        let input = read_input(8);
        // assert_eq!(part_1(&input), 14);
        assert_eq!(part_1(&input), 400);
    }

    #[test]
    fn test_part_2() {
        let input = read_input(8);
        // assert_eq!(part_2(&input), 34);
        assert_eq!(part_2(&input), 1280);
    }

    #[bench]
    fn bench_solution(b: &mut Bencher) {
        let input = read_input(8);
        let (antennas, max_row, max_col) = parse_antennas(&input);

        b.iter(|| {
            solution(&antennas, max_row, max_col, |a_1, a_2| {
                compute_antinodes_2(a_1, a_2, max_row, max_col)
            })
        });
    }

    #[test]
    fn test_compute_antinodes_1() {
        let antenna_1 = Antenna {
            frequency: 'a',
            row: 3,
            col: 4,
        };
        let antenna_2 = Antenna {
            frequency: 'a',
            row: 5,
            col: 5,
        };
        let antinodes = compute_antinodes_1(&antenna_1, &antenna_2);
        assert_eq!(antinodes, vec![AntiNode { row: 1, col: 3 }, AntiNode {
            row: 7,
            col: 6
        },]);
    }

    #[test]
    fn test_compute_antinodes_2() {
        let antenna_1 = Antenna {
            frequency: 'a',
            row: 3,
            col: 4,
        };
        let antenna_2 = Antenna {
            frequency: 'a',
            row: 5,
            col: 5,
        };
        let antinodes = compute_antinodes_2(&antenna_1, &antenna_2, 11, 11);
        assert_eq!(antinodes, vec![
            AntiNode { row: 3, col: 4 },
            AntiNode { row: 1, col: 3 },
            AntiNode { row: 5, col: 5 },
            AntiNode { row: 7, col: 6 },
            AntiNode { row: 9, col: 7 },
            AntiNode { row: 11, col: 8 }
        ]);
    }
}