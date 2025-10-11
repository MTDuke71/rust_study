fn main() {
    println!("🗺️  2D Grid Access Patterns in Rust");
    println!("===================================\n");

    // Create a 2D grid (Vec<Vec<char>>)
    let grid: Vec<Vec<char>> = vec![
        vec!['#', '.', '#', '.'],
        vec!['.', '#', '.', '#'],
        vec!['#', '.', 'S', '.'],
        vec!['.', '#', '.', 'E'],
    ];

    println!("📋 Grid (4x4):");
    for (row, line) in grid.iter().enumerate() {
        print!("Row {}: ", row);
        for &cell in line {
            print!("{} ", cell);
        }
        println!();
    }
    println!();

    println!("🎯 Access Methods:");
    println!("==================");

    // METHOD 1: Direct indexing (panics if out of bounds)
    println!("🔸 Method 1: Direct Indexing (unsafe)");
    let cell = grid[2][2]; // Row 2, Column 2
    println!("  grid[2][2] = '{}'", cell);
    println!("  ⚠️  Panics if row or column out of bounds!");
    println!();

    // METHOD 2: Safe access with get() - returns Option
    println!("🔹 Method 2: Safe Access with get()");

    // Safe access - valid coordinates
    if let Some(row) = grid.get(2) {
        if let Some(&cell) = row.get(2) {
            println!("  grid.get(2).and_then(|r| r.get(2)) = Some('{}')", cell);
        }
    }

    // Safe access - invalid coordinates
    match grid.get(10) {
        Some(row) => match row.get(5) {
            Some(&cell) => println!("  Found: {}", cell),
            None => println!("  Column 5 out of bounds"),
        },
        None => println!("  grid.get(10) = None (row out of bounds)"),
    }
    println!();

    // METHOD 3: Helper function for cleaner syntax
    println!("🔸 Method 3: Helper Function");

    fn get_cell(grid: &[Vec<char>], row: usize, col: usize) -> Option<char> {
        grid.get(row)?.get(col).copied()
    }

    println!("  get_cell(&grid, 3, 3) = {:?}", get_cell(&grid, 3, 3));
    println!("  get_cell(&grid, 10, 10) = {:?}", get_cell(&grid, 10, 10));
    println!();

    // METHOD 4: Bounds checking before access
    println!("🔹 Method 4: Manual Bounds Checking");

    fn safe_get(grid: &[Vec<char>], row: usize, col: usize) -> Option<char> {
        if row < grid.len() && col < grid[row].len() {
            Some(grid[row][col])
        } else {
            None
        }
    }

    println!("  safe_get(&grid, 1, 1) = {:?}", safe_get(&grid, 1, 1));
    println!(
        "  safe_get(&grid, 100, 100) = {:?}",
        safe_get(&grid, 100, 100)
    );
    println!();

    // METHOD 5: Using coordinates as (x, y) tuples
    println!("🔸 Method 5: Coordinate-Based Access");

    fn get_at_coord(grid: &[Vec<char>], coord: (usize, usize)) -> Option<char> {
        let (x, y) = coord;
        // Note: grid[y][x] because grid is row-major (y=row, x=col)
        grid.get(y)?.get(x).copied()
    }

    let coord = (2, 1); // x=2, y=1
    println!(
        "  get_at_coord(&grid, (2, 1)) = {:?}",
        get_at_coord(&grid, coord)
    );
    println!();

    // METHOD 6: Advanced - with direction/neighbor checking
    println!("🔹 Method 6: Neighbor Access (AoC Pattern)");

    const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    fn get_neighbors(grid: &[Vec<char>], row: usize, col: usize) -> Vec<(char, usize, usize)> {
        let mut neighbors = Vec::new();

        for (dr, dc) in DIRECTIONS {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;

            if new_row >= 0 && new_col >= 0 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if let Some(row_vec) = grid.get(new_row) {
                    if let Some(&cell) = row_vec.get(new_col) {
                        neighbors.push((cell, new_row, new_col));
                    }
                }
            }
        }
        neighbors
    }

    let neighbors = get_neighbors(&grid, 2, 2); // Around 'S'
    println!("  Neighbors of 'S' at (2,2):");
    for (cell, r, c) in neighbors {
        println!("    '{}' at ({}, {})", cell, r, c);
    }
    println!();

    // METHOD 7: Iterator-based searching
    println!("🔸 Method 7: Iterator-Based Access");

    // Find all positions of a specific character
    let start_positions: Vec<(usize, usize)> =
        grid.iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.iter().enumerate().filter_map(move |(col, &cell)| {
                    if cell == 'S' {
                        Some((row, col))
                    } else {
                        None
                    }
                })
            })
            .collect();

    println!("  Start positions ('S'): {:?}", start_positions);

    // Find first occurrence
    let end_pos = grid.iter().enumerate().find_map(|(row, line)| {
        line.iter().enumerate().find_map(
            |(col, &cell)| {
                if cell == 'E' {
                    Some((row, col))
                } else {
                    None
                }
            },
        )
    });

    println!("  End position ('E'): {:?}", end_pos);
    println!();

    println!("🎮 Common AoC Patterns:");
    println!("=======================");

    println!("✅ Safe grid access:");
    println!("  if let Some(cell) = grid.get(row)?.get(col) {{ ... }}");
    println!();

    println!("✅ Bounds-checked access:");
    println!("  if row < grid.len() && col < grid[row].len() {{");
    println!("      let cell = grid[row][col];");
    println!("  }}");
    println!();

    println!("✅ Neighbor iteration:");
    println!("  for (dr, dc) in DIRECTIONS {{");
    println!("      let (nr, nc) = (row + dr, col + dc);");
    println!("      if let Some(&cell) = grid.get(nr)?.get(nc) {{ ... }}");
    println!("  }}");
    println!();

    println!("⚠️  Common Pitfalls:");
    println!("===================");
    println!("❌ grid[row][col] - panics on out-of-bounds");
    println!("❌ Confusing (x,y) vs (row,col) - x=col, y=row!");
    println!("❌ Signed/unsigned conversion issues with directions");
    println!("✅ Always use safe access in competitive programming!");
}
