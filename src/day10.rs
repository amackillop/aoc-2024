use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
    altitude: u8,
}

const INPUT: &str = include_str!("../input/10.txt");

pub fn part_1() -> usize {
    trailheads().map(compute_score).sum()
}

pub fn part_2() -> usize {
    trailheads().map(compute_rating).sum()
}

fn compute_score(start: Position) -> usize {
    let checked = HashSet::new();
    peaks_reachable(
        start,
        INPUT.lines().next().unwrap().len() + 1, // +1 to account for newlines
        INPUT.lines().count(),
        INPUT.as_bytes(),
        0,
        &mut Some(checked),
    )
}

fn compute_rating(start: Position) -> usize {
    peaks_reachable(
        start,
        INPUT.lines().next().unwrap().len() + 1,
        INPUT.lines().count(),
        INPUT.as_bytes(),
        0,
        &mut None,
    )
}

fn peaks_reachable(
    pos: Position,
    map_width: usize,
    map_length: usize,
    map: &'static [u8],
    peaks: usize,
    checked: &mut Option<HashSet<Position>>,
) -> usize {
    if let Some(checked) = checked {
        checked.insert(pos);
    }
    if pos.altitude == b'9' {
        return 1;
    }

    let directions = [
        (0, 1),  // right
        (0, -1), // left
        (1, 0),  // up
        (-1, 0), // down
    ];

    directions.iter().fold(peaks, |current_peaks, &(dy, dx)| {
        let next_row = pos.row as isize + dy;
        let next_col = pos.col as isize + dx;

        if next_row < 0
            || next_row >= map_length as isize
            || next_col < 0
            || next_col >= map_width as isize
        {
            return current_peaks;
        }

        let next_pos = Position {
            row: next_row as usize,
            col: next_col as usize,
            altitude: pos.altitude + 1,
        };
        if checked.iter().any(|set| set.contains(&next_pos)) {
            return current_peaks;
        }

        let index = next_pos.row * map_width + next_pos.col;
        if let Some(&next_altitude) = map.get(index) {
            if next_altitude == next_pos.altitude {
                current_peaks
                    + peaks_reachable(next_pos, map_width, map_length, map, peaks, checked)
            } else {
                current_peaks
            }
        } else {
            current_peaks
        }
    })
}

fn trailheads() -> impl Iterator<Item = Position> {
    INPUT.lines().enumerate().flat_map(|(row, line)| {
        line.chars()
            .enumerate()
            .filter_map(move |(col, c)| match c {
                '0' => Some(Position {
                    row,
                    col,
                    altitude: b'0',
                }),
                _ => None,
            })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        // assert_eq!(part_1(), 36);
        assert_eq!(part_1(), 538);
    }

    #[test]
    fn test_part_2() {
        // assert_eq!(part_2(), 81);
        assert_eq!(part_2(), 1110);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part_1);
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(part_2);
    }
}
