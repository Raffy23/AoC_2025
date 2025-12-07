use winnow::ascii::dec_uint;
use winnow::combinator::terminated;
use winnow::combinator::{iterator, seq};
use winnow::error::{ContextError, ErrMode};
use winnow::token::{literal, take};

const MAX_DIGITS: usize = 20; // should be enough for u64

pub fn solve1(input: &str) -> u64 {
    // Note: Keep buffer at a top level to save on memory allocation
    //       Buffer must be at least as big as the biggest number that is expected
    let mut buf: Vec<u8> = vec![0; MAX_DIGITS];

    fold(input, |mut sum, (start, end)| {
        let mut current = start;

        while current <= end {
            let digits = to_digits(current, &mut buf);

            if digits.len() % 2 == 1 {
                // If digits are not even, skip to the starting number which has one digit more
                current = 10u64.pow(digits.len() as u32);
                continue;
            }

            let middle = digits.len() / 2;
            let Some((left, right)) = digits.split_at_checked(middle) else {
                unreachable!()
            };

            let same = left == right;
            if same {
                sum += current;

                // increase left + 1 and right + 1
                // should skip until the next number that repeats
                let inc = 10u64.pow(right.len() as u32) + 1;
                current += inc as u64;
            } else {
                let left = to_number(left);
                let right = to_number(right);

                // try to skip to the next number that repeats, either by
                // adding the diff between legt and right or by going the next
                // position where left repeats itself
                if left > right {
                    current += left - right;
                } else {
                    current = (left + 1) * 10u64.pow(middle as u32) + left + 1;
                }
            }
        }

        sum
    })
}

pub fn solve2(input: &str) -> u64 {
    let mut buf: Vec<u8> = vec![0; MAX_DIGITS];

    fold(input, |mut sum, (start, end)| {
        let mut current = start;

        while current <= end {
            let digits = to_digits(current, &mut buf);

            for block_len in 1..=digits.len() / 2 {
                let block = &digits[..block_len];

                if digits.len() % block_len != 0 {
                    continue;
                }

                let mut matched = true;
                let remaining = block_len..digits.len();
                for idx in remaining.step_by(block_len) {
                    if &digits[idx..idx + block_len] != block {
                        matched = false;
                        break;
                    }
                }

                if matched {
                    sum += current;
                    break;
                }
            }

            current += 1;
        }

        sum
    })
}

/// Fills the provided buffer `buf` with the digits of the `number` parameter from the back to front
fn to_digits(mut number: u64, buf: &mut [u8]) -> &[u8] {
    for (idx, digit) in buf.iter_mut().enumerate().rev() {
        let n = number % 10;
        number = number / 10;
        *digit = n as u8;

        if number == 0 {
            return &buf[idx..];
        };
    }

    return buf;
}

fn to_number(buf: &[u8]) -> u64 {
    buf.into_iter()
        .fold(0, |num, &digit| num * 10 + digit as u64)
}

fn fold<F>(input: &str, solver: F) -> u64
where
    F: FnMut(u64, (u64, u64)) -> u64,
{
    iterator::<_, _, ErrMode<ContextError>, _>(
        input,
        terminated(
            seq!(
                dec_uint,
                _: literal('-'),
                dec_uint
            ),
            // every entry except the last is terminated by `,`, the last entry is terminated by `\n`.
            // We can simply consume any symbol at that location, as we don't care that much about the well-soundess of the input
            take(1u32),
        ),
    )
    .fold(0u64, solver)
}

#[allow(const_item_mutation)]
#[cfg(test)]
mod tests {
    use crate::day02::{solve1, solve2, to_digits};

    const EXAMPLE_INPUT: &'static str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124\n";

    #[test]
    fn test_digits() {
        let mut buffer: Vec<u8> = vec![0; 16];

        assert_eq!(to_digits(8080123, &mut buffer), vec![8, 0, 8, 0, 1, 2, 3]);
    }

    #[test]
    fn part1() {
        assert_eq!(solve1(EXAMPLE_INPUT), 1227775554)
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(EXAMPLE_INPUT), 4174379265)
    }
}
