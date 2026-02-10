use mission6::Direction;
use std::collections::HashSet;

/// Parse move commands from input
/// Format: "R 4" -> (Direction::East, 4)
fn parse_moves(input: &str) -> Vec<(Direction, i32)> {
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.trim().split_whitespace();
            let dir_char = parts.next()?;
            let count: i32 = parts.next()?.parse().ok()?;
            
            let direction = match dir_char {
                "U" => Direction::North,
                "D" => Direction::South,
                "L" => Direction::West,
                "R" => Direction::East,
                _ => return None,
            };
            
            Some((direction, count))
        })
        .collect()
}

/// Signed 2D coordinate for rope positions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }
    
    fn origin() -> Self {
        Pos::new(0, 0)
    }
    
    /// Move one step in the given direction
    fn step(&mut self, dir: Direction) {
        match dir {
            Direction::North => self.y -= 1,      // Up
            Direction::South => self.y += 1,      // Down
            Direction::East => self.x += 1,       // Right
            Direction::West => self.x -= 1,       // Left
            _ => {} // Diagonals not used in head movement
        }
    }
    
    /// Check if two positions are touching (adjacent, diagonal, or overlapping)
    fn is_touching(&self, other: &Pos) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
    
    /// Update tail position to follow head
    fn follow(&mut self, head: &Pos) {
        // Already touching? No movement needed
        if self.is_touching(head) {
            return;
        }
        
        let dx = head.x - self.x;
        let dy = head.y - self.y;
        
        // Move tail one step toward head
        // Use signum to get direction: -1, 0, or 1
        self.x += dx.signum();
        self.y += dy.signum();
    }
}

pub fn solve_part1(input: &str) -> usize {
    let moves = parse_moves(input);
    
    let mut head = Pos::origin();
    let mut tail = Pos::origin();
    let mut visited = HashSet::new();
    
    // Track starting position
    visited.insert(tail);
    
    // Simulate rope movements
    for (direction, count) in moves {
        for _ in 0..count {
            // Move head one step
            head.step(direction);
            
            // Update tail to follow
            tail.follow(&head);
            
            // Track tail position
            visited.insert(tail);
        }
    }
    
    visited.len()
}

pub fn solve_part2(input: &str) -> usize {
    let moves = parse_moves(input);
    
    // 10 knots: head (index 0) + knots 1-9
    let mut knots = vec![Pos::origin(); 10];
    let mut visited = HashSet::new();
    
    // Track starting position of tail (knot 9)
    visited.insert(knots[9]);
    
    // Simulate rope movements
    for (direction, count) in moves {
        for _ in 0..count {
            // Move head one step
            knots[0].step(direction);
            
            // Each subsequent knot follows the one in front
            for i in 1..10 {
                let head_pos = knots[i - 1];
                knots[i].follow(&head_pos);
            }
            
            // Track tail position (knot 9)
            visited.insert(knots[9]);
        }
    }
    
    visited.len()
}

pub fn solve(input: &str) -> (usize, usize) {
    (solve_part1(input), solve_part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_parse_moves() {
        let moves = parse_moves("R 4\nU 2\nL 1\nD 3");
        assert_eq!(moves.len(), 4);
        assert_eq!(moves[0], (Direction::East, 4));
        assert_eq!(moves[1], (Direction::North, 2));
        assert_eq!(moves[2], (Direction::West, 1));
        assert_eq!(moves[3], (Direction::South, 3));
    }

    #[test]
    fn test_pos_touching() {
        let p1 = Pos::new(0, 0);
        assert!(p1.is_touching(&Pos::new(0, 0))); // Overlapping
        assert!(p1.is_touching(&Pos::new(1, 0))); // Adjacent
        assert!(p1.is_touching(&Pos::new(1, 1))); // Diagonal
        assert!(!p1.is_touching(&Pos::new(2, 0))); // Too far
        assert!(!p1.is_touching(&Pos::new(0, 2))); // Too far
    }

    #[test]
    fn test_tail_follow() {
        let mut tail = Pos::origin();
        let head = Pos::new(2, 0); // Two steps right
        tail.follow(&head);
        assert_eq!(tail, Pos::new(1, 0)); // Tail moved one step right
        
        let mut tail = Pos::origin();
        let head = Pos::new(1, 2); // Diagonal: right 1, down 2
        tail.follow(&head);
        assert_eq!(tail, Pos::new(1, 1)); // Tail moved diagonally
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(EXAMPLE), 13);
    }

    #[test]
    fn test_part2_small() {
        // With 10 knots, the small example has tail that never moves
        assert_eq!(solve_part2(EXAMPLE), 1);
    }

    #[test]
    fn test_part2_large() {
        const LARGE_EXAMPLE: &str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(solve_part2(LARGE_EXAMPLE), 36);
    }
}
