pub fn solve1(input: &str) -> u64 {
    let rows = parse_input(input);
    let c = find_start(rows[0]);

    let mut beams = vec![false; rows[0].len()];
    let mut beams_start = c;
    let mut beams_end = c;
    beams[c] = true;

    let mut splits = 0;
    for row in rows {
        for c in beams_start..beams_end + 1 {
            let beam_value = beams[c];

            if beam_value == false || row[c] == b'.' {
                continue;
            }

            if row[c] == b'^' {
                splits += 1;
                beams[c] = false;
                beams[c - 1] = true;
                beams[c + 1] = true;

                beams_start = (c - 1).min(beams_start);
                beams_end = (c + 1).max(beams_start);
            }
        }
    }

    splits
}

pub fn solve2(input: &str) -> u64 {
    let rows = parse_input(input);
    let c = find_start(rows[0]);

    let mut beams = vec![0usize; rows[0].len()];
    let mut beams_start = c;
    let mut beams_end = c;
    beams[c] = 1;

    for row in rows {
        for c in beams_start..beams_end + 1 {
            let beam_value = beams[c];

            if beam_value == 0 || row[c] == b'.' {
                continue;
            }

            if row[c] == b'^' {
                beams[c] = 0;
                beams[c - 1] += beam_value;
                beams[c + 1] += beam_value;

                beams_start = (c - 1).min(beams_start);
                beams_end = (c + 1).max(beams_start);
            }
        }
    }

    beams.into_iter().sum::<usize>() as u64
}

fn find_start(row: &[u8]) -> usize {
    row
        .iter()
        .enumerate()
        .find(|(_, symbol)| **symbol == b'S')
        .map(|(idx, _)| idx)
        .unwrap()
}

fn parse_input(input: &str) -> Vec<&[u8]> {
    input
        .trim_end()
        .split("\n")
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .map(|(_, row)| row.as_bytes())
        .collect::<Vec<_>>()
}

#[allow(const_item_mutation)]
#[cfg(test)]
mod tests {
    use crate::day07::{solve1, solve2};

    const EXAMPLE_INPUT: &'static str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(EXAMPLE_INPUT), 21)
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(EXAMPLE_INPUT), 40)
    }
}
