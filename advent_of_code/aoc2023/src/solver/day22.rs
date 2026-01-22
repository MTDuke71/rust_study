//! Day 22: Sand Slabs
//!
//! Simulating falling 3D bricks and analyzing support relationships.
//!
//! # Problem Summary
//!
//! - Parse 3D brick coordinates (x1,y1,z1~x2,y2,z2)
//! - Simulate gravity - bricks fall until they hit ground or another brick
//! - Build support graph (which bricks support which)
//! - Part 1: Count safely disintegratable bricks (removing won't cause chain reaction)

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Brick {
    id: usize,
    start: Point3D,
    end: Point3D,
}

impl Brick {
    /// Parse brick from input line like "1,0,1~1,2,1"
    fn parse(id: usize, line: &str) -> Option<Self> {
        let (start_str, end_str) = line.split_once('~')?;
        let start = Self::parse_point(start_str)?;
        let end = Self::parse_point(end_str)?;
        Some(Brick { id, start, end })
    }

    fn parse_point(s: &str) -> Option<Point3D> {
        let parts: Vec<i32> = s.split(',').filter_map(|n| n.parse().ok()).collect();
        if parts.len() == 3 {
            Some(Point3D {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            })
        } else {
            None
        }
    }

    /// Get all cube coordinates this brick occupies
    fn get_cubes(&self) -> Vec<Point3D> {
        let mut cubes = Vec::new();
        for x in self.start.x.min(self.end.x)..=self.start.x.max(self.end.x) {
            for y in self.start.y.min(self.end.y)..=self.start.y.max(self.end.y) {
                for z in self.start.z.min(self.end.z)..=self.start.z.max(self.end.z) {
                    cubes.push(Point3D { x, y, z });
                }
            }
        }
        cubes
    }

    /// Get the minimum z coordinate of this brick
    fn min_z(&self) -> i32 {
        self.start.z.min(self.end.z)
    }

    /// Get the maximum z coordinate of this brick
    fn max_z(&self) -> i32 {
        self.start.z.max(self.end.z)
    }

    /// Move brick down by dz units
    fn move_down(&mut self, dz: i32) {
        self.start.z -= dz;
        self.end.z -= dz;
    }
}

/// Parse all bricks from input
fn parse_bricks(input: &str) -> Vec<Brick> {
    input
        .lines()
        .enumerate()
        .filter_map(|(id, line)| Brick::parse(id, line.trim()))
        .collect()
}

/// Simulate all bricks falling until they settle
fn simulate_falling(bricks: &mut [Brick]) {
    // Sort bricks by minimum z (lowest bricks fall first)
    bricks.sort_by_key(|b| b.min_z());

    // Track which (x,y) coordinates are occupied at which z level
    // Key: (x, y), Value: (max_z at that position, brick_id)
    let mut height_map: HashMap<(i32, i32), (i32, usize)> = HashMap::new();

    for brick in bricks.iter_mut() {
        // Find the highest point this brick would rest on
        let mut max_z_below = 0; // Ground is at z=0
        
        // Check all (x, y) positions this brick occupies
        for cube in brick.get_cubes() {
            if let Some(&(z, _)) = height_map.get(&(cube.x, cube.y)) {
                max_z_below = max_z_below.max(z);
            }
        }

        // Drop the brick to rest on max_z_below
        let fall_distance = brick.min_z() - (max_z_below + 1);
        brick.move_down(fall_distance);

        // Update height map with this brick's final position
        let new_max_z = brick.max_z();
        for cube in brick.get_cubes() {
            height_map.insert((cube.x, cube.y), (new_max_z, brick.id));
        }
    }
}

