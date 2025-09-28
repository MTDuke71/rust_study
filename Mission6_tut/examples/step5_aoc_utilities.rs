// Step 5: AoC Utilities & Flood Fill
// Tutorial Day 5 - Aligns with Mission 6 AoC Grid Utilities & REQ-5
//
// Learning Objectives:
// - Implement flood fill algorithm for region detection
// - Understand connected components analysis
// - Learn AoC parsing patterns for grid inputs
// - Practice competitive programming grid utilities
//
// This tutorial step builds toward Mission6 REQ-5:
// "The system shall provide AoC-specific utilities including flood fill operations,
//  connected component analysis, and grid parsing from text input formats."

use mission6_tut::tutorial_helpers::{print_section, print_step_complete, TutorialGrid, TutorialCoord};
use std::collections::{HashSet, VecDeque};

fn main() {
    println!("=== Mission 6 Tutorial - Step 5: AoC Utilities & Flood Fill ===");
    println!("Day 5 Focus: Flood fill, connected components, and competitive programming patterns\n");

    // Section 1: Introduction to Flood Fill
    print_section("1. Introduction to Flood Fill");
    
    println!("Flood fill is an algorithm that fills connected regions with a value.");
    println!("Common applications:");
    println!("• Paint bucket tool in graphics software");
    println!("• Region detection in image processing");
    println!("• Connected component analysis");
    println!("• AoC problems: counting areas, detecting enclosed regions");
    
    let mut demo_grid = TutorialGrid::new(8, 6, '.');
    
    // Create some regions
    let obstacles = vec![
        TutorialCoord::new(2, 1), TutorialCoord::new(3, 1), TutorialCoord::new(4, 1),
        TutorialCoord::new(2, 2), TutorialCoord::new(4, 2),
        TutorialCoord::new(2, 3), TutorialCoord::new(3, 3), TutorialCoord::new(4, 3),
        TutorialCoord::new(6, 2), TutorialCoord::new(6, 3), TutorialCoord::new(6, 4),
    ];
    
    for obstacle in obstacles {
        demo_grid.set(obstacle, '#');
    }
    
    println!("\nDemo grid with regions:");
    println!("{}", demo_grid);
    
    // Section 2: Basic Flood Fill Implementation
    print_section("2. Basic Flood Fill Implementation");
    
    let mut fill_grid = demo_grid.clone();
    let fill_start = TutorialCoord::new(0, 0);
    
    println!("Starting flood fill from {} (filling '.' with 'X'):", fill_start);
    let filled_count = flood_fill_4(&mut fill_grid, fill_start, '.', 'X');
    println!("Filled {} cells:", filled_count);
    println!("{}", fill_grid);
    
    // Section 3: 4-Connected vs 8-Connected Flood Fill
    print_section("3. 4-Connected vs 8-Connected Flood Fill");
    
    // Create a diagonal test case
    let mut diagonal_grid = TutorialGrid::new(5, 5, '.');
    let diagonal_obstacles = vec![
        TutorialCoord::new(1, 1), TutorialCoord::new(2, 2), TutorialCoord::new(3, 3),
    ];
    for obstacle in diagonal_obstacles {
        diagonal_grid.set(obstacle, '#');
    }
    
    println!("Test grid with diagonal obstacles:");
    println!("{}", diagonal_grid);
    
    // 4-connected flood fill
    let mut grid_4 = diagonal_grid.clone();
    let count_4 = flood_fill_4(&mut grid_4, TutorialCoord::new(0, 0), '.', 'A');
    
    // 8-connected flood fill  
    let mut grid_8 = diagonal_grid.clone();
    let count_8 = flood_fill_8(&mut grid_8, TutorialCoord::new(0, 0), '.', 'B');
    
    println!("\n4-connected fill (A): {} cells", count_4);
    println!("{}", grid_4);
    
    println!("8-connected fill (B): {} cells", count_8);
    println!("{}", grid_8);
    
    println!("4-connected treats diagonals as separate regions");
    println!("8-connected considers diagonal adjacency");
    
    // Section 4: Connected Components Analysis
    print_section("4. Connected Components Analysis");
    
    let mut island_grid = TutorialGrid::new(10, 6, '.');
    
    // Create multiple islands
    let islands = vec![
        // Island 1
        vec![TutorialCoord::new(1, 1), TutorialCoord::new(2, 1), TutorialCoord::new(1, 2)],
        // Island 2  
        vec![TutorialCoord::new(5, 1), TutorialCoord::new(6, 1), TutorialCoord::new(5, 2), TutorialCoord::new(6, 2)],
        // Island 3
        vec![TutorialCoord::new(8, 3), TutorialCoord::new(9, 3), TutorialCoord::new(8, 4)],
    ];
    
    for island in &islands {
        for &coord in island {
            island_grid.set(coord, '#');
        }
    }
    
    println!("Grid with multiple islands:");
    println!("{}", island_grid);
    
    let components = find_connected_components(&island_grid, '#');
    println!("\nFound {} connected components:", components.len());
    
    for (i, component) in components.iter().enumerate() {
        println!("Component {}: {} cells", i + 1, component.len());
        for coord in component {
            print!("  {}", coord);
        }
        println!();
    }
    
    // Section 5: Region Properties Analysis  
    print_section("5. Region Properties Analysis");
    
    let test_coord = TutorialCoord::new(1, 1);
    if let Some(region) = get_region_4(&island_grid, test_coord, '#') {
        let area = region.len();
        let perimeter = calculate_perimeter(&island_grid, &region, '#');
        let bounding_box = calculate_bounding_box(&region);
        
        println!("Region analysis for {}:", test_coord);
        println!("• Area: {} cells", area);
        println!("• Perimeter: {} units", perimeter);
        println!("• Bounding box: {:?} to {:?}", bounding_box.0, bounding_box.1);
        
        // Calculate compactness (area / perimeter ratio)
        let compactness = area as f64 / perimeter as f64;
        println!("• Compactness: {:.2} (higher = more compact)", compactness);
    }
    
    // Section 6: AoC Parsing Patterns
    print_section("6. AoC Parsing Patterns");
    
    println!("Common AoC grid input patterns:");
    
    // Pattern 1: Simple character grid
    let aoc_input1 = "\
..##..
.#..#.
##..##
.####.
..##..";
    
    println!("\nPattern 1: Character grid");
    println!("{}", aoc_input1);
    
    let parsed_grid1 = parse_char_grid(aoc_input1);
    println!("Parsed as {}x{} grid", parsed_grid1.width(), parsed_grid1.height());
    
    let obstacle_count = count_chars(&parsed_grid1, '#');
    println!("Found {} obstacles", obstacle_count);
    
    // Pattern 2: Coordinate list
    let aoc_input2 = "2,3\n4,1\n1,4\n5,2";
    println!("\nPattern 2: Coordinate list");
    println!("{}", aoc_input2);
    
    let coordinates = parse_coordinate_list(aoc_input2);
    let coord_grid = coordinates_to_grid(&coordinates, 7, 6);
    println!("Created grid from {} coordinates:", coordinates.len());
    println!("{}", coord_grid);
    
    // Section 7: Competitive Programming Utilities
    print_section("7. Competitive Programming Utilities");
    
    let puzzle_grid = parse_char_grid(aoc_input1);
    println!("Original puzzle grid:");
    println!("{}", puzzle_grid);
    
    // Count distinct regions
    let regions = find_connected_components(&puzzle_grid, '#');
    println!("\nRegion analysis:");
    println!("• {} distinct regions", regions.len());
    
    let mut total_area = 0;
    let mut total_perimeter = 0;
    
    for (i, region) in regions.iter().enumerate() {
        let area = region.len();
        let perimeter = calculate_perimeter(&puzzle_grid, region, '#');
        total_area += area;
        total_perimeter += perimeter;
        
        println!("  Region {}: area={}, perimeter={}", i + 1, area, perimeter);
    }
    
    println!("• Total area: {}", total_area);
    println!("• Total perimeter: {}", total_perimeter);
    
    // Find largest region
    if let Some(largest) = regions.iter().max_by_key(|r| r.len()) {
        println!("• Largest region: {} cells", largest.len());
    }
    
    // Section 8: Advanced Flood Fill Patterns
    print_section("8. Advanced Flood Fill Patterns");
    
    // Pattern: Fill holes (flood fill from edges, then invert)
    let mut hole_grid = TutorialGrid::new(7, 5, '.');
    let border = vec![
        TutorialCoord::new(1, 1), TutorialCoord::new(2, 1), TutorialCoord::new(3, 1), TutorialCoord::new(4, 1), TutorialCoord::new(5, 1),
        TutorialCoord::new(1, 2), TutorialCoord::new(5, 2),
        TutorialCoord::new(1, 3), TutorialCoord::new(2, 3), TutorialCoord::new(4, 3), TutorialCoord::new(5, 3),
    ];
    
    for coord in border {
        hole_grid.set(coord, '#');
    }
    
    println!("Grid with enclosed region:");
    println!("{}", hole_grid);
    
    // Fill from all edges to mark exterior
    let mut exterior_grid = hole_grid.clone();
    
    // Fill from edges
    let width = exterior_grid.width();
    let height = exterior_grid.height();
    
    for x in 0..width {
        flood_fill_4(&mut exterior_grid, TutorialCoord::new(x, 0), '.', 'E'); // Top edge
        flood_fill_4(&mut exterior_grid, TutorialCoord::new(x, height - 1), '.', 'E'); // Bottom edge
    }
    for y in 0..height {
        flood_fill_4(&mut exterior_grid, TutorialCoord::new(0, y), '.', 'E'); // Left edge
        flood_fill_4(&mut exterior_grid, TutorialCoord::new(width - 1, y), '.', 'E'); // Right edge
    }
    
    println!("\nAfter marking exterior (E):");
    println!("{}", exterior_grid);
    
    // Count interior cells (not 'E' and not '#')
    let mut interior_count = 0;
    for y in 0..exterior_grid.height() {
        for x in 0..exterior_grid.width() {
            let coord = TutorialCoord::new(x, y);
            let cell = exterior_grid.get(coord).unwrap();
            if *cell != 'E' && *cell != '#' {
                interior_count += 1;
            }
        }
    }
    
    println!("Interior cells (holes): {}", interior_count);
    
    // Section 9: Performance Considerations
    print_section("9. Performance Considerations");
    
    use std::time::Instant;
    
    // Test flood fill performance on large grid
    let mut large_grid = TutorialGrid::new(100, 100, '.');
    
    // Add some obstacles
    for i in 0..50 {
        large_grid.set(TutorialCoord::new(i * 2, 50), '#');
    }
    
    let start_time = Instant::now();
    let filled = flood_fill_4(&mut large_grid, TutorialCoord::new(0, 0), '.', 'X');
    let flood_time = start_time.elapsed();
    
    println!("Large grid (100x100) flood fill:");
    println!("• Filled {} cells in {:?}", filled, flood_time);
    
    // Test connected components performance
    let start_time = Instant::now();
    let components = find_connected_components(&large_grid, 'X');
    let components_time = start_time.elapsed();
    
    println!("• Found {} components in {:?}", components.len(), components_time);
    
    print_step_complete("Step 5: AoC Utilities & Flood Fill");
    
    // Next Steps Preview
    println!("\n🔄 Next: Step 6 - Performance Optimization");
    println!("   Learn benchmarking, memory optimization, and cache-friendly patterns");
    println!("   Command: cargo run --example step6_performance");
    
    // Key Takeaways
    println!("\n📝 Key Takeaways from Step 5:");
    println!("   ✓ Flood fill detects and modifies connected regions");
    println!("   ✓ 4-connected vs 8-connected affects region boundaries");
    println!("   ✓ Connected components analysis finds all separate regions");
    println!("   ✓ Region properties: area, perimeter, bounding box");
    println!("   ✓ AoC parsing: character grids and coordinate lists");
    println!("   ✓ Edge-filling technique detects interior/exterior regions");
}

