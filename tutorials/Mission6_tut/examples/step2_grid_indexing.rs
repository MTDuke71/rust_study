// Step 2: Grid Indexing & Safety
// Tutorial Day 2 - Aligns with Mission 6 Grid Representation & Indexing & REQ-2 (Safe Indexing)
//
// Learning Objectives:
// - Master different grid access patterns (get, get_mut, safe vs unsafe)
// - Understand iterator patterns for grid traversal
// - Learn row and column iteration techniques
// - Practice memory safety with Option<T> return types
//
// This tutorial step builds toward Mission6 REQ-2:
// "The system shall provide safe indexing operations that return Option<T> for
//  out-of-bounds access, and iterator support for traversing grid contents."

use mission6_tut::tutorial_helpers::{print_section, print_step_complete, TutorialGrid, TutorialCoord};

fn main() {
    println!("=== Mission 6 Tutorial - Step 2: Grid Indexing & Safety ===");
    println!("Day 2 Focus: Safe access patterns, iterators, and memory safety\n");

    // Section 1: Safe vs Unsafe Access Patterns
    print_section("1. Safe vs Unsafe Access Patterns");
    
    let mut grid = TutorialGrid::new(4, 3, 0);
    
    // Fill with some test data
    for y in 0..3 {
        for x in 0..4 {
            let value = (y * 4 + x + 1) as i32;
            grid.set(TutorialCoord::new(x, y), value);
        }
    }
    
    println!("Test grid (4x3):");
    println!("{}", grid);
    
    // Safe access - returns Option<T>
    let coord = TutorialCoord::new(2, 1);
    match grid.get(coord) {
        Some(value) => println!("Safe access at {}: {}", coord, value),
        None => println!("Safe access failed - out of bounds"),
    }
    
    // Safe access with out-of-bounds coordinate
    let oob_coord = TutorialCoord::new(10, 10);
    match grid.get(oob_coord) {
        Some(value) => println!("Found value: {}", value),
        None => println!("Safe access at {} returned None (expected)", oob_coord),
    }
    
    // Section 2: Mutable Access Patterns
    print_section("2. Mutable Access and Modification");
    
    let mut mutable_grid = TutorialGrid::new(3, 3, '.');
    
    // Safe mutable access (conceptually - our TutorialGrid uses set method)
    mutable_grid.set(TutorialCoord::new(1, 1), 'X');
    mutable_grid.set(TutorialCoord::new(0, 0), 'O');
    mutable_grid.set(TutorialCoord::new(2, 2), 'X');
    
    println!("Grid after modifications:");
    println!("{}", mutable_grid);
    
    // Attempt to modify out-of-bounds (safe failure)
    let modified = mutable_grid.set(TutorialCoord::new(5, 5), '#');
    println!("Attempt to set (5,5): {}", if modified { "Success" } else { "Failed (safe)" });
    
    // Section 3: Grid Traversal Patterns
    print_section("3. Grid Traversal Patterns");
    
    let demo_grid = TutorialGrid::new(4, 3, 0);
    
    // Pattern 1: Row-major traversal (most cache-friendly)
    println!("Row-major traversal (cache-friendly):");
    for y in 0..demo_grid.height() {
        for x in 0..demo_grid.width() {
            let coord = TutorialCoord::new(x, y);
            let value = demo_grid.get(coord).unwrap_or(&-1);
            print!("{:3} ", value);
        }
        println!();
    }
    
    // Pattern 2: Column-major traversal (less cache-friendly)
    println!("\nColumn-major traversal:");
    for x in 0..demo_grid.width() {
        print!("Column {}: ", x);
        for y in 0..demo_grid.height() {
            let coord = TutorialCoord::new(x, y);
            let value = demo_grid.get(coord).unwrap_or(&-1);
            print!("{} ", value);
        }
        println!();
    }
    
    // Section 4: Boundary Checking Strategies
    print_section("4. Boundary Checking Strategies");
    
    let boundary_grid = TutorialGrid::new(5, 5, '.');
    
    // Strategy 1: Check bounds before access
    fn safe_access_strategy1<T: Clone>(grid: &TutorialGrid<T>, coord: TutorialCoord) -> Option<T> {
        if grid.in_bounds(coord) {
            grid.get(coord).cloned()
        } else {
            None
        }
    }
    
    // Strategy 2: Let the get method handle bounds checking
    fn safe_access_strategy2<T: Clone>(grid: &TutorialGrid<T>, coord: TutorialCoord) -> Option<T> {
        grid.get(coord).cloned()
    }
    
    // Test both strategies
    let test_coords = vec![
        TutorialCoord::new(2, 2),  // valid
        TutorialCoord::new(0, 0),  // valid corner
        TutorialCoord::new(4, 4),  // valid corner
        TutorialCoord::new(5, 2),  // invalid x
        TutorialCoord::new(2, 5),  // invalid y
    ];
    
    for coord in test_coords {
        let result1 = safe_access_strategy1(&boundary_grid, coord);
        let result2 = safe_access_strategy2(&boundary_grid, coord);
        println!("Access {}: Strategy1={:?}, Strategy2={:?}", 
                coord, 
                result1.as_ref().map(|_| "Some"), 
                result2.as_ref().map(|_| "Some"));
    }
    
    // Section 5: Error Handling Patterns
    print_section("5. Error Handling Patterns");
    
    let error_demo_grid = TutorialGrid::new(3, 3, 42);
    
    // Pattern 1: Using unwrap_or for defaults
    let safe_get = |coord| {
        error_demo_grid.get(coord).cloned().unwrap_or(-1)
    };
    
    println!("Using unwrap_or with default value:");
    println!("Valid access (1,1): {}", safe_get(TutorialCoord::new(1, 1)));
    println!("Invalid access (5,5): {}", safe_get(TutorialCoord::new(5, 5)));
    
    // Pattern 2: Using match for explicit handling
    let handle_access = |coord| {
        match error_demo_grid.get(coord) {
            Some(value) => format!("Found: {}", value),
            None => format!("Coordinate {} is out of bounds", coord),
        }
    };
    
    println!("\nUsing match for explicit handling:");
    println!("{}", handle_access(TutorialCoord::new(0, 2)));
    println!("{}", handle_access(TutorialCoord::new(3, 0)));
    
    // Pattern 3: Using if let for concise code
    println!("\nUsing if let for concise handling:");
    for test_coord in [TutorialCoord::new(1, 1), TutorialCoord::new(5, 5)] {
        if let Some(value) = error_demo_grid.get(test_coord) {
            println!("✓ {} = {}", test_coord, value);
        } else {
            println!("✗ {} is invalid", test_coord);
        }
    }
    
    // Section 6: Performance Considerations
    print_section("6. Performance Considerations");
    
    let perf_grid = TutorialGrid::new(1000, 1000, 1);
    
    // Time different access patterns (simplified timing)
    use std::time::Instant;
    
    // Row-major access (cache-friendly)
    let start = Instant::now();
    let mut sum = 0;
    for y in 0..10 { // Small sample for demo
        for x in 0..10 {
            if let Some(value) = perf_grid.get(TutorialCoord::new(x, y)) {
                sum += value;
            }
        }
    }
    let row_major_time = start.elapsed();
    println!("Row-major access time: {:?} (sum: {})", row_major_time, sum);
    
    // Column-major access (less cache-friendly)
    let start = Instant::now();
    sum = 0;
    for x in 0..10 { // Small sample for demo
        for y in 0..10 {
            if let Some(value) = perf_grid.get(TutorialCoord::new(x, y)) {
                sum += value;
            }
        }
    }
    let col_major_time = start.elapsed();
    println!("Column-major access time: {:?} (sum: {})", col_major_time, sum);
    
    // Section 7: Practical Example - Grid Validation
    print_section("7. Practical Example: Grid Validation");
    
    let mut validation_grid = TutorialGrid::new(5, 5, 0);
    
    // Simulate placing some values
    let placements = vec![
        (TutorialCoord::new(1, 1), 5),
        (TutorialCoord::new(3, 2), 10),
        (TutorialCoord::new(0, 4), 15),
        (TutorialCoord::new(6, 1), 20), // Out of bounds
    ];
    
    println!("Attempting to place values:");
    for (coord, value) in placements {
        if validation_grid.set(coord, value) {
            println!("✓ Placed {} at {}", value, coord);
        } else {
            println!("✗ Failed to place {} at {} (out of bounds)", value, coord);
        }
    }
    
    println!("\nFinal grid state:");
    for y in 0..validation_grid.height() {
        for x in 0..validation_grid.width() {
            let coord = TutorialCoord::new(x, y);
            let value = validation_grid.get(coord).unwrap();
            print!("{:2} ", value);
        }
        println!();
    }
    
    // Validate all non-zero values
    println!("\nNon-zero values found:");
    let mut found_values = Vec::new();
    for y in 0..validation_grid.height() {
        for x in 0..validation_grid.width() {
            let coord = TutorialCoord::new(x, y);
            if let Some(&value) = validation_grid.get(coord) {
                if value != 0 {
                    found_values.push((coord, value));
                }
            }
        }
    }
    
    for (coord, value) in found_values {
        println!("  {} = {}", coord, value);
    }
    
    print_step_complete("Step 2: Grid Indexing & Safety");
    
    // Next Steps Preview
    println!("\n🔄 Next: Step 3 - Coordinate Systems & Navigation");
    println!("   Learn about neighbor finding, distance calculations, and 2D mathematics");
    println!("   Command: cargo run --example step3_coordinates");
    
    // Key Takeaways
    println!("\n📝 Key Takeaways from Step 2:");
    println!("   ✓ Safe access with Option<T> prevents runtime panics");
    println!("   ✓ Row-major traversal is more cache-friendly than column-major");
    println!("   ✓ Multiple error handling patterns: unwrap_or, match, if let");
    println!("   ✓ Always validate coordinates before modification operations");
    println!("   ✓ Performance matters - access patterns affect cache efficiency");
}

// Exercise for the Reader:
// 1. Implement a function that finds all coordinates containing a specific value
// 2. Create a "safe_swap" function that swaps values between two coordinates
// 3. Write a function that validates if a rectangular region is within bounds
// 4. Implement a "fill_region" function that fills a rectangular area with a value

// Design Questions to Consider:
// - When would you choose panicking access vs Option<T> returns?
// - How do different access patterns affect CPU cache performance?
// - What are the trade-offs between bounds checking and performance?
// - How would you implement iterator patterns for custom grid traversal?