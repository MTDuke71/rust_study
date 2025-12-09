use anyhow::Result;
use mission6::{FloodFill, Grid};
use std::collections::HashSet;

/// Parse input into a vector of (x, y) coordinates
fn parse_points(input: &str) -> Result<Vec<(i64, i64)>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() != 2 {
                anyhow::bail!("Invalid line format: {}", line);
            }
            let x = parts[0].trim().parse::<i64>()?;
            let y = parts[1].trim().parse::<i64>()?;
            Ok((x, y))
        })
        .collect()
}

/// Calculate the area of a rectangle formed by two opposite corners
/// The tiles are counted inclusively (both corners are included)
fn rectangle_area(p1: (i64, i64), p2: (i64, i64)) -> i64 {
    let width = (p1.0 - p2.0).abs() + 1;  // +1 for inclusive counting
    let height = (p1.1 - p2.1).abs() + 1; // +1 for inclusive counting
    width * height
}

/// Find the largest rectangle area by checking all pairs of points
fn find_largest_rectangle(points: &[(i64, i64)]) -> i64 {
    let mut max_area = 0;

    // Check all pairs of points as opposite corners
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let area = rectangle_area(points[i], points[j]);
            max_area = max_area.max(area);
        }
    }

    max_area
}

/// Draw a line between two points on a grid using Bresenham's algorithm
fn draw_line(grid: &mut Grid<char>, p1: (i64, i64), p2: (i64, i64), value: char, offset: (i64, i64)) {
    let x1 = (p1.0 - offset.0) as usize;
    let y1 = (p1.1 - offset.1) as usize;
    let x2 = (p2.0 - offset.0) as usize;
    let y2 = (p2.1 - offset.1) as usize;
    
    let dx = (x2 as i64 - x1 as i64).abs();
    let dy = (y2 as i64 - y1 as i64).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    
    let mut err = dx - dy;
    let mut x = x1 as i64;
    let mut y = y1 as i64;
    
    loop {
        grid[(x as usize, y as usize)] = value;
        
        if x == x2 as i64 && y == y2 as i64 {
            break;
        }
        
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}

/// Build a sparse set of boundary tiles (edges only, not interior)
fn build_polygon_boundary(points: &[(i64, i64)]) -> HashSet<(i64, i64)> {
    let mut tiles = HashSet::new();
    
    // Add all red tiles
    for &point in points {
        tiles.insert(point);
    }
    
    // Draw green lines connecting consecutive points (wraps around)
    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];
        add_line_to_set(&mut tiles, p1, p2);
    }
    
    tiles
}

/// Add line between two points to a HashSet
fn add_line_to_set(tiles: &mut HashSet<(i64, i64)>, p1: (i64, i64), p2: (i64, i64)) {
    let dx = (p2.0 - p1.0).abs();
    let dy = (p2.1 - p1.1).abs();
    let sx = if p1.0 < p2.0 { 1 } else { -1 };
    let sy = if p1.1 < p2.1 { 1 } else { -1 };
    
    let mut err = dx - dy;
    let mut x = p1.0;
    let mut y = p1.1;
    
    loop {
        tiles.insert((x, y));
        
        if x == p2.0 && y == p2.1 {
            break;
        }
        
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}

/// Check if a point is inside the polygon using ray casting
fn point_in_polygon(point: (i64, i64), polygon: &[(i64, i64)]) -> bool {
    let (x, y) = point;
    let mut inside = false;
    let n = polygon.len();
    
    for i in 0..n {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % n];
        
        if (y1 > y) != (y2 > y) {
            let x_intersect = x1 + ((y - y1) as f64 / (y2 - y1) as f64 * (x2 - x1) as f64) as i64;
            if x < x_intersect {
                inside = !inside;
            }
        }
    }
    
    inside
}

