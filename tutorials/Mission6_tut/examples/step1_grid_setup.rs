// Step 1: Grid Setup & Creation
// Tutorial Day 1 - Aligns with Mission 6 Setup & REQ-1 (Grid Creation and Indexing)
//
// Learning Objectives:
// - Understand grid memory representation (row-major ordering)
// - Create grids with different data types and default values
// - Implement safe bounds checking
// - Practice generic type parameters with Grid<T>
//
// This tutorial step builds toward Mission6 REQ-1:
// "The system shall provide a Grid<T> type that stores elements in a 2D array
//  with configurable width and height, supporting O(1) indexing by coordinates."

use mission6_tut::tutorial_helpers::{
    print_section, print_step_complete, TutorialCoord, TutorialGrid,
};

fn main() {
    println!("=== Mission 6 Tutorial - Step 1: Grid Setup & Creation ===");
    println!("Day 1 Focus: Understanding grid representation and type safety\n");

    // Section 1: Basic Grid Creation
    print_section("1. Basic Grid Creation");

    // Create a simple character grid for a game board
    let mut char_grid = TutorialGrid::new(5, 5, '.');
    println!("Created 5x5 character grid:");
    println!("{}", char_grid);

    // Create an integer grid for storing values
    let int_grid = TutorialGrid::new(3, 4, 0);
    println!("\nCreated 3x4 integer grid:");
    println!("{}", int_grid);

    // Key Learning: Grids are generic over any type T
    // Generic types allow the same grid structure to hold different data types

    // Section 2: Understanding Grid Indexing
    print_section("2. Grid Indexing and Coordinates");

    // Set some values using coordinate access
    let center = TutorialCoord::new(2, 2);
    char_grid.set(center, '#');
    println!("Set center position to '#':");
    println!("{}", char_grid);

    // Try to access values safely
    if let Some(value) = char_grid.get(center) {
        println!("Value at center {}: {}", center, value);
    }

    // Demonstrate bounds checking
    let out_of_bounds = TutorialCoord::new(10, 10);
    match char_grid.get(out_of_bounds) {
        Some(value) => println!("Found value: {}", value),
        None => println!("Coordinate {} is out of bounds", out_of_bounds),
    }

    // Section 3: Different Grid Types
    print_section("3. Generic Grid Types");

    // Boolean grid for obstacle maps
    let mut obstacle_grid = TutorialGrid::new(4, 4, false);
    obstacle_grid.set(TutorialCoord::new(1, 1), true);
    obstacle_grid.set(TutorialCoord::new(2, 1), true);
    obstacle_grid.set(TutorialCoord::new(1, 2), true);

    println!("Boolean obstacle grid (true = obstacle):");
    for y in 0..obstacle_grid.height() {
        for x in 0..obstacle_grid.width() {
            let coord = TutorialCoord::new(x, y);
            let symbol = if *obstacle_grid.get(coord).unwrap() {
                "#"
            } else {
                "."
            };
            print!("{}", symbol);
        }
        println!();
    }

    // String grid for storing labels
    let mut label_grid = TutorialGrid::new(3, 3, String::from(""));
    label_grid.set(TutorialCoord::new(1, 1), String::from("START"));
    label_grid.set(TutorialCoord::new(2, 2), String::from("END"));

    println!("\nString label grid:");
    for y in 0..label_grid.height() {
        for x in 0..label_grid.width() {
            let coord = TutorialCoord::new(x, y);
            let label = label_grid.get(coord).unwrap();
            let display = if label.is_empty() {
                "."
            } else {
                label.as_str()
            };
            print!("{:>5} ", display);
        }
        println!();
    }

    // Section 4: Memory Layout Understanding
    print_section("4. Memory Layout and Performance");

    // Demonstrate row-major ordering
    let mut numbered_grid = TutorialGrid::new(4, 3, 0);
    let mut counter = 1;

    // Fill in row-major order (left-to-right, top-to-bottom)
    for y in 0..numbered_grid.height() {
        for x in 0..numbered_grid.width() {
            numbered_grid.set(TutorialCoord::new(x, y), counter);
            counter += 1;
        }
    }

    println!("Grid filled in row-major order (how data is stored in memory):");
    for y in 0..numbered_grid.height() {
        for x in 0..numbered_grid.width() {
            let coord = TutorialCoord::new(x, y);
            print!("{:2} ", numbered_grid.get(coord).unwrap());
        }
        println!();
    }

    // Section 5: Bounds Checking Patterns
    print_section("5. Safe Bounds Checking");

    let test_grid = TutorialGrid::new(3, 3, 42);

    // Different coordinate checking patterns
    let test_coords = vec![
        TutorialCoord::new(1, 1), // Valid: center
        TutorialCoord::new(0, 0), // Valid: top-left corner
        TutorialCoord::new(2, 2), // Valid: bottom-right corner
        TutorialCoord::new(3, 1), // Invalid: x out of bounds
        TutorialCoord::new(1, 3), // Invalid: y out of bounds
        TutorialCoord::new(5, 5), // Invalid: both out of bounds
    ];

    for coord in test_coords {
        if test_grid.in_bounds(coord) {
            let value = test_grid.get(coord).unwrap();
            println!("✅ {} is valid, value: {}", coord, value);
        } else {
            println!("❌ {} is out of bounds", coord);
        }
    }

    // Section 6: Practical Example - Simple Game Board
    print_section("6. Practical Example: Tic-Tac-Toe Board");

    let mut game_board = TutorialGrid::new(3, 3, ' ');

    // Make some moves
    game_board.set(TutorialCoord::new(0, 0), 'X');
    game_board.set(TutorialCoord::new(1, 1), 'O');
    game_board.set(TutorialCoord::new(2, 2), 'X');
    game_board.set(TutorialCoord::new(0, 2), 'O');

    println!("Tic-tac-toe game board:");
    println!(
        " {} | {} | {} ",
        game_board.get(TutorialCoord::new(0, 0)).unwrap(),
        game_board.get(TutorialCoord::new(1, 0)).unwrap(),
        game_board.get(TutorialCoord::new(2, 0)).unwrap()
    );
    println!("---|---|---");
    println!(
        " {} | {} | {} ",
        game_board.get(TutorialCoord::new(0, 1)).unwrap(),
        game_board.get(TutorialCoord::new(1, 1)).unwrap(),
        game_board.get(TutorialCoord::new(2, 1)).unwrap()
    );
    println!("---|---|---");
    println!(
        " {} | {} | {} ",
        game_board.get(TutorialCoord::new(0, 2)).unwrap(),
        game_board.get(TutorialCoord::new(1, 2)).unwrap(),
        game_board.get(TutorialCoord::new(2, 2)).unwrap()
    );

    print_step_complete("Step 1: Grid Setup & Creation");

    // Next Steps Preview
    println!("\n🔄 Next: Step 2 - Grid Indexing & Safety");
    println!("   Learn about iterator patterns, row/column access, and memory-safe operations");
    println!("   Command: cargo run --example step2_grid_indexing");

    // Key Takeaways
    println!("\n📝 Key Takeaways from Step 1:");
    println!("   ✓ Grids use generic types Grid<T> to store any data type");
    println!("   ✓ Row-major memory layout: data[y * width + x]");
    println!("   ✓ Always check bounds before accessing grid elements");
    println!("   ✓ Coordinates are (x, y) pairs representing column, row positions");
    println!("   ✓ Safe access patterns return Option<T> to handle out-of-bounds gracefully");
}

// Exercise for the Reader:
// 1. Create a 10x10 grid of integers and fill it with a checkerboard pattern (alternating 0 and 1)
// 2. Create a grid of custom structs (e.g., Player { name: String, health: i32 })
// 3. Implement a function that counts the number of non-default values in a grid
// 4. Try creating very large grids (1000x1000) and observe memory usage

// Design Questions to Consider:
// - Why is row-major ordering important for performance?
// - When would you choose Vec<Vec<T>> vs Vec<T> with coordinate mapping?
// - How does bounds checking impact performance in tight loops?
// - What are the trade-offs between Option<T> returns vs panicking on invalid access?
