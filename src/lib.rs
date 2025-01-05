#![feature(gen_blocks)]
#![feature(test)]
extern crate test;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

use nom::{Finish as _, IResult, character::complete::digit1, combinator::map_res};
use std::{cmp::min, str::FromStr};

// Load lines from a file
pub fn read_input(day: u8) -> String {
    std::fs::read_to_string(format!("./input/{day}.txt")).unwrap()
}

/// Parse the input using the given parser
pub fn parse_input<'a, T>(
    input: &'a str,
    mut parser: impl FnMut(&'a str) -> IResult<&'a str, T>,
) -> T {
    let (_, output) = parser(input)
        .finish()
        .unwrap_or_else(|e| panic!("Failed to parse input: {e:?}"));
    output
}

pub fn parse_number<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
{
    map_res(digit1, str::parse)(input)
}

fn transpose<T: Copy>(input: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut transposed = vec![Vec::with_capacity(input.len()); input[0].len()];
    for row in input {
        for (i, cell) in row.iter().enumerate() {
            transposed[i].push(*cell);
        }
    }
    transposed
}

/// Utility struct to iterate over grids
pub struct Grid<'a, T> {
    rows: &'a [Vec<T>],
    row_len: usize,
    col_len: usize,
}

impl<'a, T> Grid<'a, T> {
    fn new(rows: &'a [Vec<T>]) -> Self {
        Self {
            rows,
            row_len: rows[0].len(),
            col_len: rows.len(),
        }
    }

    fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        self.rows.iter().map(|row| row.iter())
    }

    fn row(&self, i: usize) -> &[T] {
        &self.rows[i]
    }

    fn cols(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.rows[0].len()).map(|i| self.col(i))
    }

    fn col(&self, i: usize) -> impl Iterator<Item = &T> {
        self.rows.iter().map(move |row| &row[i])
    }

    fn diagonals(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.rows[0].len() + self.rows.len() - 1).map(|i| self.diagonal(i))
    }

    fn diagonals_rev(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.rows[0].len() + self.rows.len() - 1).map(|i| self.diagonal_rev(i))
    }

    fn diagonal(&self, i: usize) -> impl Iterator<Item = &T> {
        self.diagonal_inner(i, self.rows.iter())
    }

    fn diagonal_rev(&self, i: usize) -> impl Iterator<Item = &T> {
        self.diagonal_inner(i, self.rows.iter().rev())
    }

    fn diagonal_inner(
        &self,
        i: usize,
        rows: impl Iterator<Item = &'a Vec<T>>,
    ) -> impl Iterator<Item = &T> {
        assert!(
            i < self.row_len + self.col_len - 1,
            "Invalid diagonal index. Should be less than row_len + col_len - 1"
        );
        let skip = i.saturating_sub(self.row_len - 1);
        let take = min(i + 1, self.row_len) - skip;
        gen move {
            for (i, row) in rows.skip(skip).take(take).enumerate() {
                let row_index = take - i - 1 + skip;
                yield &row[row_index];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_grid_rows() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::new(&grid);
        let rows = grid
            .rows()
            .map(|row| row.collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(rows, vec![vec![&1, &2, &3], vec![&4, &5, &6], vec![
            &7, &8, &9
        ]]);
    }

    #[test]
    fn test_grid_cols() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::new(&grid);
        let cols = grid
            .cols()
            .map(|col| col.collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(cols, vec![vec![&1, &4, &7], vec![&2, &5, &8], vec![
            &3, &6, &9
        ]]);
    }

    #[test]
    fn test_grid_diagonal() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::new(&grid);
        let diagonal: Vec<&i32> = grid.diagonal(0).collect();
        assert_eq!(diagonal, vec![&1]);
        let diagonal: Vec<&i32> = grid.diagonal(1).collect();
        assert_eq!(diagonal, vec![&2, &4]);
        let diagonal: Vec<&i32> = grid.diagonal(2).collect();
        assert_eq!(diagonal, vec![&3, &5, &7]);
        let diagonal: Vec<&i32> = grid.diagonal(3).collect();
        assert_eq!(diagonal, vec![&6, &8]);
        let diagonal: Vec<&i32> = grid.diagonal(4).collect();
        assert_eq!(diagonal, vec![&9]);
    }

    #[test]
    fn test_grid_diagonal_rev() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::new(&grid);
        let diagonal: Vec<&i32> = grid.diagonal_rev(0).collect();
        assert_eq!(diagonal, vec![&7]);
        let diagonal: Vec<&i32> = grid.diagonal_rev(1).collect();
        assert_eq!(diagonal, vec![&8, &4]);
        let diagonal: Vec<&i32> = grid.diagonal_rev(2).collect();
        assert_eq!(diagonal, vec![&9, &5, &1]);
        let diagonal: Vec<&i32> = grid.diagonal_rev(3).collect();
        assert_eq!(diagonal, vec![&6, &2]);
        let diagonal: Vec<&i32> = grid.diagonal_rev(4).collect();
        assert_eq!(diagonal, vec![&3]);
    }

    #[test]
    fn test_grid_diagonals() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::new(&grid);
        let diagonals: Vec<Vec<&i32>> = grid.diagonals().map(|diag| diag.collect()).collect();
        assert_eq!(diagonals, vec![
            vec![&1],
            vec![&2, &4],
            vec![&3, &5, &7],
            vec![&6, &8],
            vec![&9]
        ]);
    }

    #[test]
    fn test_grid_diagonals_rev() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::new(&grid);
        let diagonals: Vec<Vec<&i32>> = grid.diagonals_rev().map(|diag| diag.collect()).collect();
        assert_eq!(diagonals, vec![
            vec![&7],
            vec![&8, &4],
            vec![&9, &5, &1],
            vec![&6, &2],
            vec![&3]
        ]);
    }
}
