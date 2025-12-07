pub fn solve1(input: &str) -> u64 {
    let rows = input
        .trim_end()
        .split("\n")
        .map(|row| row.as_bytes())
        .collect::<Vec<_>>();

    let c = rows[0]
        .iter()
        .enumerate()
        .find(|(_, symbol)| **symbol == b'S')
        .map(|(idx, _)| idx)
        .unwrap();

    let mut beams = vec![0u8; rows[0].len()];
    let mut beams_start = c;
    let mut beams_end = c;
    beams[c] = 1;

    let mut splits = 0;
    for row in rows {
        for c in beams_start..beams_end + 1 {
            let beam_value = beams[c];

            if beam_value == 0 || row[c] == b'.' {
                continue;
            }

            if row[c] == b'^' {
                splits += 1;
                beams[c] = 0;
                beams[c - 1] = 1;
                beams[c + 1] = 1;

                beams_start = (c - 1).min(beams_start);
                beams_end = (c + 1).max(beams_start);
            }
        }
    }

    splits
}

pub fn solve2(input: &str) -> u64 {
    let rows = input
        .trim_end()
        .split("\n")
        .map(|row| row.as_bytes())
        .collect::<Vec<_>>();

    let c = rows[0]
        .iter()
        .enumerate()
        .find(|(_, symbol)| **symbol == b'S')
        .map(|(idx, _)| idx)
        .unwrap();

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
