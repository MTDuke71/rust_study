use aoc2023::solver::day24;
fn main() {
    let input = std::fs::read_to_string(\"inputs/day24.txt\").unwrap();
    match day24::solve_part1(&input) {
        Ok(result) => println!(\"Part 1: {}\", result),
        Err(e) => eprintln!(\"Error: {}\", e),
    }
}
