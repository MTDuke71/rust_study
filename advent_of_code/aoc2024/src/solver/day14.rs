use mission6::{Grid, Coord};
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn parse(line: &str) -> Option<Self> {
        // Parse: p=0,4 v=3,-3
        let line = line.trim();
        if line.is_empty() {
            return None;
        }
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            eprintln!("Invalid format, expected 2 parts: {}", line);
            return None;
        }
        
        let pos = parts[0].strip_prefix("p=")?;
        let vel = parts[1].strip_prefix("v=")?;
        
        let pos_parts: Vec<&str> = pos.split(',').collect();
        let vel_parts: Vec<&str> = vel.split(',').collect();
        
        if pos_parts.len() != 2 || vel_parts.len() != 2 {
            eprintln!("Invalid coordinate format: {}", line);
            return None;
        }
        
        Some(Robot {
            px: pos_parts[0].parse().ok()?,
            py: pos_parts[1].parse().ok()?,
            vx: vel_parts[0].parse().ok()?,
            vy: vel_parts[1].parse().ok()?,
        })
    }
    
    /// Calculate position after `seconds` with wraparound
    fn position_at(&self, seconds: i32, width: i32, height: i32) -> (i32, i32) {
        let x = (self.px + self.vx * seconds).rem_euclid(width);
        let y = (self.py + self.vy * seconds).rem_euclid(height);
        (x, y)
    }
}

fn parse_robots(input: &str) -> Vec<Robot> {
    input
        .lines()
        .filter_map(|line| Robot::parse(line.trim()))
        .collect()
}

/// Calculate safety factor by counting robots in each quadrant
fn calculate_safety_factor(robots: &[Robot], seconds: i32, width: i32, height: i32) -> usize {
    let mid_x = width / 2;
    let mid_y = height / 2;
    
    let mut quadrants = [0usize; 4]; // [top-left, top-right, bottom-left, bottom-right]
    
    for robot in robots {
        let (x, y) = robot.position_at(seconds, width, height);
        
        // Skip robots exactly in the middle (horizontally or vertically)
        if x == mid_x || y == mid_y {
            continue;
        }
        
        let quad_idx = match (x < mid_x, y < mid_y) {
            (true, true) => 0,   // top-left
            (false, true) => 1,  // top-right
            (true, false) => 2,  // bottom-left
            (false, false) => 3, // bottom-right
        };
        
        quadrants[quad_idx] += 1;
    }
    
    quadrants.iter().product()
}

/// PARALLEL VERSION: Calculate safety factor using rayon
/// 
/// This is an excellent learning example for rayon:
/// - Each robot's position calculation is independent
/// - No shared mutable state during calculation
/// - Reduction operation (counting by quadrant) is associative
#[allow(dead_code)]
fn calculate_safety_factor_parallel(robots: &[Robot], seconds: i32, width: i32, height: i32) -> usize {
    let mid_x = width / 2;
    let mid_y = height / 2;
    
    // Parallel map-reduce: calculate positions in parallel, then reduce to quadrant counts
    let quadrants: [usize; 4] = robots
        .par_iter()  // Parallel iterator - rayon magic!
        .map(|robot| {
            let (x, y) = robot.position_at(seconds, width, height);
            
            // Skip robots exactly in the middle
            if x == mid_x || y == mid_y {
                return [0, 0, 0, 0];
            }
            
            // Return array with 1 in the correct quadrant
            match (x < mid_x, y < mid_y) {
                (true, true) => [1, 0, 0, 0],   // top-left
                (false, true) => [0, 1, 0, 0],  // top-right
                (true, false) => [0, 0, 1, 0],  // bottom-left
                (false, false) => [0, 0, 0, 1], // bottom-right
            }
        })
        .reduce(
            || [0, 0, 0, 0],  // Identity element
            |mut a, b| {      // Combine two results
                a[0] += b[0];
                a[1] += b[1];
                a[2] += b[2];
                a[3] += b[3];
                a
            }
        );
    
    quadrants.iter().product()
}

/// Build a grid showing robot positions (for visualization)
#[allow(dead_code)]
fn build_grid(robots: &[Robot], seconds: i32, width: i32, height: i32) -> Grid<usize> {
    let mut grid = Grid::new(height as usize, width as usize, 0usize);
    
    for robot in robots {
        let (x, y) = robot.position_at(seconds, width, height);
        let coord = Coord::new(y as usize, x as usize);
        if let Some(cell) = grid.get_mut(coord) {
            *cell += 1;
        }
    }
    
    grid
}