// Flood fill implementation (4-connected)
fn flood_fill_4<T: Clone + PartialEq>(grid: &mut TutorialGrid<T>, start: TutorialCoord, target: T, replacement: T) -> usize {
    if !grid.in_bounds(start) || grid.get(start) != Some(&target) || target == replacement {
        return 0;
    }
    
    let mut stack = vec![start];
    let mut count = 0;
    
    while let Some(coord) = stack.pop() {
        if !grid.in_bounds(coord) || grid.get(coord) != Some(&target) {
            continue;
        }
        
        grid.set(coord, replacement.clone());
        count += 1;
        
        // Add 4-connected neighbors
        let neighbors = [
            TutorialCoord::new(coord.x, coord.y.wrapping_sub(1)), // North
            TutorialCoord::new(coord.x.wrapping_add(1), coord.y), // East
            TutorialCoord::new(coord.x, coord.y.wrapping_add(1)), // South
            TutorialCoord::new(coord.x.wrapping_sub(1), coord.y), // West
        ];
        
        for neighbor in neighbors {
            if grid.in_bounds(neighbor) && grid.get(neighbor) == Some(&target) {
                stack.push(neighbor);
            }
        }
    }
    
    count
}

// Flood fill implementation (8-connected)
fn flood_fill_8<T: Clone + PartialEq>(grid: &mut TutorialGrid<T>, start: TutorialCoord, target: T, replacement: T) -> usize {
    if !grid.in_bounds(start) || grid.get(start) != Some(&target) || target == replacement {
        return 0;
    }
    
    let mut stack = vec![start];
    let mut count = 0;
    
    while let Some(coord) = stack.pop() {
        if !grid.in_bounds(coord) || grid.get(coord) != Some(&target) {
            continue;
        }
        
        grid.set(coord, replacement.clone());
        count += 1;
        
        // Add 8-connected neighbors
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 { continue; }
                
                let new_x = coord.x as i32 + dx;
                let new_y = coord.y as i32 + dy;
                
                if new_x >= 0 && new_y >= 0 {
                    let neighbor = TutorialCoord::new(new_x as usize, new_y as usize);
                    if grid.in_bounds(neighbor) && grid.get(neighbor) == Some(&target) {
                        stack.push(neighbor);
                    }
                }
            }
        }
    }
    
    count
}

