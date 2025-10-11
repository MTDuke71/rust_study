use mission6::{AocGridParser, Coord, Direction, FloodFill, Grid, GridVisualizer, PathFinder};

fn main() {
    println!("=== Mission 6: Grids & 2D Arrays Demo ===\n");

    // REQ-1: Grid creation and basic operations
    println!("📐 REQ-1: Grid Creation and Indexing");
    demo_grid_creation();
    println!();

    // REQ-2: Coordinate system and navigation
    println!("🧭 REQ-2: Coordinate Navigation");
    demo_coordinate_navigation();
    println!();

    // REQ-3: Pathfinding algorithms
    println!("🗺️  REQ-3: Pathfinding Algorithms");
    demo_pathfinding();
    println!();

    // REQ-4: AoC utilities
    println!("🎄 REQ-4: AoC Utilities");
    demo_aoc_utilities();
    println!();

    // REQ-5: Performance characteristics
    println!("⚡ REQ-5: Performance Characteristics");
    demo_performance();
    println!();

    // REQ-6: Integration with Mission 5 patterns
    println!("🔗 REQ-6: Integration Patterns");
    demo_integration_patterns();
    println!();

    println!("✅ All Mission 6 requirements demonstrated successfully!");
}

fn demo_grid_creation() {
    // Create different types of grids
    let mut char_grid = Grid::new(5, 5, '.');
    let mut int_grid = Grid::new(3, 3, 0i32);

    println!("Created 5x5 char grid with '.' default");
    println!("Created 3x3 int grid with 0 default");

    // Test indexing
    char_grid[(2, 2)] = '#';
    int_grid[(1, 1)] = 42;

    println!("Set center of char grid to '#'");
    println!("Set center of int grid to 42");

    // Test coordinate indexing
    let coord = Coord::new(2, 2);
    println!("Value at {:?}: {}", coord, char_grid[coord]);

    // Test bounds checking
    println!(
        "Is (4,4) in bounds for 5x5 grid? {}",
        char_grid.in_bounds(Coord::new(4, 4))
    );
    println!(
        "Is (5,5) in bounds for 5x5 grid? {}",
        char_grid.in_bounds(Coord::new(5, 5))
    );
}

fn demo_coordinate_navigation() {
    let center = Coord::new(10, 10);
    println!("Starting at center: {:?}", center);

    // Test direction movement
    let north = center.move_in_direction(Direction::North, 3);
    let east = center.move_in_direction(Direction::East, 2);
    let northeast = center.move_in_direction(Direction::NorthEast, 1);

    println!("Move 3 north: {:?}", north);
    println!("Move 2 east: {:?}", east);
    println!("Move 1 northeast: {:?}", northeast);

    // Test distance calculations
    println!(
        "Manhattan distance to north point: {}",
        center.manhattan_distance(north)
    );
    println!(
        "Euclidean distance to northeast point: {:.2}",
        (center.euclidean_distance_squared(northeast) as f64).sqrt()
    );

    // Test neighbors
    let neighbors_4: Vec<_> = center.neighbors_4().collect();
    let neighbors_8: Vec<_> = center.neighbors_8().collect();

    println!("4-connected neighbors: {} total", neighbors_4.len());
    println!("8-connected neighbors: {} total", neighbors_8.len());

    // Test bounded neighbors (within grid)
    let bounded_4: Vec<_> = center.neighbors_4_bounded(15, 15).collect();
    println!(
        "4-connected neighbors within 15x15 grid: {} total",
        bounded_4.len()
    );
}

fn demo_pathfinding() {
    // Create a maze
    let mut maze = Grid::new(7, 7, '.');

    // Add obstacles
    for y in 1..6 {
        maze[(3, y)] = '#';
    }
    maze[(3, 3)] = '.'; // Gap in the wall

    println!("Created 7x7 maze:");
    println!("{}", GridVisualizer::to_string(&maze));
    println!();

    let start = Coord::new(0, 0);
    let goal = Coord::new(6, 6);

    // BFS pathfinding
    if let Some(path) = PathFinder::bfs(&maze, start, goal, |&cell| cell != '#') {
        println!("BFS found path with {} steps", path.len() - 1);
        let path_display = GridVisualizer::highlight_coords(&maze, &path, '*');
        println!("Path visualization (* = path):");
        println!("{}", path_display);
    } else {
        println!("No path found with BFS");
    }

    println!();

    // A* pathfinding with uniform costs
    if let Some(path) = PathFinder::astar(
        &maze,
        start,
        goal,
        |&cell| cell != '#',
        |_| 1, // Uniform cost
    ) {
        println!("A* found path with {} steps", path.len() - 1);
    }

    // Distance map
    let distances = PathFinder::distance_map(&maze, start, |&cell| cell != '#');
    println!("Distance from start to goal: {:?}", distances.get(&goal));
}