/// Build support relationships between bricks
/// Returns (supports, supported_by) where:
/// - supports[i] = set of brick IDs that brick i supports
/// - supported_by[i] = set of brick IDs that support brick i
fn build_support_graph(bricks: &[Brick]) -> (Vec<HashSet<usize>>, Vec<HashSet<usize>>) {
    let n = bricks.len();
    let mut supports = vec![HashSet::new(); n];
    let mut supported_by = vec![HashSet::new(); n];

    // Build a map of (x, y, z) -> brick_id for quick lookup
    let mut space: HashMap<(i32, i32, i32), usize> = HashMap::new();
    for brick in bricks {
        for cube in brick.get_cubes() {
            space.insert((cube.x, cube.y, cube.z), brick.id);
        }
    }

    // For each brick, check if it supports any brick directly above it
    for brick in bricks {
        let top_z = brick.max_z();
        let brick_id = brick.id;

        // Check all positions one level above this brick
        let mut supported_ids = HashSet::new();
        for cube in brick.get_cubes() {
            if cube.z == top_z {
                // Check the position directly above
                if let Some(&above_id) = space.get(&(cube.x, cube.y, top_z + 1)) {
                    if above_id != brick_id {
                        supported_ids.insert(above_id);
                    }
                }
            }
        }

        // Update support relationships
        for &above_id in &supported_ids {
            supports[brick_id].insert(above_id);
            supported_by[above_id].insert(brick_id);
        }
    }

    (supports, supported_by)
}

/// Simulate removing a brick and count how many other bricks would fall
fn count_chain_reaction(
    brick_id: usize,
    supports: &[HashSet<usize>],
    supported_by: &[HashSet<usize>],
) -> usize {
    let mut fallen = HashSet::new();
    fallen.insert(brick_id);

    // Keep checking for bricks that would fall
    let mut changed = true;
    while changed {
        changed = false;
        for id in 0..supports.len() {
            if fallen.contains(&id) {
                continue; // Already fallen
            }
            // Check if all supporters of this brick have fallen
            if !supported_by[id].is_empty() && supported_by[id].iter().all(|&s| fallen.contains(&s))
            {
                fallen.insert(id);
                changed = true;
            }
        }
    }

    // Return count of fallen bricks (excluding the original brick we removed)
    fallen.len() - 1
}

pub fn solve_part1(input: &str) -> usize {
    let mut bricks = parse_bricks(input);
    
    // Simulate bricks falling
    simulate_falling(&mut bricks);
    
    // Build support relationships
    let (supports, supported_by) = build_support_graph(&bricks);
    
    // Count bricks that can be safely disintegrated
    // A brick can be removed if every brick it supports has at least one other supporter
    let mut safe_count = 0;
    for brick_id in 0..bricks.len() {
        let can_remove = supports[brick_id].iter().all(|&above_id| {
            supported_by[above_id].len() > 1
        });
        if can_remove {
            safe_count += 1;
        }
    }
    
    safe_count
}

pub fn solve_part2(input: &str) -> usize {
    let mut bricks = parse_bricks(input);
    
    // Simulate bricks falling
    simulate_falling(&mut bricks);
    
    // Build support relationships
    let (supports, supported_by) = build_support_graph(&bricks);
    
    // For each brick, count how many would fall in a chain reaction
    let mut total_fallen = 0;
    for brick_id in 0..bricks.len() {
        total_fallen += count_chain_reaction(brick_id, &supports, &supported_by);
    }
    
    total_fallen
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn test_parse_brick() {
        let brick = Brick::parse(0, "1,0,1~1,2,1").unwrap();
        assert_eq!(brick.start, Point3D { x: 1, y: 0, z: 1 });
        assert_eq!(brick.end, Point3D { x: 1, y: 2, z: 1 });
    }

    #[test]
    fn test_brick_cubes() {
        // Single cube
        let brick = Brick::parse(0, "2,2,2~2,2,2").unwrap();
        assert_eq!(brick.get_cubes().len(), 1);

        // Horizontal x-direction (2 cubes)
        let brick = Brick::parse(1, "0,0,10~1,0,10").unwrap();
        assert_eq!(brick.get_cubes().len(), 2);

        // Vertical (10 cubes)
        let brick = Brick::parse(2, "0,0,1~0,0,10").unwrap();
        assert_eq!(brick.get_cubes().len(), 10);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 5);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 7);
    }
}