/// Check if robots form a suspicious pattern (many consecutive robots in a row)
/// Christmas tree likely has a trunk/outline with consecutive robots
fn has_pattern(robots: &[Robot], seconds: i32, width: i32, height: i32) -> bool {
    let mut positions = HashMap::new();
    
    for robot in robots {
        let (x, y) = robot.position_at(seconds, width, height);
        positions.insert((x, y), true);
    }
    
    // Look for horizontal runs of at least 10 consecutive robots
    for y in 0..height {
        let mut consecutive = 0;
        for x in 0..width {
            if positions.contains_key(&(x, y)) {
                consecutive += 1;
                if consecutive >= 10 {
                    return true;
                }
            } else {
                consecutive = 0;
            }
        }
    }
    
    false
}

pub fn part1(input: &str) -> String {
    let robots = parse_robots(input);
    let (width, height) = (101, 103);
    let seconds = 100;
    
    // For this problem size (~500 robots), serial is faster
    // Parallel has overhead from thread spawning/coordination
    // Use parallel version when you have >10,000 robots or multiple simulations
    let safety_factor = calculate_safety_factor(&robots, seconds, width, height);
    
    // Uncomment to use parallel (mainly educational):
    // let safety_factor = calculate_safety_factor_parallel(&robots, seconds, width, height);
    
    safety_factor.to_string()
}

pub fn part2(input: &str) -> String {
    let robots = parse_robots(input);
    let (width, height) = (101, 103);
    
    // Search for the Christmas tree pattern
    // The pattern repeats after width * height seconds (10403)
    // but we'll find it much earlier
    for seconds in 0..10500 {
        if has_pattern(&robots, seconds, width, height) {
            return seconds.to_string();
        }
    }
    
    "Pattern not found".to_string()
}

/// PARALLEL VERSION: Find Christmas tree pattern using rayon
/// 
/// Rayon learning: parallel search with early exit
/// - Each timestep check is independent
/// - find_first returns as soon as any thread finds a match
/// - Excellent for embarrassingly parallel searches
#[allow(dead_code)]
pub fn part2_parallel(input: &str) -> String {
    let robots = parse_robots(input);
    let (width, height) = (101, 103);
    
    // Parallel search across timesteps
    (0..10500)
        .into_par_iter()  // Convert range to parallel iterator
        .find_first(|&seconds| has_pattern(&robots, seconds, width, height))
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Pattern not found".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const EXAMPLE: &str = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    
    #[test]
    fn test_parse_robot() {
        let robot = Robot::parse("p=0,4 v=3,-3").unwrap();
        assert_eq!(robot.px, 0);
        assert_eq!(robot.py, 4);
        assert_eq!(robot.vx, 3);
        assert_eq!(robot.vy, -3);
    }
    
    #[test]
    fn test_robot_movement() {
        let robot = Robot { px: 2, py: 4, vx: 2, vy: -3 };
        
        // Test with 11x7 grid from example
        assert_eq!(robot.position_at(0, 11, 7), (2, 4));
        assert_eq!(robot.position_at(1, 11, 7), (4, 1));
        assert_eq!(robot.position_at(2, 11, 7), (6, 5));
        assert_eq!(robot.position_at(3, 11, 7), (8, 2));
        assert_eq!(robot.position_at(4, 11, 7), (10, 6));
        assert_eq!(robot.position_at(5, 11, 7), (1, 3)); // Wraparound
    }
    
    #[test]
    fn test_example_part1() {
        let robots = parse_robots(EXAMPLE);
        // Example uses 11x7 grid, not 101x103
        let safety_factor = calculate_safety_factor(&robots, 100, 11, 7);
        assert_eq!(safety_factor, 12);
    }
    
    #[test]
    fn test_parallel_vs_serial() {
        let robots = parse_robots(EXAMPLE);
        let serial = calculate_safety_factor(&robots, 100, 11, 7);
        let parallel = calculate_safety_factor_parallel(&robots, 100, 11, 7);
        assert_eq!(serial, parallel, "Serial and parallel should give same result");
    }
    
    #[test]
    fn test_large_dataset_parallel() {
        // Generate larger dataset to see parallel benefits
        use std::time::Instant;
        
        let mut robots = Vec::new();
        for i in 0..1000 {
            robots.push(Robot {
                px: i % 101,
                py: i % 103,
                vx: (i * 17) % 11 - 5,
                vy: (i * 23) % 13 - 6,
            });
        }
        
        let start = Instant::now();
        let serial = calculate_safety_factor(&robots, 100, 101, 103);
        let serial_time = start.elapsed();
        
        let start = Instant::now();
        let parallel = calculate_safety_factor_parallel(&robots, 100, 101, 103);
        let parallel_time = start.elapsed();
        
        println!("\n🏁 Performance comparison (1000 robots):");
        println!("  Serial:   {:?}", serial_time);
        println!("  Parallel: {:?}", parallel_time);
        println!("  Speedup:  {:.2}x", serial_time.as_secs_f64() / parallel_time.as_secs_f64());
        
        assert_eq!(serial, parallel);
    }
}
