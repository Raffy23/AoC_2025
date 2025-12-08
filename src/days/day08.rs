use std::collections::BinaryHeap;
use std::fmt::Debug;
use winnow::ascii::{dec_uint, newline};
use winnow::combinator::{iterator, seq, terminated};
use winnow::error::{EmptyError, ErrMode};
use winnow::token::take;

type Point3D = (u32, u32, u32);

#[derive(Clone, Copy, Debug)]
struct PointDistance(u16, u16, f32);

pub fn solve1(input: &str) -> u64 {
    solve1_for_size::<1000>(input)
}

pub fn solve1_for_size<const N: usize>(input: &str) -> u64 {
    let points = parse_input::<1000>(input);
    let mut distances = connection_combinations(&points);

    let (mut circuits, mut circuit_members) = init_circuits(points.len());

    for _ in 0..N {
        let Some(PointDistance(i, j, _)) = distances.pop() else {
            panic!("input is too short for N")
        };

        let i = i as usize;
        let j = j as usize;

        if circuits[i] == circuits[j] {
            continue;
        }

        connect_boxes(i, j, &mut circuits, &mut circuit_members);
    }

    circuit_members.sort_unstable_by_key(|vec| vec.len());
    circuit_members
        .into_iter()
        .rev()
        .take(3)
        .map(|vec| vec.len() as u64)
        .product()
}

pub fn solve2(input: &str) -> u64 {
    let points = parse_input::<1000>(input);
    let mut distances = connection_combinations(&points);

    let (mut circuits, mut circuit_members) = init_circuits(points.len());

    while let Some(PointDistance(i, j, _)) = distances.pop() {
        let i = i as usize;
        let j = j as usize;

        if circuits[i] == circuits[j] {
            continue;
        }

        let new_circuit_len = connect_boxes(i, j, &mut circuits, &mut circuit_members);

        if new_circuit_len == points.len() {
            let p1 = points[i];
            let p2 = points[j];

            return p1.0 as u64 * p2.0 as u64;
        }
    }

    panic!("No solution was found!");
}

#[rustfmt::skip]
fn distance(p1: &Point3D, p2: &Point3D) -> f32 {
    (
        (p1.0 as i64 - p2.0 as i64).pow(2) as f32 + 
        (p1.1 as i64 - p2.1 as i64).pow(2) as f32 + 
        (p1.2 as i64 - p2.2 as i64).pow(2) as f32
    ).sqrt()
}

fn connection_combinations(points: &[Point3D]) -> BinaryHeap<PointDistance> {
    // Note: Capacity estimate is enough to fit the input into memory without reallocations
    let mut distances = BinaryHeap::with_capacity(500_000);

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            distances.push(PointDistance(
                i as u16,
                j as u16,
                distance(&points[i], &points[j]),
            ));
        }
    }

    distances
}

fn init_circuits(size: usize) -> (Vec<usize>, Vec<Vec<usize>>) {
    let mut circuits = vec![0; size];
    for i in 0..circuits.len() {
        circuits[i] = i;
    }

    // This might be a little bit wasteful with memory, but at least no reallocations are needed
    let mut circuit_members = vec![Vec::with_capacity(size); size];
    for i in 0..circuits.len() {
        circuit_members[i].push(i);
    }

    (circuits, circuit_members)
}

fn connect_boxes(
    i: usize,
    j: usize,
    circuits: &mut [usize],
    circuit_members: &mut [Vec<usize>],
) -> usize {
    let circuit_p1 = circuits[i].min(circuits[j]);
    let circuit_p2 = circuits[j].max(circuits[i]);

    let mid = circuit_p1 + 1;
    let (left, right) = circuit_members.split_at_mut(mid);
    let circuit_members_p1 = &mut left[left.len() - 1];
    let circuit_members_p2 = &mut right[circuit_p2 - mid];

    let new_circuit = circuits[circuit_p1];
    circuits[circuit_p2] = new_circuit;

    for member in circuit_members_p2.iter() {
        circuits[*member] = new_circuit;
        circuit_members_p1.push(*member);
    }

    circuit_members_p2.clear();
    circuit_members_p1.len()
}

impl Ord for PointDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.2.total_cmp(&self.2)
    }
}

impl PartialOrd for PointDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.2.partial_cmp(&self.2)
    }
}

impl PartialEq for PointDistance {
    fn eq(&self, other: &Self) -> bool {
        other.2 == self.2
    }
}

impl Eq for PointDistance {}

fn parse_input<const N: usize>(input: &str) -> Vec<Point3D> {
    iterator::<_, _, ErrMode<EmptyError>, _>(
        input,
        terminated(
            seq!(
                dec_uint,
                _: take(1u16),
                dec_uint,
                _: take(1u16),
                dec_uint
            ),
            newline,
        ),
    )
    .fold(Vec::with_capacity(N), |mut vec, point| {
        vec.push(point);
        vec
    })
}

#[allow(const_item_mutation)]
#[cfg(test)]
mod tests {
    use crate::day08::{solve1_for_size, solve2};

    const EXAMPLE_INPUT: &'static str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1_for_size::<10>(EXAMPLE_INPUT), 40)
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(EXAMPLE_INPUT), 25272)
    }
}
