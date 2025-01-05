use std::{
    collections::HashSet,
    fmt::{Debug, Display},
};

type Map = Vec<Vec<Tile>>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Obstruction,
    Exit,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Tile::Empty => '.',
            Tile::Obstruction => '#',
            Tile::Exit => 'E',
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    row: usize,
    col: usize,
    direction: Direction,
    exited: bool,
}

pub fn part_1(input: &str) -> usize {
    let (map, guard) = parse_map(input);
    let visited = tiles_visited(&map, guard);
    visited.len()
}

// Place tiles and see if the guard returns to the same position with same direction.
// Reduce search space by only placing tiles on the original path of the guard.
// TODO: Try to more efficiently move the guard by taking many steps at once
pub fn part_2(input: &str) -> usize {
    let (mut map, starting_guard) = parse_map(input);
    let visited = tiles_visited(&map, starting_guard);

    let mut loop_positions = 0;
    for (row, col) in visited {
        let mut current_guard = starting_guard;
        let mut guards = HashSet::from([current_guard]);
        map[row][col] = Tile::Obstruction;
        loop {
            let new_guard = move_guard(&map, current_guard);
            if new_guard.exited {
                break;
            }
            if guards.contains(&new_guard) {
                // The guard looped
                loop_positions += 1;
                break;
            }
            guards.insert(new_guard);
            current_guard = new_guard;
        }
        map[row][col] = Tile::Empty;
    }
    loop_positions
}

fn tiles_visited(map: &Vec<Vec<Tile>>, guard: Guard) -> HashSet<(usize, usize)> {
    let mut current_guard = guard;
    let mut visited = HashSet::from([(current_guard.row, current_guard.col)]);
    loop {
        let new_guard = move_guard(map, current_guard);
        if new_guard.exited {
            break;
        }
        visited.insert((new_guard.row, new_guard.col));
        current_guard = new_guard;
    }
    visited
}

/// Move the guard according to the direction and the map.
fn move_guard(map: &Map, guard: Guard) -> Guard {
    let (next_row, next_col, next_direction) = match guard.direction {
        Direction::Up => (guard.row - 1, guard.col, Direction::Right),
        Direction::Down => (guard.row + 1, guard.col, Direction::Left),
        Direction::Left => (guard.row, guard.col - 1, Direction::Up),
        Direction::Right => (guard.row, guard.col + 1, Direction::Down),
    };

    match map[next_row][next_col] {
        Tile::Empty => Guard {
            row: next_row,
            col: next_col,
            ..guard
        },
        Tile::Obstruction => {
            let guard = Guard {
                direction: next_direction,
                ..guard
            };
            move_guard(map, guard)
        }
        Tile::Exit => Guard {
            exited: true,
            ..guard
        },
    }
}

/// Parse a tile and the direction of the guard if he is present
fn parse_tile(input: char) -> (Tile, Option<Direction>) {
    match input {
        '.' => (Tile::Empty, None),
        '^' => (Tile::Empty, Some(Direction::Up)),
        'v' => (Tile::Empty, Some(Direction::Down)),
        '>' => (Tile::Empty, Some(Direction::Right)),
        '<' => (Tile::Empty, Some(Direction::Left)),
        '#' => (Tile::Obstruction, None),
        _ => panic!("Invalid tile: {}", input),
    }
}

/// Parse a row of tiles and capture the column index of the guard if he is present.
/// Include the exit tiles on both sides of the row.
fn parse_row(input_row: &str) -> (Vec<Tile>, Option<(Direction, usize)>) {
    let (mut row, guard_direction) = input_row.chars().enumerate().fold(
        (vec![Tile::Exit], None),
        |(mut row, mut direction), (col_index, tile)| {
            let (tile, guard_direction) = parse_tile(tile);
            row.push(tile);
            if guard_direction.is_some() {
                direction = guard_direction.map(|direction| (direction, col_index));
            }
            (row, direction)
        },
    );
    row.push(Tile::Exit);
    (row, guard_direction)
}

/// Parse the map and the guard. Include the exit tiles on the top and bottom of the map.
fn parse_map(input: &str) -> (Vec<Vec<Tile>>, Guard) {
    let mut rows = input.lines().peekable();
    let width = rows.peek().unwrap().chars().count() + 2;
    let mut map = vec![vec![Tile::Exit; width]];
    let mut guard = None;
    for (row_index, row) in input.lines().enumerate() {
        let (tiles, guard_direction) = parse_row(row);
        map.push(tiles);
        if let Some((direction, col)) = guard_direction {
            guard = Some(Guard {
                // Account for added exit tiles
                row: row_index + 1,
                col: col + 1,
                direction,
                exited: false,
            });
        }
    }
    map.push(vec![Tile::Exit; width]);
    (map, guard.unwrap())
}

/// Debugging function to print the map with the guard's position and direction.
fn print_map(map: &Map, guard_position: &Guard) -> String {
    let mut result = String::new();
    for (row_index, row) in map.iter().enumerate() {
        for (col_index, tile) in row.iter().enumerate() {
            if row_index == guard_position.row && col_index == guard_position.col {
                result.push(guard_position.direction.to_string().chars().next().unwrap());
            } else {
                result.push(tile.to_string().chars().next().unwrap());
            }
        }
        if row_index < map.len() - 1 {
            result.push('\n');
        }
    }
    result
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

    #[test]
    fn test_parse_map_and_display() {
        let input = [
            "....#.....",
            ".........#",
            "..........",
            "..#.......",
            ".......#..",
            "..........",
            ".#..^.....",
            "........#.",
            "#.........",
            "......#...",
        ]
        .join("\n");

        let expected = [
            "EEEEEEEEEEEE",
            "E....#.....E",
            "E.........#E",
            "E..........E",
            "E..#.......E",
            "E.......#..E",
            "E..........E",
            "E.#..^.....E",
            "E........#.E",
            "E#.........E",
            "E......#...E",
            "EEEEEEEEEEEE",
        ]
        .join("\n");
        let (map, guard_position) = parse_map(&input);
        assert_eq!(print_map(&map, &guard_position).to_string(), expected);
    }
}
