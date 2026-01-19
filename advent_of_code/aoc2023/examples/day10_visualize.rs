use aoc2023::solver::day10;

fn main() {
    let example = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    // Show all visualizations
    println!("{}", day10::visualize_all(example));

    // Individual parts can also be accessed:
    // println!("{}", day10::visualize_loop(example));
    // println!("{}", day10::visualize_distances(example));
    // println!("{}", day10::visualize_inside_outside(example));
}