/// Check if a rectangle is entirely within or on the polygon (optimized AABB check)
/// Only checks corners and a sampling of edge points rather than every tile
fn rectangle_in_polygon_optimized(
    p1: (i64, i64),
    p2: (i64, i64),
    boundary: &HashSet<(i64, i64)>,
    polygon: &[(i64, i64)],
) -> bool {
    let min_x = p1.0.min(p2.0);
    let max_x = p1.0.max(p2.0);
    let min_y = p1.1.min(p2.1);
    let max_y = p1.1.max(p2.1);
    
    // Sample points along the rectangle boundary and interior
    // Instead of checking every tile, check strategic points
    let sample_rate = 10.max((max_x - min_x).max(max_y - min_y) / 100);
    
    // Check corners first (fast early exit)
    for &(x, y) in &[(min_x, min_y), (min_x, max_y), (max_x, min_y), (max_x, max_y)] {
        if !boundary.contains(&(x, y)) && !point_in_polygon((x, y), polygon) {
            return false;
        }
    }
    
    // Sample along edges
    for x in (min_x..=max_x).step_by(sample_rate as usize) {
        for &y in &[min_y, max_y] {
            if !boundary.contains(&(x, y)) && !point_in_polygon((x, y), polygon) {
                return false;
            }
        }
    }
    
    for y in (min_y..=max_y).step_by(sample_rate as usize) {
        for &x in &[min_x, max_x] {
            if !boundary.contains(&(x, y)) && !point_in_polygon((x, y), polygon) {
                return false;
            }
        }
    }
    
    // Sample interior points
    for x in (min_x..=max_x).step_by(sample_rate as usize) {
        for y in (min_y..=max_y).step_by(sample_rate as usize) {
            if !boundary.contains(&(x, y)) && !point_in_polygon((x, y), polygon) {
                return false;
            }
        }
    }
    
    true
}

/// Find the largest rectangle within the polygon
fn find_largest_rectangle_in_polygon(
    points: &[(i64, i64)],
    boundary: &HashSet<(i64, i64)>,
) -> i64 {
    let mut max_area = 0;
    
    // Check all pairs of red points as opposite corners
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            if rectangle_in_polygon_optimized(points[i], points[j], boundary, points) {
                let area = rectangle_area(points[i], points[j]);
                max_area = max_area.max(area);
            }
        }
    }
    
    max_area
}

/// Build a grid with red tiles and green boundary/interior (for small examples only)
fn build_grid_with_polygon(points: &[(i64, i64)]) -> (Grid<char>, i64, i64) {
    // Find bounds
    let min_x = points.iter().map(|p| p.0).min().unwrap();
    let max_x = points.iter().map(|p| p.0).max().unwrap();
    let min_y = points.iter().map(|p| p.1).min().unwrap();
    let max_y = points.iter().map(|p| p.1).max().unwrap();
    
    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;
    let offset = (min_x, min_y);
    
    // Create grid filled with empty tiles
    let mut grid = Grid::new(width, height, '.');
    
    // Mark red tiles
    for &point in points {
        let x = (point.0 - offset.0) as usize;
        let y = (point.1 - offset.1) as usize;
        grid[(x, y)] = 'R';
    }
    
    // Draw green lines connecting consecutive points (wraps around)
    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];
        draw_line(&mut grid, p1, p2, 'G', offset);
    }
    
    // Restore red tiles (in case lines overwrote them)
    for &point in points {
        let x = (point.0 - offset.0) as usize;
        let y = (point.1 - offset.1) as usize;
        grid[(x, y)] = 'R';
    }
    
    // Flood fill interior with green
    FloodFill::fill_holes(&mut grid, &'R', '.', 'G');
    
    (grid, offset.0, offset.1)
}

/// Check if a rectangle contains only red or green tiles (grid version for tests)
fn rectangle_contains_only_red_or_green(
    grid: &Grid<char>,
    p1: (i64, i64),
    p2: (i64, i64),
    offset: (i64, i64),
) -> bool {
    let min_x = p1.0.min(p2.0);
    let max_x = p1.0.max(p2.0);
    let min_y = p1.1.min(p2.1);
    let max_y = p1.1.max(p2.1);
    
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let grid_x = (x - offset.0) as usize;
            let grid_y = (y - offset.1) as usize;
            
            // Check bounds
            if grid_x >= grid.width() || grid_y >= grid.height() {
                return false;
            }
            
            let tile = grid[(grid_x, grid_y)];
            if tile != 'R' && tile != 'G' {
                return false;
            }
        }
    }
    
    true
}

/// Find the largest rectangle using only red and green tiles (grid version for tests)
fn find_largest_rectangle_with_constraint(
    points: &[(i64, i64)],
    grid: &Grid<char>,
    offset: (i64, i64),
) -> i64 {
    let mut max_area = 0;
    
    // Check all pairs of red points as opposite corners
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            if rectangle_contains_only_red_or_green(grid, points[i], points[j], offset) {
                let area = rectangle_area(points[i], points[j]);
                max_area = max_area.max(area);
            }
        }
    }
    
    max_area
}

