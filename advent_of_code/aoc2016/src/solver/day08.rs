//! Day 8: Two-Factor Authentication
//!
//! Simulate a 50x6 pixel screen with three operations:
//! - `rect AxB` turns on a rectangle at top-left
//! - `rotate row y=A by B` shifts row right with wrap
//! - `rotate column x=A by B` shifts column down with wrap
//!
//! Part 1: Count lit pixels. Part 2: Read the displayed code.

use mission6::Grid;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            if let Some(rest) = line.strip_prefix("rect ") {
                let (w, h) = rest.split_once('x').unwrap();
                Instruction::Rect(w.parse().unwrap(), h.parse().unwrap())
            } else if let Some(rest) = line.strip_prefix("rotate row y=") {
                let (y, amount) = rest.split_once(" by ").unwrap();
                Instruction::RotateRow(y.parse().unwrap(), amount.parse().unwrap())
            } else if let Some(rest) = line.strip_prefix("rotate column x=") {
                let (x, amount) = rest.split_once(" by ").unwrap();
                Instruction::RotateCol(x.parse().unwrap(), amount.parse().unwrap())
            } else {
                panic!("Unknown instruction: {line}");
            }
        })
        .collect()
}

fn simulate(instructions: &[Instruction]) -> Grid<bool> {
    let mut screen = Grid::new(WIDTH, HEIGHT, false);

    for inst in instructions {
        match *inst {
            Instruction::Rect(w, h) => {
                for y in 0..h {
                    for x in 0..w {
                        screen[(x, y)] = true;
                    }
                }
            }
            Instruction::RotateRow(y, amount) => {
                let old: Vec<bool> = (0..WIDTH).map(|x| screen[(x, y)]).collect();
                for x in 0..WIDTH {
                    screen[((x + amount) % WIDTH, y)] = old[x];
                }
            }
            Instruction::RotateCol(x, amount) => {
                let old: Vec<bool> = (0..HEIGHT).map(|y| screen[(x, y)]).collect();
                for y in 0..HEIGHT {
                    screen[(x, (y + amount) % HEIGHT)] = old[y];
                }
            }
        }
    }

    screen
}

fn render(screen: &Grid<bool>) -> String {
    (0..screen.height())
        .map(|y| {
            (0..screen.width())
                .map(|x| if screen[(x, y)] { '#' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn solve(input: &str) -> (String, String) {
    let instructions = parse_instructions(input);
    let screen = simulate(&instructions);

    let p1 = screen.iter().filter(|&&p| p).count();
    let display = render(&screen);

    (p1.to_string(), display)
}

pub fn solve_part1(input: &str) -> String {
    let instructions = parse_instructions(input);
    let screen = simulate(&instructions);
    screen.iter().filter(|&&p| p).count().to_string()
}

pub fn solve_part2(input: &str) -> String {
    let instructions = parse_instructions(input);
    let screen = simulate(&instructions);
    render(&screen)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_rect_and_rotate() {
        // Test rect creates correct rectangle
        let screen = simulate(&parse_instructions("rect 3x2"));
        assert_eq!(screen.iter().filter(|&&p| p).count(), 6);
        for y in 0..2 {
            for x in 0..3 {
                assert!(screen[(x, y)]);
            }
        }

        // Test rotate row wraps correctly
        let screen = simulate(&parse_instructions("rect 3x1\nrotate row y=0 by 49"));
        assert!(screen[(49, 0)]); // col 0 shifted to 49
        assert!(screen[(0, 0)]); // col 1 shifted to 50%50=0
        assert!(screen[(1, 0)]); // col 2 shifted to 51%50=1

        // Test rotate column wraps correctly
        let screen = simulate(&parse_instructions("rect 1x3\nrotate column x=0 by 5"));
        assert!(screen[(0, 5)]); // row 0 shifted to 5
        assert!(screen[(0, 0)]); // row 1 shifted to 6%6=0
        assert!(screen[(0, 1)]); // row 2 shifted to 7%6=1
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day08.txt");
        assert_eq!(solve_part1(input), "123");
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day08.txt");
        let (p1, _p2) = solve(input);
        assert_eq!(p1, "123");
    }
}
