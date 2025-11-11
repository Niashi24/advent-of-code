# Advent of Code

[Advent of Code](https://adventofcode.com/) is a daily programming challenge hosted online in December each year. Competitors read the problem description and download their input files from the website and run the problem with their input file on their local machine. All problems also have a second part which is viewable after solving the first part. 

This repository is my collection of solutions for all days of 2024 (which were done as the contest was proceeding) and 2021 (done early 2024 as an exercise).

## Usage

Build using `cargo build --release` to create a binary at `target/{platform}/release/advent-of-code.exe`, then run with `advent-of-code.exe [OPTIONS]`, or run with `cargo run --release -- [OPTIONS]`.

### Options

```
  -d, --day <DAY>    The day to run
  -y, --year <YEAR>  The year to run
  -e, --example      Run example only
  -a, --all          Run all days
  -m, --main         Run main input
  -f, --file <FILE>  Run with file as input
  -t, --text <TEXT>  Run with text as input
  -1, --one          Run part 1
  -2, --two          Run part 2
  -h, --help         Print help
```

Ex. Run `advent-of-code.exe -d 10 -y 2024` to run the solution for Day 10 of Advent of Code 2024 on the input set for that day in day/meta.json.

Notably, you can run `advent-of-code.exe -y 2024 -a` to run the solutions for every day.

The answer for that day (or set of days) is outputted along with completion times.

### Manifest

The program uses a manifest to select which input file(s) to use for a given day, found in `day/meta.json`. 