// Find all connected components
fn find_connected_components<T: Clone + PartialEq>(grid: &TutorialGrid<T>, target: T) -> Vec<Vec<TutorialCoord>> {
    let mut visited = HashSet::new();
    let mut components = Vec::new();
    
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let coord = TutorialCoord::new(x, y);
            if !visited.contains(&coord) && grid.get(coord) == Some(&target) {
                let component = get_region_4(grid, coord, target.clone()).unwrap_or_default();
                for &c in &component {
                    visited.insert(c);
                }
                if !component.is_empty() {
                    components.push(component);
                }
            }
        }
    }
    
    components
}

// Get a single connected region
fn get_region_4<T: Clone + PartialEq>(grid: &TutorialGrid<T>, start: TutorialCoord, target: T) -> Option<Vec<TutorialCoord>> {
    if grid.get(start) != Some(&target) {
        return None;
    }
    
    let mut region = Vec::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    
    queue.push_back(start);
    visited.insert(start);
    
    while let Some(coord) = queue.pop_front() {
        region.push(coord);
        
        // Check 4-connected neighbors
        let neighbors = [
            TutorialCoord::new(coord.x, coord.y.wrapping_sub(1)),
            TutorialCoord::new(coord.x.wrapping_add(1), coord.y),
            TutorialCoord::new(coord.x, coord.y.wrapping_add(1)),
            TutorialCoord::new(coord.x.wrapping_sub(1), coord.y),
        ];
        
        for neighbor in neighbors {
            if grid.in_bounds(neighbor) && 
               !visited.contains(&neighbor) && 
               grid.get(neighbor) == Some(&target) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }
    
    Some(region)
}

// Calculate perimeter of a region
fn calculate_perimeter<T: Clone + PartialEq>(grid: &TutorialGrid<T>, region: &[TutorialCoord], target: T) -> usize {
    let region_set: HashSet<_> = region.iter().collect();
    let mut perimeter = 0;
    
    for &coord in region {
        let neighbors = [
            TutorialCoord::new(coord.x, coord.y.wrapping_sub(1)),
            TutorialCoord::new(coord.x.wrapping_add(1), coord.y),
            TutorialCoord::new(coord.x, coord.y.wrapping_add(1)),
            TutorialCoord::new(coord.x.wrapping_sub(1), coord.y),
        ];
        
        for neighbor in neighbors {
            if !grid.in_bounds(neighbor) || 
               grid.get(neighbor) != Some(&target) ||
               !region_set.contains(&neighbor) {
                perimeter += 1;
            }
        }
    }
    
    perimeter
}

// Calculate bounding box of a region
fn calculate_bounding_box(region: &[TutorialCoord]) -> (TutorialCoord, TutorialCoord) {
    if region.is_empty() {
        return (TutorialCoord::new(0, 0), TutorialCoord::new(0, 0));
    }
    
    let mut min_x = region[0].x;
    let mut max_x = region[0].x;
    let mut min_y = region[0].y;
    let mut max_y = region[0].y;
    
    for coord in region {
        min_x = min_x.min(coord.x);
        max_x = max_x.max(coord.x);
        min_y = min_y.min(coord.y);
        max_y = max_y.max(coord.y);
    }
    
    (TutorialCoord::new(min_x, min_y), TutorialCoord::new(max_x, max_y))
}

// Parse character grid from string
fn parse_char_grid(input: &str) -> TutorialGrid<char> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return TutorialGrid::new(0, 0, '.');
    }
    
    let height = lines.len();
    let width = lines[0].len();
    let mut grid = TutorialGrid::new(width, height, '.');
    
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid.set(TutorialCoord::new(x, y), ch);
        }
    }
    
    grid
}

