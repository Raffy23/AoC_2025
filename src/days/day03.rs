use std::ops::Range;

use winnow::Parser;
use winnow::ascii::newline;
use winnow::combinator::iterator;
use winnow::combinator::terminated;
use winnow::error::{ContextError, ErrMode};
use winnow::token::take_till;

pub fn solve1(input: &str) -> u64 {
    find_charge_for::<2>(input)
}

pub fn solve2(input: &str) -> u64 {
    find_charge_for::<12>(input)
}

/// finds the max charge for N amount of batteries
fn find_charge_for<const N: usize>(input: &str) -> u64 {
    fold(input, |sum, batteries| {
        let mut num: u64 = 0;
        let mut last_index = 0;
        
        for idx in 0..N {
            let (max_value, max_index) =
                find_max(&batteries, last_index..batteries.len() - (N - 1 - idx));

            last_index = max_index + 1;
            num = num * 10 + max_value as u64;
        }

        sum + num
    })
}

fn find_max(hay: &[u8], range: Range<usize>) -> (u8, usize) {
    let mut max_value = hay[range.start];
    let mut max_index = range.start;

    if max_value < 9 {
        for idx in range {
            if hay[idx] > max_value {
                max_value = hay[idx];
                max_index = idx;
            }

            if max_value == 9 {
                break;
            }
        }
    }

    (max_value, max_index)
}

fn fold<F>(input: &str, solver: F) -> u64
where
    F: FnMut(u64, Vec<u8>) -> u64,
{
    iterator::<_, _, ErrMode<ContextError>, _>(
        input,
        terminated(
            take_till(0.., |c| c == '\n').map(|line: &str| {
                let mut vec = Vec::with_capacity(line.len());

                for char in line.chars() {
                    vec.push(char as u8 - b'0');
                }

                vec
            }),
            newline,
        ),
    )
    .fold(0u64, solver)
}

#[allow(const_item_mutation)]
#[cfg(test)]
mod tests {
    use crate::day03::{solve1, solve2};

    const EXAMPLE_INPUT: &'static str = r#"987654321111111
811111111111119
234234234234278
818181911112111
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(EXAMPLE_INPUT), 357)
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(EXAMPLE_INPUT), 3121910778619)
    }
}
