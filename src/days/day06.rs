use winnow::{
    ascii::{dec_uint, space0, space1},
    combinator::{iterator, preceded, terminated},
    error::{EmptyError, ErrMode},
    token::one_of,
};

pub fn solve1(input: &str) -> u64 {
    let rows: Vec<&str> = input.split("\n").collect();
    let operator_row_index = rows.len() - 2;

    let operators = parse_operator_line(rows[operator_row_index]);

    let mut row_iters = rows[0..operator_row_index]
        .into_iter()
        .map(|&input| {
            iterator::<_, _, ErrMode<EmptyError>, _>(
                input,
                preceded(space0, terminated(dec_uint::<_, u64, _>, space0)),
            )
        })
        .collect::<Vec<_>>();

    let mut result: u64 = 0;
    for operator in operators {
        let mut sum = 0;
        for mut row_iterator in &mut row_iters {
            let value = row_iterator.next().unwrap();

            match operator {
                '*' if sum > 0 => sum *= value,
                '*' if sum == 0 => sum = value,
                '+' => sum += value,
                _ => unreachable!(),
            }
        }

        result += sum;
    }

    result
}

pub fn solve2(input: &str) -> u64 {
    let rows: Vec<&str> = input.split("\n").collect();
    let operator_row_index = rows.len() - 2;

    let operators = parse_operator_line(rows[operator_row_index]);

    let row_bytes: Vec<&[u8]> = rows[0..operator_row_index]
        .into_iter()
        .map(|row| row.as_bytes())
        .collect();

    let mut sum: u64 = 0;
    let mut result: u64 = 0;
    let mut column_index: usize = operators.len() - 1;
    for column in (0..row_bytes[0].len()).rev() {
        let mut value: u64 = 0;
        for &row in &row_bytes {
            if row[column] == b' ' {
                continue;
            }

            value = value * 10 + (row[column] - b'0') as u64;
        }

        if value == 0 {
            result += sum;
            column_index -= 1;
            sum = 0;
        } else {
            match operators[column_index] {
                '*' if sum > 0 => sum *= value,
                '*' if sum == 0 => sum = value,
                '+' => sum += value,
                _ => unreachable!(),
            }
        }
    }

    result + sum
}

fn parse_operator_line(input: &str) -> Vec<char> {
    let mut operators: Vec<char> = Vec::with_capacity(1000);

    for op in
        &mut iterator::<_, _, ErrMode<EmptyError>, _>(input, terminated(one_of(['*', '+']), space1))
    {
        operators.push(op);
    }

    operators
}

#[allow(const_item_mutation)]
#[cfg(test)]
mod tests {
    use crate::day06::{solve1, solve2};

    const EXAMPLE_INPUT: &'static str = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(EXAMPLE_INPUT), 4277556)
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(EXAMPLE_INPUT), 3263827)
    }
}
