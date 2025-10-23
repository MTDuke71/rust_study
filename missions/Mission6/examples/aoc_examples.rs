use mission6::{AocAlgorithms, AocGridParser, Coord, Direction, FloodFill, Grid, PathFinder};

fn main() {
    println!("=== Advent of Code Style Examples ===\n");

    // Example 1: Grid navigation (like AoC 2015 Day 3)
    santa_delivery_simulation();
    println!();

    // Example 2: BFS pathfinding (common AoC pattern)
    maze_escape_simulation();
    println!();

    // Example 3: Flood fill (like AoC region problems)
    island_counting_simulation();
    println!();

    // Example 4: Distance calculations (like AoC 2018 Day 6)
    voronoi_regions_simulation();
    println!();

    // Example 5: Conway's Game of Life style (AoC cellular automata)
    cellular_automaton_simulation();
}

fn santa_delivery_simulation() {
    println!("🎅 Santa Delivery Simulation (AoC 2015 Day 3 style)");

    // Simulate Santa following direction commands
    let commands = "^>v<^>v<^>v<^>v<";
    let mut santa_pos = Coord::new(50, 50); // Start in middle of large grid to allow movement
    let mut visited = std::collections::HashSet::new();

    // Track Santa's movement
    visited.insert(santa_pos);

    println!("Santa starts at {:?} (offset from origin)", santa_pos);
    println!("Following commands: {}", commands);

    for (step, ch) in commands.chars().enumerate() {
        if let Some(direction) = Direction::from_char(ch) {
            santa_pos = santa_pos.move_in_direction(direction, 1);
            visited.insert(santa_pos);

            if step < 10 {
                // Show first 10 steps
                println!("Step {}: {} -> {:?}", step + 1, ch, santa_pos);
            }
        }
    }

    println!("Santa visited {} unique houses", visited.len());

    // Simple visualization - just mark the bounding box
    let min_x = visited.iter().map(|c| c.x).min().unwrap();
    let max_x = visited.iter().map(|c| c.x).max().unwrap();
    let min_y = visited.iter().map(|c| c.y).min().unwrap();
    let max_y = visited.iter().map(|c| c.y).max().unwrap();

    println!(
        "Santa moved within bounds: x=[{}, {}], y=[{}, {}]",
        min_x, max_x, min_y, max_y
    );

    if max_x - min_x <= 20 && max_y - min_y <= 20 {
        // Only visualize if grid is small enough
        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;
        let mut grid = Grid::new(width, height, '.');

        // Mark visited houses
        for coord in &visited {
            let grid_x = coord.x - min_x;
            let grid_y = coord.y - min_y;
            grid[(grid_x, grid_y)] = '#';
        }

        // Mark starting position
        let start_x = 50 - min_x;
        let start_y = 50 - min_y;
        grid[(start_x, start_y)] = 'S';

        println!("\nVisited houses visualization (S=start, #=visited):");
        println!("{}", mission6::GridVisualizer::to_string(&grid));
    }
}

fn maze_escape_simulation() {
    println!("🏰 Maze Escape Simulation");

    // Create a more complex maze
    let maze_input = r"#########
#.......#
#.#####.#
#.....#.#
#####.#.#
#.....#.#
#.#####.#
#.......#
#########";

    let maze = AocGridParser::parse_char_grid(maze_input);
    println!("Maze layout:");
    println!("{}", mission6::GridVisualizer::to_string(&maze));

    // Find start and end positions
    let start = Coord::new(1, 1);
    let goal = Coord::new(7, 7);

    println!("\nFinding path from {:?} to {:?}", start, goal);

    // Use BFS to find shortest path
    if let Some(bfs_result) = PathFinder::bfs_detailed(&maze, start, goal, |&cell| cell != '#') {
        println!("BFS found path:");
        println!("  Length: {} steps", bfs_result.distance);
        println!("  Cells visited: {}", bfs_result.visited_count);

        // Visualize the path
        let path_viz = mission6::GridVisualizer::highlight_coords(&maze, &bfs_result.path, '*');
        println!("\nPath visualization (*=path, #=wall, .=floor):");
        println!("{}", path_viz);
    } else {
        println!("No path found!");
    }

    // Compare with A* algorithm
    if let Some(astar_result) = PathFinder::astar_detailed(
        &maze,
        start,
        goal,
        |&cell| cell != '#',
        |_| 1, // Uniform cost
    ) {
        println!("\nA* comparison:");
        println!("  Length: {} steps", astar_result.cost);
        println!("  Cells visited: {}", astar_result.visited_count);
    }
}

fn island_counting_simulation() {
    println!("🏝️  Island Counting Simulation");

    // Create a map with islands
    let map_input = r"..##......##..
.####....####.
..##......##..
..............
###...........
###...........
..............
....##........
....##........
..............
.......#####..
.......#####..";

    let map = AocGridParser::parse_char_grid(map_input);
    println!("Island map (#=land, .=water):");
    println!("{}", mission6::GridVisualizer::to_string(&map));

    // Count islands using flood fill
    let islands = FloodFill::find_all_regions(&map, &'#');
    println!("\nFound {} separate islands:", islands.len());

    for (i, island) in islands.iter().enumerate() {
        println!("  Island {}: {} cells", i + 1, island.len());
    }

    // Find the largest island
    if let Some(largest) = FloodFill::find_largest_region(&map, &'#') {
        println!("\nLargest island has {} cells", largest.len());

        // Visualize just the largest island
        let largest_viz = mission6::GridVisualizer::highlight_coords(&map, &largest, 'L');
        println!("Largest island highlighted (L):");
        println!("{}", largest_viz);
    }

    // Analyze each island's properties
    println!("\nIsland analysis:");
    for (i, island) in islands.iter().enumerate() {
        if let Some(&first_cell) = island.first() {
            let analysis = FloodFill::analyze_region_4(&map, first_cell, &'#');
            println!(
                "  Island {}: area={}, perimeter={}",
                i + 1,
                analysis.area,
                analysis.perimeter
            );
        }
    }
}