// Parse coordinate list from string
fn parse_coordinate_list(input: &str) -> Vec<TutorialCoord> {
    input.lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 2 {
                if let (Ok(x), Ok(y)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                    return Some(TutorialCoord::new(x, y));
                }
            }
            None
        })
        .collect()
}

// Convert coordinate list to grid
fn coordinates_to_grid(coords: &[TutorialCoord], width: usize, height: usize) -> TutorialGrid<char> {
    let mut grid = TutorialGrid::new(width, height, '.');
    for &coord in coords {
        if grid.in_bounds(coord) {
            grid.set(coord, '#');
        }
    }
    grid
}

// Count occurrences of a character
fn count_chars(grid: &TutorialGrid<char>, target: char) -> usize {
    let mut count = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid.get(TutorialCoord::new(x, y)) == Some(&target) {
                count += 1;
            }
        }
    }
    count
}

// Exercise for the Reader:
// 1. Implement a "bucket fill" that only fills within certain boundaries
// 2. Create a function that finds the largest hole (interior region) in a shape
// 3. Write a region merger that combines adjacent regions based on criteria
// 4. Implement connected component labeling with unique IDs for each region

// Design Questions to Consider:
// - When is recursive vs iterative flood fill better?
// - How do you handle very large regions without stack overflow?
// - What are efficient data structures for region adjacency queries?
// - How would you optimize flood fill for repeated operations on the same grid?