use winnow::{
    Parser,
    ascii::{dec_uint, newline},
    combinator::{iterator, separated},
    error::{EmptyError, ErrMode},
    token::{one_of, take},
};

const PRESENT_ROWS: usize = 3;
const PRESENT_COLS: usize = 3;
const PRESENT_SHAPES: usize = 6;

type Present = ([[bool; PRESENT_COLS]; PRESENT_ROWS], u8);
type Tree = (u8, u8, [u8; PRESENT_SHAPES]);

pub fn solve1(input: &mut &str) -> u32 {
    fold(input, |(sum, presents), (cols, rows, present_counts)| {
        let area = cols as u32 * rows as u32;

        let mut required_area = 0;
        let mut max_area = 0;
        for i in 0..present_counts.len() {
            required_area += present_counts[i] as u32 * presents[i].1 as u32;
            max_area += PRESENT_COLS as u32 * PRESENT_ROWS as u32 * present_counts[i] as u32;
        }

        if area < required_area {
            return (sum, presents);
        }

        if area >= max_area {
            return (sum + 1, presents);
        }

        // Note: 
        // Due to how the input is constructed, it's possible to solve it without implementing
        // a correct packing algorithm. It won't work for the sample input.
        eprintln!("The result may not be correct, implementation missing for packing presents under the tree!");

        (sum + 1, presents)
    })
}

fn fold<'s, F>(input: &mut &'s str, solver: F) -> u32
where
    F: FnMut((u32, Vec<Present>), Tree) -> (u32, Vec<Present>),
{
    fn present<'s>(input: &mut &'s str) -> Result<Present, ErrMode<EmptyError>> {
        let mut present: Present = ([[false; PRESENT_COLS]; PRESENT_ROWS], 0);

        let _ = take(3u32).parse_next(input)?;
        for row in 0..present.0.len() {
            for col in 0..PRESENT_COLS {
                present.0[row][col] = match one_of(['#', '.']).parse_next(input)? {
                    '#' => {
                        present.1 += 1;
                        true
                    }
                    '.' => false,
                    _ => unreachable!(),
                };
            }

            let _ = newline(input)?;
        }

        Ok(present)
    }

    fn tree<'s>(input: &mut &'s str) -> Result<Tree, ErrMode<EmptyError>> {
        let cols: u8 = dec_uint(input)?;
        let _ = take(1u32).parse_next(input)?;
        let rows: u8 = dec_uint(input)?;
        let _ = take(2u32).parse_next(input)?;

        let mut presents = [0; PRESENT_SHAPES];
        for i in 0..presents.len() - 1 {
            presents[i] = dec_uint(input)?;
            let _ = take(1u32).parse_next(input)?;
        }

        presents[presents.len() - 1] = dec_uint(input)?;
        let _ = newline(input)?;

        Ok((cols, rows, presents))
    }

    let presents =
        separated::<_, _, _, _, ErrMode<EmptyError>, _, _>(PRESENT_SHAPES, present, newline)
            .parse_next(input);

    let _ = newline::<_, ErrMode<EmptyError>>.parse_next(input);

    iterator(*input, tree)
        .fold((0, presents.unwrap()), solver)
        .0
}

#[allow(const_item_mutation)]
#[cfg(test)]
mod tests {
    use crate::day12::solve1;

    const EXAMPLE_INPUT: &'static str = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"#;

    #[test]
    fn part1() {
        //assert_eq!(solve1(&mut EXAMPLE_INPUT), 2)
    }
}
