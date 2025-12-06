# Advent of Code 2025

## Running
Input files must be placed into the `input` folder with the pattern `dayXX.txt`.

```shell
cargo run -- --day <xx> --part <xx>
```

## Benchmarking
Input files are required for running the benchmarks.

```shell
cargo bench -- "DayXX [PartX]" [--exact]
```

### Analyzing performance
A flamegraph can be generated with [flamegraph](https://github.com/flamegraph-rs/flamegraph):
```shell
cargo flamegraph --bench aoc -- "DayXX PartY" --exact --bench
```