fn voronoi_regions_simulation() {
    println!("🎯 Voronoi Regions Simulation (Distance-based)");

    // Create a grid and place some "seed" points
    let width = 15;
    let height = 10;
    let mut grid = Grid::new(width, height, '.');

    let seeds = vec![
        Coord::new(3, 2),
        Coord::new(11, 2),
        Coord::new(7, 7),
        Coord::new(2, 8),
    ];

    println!("Seed points: {:?}", seeds);

    // Mark seeds on the grid
    let seed_chars = ['A', 'B', 'C', 'D'];
    for (i, &seed) in seeds.iter().enumerate() {
        grid[seed] = seed_chars[i];
    }

    // Calculate distance map from all seeds
    let distance_info = AocAlgorithms::multi_source_bfs(&grid, &seeds, |&cell| true);

    // Color the grid based on closest seed
    for (coord, &(distance, closest_seed)) in &distance_info {
        if !seeds.contains(coord) {
            // Find which seed this is closest to
            if let Some(seed_idx) = seeds.iter().position(|&s| s == closest_seed) {
                grid[*coord] = seed_chars[seed_idx].to_ascii_lowercase();
            }
        }
    }

    println!("\nVoronoi diagram (A-D = seeds, a-d = regions):");
    println!("{}", mission6::GridVisualizer::to_string(&grid));

    // Calculate some statistics
    let mut region_sizes = vec![0; seeds.len()];
    for &(_, closest_seed) in distance_info.values() {
        if let Some(seed_idx) = seeds.iter().position(|&s| s == closest_seed) {
            region_sizes[seed_idx] += 1;
        }
    }

    println!("\nRegion sizes:");
    for (i, &size) in region_sizes.iter().enumerate() {
        println!("  Seed {} ({}): {} cells", seed_chars[i], seeds[i], size);
    }

    // Find the centroid of all seeds
    if let Some(centroid) = AocAlgorithms::find_centroid(&seeds) {
        println!("\nCentroid of all seeds: {:?}", centroid);
        let total_distance = AocAlgorithms::total_manhattan_distance(&seeds);
        println!("Total distance between seeds: {}", total_distance);
    }
}

fn cellular_automaton_simulation() {
    println!("🧬 Cellular Automaton Simulation");

    // Create initial pattern (like a glider in Conway's Game of Life)
    let mut grid = Grid::new(8, 8, false);

    // Set up a simple oscillator pattern
    grid[(3, 2)] = true;
    grid[(3, 3)] = true;
    grid[(3, 4)] = true;

    grid[(2, 3)] = true;
    grid[(4, 3)] = true;

    println!("Initial state:");
    print_life_grid(&grid);

    // Simple cellular automaton rule:
    // Cell survives if it has 2-3 neighbors, born if it has exactly 3
    let life_rule = |grid: &Grid<bool>, coord: Coord, &current: &bool| -> bool {
        let live_neighbors = AocAlgorithms::count_adjacent(grid, coord, true, |&alive| alive);

        match (current, live_neighbors) {
            (true, 2) | (true, 3) => true, // Survive
            (false, 3) => true,            // Birth
            _ => false,                    // Die or stay dead
        }
    };

    // Run simulation for several steps
    let mut current_grid = grid;
    for step in 1..=5 {
        current_grid = AocAlgorithms::apply_cellular_automaton(&current_grid, life_rule);
        println!("\nStep {}:", step);
        print_life_grid(&current_grid);

        // Check for stability
        let live_cells: usize = current_grid
            .iter()
            .map(|&alive| if alive { 1 } else { 0 })
            .sum();
        println!("Live cells: {}", live_cells);
    }

    // Simple wrapper function for cycle detection
    fn life_step(grid: &Grid<bool>) -> Grid<bool> {
        let life_rule = |grid: &Grid<bool>, coord: Coord, &current: &bool| -> bool {
            let live_neighbors = AocAlgorithms::count_adjacent(grid, coord, true, |&alive| alive);

            match (current, live_neighbors) {
                (true, 2) | (true, 3) => true, // Survive
                (false, 3) => true,            // Birth
                _ => false,                    // Die or stay dead
            }
        };
        AocAlgorithms::apply_cellular_automaton(grid, life_rule)
    }

    // Try to detect cycles
    if let Some((cycle_start, cycle_length)) =
        AocAlgorithms::detect_cycle(current_grid, life_step, 20)
    {
        println!(
            "\nCycle detected: starts at step {}, length {}",
            cycle_start, cycle_length
        );
    } else {
        println!("\nNo cycle detected in first 20 steps");
    }
}

fn print_life_grid(grid: &Grid<bool>) {
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            print!("{}", if grid[(x, y)] { '#' } else { '.' });
        }
        println!();
    }
}
