use aoc2023::solver::day10;

fn main() {
    let input = include_str!("../inputs/day10.txt");
    let part1 = day10::solve_part1(input).unwrap();
    let part2 = day10::solve_part2(input).unwrap();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    
    assert_eq!(part1, "6733", "Part 1 answer changed!");
    assert_eq!(part2, "435", "Part 2 answer changed!");
    println!("\n✅ Both answers are correct!");
}
