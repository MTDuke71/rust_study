use mission6::{Grid, Coord, Direction, GridVisualizer};

fn main() {
    println!("=== Grid Data Structure Exploration ===\n");

    basic_grid_operations();
    println!();
    
    coordinate_system_demo();
    println!();
    
    direction_system_demo();
    println!();
    
    advanced_indexing();
    println!();
    
    grid_iteration_patterns();
    println!();
    
    grid_transformations();
}

fn basic_grid_operations() {
    println!("📐 Basic Grid Operations");
    
    // Create grids of different types
    let mut char_grid = Grid::new(4, 3, '.');
    let mut number_grid = Grid::new(3, 3, 0i32);
    let bool_grid = Grid::new(2, 2, true);
    
    println!("Created character grid (4x3): {} cells", char_grid.len());
    println!("Created number grid (3x3): {} cells", number_grid.len());  
    println!("Created boolean grid (2x2): {} cells", bool_grid.len());
    
    // Modify grids
    char_grid[(0, 0)] = 'S'; // Start
    char_grid[(3, 2)] = 'E'; // End
    char_grid[(1, 1)] = '#'; // Obstacle
    
    // Fill number grid with coordinates
    for y in 0..number_grid.height() {
        for x in 0..number_grid.width() {
            number_grid[(x, y)] = (x + y * number_grid.width()) as i32;
        }
    }
    
    println!("\nCharacter grid:");
    println!("{}", GridVisualizer::to_string(&char_grid));
    
    println!("\nNumber grid:");
    println!("{}", GridVisualizer::to_string(&number_grid));
    
    // Test bounds checking
    println!("\nBounds checking:");
    println!("Is (3,2) valid in 4x3 grid? {}", char_grid.in_bounds(Coord::new(3, 2)));
    println!("Is (4,2) valid in 4x3 grid? {}", char_grid.in_bounds(Coord::new(4, 2)));
    println!("Is (-1,0) valid? {}", char_grid.in_bounds(Coord::new(0, 0))); // Can't use negative with usize
}

fn coordinate_system_demo() {
    println!("🧭 Coordinate System");
    
    let center = Coord::new(5, 5);
    println!("Center coordinate: {:?}", center);
    
    // Test coordinate arithmetic (using signed coordinates internally)
    let (signed_x, signed_y) = center.to_signed();
    let offset_x = signed_x + 2;
    let offset_y = signed_y - 3;
    let new_pos = if offset_x >= 0 && offset_y >= 0 {
        Some(Coord::new(offset_x as usize, offset_y as usize))
    } else {
        None
    };
    
    if let Some(pos) = new_pos {
        println!("Center + (2, -3) = {:?}", pos);
        let difference = (pos.to_signed().0 - signed_x, pos.to_signed().1 - signed_y);
        println!("Difference back to center: {:?}", difference);
    } else {
        println!("Offset would result in negative coordinates");
    }
    
    // Distance calculations
    let corner1 = Coord::new(0, 0);
    let corner2 = Coord::new(3, 4);
    
    println!("\nDistance calculations between (0,0) and (3,4):");
    println!("Manhattan distance: {}", corner1.manhattan_distance(corner2));
    println!("Euclidean distance: {:.2}", (corner1.euclidean_distance_squared(corner2) as f64).sqrt());
    println!("Chebyshev distance: {}", corner1.chebyshev_distance(corner2));
    
    // Test coordinate as signed
    let signed = center.to_signed();
    println!("\nCenter as signed coordinates: ({}, {})", signed.0, signed.1);
}

fn direction_system_demo() {
    println!("🧭 Direction System");
    
    let start = Coord::new(10, 10);
    println!("Starting position: {:?}", start);
    
    // Test all cardinal directions
    println!("\nCardinal directions:");
    for direction in [Direction::North, Direction::East, Direction::South, Direction::West] {
        let new_pos = start.move_in_direction(direction, 3);
        let angle = direction.angle_degrees();
        println!("  {:?}: {:?} (angle: {}°)", direction, new_pos, angle);
    }
    
    // Test diagonal directions
    println!("\nDiagonal directions:");
    for direction in [Direction::NorthEast, Direction::SouthEast, Direction::SouthWest, Direction::NorthWest] {
        let new_pos = start.move_in_direction(direction, 2);
        let delta = direction.delta();
        println!("  {:?}: {:?} (delta: {:?})", direction, new_pos, delta);
    }
    
    // Test direction parsing
    println!("\nDirection parsing:");
    let test_chars = ['N', 'E', 'S', 'W', '^', '>', 'v', '<'];
    for ch in test_chars {
        if let Some(dir) = Direction::from_char(ch) {
            println!("  '{}' -> {:?}", ch, dir);
        }
    }
    
    // Test rotations
    let mut current_dir = Direction::North;
    println!("\nRotations from North:");
    for i in 0..8 {
        println!("  Step {}: {:?} ({}°)", i, current_dir, current_dir.angle_degrees());
        current_dir = current_dir.rotate_clockwise();
    }
}

