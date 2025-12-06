use rangemap::RangeSet;

pub fn solve1(input: &str) -> u64 {
    let input: Vec<&str> = input.split("\n\n").collect();

    let range_set = input[0]
        .split("\n")
        .flat_map(|input| {
            input.split("\n").into_iter().map(|str| {
                let tmp: Vec<&str> = str.split("-").collect();
                let start: u64 = tmp[0].parse().unwrap();
                let end: u64 = tmp[1].parse().unwrap();

                start..end + 1
            })
        })
        .fold(RangeSet::new(), |mut set, range| { set.insert(range); set });

    input[1]
        .trim_end()
        .split("\n")
        .into_iter()
        .map(|str| str.parse().unwrap())
        .fold(0u64, |sum, ingredient: u64| {
            if range_set.contains(&ingredient) {
                sum + 1
            } else {
                sum
            }
        })
}

pub fn solve2(input: &str) -> u64 {
    let range_set = input
        .split("\n\n")
        .take(1)
        .flat_map(|input| {
            input.split("\n").into_iter().map(|str| {
                let tmp: Vec<&str> = str.split("-").collect();
                let start: u64 = tmp[0].parse().unwrap();
                let end: u64 = tmp[1].parse().unwrap();

                start..end + 1
            })
        })
        .fold(RangeSet::new(), |mut set, range| { set.insert(range); set });

    let mut count: u64 = 0;
    for range in range_set {
        count += range.count() as u64;
    }

    count
}

#[allow(const_item_mutation)]
#[cfg(test)]
mod tests {
    use crate::day05::{solve1, solve2};

    const EXAMPLE_INPUT: &'static str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(EXAMPLE_INPUT), 3)
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(EXAMPLE_INPUT), 14)
    }
}
