use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day15_example.txt").unwrap();
    
    let part1 = aoc2024::solver::day15::solve_part1(&input).unwrap();
    let part2 = aoc2024::solver::day15::solve_part2(&input).unwrap();
    
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
