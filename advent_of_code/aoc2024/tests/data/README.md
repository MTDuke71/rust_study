# Test Data Directory

This directory contains test examples and sample data for Advent of Code 2024 solutions.

## Structure

- `day01_simple.txt` - Simple example from Day 1 problem statement
- `day##_*.txt` - Additional test cases for each day as needed

## Usage

Test data files can be used with the CLI application:

```bash
cargo run -p aoc2024 -- 1 tests/data/day01_simple.txt --debug
```

Or referenced in integration tests for validation against known expected outputs.

## vs inputs/ Directory

- `inputs/` - Real AoC puzzle inputs (day01.txt, day02.txt, etc.)
- `tests/data/` - Example data, edge cases, and test scenarios