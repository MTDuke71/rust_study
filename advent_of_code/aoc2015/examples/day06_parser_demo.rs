//! Day 6 Parser Demonstration
//!
//! This example demonstrates the separated parser functions for Day 6 light commands.
//! Shows how parsing is now separated from grid operations for better code organization.

use aoc2015::solver::day06::*;
use mission6::Grid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎄 DAY 6 PARSER DEMONSTRATION");
    println!("============================\n");

    // Demonstrate parsing individual commands
    println!("🔍 Parsing Individual Commands:");
    let commands = [
        "turn on 0,0 through 999,999",
        "turn off 499,499 through 500,500",
        "toggle 0,0 through 999,0",
    ];

    for (i, command_str) in commands.iter().enumerate() {
        println!("\n{}. Command: \"{}\"", i + 1, command_str);

        let parsed = parse_light_command(command_str)?;
        println!("   Action: {:?}", parsed.action);
        println!(
            "   Region: ({},{}) to ({},{})",
            parsed.x1, parsed.y1, parsed.x2, parsed.y2
        );

        let area = (parsed.x2 - parsed.x1 + 1) * (parsed.y2 - parsed.y1 + 1);
        println!("   Affects: {} lights", area);
    }

    // Demonstrate parser with grid operations
    println!("\n🗂️ Parser + Grid Integration:");
    let mut grid = Grid::new(5, 5, false);

    println!("\nStep 1: Turn on center region");
    apply_light_command(&mut grid, "turn on 1,1 through 3,3")?;
    print_grid(&grid);

    println!("\nStep 2: Toggle corner");
    apply_light_command(&mut grid, "toggle 0,0 through 0,0")?;
    print_grid(&grid);

    println!("\nStep 3: Turn off middle column");
    apply_light_command(&mut grid, "turn off 2,0 through 2,4")?;
    print_grid(&grid);

    // Demonstrate brightness operations
    println!("\n🌟 Brightness Parser Integration:");
    let mut brightness_grid = Grid::new(3, 3, 0u32);

    println!("\nInitial state (all brightness 0):");
    print_brightness_grid(&brightness_grid);

    println!("\nTurn on center (brightness +1):");
    apply_brightness_command(&mut brightness_grid, "turn on 1,1 through 1,1")?;
    print_brightness_grid(&brightness_grid);

    println!("\nToggle all (brightness +2):");
    apply_brightness_command(&mut brightness_grid, "toggle 0,0 through 2,2")?;
    print_brightness_grid(&brightness_grid);

    println!("\nTurn off center (brightness -1, min 0):");
    apply_brightness_command(&mut brightness_grid, "turn off 1,1 through 1,1")?;
    apply_brightness_command(&mut brightness_grid, "turn off 1,1 through 1,1")?;
    apply_brightness_command(&mut brightness_grid, "turn off 1,1 through 1,1")?; // Should saturate at 0
    print_brightness_grid(&brightness_grid);

    // Demonstrate error handling
    println!("\n❌ Error Handling:");
    match parse_light_command("invalid command") {
        Ok(_) => println!("ERROR: Should have failed!"),
        Err(e) => println!("✅ Correctly rejected invalid command: {}", e),
    }

    match parse_light_command("turn purple 0,0 through 1,1") {
        Ok(_) => println!("ERROR: Should have failed!"),
        Err(e) => println!("✅ Correctly rejected invalid action: {}", e),
    }

    println!("\n🎉 Parser demonstration complete!");
    println!("📚 Benefits of separated parser:");
    println!("  • Reusable across Grid and HashMap implementations");
    println!("  • Easier to test parsing logic independently");
    println!("  • Clear separation of concerns");
    println!("  • Better error handling and validation");
    println!("  • Type-safe command representation");

    Ok(())
}

/// Helper function to print a boolean grid
fn print_grid(grid: &Grid<bool>) {
    println!("Grid state:");
    for y in 0..grid.height() {
        print!("  ");
        for x in 0..grid.width() {
            let coord = (x, y).into();
            let symbol = match grid.get(coord) {
                Some(true) => "🟡",  // On
                Some(false) => "⚫", // Off
                None => "❓",        // Error
            };
            print!("{}", symbol);
        }
        println!();
    }
}

/// Helper function to print a brightness grid
fn print_brightness_grid(grid: &Grid<u32>) {
    println!("Brightness grid:");
    for y in 0..grid.height() {
        print!("  ");
        for x in 0..grid.width() {
            let coord = (x, y).into();
            match grid.get(coord) {
                Some(brightness) => print!("{:2} ", brightness),
                None => print!("?? "),
            };
        }
        println!();
    }
}
