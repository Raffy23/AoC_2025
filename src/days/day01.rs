use winnow::ascii::dec_uint;
use winnow::combinator::iterator;
use winnow::{ModalResult, Parser, ascii::newline, combinator::terminated, token::one_of};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

pub type Input<'s> = Result<Vec<(Direction, u32)>, anyhow::Error>;

pub fn solve1(input: &str) -> u32 {
    const DIAL_SIZE: i32 = 100;

    fold(input, |(count, number), (direction, steps)| {
        let mut steps = steps as i32;
        if direction == Direction::Left {
            steps = -steps;
        }

        let number = ((number + steps) % DIAL_SIZE + DIAL_SIZE) % DIAL_SIZE;

        if number == 0 {
            (count + 1, number)
        } else {
            (count, number)
        }
    })
}

pub fn solve2(input: &str) -> u32 {
    const DIAL_SIZE: i32 = 100;

    fold(input, |(count, number), (direction, steps)| {
        let steps = steps as i32;

        let revolutions = steps / DIAL_SIZE;
        let mut steps = steps % DIAL_SIZE;

        if direction == Direction::Left {
            steps = -steps;
        }

        let mut count = count;
        if revolutions > 0 {
            count += revolutions as u32;
        }

        let change = number + steps;
        let next_number = (change % DIAL_SIZE + DIAL_SIZE) % DIAL_SIZE;

        if number != 0 && (change < 0 || change > DIAL_SIZE) {
            count += 1;
        }

        if next_number == 0 {
            count += 1;
        }

        return (count, next_number);
    })
}

fn fold<F>(input: &str, solver: F) -> u32
where
    F: FnMut((u32, i32), (Direction, u32)) -> (u32, i32),
{
    const DIAL_START: i32 = 50;

    // Note: Using the `iterator` is slightly faster than building a Vec<_> and then iterating over it
    iterator(
        input,
        terminated((Direction::parse, dec_uint::<&str, u32, _>), newline),
    )
    .fold((0u32, DIAL_START), solver)
    .0
}

impl Direction {
    pub fn parse<'s>(input: &mut &'s str) -> ModalResult<Direction> {
        one_of(['L', 'R']).parse_next(input).map(|char| match char {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        })
    }
}

#[allow(const_item_mutation)]
#[cfg(test)]
mod tests {
    use crate::day01::{solve1, solve2};

    const EXAMPLE_INPUT: &'static str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(EXAMPLE_INPUT), 3)
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(EXAMPLE_INPUT), 6)
    }

    const EXAMPLE_INPUT_SMALL: &'static str = r#"R1000
"#;

    #[test]
    fn part2_small() {
        assert_eq!(solve2(EXAMPLE_INPUT_SMALL), 10)
    }
}
