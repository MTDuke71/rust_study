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

---

## 🔗 **Related Resources**

**Testing Patterns**:
- [[../../../../zettelkasten/AoC Patterns MOC]] - Testing infrastructure and validation strategies
- [[../../../../zettelkasten/test-pyramid]] - Testing methodology and best practices
- [[../../../../missions/Mission10/tests/unit_tests.rs]] - Example of comprehensive test coverage

**Problem Statements**:
- [[../Problem_Statements/day01]] - Historian Hysteria problem reference
- [[../Problem_Statements/day02]] - Red-Nosed Reports problem reference
- [[../Problem_Statements/day03]] - Mull It Over problem reference

**Integration**:
- [[../../../../zettelkasten/AoC 2015 MOC]] - Cross-year testing comparison and patterns