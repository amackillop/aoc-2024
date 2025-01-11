use std::iter;

#[derive(Debug, PartialEq, Eq)]
struct File {
    id: usize,
    index: u32,
    size: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct Free {
    index: u32,
    size: u32,
}

/// Iterate over the blocks from left to right and replace the empty blocks with
/// the right most file block. We compute the number of file blocks up front
/// and end the iteration after this many steps since the rest must be empty.
pub fn part_1(input: &str) -> usize {
    let disk_map = decompress_disk_map(input);
    let mut file_blocks_rev = decompress_disk_map_rev(input).flatten();
    disk_map
        .take(file_block_count(input) as usize)
        .map(move |maybe_file_block| {
            if let Some(file_id) = maybe_file_block {
                file_id
            } else {
                file_blocks_rev
                    .next()
                    .expect("iteration should stop before consuming all of the file blocks")
            }
        })
        .enumerate()
        .map(|(index, block_id)| index * block_id)
        .sum()
}

/// From left to right, iterate over the files and update their indices if they
/// can moved to some free space to the left of the file.
pub fn part_2(input: &str) -> usize {
    let mut free_blocks: Vec<_> = read_free_blocks(input).collect();
    let files_rev = read_files_rev(input);

    // Iterate over the files from right to left
    files_rev
        .map(|file| {
            let mut new_index = file.index;
            // Iterate over the eligible free blocks
            for free in free_blocks
                .iter_mut()
                .filter(|empty| empty.size > 0)
                .take_while(|empty| empty.index < file.index)
            {
                if file.size <= free.size {
                    // This file can be moved
                    new_index = free.index;
                    // Shrink the free space where the file will be moved
                    *free = Free {
                        index: free.index + file.size,
                        size: free.size - file.size,
                    };
                    break;
                }
            }
            // "Move" the file by updating its index
            File {
                index: new_index,
                ..file
            }
        })
        .map(|file| {
            // Compute the checksum for the file
            (file.index..file.index + file.size)
                .map(move |index| index as usize * file.id)
                .sum::<usize>()
        })
        .sum()
}

/// Iterate over the blocks from the compact representation. None represents free blocks.
fn decompress_disk_map(compact: &str) -> impl Iterator<Item = Option<usize>> {
    compact
        .chars()
        .chain(iter::once('0'))
        .array_chunks()
        .enumerate()
        .flat_map(|(id, [block_count, free_count])| {
            let file_blocks = iter::repeat_n(Some(id), block_count.to_digit(10).unwrap() as usize);
            let free_blocks = iter::repeat_n(None, free_count.to_digit(10).unwrap() as usize);
            file_blocks.chain(free_blocks)
        })
}

/// Same as above but from end to start.
fn decompress_disk_map_rev(compact: &str) -> impl Iterator<Item = Option<usize>> {
    let num_files = compact.len() / 2;
    compact
        .chars()
        .chain(iter::once('0'))
        .rev()
        .array_chunks()
        .enumerate()
        .flat_map(move |(i, [free_count, block_count])| {
            let free_blocks = iter::repeat_n(None, free_count.to_digit(10).unwrap() as usize);
            let file_blocks = iter::repeat_n(
                Some(num_files - i),
                block_count.to_digit(10).unwrap() as usize,
            );
            free_blocks.chain(file_blocks)
        })
}

/// Iterate over the free blocks from the compact representation.
fn read_free_blocks(compact: &str) -> impl Iterator<Item = Free> {
    compact
        .chars()
        .array_chunks()
        .scan(0u32, move |index, [occupied_blocks, free_blocks]| {
            let occupied_blocks = occupied_blocks.to_digit(10).unwrap();
            let free_blocks = free_blocks.to_digit(10).unwrap();
            let free = Free {
                index: *index + occupied_blocks,
                size: free_blocks,
            };
            *index += occupied_blocks + free_blocks;
            Some(free)
        })
}

/// Iterate over the file blocks from end to start from the compact representation.
fn read_files_rev(compact: &str) -> impl Iterator<Item = File> {
    let file_count = compact.len() / 2;
    compact
        .chars()
        .chain(iter::once('0'))
        .rev()
        .array_chunks()
        .enumerate()
        .scan(
            block_count(compact),
            move |index, (i, [free_count, block_count])| {
                let block_count = block_count.to_digit(10).unwrap();
                let free_count = free_count.to_digit(10).unwrap();
                let file = File {
                    index: *index - block_count - free_count,
                    id: file_count - i,
                    size: block_count,
                };
                *index -= block_count + free_count;
                Some(file)
            },
        )
}

fn block_count(compact: &str) -> u32 {
    compact.chars().map(|c| c.to_digit(10).unwrap()).sum()
}

fn file_block_count(compact: &str) -> u32 {
    compact
        .chars()
        .step_by(2)
        .map(|c| c.to_digit(10).unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        let input = read_input(9);
        // assert_eq!(part_1(&input), 1928);
        assert_eq!(part_1(&input), 6344673854800);
    }

    #[test]
    fn test_part_2() {
        let high = 8515929533392;
        // let high = 15778929031023;
        let input = read_input(9);
        // assert_eq!(part_2(&input), 2858);
        assert!(part_2(&input) < high);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = read_input(9);

        b.iter(|| part_1(&input));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = read_input(9);
        b.iter(|| part_2(&input));
    }

    #[test]
    fn test_decompress_disk_map() {
        let input = "12345";
        let expected = vec![
            Some(0),
            None,
            None,
            Some(1),
            Some(1),
            Some(1),
            None,
            None,
            None,
            None,
            Some(2),
            Some(2),
            Some(2),
            Some(2),
            Some(2),
        ];
        assert_eq!(
            decompress_disk_map(input).collect::<Vec<Option<_>>>(),
            expected
        );
    }

    #[test]
    fn test_decompress_disk_map_rev() {
        let input = "12345";
        let expected = vec![
            Some(2),
            Some(2),
            Some(2),
            Some(2),
            Some(2),
            None,
            None,
            None,
            None,
            Some(1),
            Some(1),
            Some(1),
            None,
            None,
            Some(0),
        ];
        assert_eq!(
            decompress_disk_map_rev(input).collect::<Vec<Option<_>>>(),
            expected
        );
    }

    #[test]
    fn test_read_free_blocks() {
        let input = "12345";
        let expected = vec![Free { index: 1, size: 2 }, Free { index: 6, size: 4 }];
        assert_eq!(read_free_blocks(input).collect::<Vec<_>>(), expected);
    }

    #[test]
    fn test_read_files_rev() {
        let input = "12345";
        let expected = vec![
            File {
                id: 2,
                index: 10,
                size: 5,
            },
            File {
                id: 1,
                index: 3,
                size: 3,
            },
            File {
                id: 0,
                index: 0,
                size: 1,
            },
        ];
        assert_eq!(read_files_rev(input).collect::<Vec<_>>(), expected);
    }

    #[test]
    fn test_block_count() {
        let input = "12345";
        let expected = 15;
        assert_eq!(block_count(input), expected);
    }

    #[test]
    fn test_file_block_count() {
        let input = "12345";
        let expected = 9;
        assert_eq!(file_block_count(input), expected);
    }
}