fn demo_aoc_utilities() {
    // Parse AoC-style input
    let aoc_input = r"..#..#..#.
.......#..
#........#
..#.......
.#......#.
#.##......
..#...#.##
..##...##.
";

    println!("Parsing AoC grid input:");
    let grid = AocGridParser::parse_char_grid(aoc_input);
    println!("Parsed {}x{} grid", grid.width(), grid.height());

    // Find all special positions
    let obstacles = AocGridParser::find_all(&grid, &'#');
    println!("Found {} obstacles", obstacles.len());

    // Demonstrate flood fill
    let mut fill_grid = grid.clone();
    let filled = FloodFill::fill_4(&mut fill_grid, Coord::new(0, 0), '.', 'O');
    println!("Flood fill filled {} cells", filled.len());

    // Analyze regions
    let result = FloodFill::analyze_region_4(&fill_grid, Coord::new(0, 0), &'O');
    println!(
        "Region analysis: {} cells, {} perimeter",
        result.area, result.perimeter
    );

    // Find all regions of obstacles
    let obstacle_regions = FloodFill::find_all_regions(&grid, &'#');
    println!("Found {} separate obstacle regions", obstacle_regions.len());

    // Display with coordinates
    println!("\nGrid with coordinates:");
    println!("{}", GridVisualizer::to_string_with_coords(&grid));
}

fn demo_performance() {
    use std::time::Instant;

    let size = 1000;
    println!("Creating {}x{} grid...", size, size);

    let start = Instant::now();
    let grid = Grid::new(size, size, 0u32);
    let creation_time = start.elapsed();

    println!("Grid creation time: {:?}", creation_time);
    println!("Grid size: {} cells", grid.len());

    // Test iteration performance
    let start = Instant::now();
    let sum: u32 = grid.iter().sum();
    let iter_time = start.elapsed();

    println!("Iteration time: {:?} (sum: {})", iter_time, sum);

    // Test coordinate generation performance
    let start = Instant::now();
    let coord_count = grid.enumerate().count();
    let coord_time = start.elapsed();

    println!(
        "Coordinate enumeration time: {:?} ({} coords)",
        coord_time, coord_count
    );

    // Memory usage approximation
    let memory_bytes = grid.len() * std::mem::size_of::<u32>();
    println!(
        "Approximate memory usage: {} bytes ({:.2} MB)",
        memory_bytes,
        memory_bytes as f64 / 1_000_000.0
    );
}

fn demo_integration_patterns() {
    use std::collections::{HashMap, HashSet};

    // Create a grid with some interesting data
    let mut grid = Grid::new(5, 5, '.');
    grid[(1, 1)] = 'A';
    grid[(3, 1)] = 'B';
    grid[(1, 3)] = 'A';
    grid[(3, 3)] = 'C';

    println!("Grid with labeled points:");
    println!("{}", GridVisualizer::to_string(&grid));
    println!();

    // Use Coord as HashMap key (Mission 5 integration)
    let mut coord_labels: HashMap<Coord, String> = HashMap::new();

    for (coord, value) in grid.enumerate() {
        if *value != '.' {
            coord_labels.insert(coord, format!("Point {}", value));
        }
    }

    println!("Coordinate labels in HashMap:");
    for (coord, label) in &coord_labels {
        println!("  {:?}: {}", coord, label);
    }

    // Use Coord in HashSet for visited tracking
    let mut visited: HashSet<Coord> = HashSet::new();
    let start = Coord::new(0, 0);

    // Simulate BFS traversal tracking
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(start);
    visited.insert(start);

    let mut steps = 0;
    while let Some(current) = queue.pop_front() {
        steps += 1;
        if steps > 10 {
            break;
        } // Limit for demo

        for neighbor in current.neighbors_4_bounded(grid.width(), grid.height()) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }

    println!("\nBFS simulation visited {} coordinates", visited.len());

    // Distance calculations for all labeled points
    let labeled_coords: Vec<_> = coord_labels.keys().copied().collect();
    println!("\nDistances between labeled points:");
    for i in 0..labeled_coords.len() {
        for j in (i + 1)..labeled_coords.len() {
            let coord1 = labeled_coords[i];
            let coord2 = labeled_coords[j];
            let distance = coord1.manhattan_distance(coord2);
            println!("  {:?} to {:?}: {} steps", coord1, coord2, distance);
        }
    }
}
