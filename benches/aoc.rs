use std::hint::black_box;

use aoc_2025::{day01, day02, day03, day04, day05, day06, day07, utils::read_input};
use criterion::{Criterion, criterion_group, criterion_main};

criterion_group!(
    benches,
    day01_benchmark,
    day02_benchmark,
    day03_benchmark,
    day04_benchmark,
    day05_benchmark,
    day06_benchmark,
    day07_benchmark,
);
criterion_main!(benches);

fn day01_benchmark(c: &mut Criterion) {
    let input = read_input(1, None).expect("Unable to read input file!");

    c.bench_function("Day01 Part1", |b| {
        b.iter(|| day01::solve1(black_box(&mut input.as_str())))
    });

    c.bench_function("Day01 Part2", |b| {
        b.iter(|| day01::solve2(black_box(&mut input.as_str())))
    });
}

fn day02_benchmark(c: &mut Criterion) {
    let input = read_input(2, None).expect("Unable to read input file!");

    c.bench_function("Day02 Part1", |b| {
        b.iter(|| day02::solve1(black_box(&mut input.as_str())))
    });

    c.bench_function("Day02 Part2", |b| {
        b.iter(|| day02::solve2(black_box(&mut input.as_str())))
    });
}

fn day03_benchmark(c: &mut Criterion) {
    let input = read_input(3, None).expect("Unable to read input file!");

    c.bench_function("Day03 Part1", |b| {
        b.iter(|| day03::solve1(black_box(&mut input.as_str())))
    });

    c.bench_function("Day03 Part2", |b| {
        b.iter(|| day03::solve2(black_box(&mut input.as_str())))
    });
}

fn day04_benchmark(c: &mut Criterion) {
    let input = read_input(4, None).expect("Unable to read input file!");

    c.bench_function("Day04 Part1", |b| {
        b.iter(|| day04::solve1(black_box(&mut input.as_str())))
    });

    c.bench_function("Day04 Part2", |b| {
        b.iter(|| day04::solve2(black_box(&mut input.as_str())))
    });
}

fn day05_benchmark(c: &mut Criterion) {
    let input = read_input(5, None).expect("Unable to read input file!");

    c.bench_function("Day05 Part1", |b| {
        b.iter(|| day05::solve1(black_box(&mut input.as_str())))
    });

    c.bench_function("Day05 Part2", |b| {
        b.iter(|| day05::solve2(black_box(&mut input.as_str())))
    });
}

fn day06_benchmark(c: &mut Criterion) {
    let input = read_input(6, None).expect("Unable to read input file!");

    c.bench_function("Day06 Part1", |b| {
        b.iter(|| day06::solve1(black_box(&mut input.as_str())))
    });

    c.bench_function("Day06 Part2", |b| {
        b.iter(|| day06::solve2(black_box(&mut input.as_str())))
    });
}

fn day07_benchmark(c: &mut Criterion) {
    let input = read_input(7, None).expect("Unable to read input file!");

    c.bench_function("Day07 Part1", |b| {
        b.iter(|| day07::solve1(black_box(&mut input.as_str())))
    });

    c.bench_function("Day07 Part2", |b| {
        b.iter(|| day07::solve2(black_box(&mut input.as_str())))
    });
}
