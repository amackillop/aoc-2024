/// Scan every row column and diagonal for XMAS or SAMX
pub fn part_1(input: &str) -> usize {
    let rows: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let cols = transpose(&rows);
    let diagonals_1 = diagonals(&rows);
    let diagonals_2 = diagonals_rev(&rows);
    rows.iter()
        .chain(cols.iter())
        .chain(diagonals_1.iter())
        .chain(diagonals_2.iter())
        .map(|row| find_all_xmas(row))
        .sum()
}

/// Slide a 2d window over the input and count the number of MAS crosses
pub fn part_2(input: &str) -> usize {
    let rows: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    windows_2d(&rows, 3)
        .into_iter()
        .filter(|window| window_is_xmas(window))
        .count()
}

/// Transpose the input grid of characters
fn transpose<T: Copy>(input: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut result = vec![Vec::with_capacity(input.len()); input[0].len()];
    for row in input {
        for (i, &cell) in row.iter().enumerate() {
            result[i].push(cell);
        }
    }
    result
}

/// Get the diagonals of the input grid from NE to SW
fn diagonals(input: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut diagonals = vec![vec![]; input.len() + input[0].len() - 1];
    for (r, row) in input.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            diagonals[r + c].push(*col);
        }
    }
    diagonals
}

/// Get the diagonals of the input grid from NW to SE
fn diagonals_rev(input: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut diagonals = vec![vec![]; input.len() + input[0].len() - 1];
    for (r, row) in input.iter().rev().enumerate() {
        for (c, col) in row.iter().enumerate() {
            diagonals[r + c].push(*col);
        }
    }
    diagonals
}

/// Find all XMAS or SAMX in the input
fn find_all_xmas(chars: &[char]) -> usize {
    chars
        .windows(4)
        .filter(|w| w == &['X', 'M', 'A', 'S'] || w == &['S', 'A', 'M', 'X'])
        .count()
}

/// Check if the 2d window contains a MAS cross
fn window_is_xmas(chars: &[&[char]]) -> bool {
    // A should be in the middle
    if chars[1][1] != 'A' {
        return false;
    }

    // M's at top
    chars[0][0] == 'M' && chars[0][2] == 'M' && chars[2][0] == 'S' && chars[2][2] == 'S' ||
        // M's at bottom
        chars[0][0] == 'S' && chars[0][2] == 'S' && chars[2][0] == 'M' && chars[2][2] == 'M' ||
        // M's at left
        chars[0][0] == 'M' && chars[0][2] == 'S' && chars[2][0] == 'M' && chars[2][2] == 'S' ||
        // M's at right
        chars[0][0] == 'S' && chars[0][2] == 'M' && chars[2][0] == 'S' && chars[2][2] == 'M'
}

/// Slide a 2d window over the input
fn windows_2d(input: &[Vec<char>], size: usize) -> Vec<Vec<&[char]>> {
    let mut windows = Vec::new();
    for r in 0..=input.len() - size {
        for c in 0..=input[0].len() - size {
            windows.push(
                input[r..r + size]
                    .iter()
                    .map(|row| &row[c..c + size])
                    .collect(),
            );
        }
    }
    windows
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        let input = read_input(4);
        assert_eq!(part_1(&input), 2521);
    }

    #[test]
    fn test_part_2() {
        let input = read_input(4);
        assert_eq!(part_2(&input), 1912);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = read_input(4);
        b.iter(|| part_1(&input));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = read_input(4);
        b.iter(|| part_2(&input));
    }

    #[test]
    fn test_transpose() {
        let input = vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f'], vec![
            'g', 'h', 'i',
        ]];
        let expected = vec![vec!['a', 'd', 'g'], vec!['b', 'e', 'h'], vec![
            'c', 'f', 'i',
        ]];
        assert_eq!(transpose(&input), expected);
    }

    #[test]
    fn test_diagonals_symmetric() {
        let input = vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f'], vec![
            'g', 'h', 'i',
        ]];
        let expected = vec![
            vec!['a'],
            vec!['b', 'd'],
            vec!['c', 'e', 'g'],
            vec!['f', 'h'],
            vec!['i'],
        ];
        assert_eq!(diagonals(&input), expected);
    }

    #[test]
    fn test_diagonals_asymmetric_1() {
        let input = vec![vec!['a', 'b', 'c', 'd', 'e', 'f'], vec![
            'g', 'h', 'i', 'j', 'k', 'l',
        ]];
        let expected = vec![
            vec!['a'],
            vec!['b', 'g'],
            vec!['c', 'h'],
            vec!['d', 'i'],
            vec!['e', 'j'],
            vec!['f', 'k'],
            vec!['l'],
        ];
        assert_eq!(diagonals(&input), expected);
    }

    #[test]
    fn test_diagonals_asymmetric_2() {
        let input = vec![
            vec!['a', 'b'],
            vec!['c', 'd'],
            vec!['e', 'f'],
            vec!['g', 'h'],
            vec!['i', 'j'],
        ];
        let expected = vec![
            vec!['a'],
            vec!['b', 'c'],
            vec!['d', 'e'],
            vec!['f', 'g'],
            vec!['h', 'i'],
            vec!['j'],
        ];
        assert_eq!(diagonals(&input), expected);
    }

    #[test]
    fn test_windows_2d() {
        let input = vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f'], vec![
            'g', 'h', 'i',
        ]];
        let expected = vec![
            vec![vec!['a', 'b'], vec!['d', 'e']],
            vec![vec!['b', 'c'], vec!['e', 'f']],
            vec![vec!['d', 'e'], vec!['g', 'h']],
            vec![vec!['e', 'f'], vec!['h', 'i']],
        ];
        assert_eq!(windows_2d(&input, 2), expected);
    }
}
