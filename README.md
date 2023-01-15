# Advent of Code 2022

These are my solutions to the [Advent of Code 2022](https://adventofcode.com/2022/).

I was using these problems as a way to learn Rust, which I've not used before. All the implementations work and produce the expected answers, but the implementations are a bit of a mix in quality.

## Getting started

To run a solution:

```bash
cargo run <number> <part> <input>
```

For example, to solve part A of problem 18 using the small dataset defined in `input/18-small.txt`:

```bash
cargo run 18 a small
```

Some problems have less efficient solutions than others, and will run very slowly in development. To run in release mode:

```bash
cargo run --release 24 a large
```

## Tests

To run the tests:

```bash
cargo test
```

To run the tests for just a single module:

```bash
cargo test aoc_24
```
