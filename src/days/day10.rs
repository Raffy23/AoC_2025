use std::collections::{BTreeSet, VecDeque};
use winnow::ascii::dec_uint;
use winnow::combinator::{delimited, iterator, separated, seq};
use winnow::error::{EmptyError, ErrMode};
use winnow::token::{take, take_till};
use winnow::{Parser, ascii::newline, combinator::terminated};
use z3::Optimize;
use z3::ast::Int;

pub fn solve1(input: &str) -> usize {
    fold(input, |sum, (target_pattern, buttons, _)| {
        let mut seen = BTreeSet::<u16>::new();

        let buttons = buttons
            .into_iter()
            .map(|value| {
                let mut pattern: u16 = 0;

                for i in value {
                    pattern |= 1 << i;
                }

                pattern
            })
            .collect::<Vec<_>>();

        let mut queue = VecDeque::with_capacity(400);
        queue.push_back((0, 0));

        while let Some((pattern, presses)) = queue.pop_front() {
            for &button in &buttons {
                let new_pattern = pattern ^ button;
                if new_pattern == 0 {
                    continue;
                }

                if pattern == target_pattern {
                    return sum + presses;
                }

                if seen.contains(&new_pattern) {
                    continue;
                } else {
                    seen.insert(new_pattern);
                }

                queue.push_back((new_pattern, presses + 1));
            }
        }

        panic!("No solution was found for input!");
    })
}

pub fn solve2(input: &str) -> usize {
    fold(input, |sum, (_, buttons, joltage)| {
        let mut button_symbols = Vec::with_capacity(buttons.len());
        for (i, _) in buttons.iter().enumerate() {
            button_symbols.push(Int::new_const(format!("button_{i}")));
        }

        let mut light_equations = vec![Vec::new(); joltage.len()];
        for (btn_symbol, lights) in buttons.iter().enumerate() {
            for &light in lights {
                light_equations[light as usize].push(&button_symbols[btn_symbol]);
            }
        }

        let optimizer = Optimize::new();

        for btn in &button_symbols {
            optimizer.assert(&btn.ge(0));
        }

        for (i, equation) in light_equations.iter().enumerate() {
            optimizer.assert(&Int::add(equation).eq(joltage[i]));
        }

        optimizer.minimize(&Int::add(&button_symbols));

        sum + match optimizer.check(&[]) {
            z3::SatResult::Sat => {
                let model = optimizer.get_model().unwrap();

                let s = button_symbols
                    .into_iter()
                    .map(|btn| {
                        model
                            .eval(&btn, true)
                            .map(|value| value.as_u64())
                            .unwrap()
                            .unwrap()
                    })
                    .sum::<u64>();
                
                s as usize
            }
            _ => {
                panic!("No solution was found for input!");
            }
        }
    })
}

fn fold<F>(input: &str, solver: F) -> usize
where
    F: FnMut(usize, (u16, Vec<Vec<u8>>, Vec<u16>)) -> usize,
{
    #[inline]
    fn parse_pattern<'s>(input: &mut &'s str) -> Result<u16, ErrMode<EmptyError>> {
        let input = take_till(1.., ']').parse_next(input)?;

        let mut value: u16 = 0;
        for char in input.chars().rev() {
            value = value << 1;
            value |= match char {
                '.' => 0,
                '#' => 1u16,
                _ => unreachable!(),
            };
        }

        Ok(value)
    }

    let mut lines = iterator::<_, _, ErrMode<EmptyError>, _>(
        input,
        terminated(
            seq!(
                delimited('[', parse_pattern, ']'),
                _: take(1u32),
                separated::<_, Vec<u8>, Vec<Vec<u8>>, _, _, _, _>(
                    1..,
                    delimited('(', separated::<_, u8, Vec<u8>, _, _, _, _>(1.., dec_uint::<_, u8, _>, ','), ')',),
                    ' '
                ),
                _: take(1u32),
                delimited('{', separated(1.., dec_uint::<_, u16, _>, ','), '}'),
            ),
            newline,
        ),
    );

    let result = lines.fold(0, solver);

    match lines.finish() {
        Ok((rest, _)) if rest.len() > 0 => panic!("Unable to fully parse input!"),
        Err(_) => panic!("Unable to parse input"),
        _ => {}
    };

    result
}

#[allow(const_item_mutation)]
#[cfg(test)]
mod tests {
    use z3::{Optimize, ast::Int};

    use crate::day10::{solve1, solve2};

    const EXAMPLE_INPUT: &'static str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(EXAMPLE_INPUT), 7)
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(EXAMPLE_INPUT), 33)
    }

    #[test]
    fn test_linear_system_with_z3() {
        let light_0 = Int::from_u64(3);
        let light_1 = Int::from_u64(5);
        let light_2 = Int::from_u64(4);
        let light_3 = Int::from_u64(7);

        let b_0 = Int::new_const("b_0");
        let b_1 = Int::new_const("b_1");
        let b_2 = Int::new_const("b_2");
        let b_3 = Int::new_const("b_3");
        let b_4 = Int::new_const("b_4");
        let b_5 = Int::new_const("b_5");

        let sum = Int::new_const("sum");

        let optimizer = Optimize::new();
        optimizer.assert(&b_0.ge(0));
        optimizer.assert(&b_1.ge(0));
        optimizer.assert(&b_2.ge(0));
        optimizer.assert(&b_3.ge(0));
        optimizer.assert(&b_4.ge(0));
        optimizer.assert(&b_5.ge(0));
        optimizer.assert(&sum.gt(0));

        optimizer.assert(&(&b_4 + &b_5).eq(light_0));
        optimizer.assert(&(&b_1 + &b_5).eq(light_1));
        optimizer.assert(&(&b_2 + &b_3 + &b_4).eq(light_2));
        optimizer.assert(&(&b_1 + &b_3 + &b_0).eq(light_3));

        optimizer.assert(&Int::add(&[&b_1, &b_2, &b_3, &b_4, &b_5, &b_0]).eq(&sum));
        optimizer.minimize(&sum);

        let check = optimizer.check(&[]);
        println!("{:?}", check);

        let result = optimizer.get_model().unwrap();
        println!("{:?}", result.eval(&sum, true));
        println!("b_0 = {:?}", result.eval(&b_0, true));
        println!("b_1 = {:?}", result.eval(&b_1, true));
        println!("b_2 = {:?}", result.eval(&b_2, true));
        println!("b_3 = {:?}", result.eval(&b_3, true));
        println!("b_4 = {:?}", result.eval(&b_4, true));
        println!("b_5 = {:?}", result.eval(&b_5, true));
    }
}