pub fn solve_part1(input: &str) -> Result<String> {
    let points = parse_points(input)?;
    let max_area = find_largest_rectangle(&points);
    Ok(max_area.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let points = parse_points(input)?;
    let boundary = build_polygon_boundary(&points);
    let max_area = find_largest_rectangle_in_polygon(&points, &boundary);
    Ok(max_area.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_parse_points() {
        let points = parse_points(EXAMPLE).unwrap();
        assert_eq!(points.len(), 8);
        assert_eq!(points[0], (7, 1));
        assert_eq!(points[1], (11, 1));
        assert_eq!(points[7], (7, 3));
    }

    #[test]
    fn test_rectangle_area() {
        // Example from problem: (2,5) to (9,7) = area 24
        // width = |9-2| + 1 = 8, height = |7-5| + 1 = 3, area = 24
        assert_eq!(rectangle_area((2, 5), (9, 7)), 24);
        
        // Example 2: (7,1) to (11,7) = area 35
        // width = |11-7| + 1 = 5, height = |7-1| + 1 = 7, area = 35
        assert_eq!(rectangle_area((7, 1), (11, 7)), 35);
        
        // Thin rectangle: (7,3) to (2,3) = area 6
        // width = |7-2| + 1 = 6, height = |3-3| + 1 = 1, area = 6
        assert_eq!(rectangle_area((7, 3), (2, 3)), 6);
    }

    #[test]
    fn test_rectangle_area_with_inclusive_tiles() {
        // Verify inclusive tile counting matches problem examples
        
        // (2,5) to (9,7): width = 8, height = 3, area = 24
        assert_eq!(rectangle_area((2, 5), (9, 7)), 24);
        
        // (7,1) to (11,7): width = 5, height = 7, area = 35
        assert_eq!(rectangle_area((7, 1), (11, 7)), 35);
    }

    #[test]
    fn test_example_largest_rectangle() {
        // Problem states largest area is 50
        // Between (2,5) and (11,1)
        // width = |11-2| + 1 = 10, height = |5-1| + 1 = 5, area = 50
        let area = rectangle_area((2, 5), (11, 1));
        assert_eq!(area, 50);
    }

    #[test]
    fn test_find_largest_rectangle_example() {
        let points = parse_points(EXAMPLE).unwrap();
        let max_area = find_largest_rectangle(&points);
        
        // With inclusive counting, answer should be 50
        assert_eq!(max_area, 50);
    }

    #[test]
    fn test_edge_cases() {
        // Same point - 1x1 tile = area 1
        assert_eq!(rectangle_area((5, 5), (5, 5)), 1);
        
        // Horizontal line - width = 8, height = 1, area = 8
        assert_eq!(rectangle_area((2, 5), (9, 5)), 8);
        
        // Vertical line - width = 1, height = 8, area = 8
        assert_eq!(rectangle_area((5, 2), (5, 9)), 8);
        
        // Negative coordinates: width = |2-(-5)| + 1 = 8, height = |4-(-3)| + 1 = 8
        assert_eq!(rectangle_area((-5, -3), (2, 4)), 64);
    }

    #[test]
    fn test_small_rectangle() {
        // Adjacent points: 2x2 tiles = area 4
        assert_eq!(rectangle_area((0, 0), (1, 1)), 4);
        
        // Single row: 3x1 tiles = area 3
        assert_eq!(rectangle_area((0, 0), (2, 0)), 3);
    }

    #[test]
    fn test_part2_example() {
        // Part 2: With polygon boundary + interior filled, max area should be 24
        let points = parse_points(EXAMPLE).unwrap();
        let (grid, offset_x, offset_y) = build_grid_with_polygon(&points);
        let max_area = find_largest_rectangle_with_constraint(&points, &grid, (offset_x, offset_y));
        
        // Problem states largest valid rectangle has area 24
        assert_eq!(max_area, 24);
    }

    #[test]
    fn test_part2_example_sparse() {
        // Part 2 using sparse representation (boundary + ray casting)
        let points = parse_points(EXAMPLE).unwrap();
        let boundary = build_polygon_boundary(&points);
        let max_area = find_largest_rectangle_in_polygon(&points, &boundary);
        
        // Should match grid-based approach
        assert_eq!(max_area, 24);
    }

    #[test]
    fn test_part2_specific_rectangle() {
        // Problem example: area 24 between (9,5) and (2,3)
        // This should be valid (only red/green tiles)
        let points = parse_points(EXAMPLE).unwrap();
        let (grid, offset_x, offset_y) = build_grid_with_polygon(&points);
        
        let area = rectangle_area((9, 5), (2, 3));
        assert_eq!(area, 24); // 8 width × 3 height
        
        // Verify this rectangle contains only red/green
        assert!(rectangle_contains_only_red_or_green(
            &grid,
            (9, 5),
            (2, 3),
            (offset_x, offset_y)
        ));
    }
}