fn advanced_indexing() {
    println!("🎯 Advanced Indexing");
    
    let mut grid = Grid::new(5, 5, 0);
    
    // Fill with interesting pattern
    for (coord, value) in grid.enumerate_mut() {
        *value = (coord.x + coord.y) as i32;
    }
    
    println!("Grid filled with coordinate sum:");
    println!("{}", GridVisualizer::to_string(&grid));
    
    // Test different indexing methods
    let center_coord = Coord::new(2, 2);
    println!("\nIndexing the center (2,2):");
    println!("Via tuple: grid[(2, 2)] = {}", grid[(2, 2)]);
    println!("Via Coord: grid[{:?}] = {}", center_coord, grid[center_coord]);
    
    // Test with coordinates generated from strings
    println!("\nParsing coordinate from string:");
    let coord_str = "(3, 1)";
    // In real code, you'd parse this properly
    let parsed_coord = Coord::new(3, 1);
    println!("Parsed '{}' as {:?}, value = {}", coord_str, parsed_coord, grid[parsed_coord]);
}

fn grid_iteration_patterns() {
    println!("🔄 Grid Iteration Patterns");
    
    let mut grid = Grid::new(4, 3, '.');
    
    // Set up a pattern
    grid[(1, 1)] = '#';
    grid[(2, 1)] = '#';
    grid[(1, 2)] = 'X';
    
    println!("Test grid:");
    println!("{}", GridVisualizer::to_string(&grid));
    
    // Basic iteration
    println!("\nAll non-empty cells:");
    for (coord, value) in grid.enumerate() {
        if *value != '.' {
            println!("  {:?}: '{}'", coord, value);
        }
    }
    
    // Row-wise iteration
    println!("\nRow-wise iteration:");
    for row_idx in 0..grid.height() {
        print!("Row {}: ", row_idx);
        for col_idx in 0..grid.width() {
            print!("'{}'", grid[(col_idx, row_idx)]);
        }
        println!();
    }
    
    // Column-wise iteration
    println!("\nColumn-wise iteration:");
    for col_idx in 0..grid.width() {
        print!("Col {}: ", col_idx);
        for row_idx in 0..grid.height() {
            print!("'{}'", grid[(col_idx, row_idx)]);
        }
        println!();
    }
    
    // Neighbor iteration
    let center = Coord::new(1, 1);
    println!("\nNeighbors of {:?}:", center);
    
    println!("4-connected:");
    for neighbor in center.neighbors_4_bounded(grid.width(), grid.height()) {
        println!("  {:?}: '{}'", neighbor, grid[neighbor]);
    }
    
    println!("8-connected:");
    for neighbor in center.neighbors_8_bounded(grid.width(), grid.height()) {
        println!("  {:?}: '{}'", neighbor, grid[neighbor]);
    }
}

fn grid_transformations() {
    println!("🔄 Grid Transformations");
    
    let mut original = Grid::new(3, 4, '.');
    
    // Create an asymmetric pattern to see transformations clearly
    original[(0, 0)] = '1';
    original[(1, 0)] = '2';
    original[(2, 0)] = '3';
    original[(0, 1)] = '4';
    original[(1, 2)] = 'X';
    original[(2, 3)] = '9';
    
    println!("Original grid (3x4):");
    println!("{}", GridVisualizer::to_string(&original));
    
    // Clone and modify
    let mut modified = original.clone();
    modified[(1, 1)] = 'M';
    
    println!("\nModified clone:");
    println!("{}", GridVisualizer::to_string(&modified));
    
    // Create a subgrid pattern
    let mut large_grid = Grid::new(7, 5, '.');
    
    // Fill corners
    large_grid[(0, 0)] = 'A';
    large_grid[(6, 0)] = 'B';
    large_grid[(0, 4)] = 'C';
    large_grid[(6, 4)] = 'D';
    large_grid[(3, 2)] = '+'; // Center
    
    println!("\nLarge grid with corner markers:");
    println!("{}", GridVisualizer::to_string_with_coords(&large_grid));
    
    // Highlight specific coordinates
    let important_coords = vec![
        Coord::new(0, 0),
        Coord::new(6, 0),
        Coord::new(3, 2),
        Coord::new(0, 4),
        Coord::new(6, 4),
    ];
    
    println!("\nHighlighted important coordinates (*):");
    println!("{}", GridVisualizer::highlight_coords(&large_grid, &important_coords, '*'));
}