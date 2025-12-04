use std::collections::VecDeque;

pub fn solve1(input: &str) -> u64 {
    let input: Vec<Vec<u8>> = input.split("\n").map(|s| s.as_bytes().to_vec()).collect();

    let mut sum = 0;
    for (row, columns) in input.iter().enumerate() {
        for col in 0..columns.len() {
            if columns[col] == b'.' {
                continue;
            }

            if get_neighbors(&input, row as isize, col as isize) < 4 {
                sum += 1;
            }
        }
    }

    sum
}

pub fn solve2(input: &str) -> u64 {
    let mut input: Vec<Vec<u8>> = input.split("\n").map(|s| s.as_bytes().to_vec()).collect();

    let mut removable: VecDeque<(usize, usize)> = VecDeque::with_capacity(2048);
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if input[row][col] == b'.' {
                continue;
            }

            let n = get_neighbors(&input, row as isize, col as isize);
            input[row][col] = n as u8;

            if n < 4 {
                removable.push_back((row, col));
            }
        }
    }

    let mut sum = 0;
    while let Some((r, c)) = removable.pop_back() {
        let value = input[r][c];
        if value == b'.' {
            continue;
        }

        if value < 4 {
            input[r][c] = b'.';
            sum += 1;

            for (r, c) in NEIGHBOR_AREA
                .into_iter()
                .map(|(row, col)| (r as isize + row, c as isize + col))
            {
                let valid = r >= 0
                    && c >= 0
                    && (r as usize) < input.len()
                    && (c as usize) < input[r as usize].len();

                if !valid {
                    continue;
                }

                if input[r as usize][c as usize] != b'.' {
                    input[r as usize][c as usize] -= 1;

                    if input[r as usize][c as usize] < 4 {
                        removable.push_back((r as usize, c as usize));
                    }
                }
            }
        }
    }

    sum
}

#[rustfmt::skip]
const NEIGHBOR_AREA: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

fn get_neighbors(input: &Vec<Vec<u8>>, row: isize, col: isize) -> u64 {
    NEIGHBOR_AREA
        .into_iter()
        .map(|(r, c)| (r + row, c + col))
        .filter(|&(r, c)| {
            r >= 0 && c >= 0 && (r as usize) < input.len() && (c as usize) < input[r as usize].len()
        })
        .fold(0, |sum, (r, c)| {
            if input[r as usize][c as usize] != b'.' {
                sum + 1
            } else {
                sum
            }
        })
}

#[allow(const_item_mutation)]
#[cfg(test)]
mod tests {
    use crate::day04::{solve1, solve2};

    const EXAMPLE_INPUT: &'static str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(EXAMPLE_INPUT), 13)
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(EXAMPLE_INPUT), 43)
    }
}
