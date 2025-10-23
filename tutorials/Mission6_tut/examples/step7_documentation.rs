// Step 7: Documentation & Integration
// Tutorial Day 7 - Aligns with Mission 6 Documentation & Examples & All REQs
//
// Learning Objectives:
// - Create comprehensive API documentation
// - Implement complete working examples and integration patterns
// - Practice professional documentation standards
// - Master integration with other Rust data structures
//
// This tutorial step integrates all Mission6 requirements (REQ-1 through REQ-6):
// "Complete the learning journey with professional documentation, integration
//  examples, and best practices for using grid systems in real applications."

use mission6_tut::tutorial_helpers::{
    print_section, print_step_complete, TutorialCoord, TutorialGrid,
};
use std::collections::{HashMap, HashSet};
use std::fmt;

fn main() {
    println!("=== Mission 6 Tutorial - Step 7: Documentation & Integration ===");
    println!("Day 7 Focus: Professional documentation, integration patterns, and best practices\n");

    // Section 1: Professional Documentation Standards
    print_section("1. Professional Documentation Standards");

    println!("Complete grid documentation should include:");
    println!("• Module-level documentation (//!)");
    println!("• Function-level documentation (///)");
    println!("• Examples and doctests");
    println!("• Performance characteristics (Big-O notation)");
    println!("• Error conditions and safety guarantees");
    println!("• Integration examples with other systems");

    // Example of well-documented function
    demonstrate_documentation_example();

    // Section 2: Integration with Standard Collections
    print_section("2. Integration with Standard Collections");

    println!("Grids work seamlessly with Rust standard collections:");

    // HashMap integration for coordinate mapping
    let mut coord_labels = HashMap::new();
    coord_labels.insert(TutorialCoord::new(1, 1), "Player Start");
    coord_labels.insert(TutorialCoord::new(8, 8), "Boss Room");
    coord_labels.insert(TutorialCoord::new(4, 2), "Treasure");
    coord_labels.insert(TutorialCoord::new(7, 3), "Secret Door");

    let game_grid = TutorialGrid::new(10, 10, '.');

    println!("\nGame map with labeled locations:");
    for y in 0..game_grid.height() {
        for x in 0..game_grid.width() {
            let coord = TutorialCoord::new(x, y);
            if coord_labels.contains_key(&coord) {
                print!("* ");
            } else {
                print!(". ");
            }
        }
        println!();
    }

    println!("\nSpecial locations:");
    for (coord, label) in &coord_labels {
        println!("  {} -> {}", coord, label);
    }

    // HashSet integration for visited tracking
    let mut visited_coords = HashSet::new();
    visited_coords.insert(TutorialCoord::new(1, 1));
    visited_coords.insert(TutorialCoord::new(2, 1));
    visited_coords.insert(TutorialCoord::new(2, 2));

    println!("\nPlayer has visited {} locations:", visited_coords.len());
    for coord in &visited_coords {
        println!("  {}", coord);
    }

    // Section 3: Complete Game Example - Snake Game
    print_section("3. Complete Game Example - Snake Game");

    let mut snake_game = SnakeGame::new(12, 8);

    println!("Snake Game Implementation:");
    println!("{}", snake_game);

    // Simulate some moves
    let moves = vec![
        Direction::Right,
        Direction::Right,
        Direction::Down,
        Direction::Down,
        Direction::Left,
        Direction::Left,
    ];

    println!("\nSimulating moves: Right, Right, Down, Down, Left, Left");
    for direction in moves {
        if snake_game.make_move(direction) {
            println!("Move successful!");
        } else {
            println!("Game over!");
            break;
        }
    }

    println!("\nFinal game state:");
    println!("{}", snake_game);
    println!("Score: {}", snake_game.score());

    // Section 4: Pathfinding Integration Example
    print_section("4. Pathfinding Integration Example");

    println!("Real-world pathfinding scenario - Robot navigation:");

    let mut robot_world = TutorialGrid::new(15, 10, '.');

    // Add obstacles (furniture in a room)
    let obstacles = vec![
        // Table
        TutorialCoord::new(4, 3),
        TutorialCoord::new(5, 3),
        TutorialCoord::new(6, 3),
        TutorialCoord::new(4, 4),
        TutorialCoord::new(5, 4),
        TutorialCoord::new(6, 4),
        // Couch
        TutorialCoord::new(9, 2),
        TutorialCoord::new(10, 2),
        TutorialCoord::new(11, 2),
        TutorialCoord::new(9, 3),
        TutorialCoord::new(10, 3),
        TutorialCoord::new(11, 3),
        // Bookshelf
        TutorialCoord::new(2, 7),
        TutorialCoord::new(3, 7),
        TutorialCoord::new(4, 7),
        TutorialCoord::new(5, 7),
        TutorialCoord::new(6, 7),
    ];

    for obstacle in &obstacles {
        robot_world.set(*obstacle, '#');
    }

    let robot_start = TutorialCoord::new(1, 1);
    let robot_goal = TutorialCoord::new(13, 8);

    robot_world.set(robot_start, 'R');
    robot_world.set(robot_goal, 'G');

    println!("Room layout (R=robot, G=goal, #=furniture):");
    println!("{}", robot_world);

    // Find path using BFS
    if let Some(path) = simple_bfs(&robot_world, robot_start, robot_goal) {
        println!("\nRobot path found ({} steps):", path.len() - 1);

        // Visualize path
        let mut path_grid = robot_world.clone();
        for (i, &coord) in path.iter().enumerate() {
            if i > 0 && i < path.len() - 1 {
                // Skip start and goal
                path_grid.set(coord, '*');
            }
        }
        println!("{}", path_grid);

        // Calculate path metrics
        let total_distance = path.len() - 1;
        let straight_line_distance = manhattan_distance(robot_start, robot_goal);
        let efficiency = straight_line_distance as f64 / total_distance as f64 * 100.0;

        println!("Path analysis:");
        println!("  Actual path length: {} steps", total_distance);
        println!("  Straight-line distance: {} steps", straight_line_distance);
        println!("  Path efficiency: {:.1}%", efficiency);
    } else {
        println!("No path found from robot to goal!");
    }

    // Section 5: Data Processing Pipeline
    print_section("5. Data Processing Pipeline");

    println!("Grid-based data processing pipeline example:");

    // Create a data grid representing sensor readings
    let mut sensor_grid = TutorialGrid::new(8, 6, 0.0f32);

    // Simulate sensor data
    let sensor_data = vec![
        (TutorialCoord::new(1, 1), 23.5),
        (TutorialCoord::new(3, 1), 24.1),
        (TutorialCoord::new(5, 1), 22.8),
        (TutorialCoord::new(1, 3), 25.2),
        (TutorialCoord::new(3, 3), 26.0),
        (TutorialCoord::new(5, 3), 24.7),
        (TutorialCoord::new(1, 5), 23.9),
        (TutorialCoord::new(3, 5), 24.5),
        (TutorialCoord::new(5, 5), 23.2),
    ];

    for (coord, temperature) in sensor_data {
        sensor_grid.set(coord, temperature);
    }

    println!("Temperature sensor grid (°C):");
    for y in 0..sensor_grid.height() {
        for x in 0..sensor_grid.width() {
            let temp = sensor_grid.get(TutorialCoord::new(x, y)).unwrap();
            if *temp > 0.0 {
                print!("{:5.1} ", temp);
            } else {
                print!("  --  ");
            }
        }
        println!();
    }

    // Process the data
    let stats: TemperatureStats = calculate_temperature_stats(&sensor_grid);
    println!("\nTemperature analysis:");
    println!("  Active sensors: {}", stats.sensor_count);
    println!("  Average temperature: {:.1}°C", stats.average);
    println!(
        "  Temperature range: {:.1}°C - {:.1}°C",
        stats.min, stats.max
    );
    println!("  Hot spots (>25°C): {}", stats.hot_spots);

    // Section 6: Error Handling Patterns
    print_section("6. Error Handling Patterns");

    println!("Robust error handling for grid operations:");

    #[derive(Debug)]
    #[allow(dead_code)]
    enum GridError {
        OutOfBounds {
            coord: TutorialCoord,
            width: usize,
            height: usize,
        },
        InvalidDimensions {
            width: usize,
            height: usize,
        },
        InsufficientData {
            expected: usize,
            actual: usize,
        },
    }

    impl fmt::Display for GridError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                GridError::OutOfBounds {
                    coord,
                    width,
                    height,
                } => {
                    write!(
                        f,
                        "Coordinate {} is out of bounds for {}x{} grid",
                        coord, width, height
                    )
                }
                GridError::InvalidDimensions { width, height } => {
                    write!(f, "Invalid grid dimensions: {}x{}", width, height)
                }
                GridError::InsufficientData { expected, actual } => {
                    write!(
                        f,
                        "Insufficient data: expected {}, got {}",
                        expected, actual
                    )
                }
            }
        }
    }

    impl std::error::Error for GridError {}

    // Demonstrate error handling
    fn safe_grid_operation(
        grid: &TutorialGrid<i32>,
        coord: TutorialCoord,
    ) -> Result<i32, GridError> {
        if !grid.in_bounds(coord) {
            return Err(GridError::OutOfBounds {
                coord,
                width: grid.width(),
                height: grid.height(),
            });
        }

        Ok(*grid.get(coord).unwrap())
    }

    let error_test_grid = TutorialGrid::new(5, 5, 42);

    // Valid operation
    match safe_grid_operation(&error_test_grid, TutorialCoord::new(2, 2)) {
        Ok(value) => println!("✓ Valid access returned: {}", value),
        Err(e) => println!("✗ Error: {}", e),
    }

    // Invalid operation
    match safe_grid_operation(&error_test_grid, TutorialCoord::new(10, 10)) {
        Ok(value) => println!("✓ Valid access returned: {}", value),
        Err(e) => println!("✗ Error: {}", e),
    }

    // Section 7: Testing Strategies
    print_section("7. Testing Strategies");

    println!("Comprehensive testing approaches for grid systems:");
    println!("\n🧪 Unit Testing:");
    println!("  • Boundary condition testing (edges, corners, out-of-bounds)");
    println!("  • Property-based testing with random coordinates");
    println!("  • Performance regression testing");

    println!("\n🔍 Integration Testing:");
    println!("  • End-to-end pathfinding scenarios");
    println!("  • Data pipeline validation");
    println!("  • Multi-grid system interactions");

    println!("\n📊 Performance Testing:");
    println!("  • Memory usage validation");
    println!("  • Scalability testing with large grids");
    println!("  • Cache efficiency measurements");

    // Demonstrate simple property-based testing
    println!("\n📋 Property-based testing example:");
    for i in 0..10 {
        let x = (i * 17) % 8;
        let y = (i * 31) % 6;
        let coord = TutorialCoord::new(x, y);
        let test_grid = TutorialGrid::new(8, 6, i);

        // Property: Any coordinate within bounds should return the stored value
        if test_grid.in_bounds(coord) {
            assert_eq!(test_grid.get(coord), Some(&i));
            println!("  ✓ Property test {}: {} -> {}", i + 1, coord, i);
        }
    }

    // Section 8: Best Practices Summary
    print_section("8. Best Practices Summary");

    println!("🏆 Grid Programming Best Practices:\n");

    println!("📐 Design:");
    println!("  ✓ Choose coordinate system consistently (screen vs mathematical)");
    println!("  ✓ Use appropriate data types for your domain (u8, f32, custom structs)");
    println!("  ✓ Consider memory layout for performance-critical applications");
    println!("  ✓ Design APIs that prevent common mistakes (type safety, bounds checking)");

    println!("\n🛡️  Safety:");
    println!("  ✓ Always validate coordinates before access");
    println!("  ✓ Use Option<T> or Result<T, E> for fallible operations");
    println!("  ✓ Implement Debug and Display traits for debugging");
    println!("  ✓ Add comprehensive error messages with context");

    println!("\n⚡ Performance:");
    println!("  ✓ Use row-major iteration for cache efficiency");
    println!("  ✓ Batch operations when possible to reduce overhead");
    println!("  ✓ Profile and benchmark with realistic data");
    println!("  ✓ Consider sparse representations for mostly-empty grids");

    println!("\n📚 Documentation:");
    println!("  ✓ Document coordinate conventions and edge cases");
    println!("  ✓ Provide runnable examples for common use cases");
    println!("  ✓ Include performance characteristics in API docs");
    println!("  ✓ Show integration patterns with standard library");

    println!("\n🧪 Testing:");
    println!("  ✓ Test boundary conditions systematically");
    println!("  ✓ Use property-based testing for coordinate operations");
    println!("  ✓ Validate performance regressions with benchmarks");
    println!("  ✓ Test integration scenarios with realistic data");

    // Section 9: Complete Integration Example
    print_section("9. Complete Integration Example");

    println!("Final integration: Multi-system coordination");

    // Create a complete system that uses multiple grid concepts
    let mut ecosystem = GameEcosystem::new(10, 8);

    ecosystem.add_creature(TutorialCoord::new(2, 2), CreatureType::Herbivore);
    ecosystem.add_creature(TutorialCoord::new(7, 5), CreatureType::Carnivore);
    ecosystem.add_creature(TutorialCoord::new(1, 6), CreatureType::Herbivore);

    ecosystem.add_food(TutorialCoord::new(3, 3));
    ecosystem.add_food(TutorialCoord::new(8, 2));
    ecosystem.add_food(TutorialCoord::new(5, 6));

    println!("Ecosystem simulation:");
    println!("{}", ecosystem);

    // Simulate ecosystem dynamics
    for turn in 1..=3 {
        ecosystem.simulate_turn();
        println!("\nAfter turn {}:", turn);
        println!("{}", ecosystem);
    }

    println!("\nEcosystem statistics:");
    println!("  Total creatures: {}", ecosystem.creature_count());
    println!("  Food sources: {}", ecosystem.food_count());
    println!(
        "  Grid utilization: {:.1}%",
        ecosystem.utilization() * 100.0
    );

    print_step_complete("Step 7: Documentation & Integration");

    // Tutorial Completion
    println!("\n🎉 Mission 6 Tutorial Complete! 🎉");
    println!("\n🏆 You have mastered:");
    println!("   ✅ Grid creation and safe indexing (REQ-1, REQ-2)");
    println!("   ✅ Coordinate systems and navigation (REQ-3)");
    println!("   ✅ Pathfinding algorithms (BFS, A*) (REQ-4)");
    println!("   ✅ AoC utilities and flood fill (REQ-5)");
    println!("   ✅ Performance optimization (REQ-6)");
    println!("   ✅ Professional documentation and integration");

    println!("\n🔄 Next Steps:");
    println!("   • Apply these concepts to your own projects");
    println!("   • Try solving AoC grid problems with confidence");
    println!("   • Experiment with advanced optimizations");
    println!("   • Build complex spatial applications");

    println!("\n📚 Continue Learning:");
    println!("   • Mission6 main implementation for production use");
    println!("   • Advanced spatial data structures (quadtrees, R-trees)");
    println!("   • GPU-accelerated grid processing");
    println!("   • Real-time spatial simulations");
}

/// Example of well-documented function following professional standards
///
/// Calculates the Manhattan distance between two coordinates.
///
/// Manhattan distance is the sum of absolute differences of their coordinates,
/// representing the distance a taxi would travel on a grid of streets.
///
/// # Arguments
///
/// * `a` - First coordinate point
/// * `b` - Second coordinate point
///
/// # Returns
///
/// The Manhattan distance as a usize value
///
/// # Performance
///
/// This function runs in O(1) time complexity.
///
/// # Examples
///
/// ```
/// let start = TutorialCoord::new(0, 0);
/// let end = TutorialCoord::new(3, 4);
/// let distance = manhattan_distance(start, end);
/// assert_eq!(distance, 7); // 3 + 4 = 7
/// ```
fn manhattan_distance(a: TutorialCoord, b: TutorialCoord) -> usize {
    let dx = a.x.abs_diff(b.x);
    let dy = a.y.abs_diff(b.y);
    dx + dy
}

fn demonstrate_documentation_example() {
    let point1 = TutorialCoord::new(1, 1);
    let point2 = TutorialCoord::new(4, 5);
    let dist = manhattan_distance(point1, point2);

    println!("\nDocumentation example:");
    println!("  manhattan_distance({}, {}) = {}", point1, point2, dist);
    println!("  This function is fully documented with examples and complexity analysis");
}

// Supporting types and implementations for examples

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct SnakeGame {
    grid: TutorialGrid<char>,
    snake_body: Vec<TutorialCoord>,
    food: TutorialCoord,
    game_score: usize,
}

impl SnakeGame {
    fn new(width: usize, height: usize) -> Self {
        let mut game = Self {
            grid: TutorialGrid::new(width, height, '.'),
            snake_body: vec![TutorialCoord::new(width / 2, height / 2)],
            food: TutorialCoord::new(1, 1),
            game_score: 0,
        };
        game.update_display();
        game
    }

    fn make_move(&mut self, direction: Direction) -> bool {
        let head = self.snake_body[0];
        let new_head = match direction {
            Direction::Up if head.y > 0 => TutorialCoord::new(head.x, head.y - 1),
            Direction::Down if head.y < self.grid.height() - 1 => {
                TutorialCoord::new(head.x, head.y + 1)
            }
            Direction::Left if head.x > 0 => TutorialCoord::new(head.x - 1, head.y),
            Direction::Right if head.x < self.grid.width() - 1 => {
                TutorialCoord::new(head.x + 1, head.y)
            }
            _ => return false, // Hit wall
        };

        if self.snake_body.contains(&new_head) {
            return false; // Hit self
        }

        self.snake_body.insert(0, new_head);

        if new_head == self.food {
            self.game_score += 10;
            // Place new food
            self.food = TutorialCoord::new(
                (self.food.x + 3) % self.grid.width(),
                (self.food.y + 2) % self.grid.height(),
            );
        } else {
            self.snake_body.pop();
        }

        self.update_display();
        true
    }

    fn update_display(&mut self) {
        // Clear grid
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                self.grid.set(TutorialCoord::new(x, y), '.');
            }
        }

        // Draw snake
        for (i, &coord) in self.snake_body.iter().enumerate() {
            let symbol = if i == 0 { 'H' } else { 'S' };
            self.grid.set(coord, symbol);
        }

        // Draw food
        self.grid.set(self.food, 'F');
    }

    fn score(&self) -> usize {
        self.game_score
    }
}

impl fmt::Display for SnakeGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid)
    }
}

struct TemperatureStats {
    sensor_count: usize,
    average: f32,
    min: f32,
    max: f32,
    hot_spots: usize,
}

fn calculate_temperature_stats(grid: &TutorialGrid<f32>) -> TemperatureStats {
    let mut sum = 0.0;
    let mut count = 0;
    let mut min_temp = f32::INFINITY;
    let mut max_temp = f32::NEG_INFINITY;
    let mut hot_spots = 0;

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let temp = *grid.get(TutorialCoord::new(x, y)).unwrap();
            if temp > 0.0 {
                // Active sensor
                sum += temp;
                count += 1;
                min_temp = min_temp.min(temp);
                max_temp = max_temp.max(temp);

                if temp > 25.0 {
                    hot_spots += 1;
                }
            }
        }
    }

    TemperatureStats {
        sensor_count: count,
        average: if count > 0 { sum / count as f32 } else { 0.0 },
        min: if min_temp != f32::INFINITY {
            min_temp
        } else {
            0.0
        },
        max: if max_temp != f32::NEG_INFINITY {
            max_temp
        } else {
            0.0
        },
        hot_spots,
    }
}

fn simple_bfs(
    grid: &TutorialGrid<char>,
    start: TutorialCoord,
    goal: TutorialCoord,
) -> Option<Vec<TutorialCoord>> {
    use std::collections::{HashMap, VecDeque};

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent = HashMap::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        if current == goal {
            let mut path = Vec::new();
            let mut backtrack = goal;

            while backtrack != start {
                path.push(backtrack);
                backtrack = parent[&backtrack];
            }
            path.push(start);
            path.reverse();

            return Some(path);
        }

        // Check 4-connected neighbors
        let neighbors = [
            TutorialCoord::new(current.x, current.y.wrapping_sub(1)),
            TutorialCoord::new(current.x.wrapping_add(1), current.y),
            TutorialCoord::new(current.x, current.y.wrapping_add(1)),
            TutorialCoord::new(current.x.wrapping_sub(1), current.y),
        ];

        for neighbor in neighbors {
            if grid.in_bounds(neighbor)
                && !visited.contains(&neighbor)
                && grid.get(neighbor) != Some(&'#')
            {
                visited.insert(neighbor);
                parent.insert(neighbor, current);
                queue.push_back(neighbor);
            }
        }
    }

    None
}

#[derive(Debug, Clone, Copy)]
enum CreatureType {
    Herbivore,
    Carnivore,
}

struct GameEcosystem {
    grid: TutorialGrid<char>,
    creatures: HashMap<TutorialCoord, CreatureType>,
    food: HashSet<TutorialCoord>,
}

impl GameEcosystem {
    fn new(width: usize, height: usize) -> Self {
        Self {
            grid: TutorialGrid::new(width, height, '.'),
            creatures: HashMap::new(),
            food: HashSet::new(),
        }
    }

    fn add_creature(&mut self, coord: TutorialCoord, creature_type: CreatureType) {
        self.creatures.insert(coord, creature_type);
        self.update_display();
    }

    fn add_food(&mut self, coord: TutorialCoord) {
        self.food.insert(coord);
        self.update_display();
    }

    fn simulate_turn(&mut self) {
        // Simple simulation - just move creatures randomly
        let creature_coords: Vec<_> = self.creatures.keys().cloned().collect();

        for old_coord in creature_coords {
            if let Some(creature_type) = self.creatures.remove(&old_coord) {
                // Try to move to adjacent cell
                let directions = [
                    TutorialCoord::new(old_coord.x, old_coord.y.wrapping_sub(1)),
                    TutorialCoord::new(old_coord.x.wrapping_add(1), old_coord.y),
                    TutorialCoord::new(old_coord.x, old_coord.y.wrapping_add(1)),
                    TutorialCoord::new(old_coord.x.wrapping_sub(1), old_coord.y),
                ];

                let new_coord = directions
                    .into_iter()
                    .find(|coord| {
                        self.grid.in_bounds(*coord) && !self.creatures.contains_key(coord)
                    })
                    .unwrap_or(old_coord);

                // Eat food if present
                self.food.remove(&new_coord);

                self.creatures.insert(new_coord, creature_type);
            }
        }

        self.update_display();
    }

    fn update_display(&mut self) {
        // Clear grid
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                self.grid.set(TutorialCoord::new(x, y), '.');
            }
        }

        // Draw food
        for &coord in &self.food {
            self.grid.set(coord, '*');
        }

        // Draw creatures
        for (&coord, &creature_type) in &self.creatures {
            let symbol = match creature_type {
                CreatureType::Herbivore => 'H',
                CreatureType::Carnivore => 'C',
            };
            self.grid.set(coord, symbol);
        }
    }

    fn creature_count(&self) -> usize {
        self.creatures.len()
    }

    fn food_count(&self) -> usize {
        self.food.len()
    }

    fn utilization(&self) -> f64 {
        let occupied = self.creatures.len() + self.food.len();
        let total = self.grid.width() * self.grid.height();
        occupied as f64 / total as f64
    }
}

impl fmt::Display for GameEcosystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid)
    }
}

// Exercise for the Reader:
// 1. Add comprehensive API documentation to your own grid implementation
// 2. Create a complete game using grid systems (e.g., Conway's Game of Life)
// 3. Build a data visualization system using grids and external plotting libraries
// 4. Implement a spatial database query system with grid-based indexing

// Design Questions to Consider:
//
// Q: How do you balance API simplicity with feature completeness?
// A: Follow the "Progressive Disclosure" principle: Provide simple, safe defaults for
//    common cases (e.g., `grid.get(coord)`) while offering advanced methods for power
//    users (e.g., `grid.get_unchecked()` for hot paths after validation). Use builder
//    patterns for complex configuration, trait implementations for standard operations
//    (Index, Iterator), and comprehensive documentation showing both basic and advanced
//    usage. Example: TutorialGrid offers simple `get()`/`set()` but could add `iter()`,
//    `iter_mut()`, `rows()`, `neighbors()` for advanced use without complicating the
//    basic API. Key principle: "Make simple things simple, complex things possible."
//
// Q: What documentation strategies work best for spatial data structures?
// A: 1) Visual ASCII diagrams showing coordinate systems and transformations
//    2) Worked examples with grid visualizations (as demonstrated in this tutorial)
//    3) Document coordinate conventions prominently (screen vs math, 0-indexed vs 1-indexed)
//    4) Show common patterns: pathfinding, flood fill, neighbor iteration
//    5) Include performance characteristics (O(1) access, cache-friendly iteration)
//    6) Provide doctests that create small grids and demonstrate operations
//    7) Document edge cases explicitly (corners, boundaries, wrapping behavior)
//    8) Cross-reference related functions (e.g., get() → set(), in_bounds())
//    Real-world tip: One clear diagram beats 1000 words of coordinate explanation!
//
// Q: How do you test integration scenarios systematically?
// A: Use hierarchical testing strategy:
//    1) Unit tests: Individual operations (get, set, in_bounds) - 70% of tests
//    2) Property tests: Invariants hold (set then get returns same value) - 20%
//    3) Integration tests: Multi-component scenarios (pathfinding + obstacles) - 10%
//    4) End-to-end tests: Complete workflows (game simulation, data pipeline)
//    5) Regression tests: Capture and replay real failure scenarios
//    Create test fixtures for common grid configurations (empty, obstacles, filled).
//    Use snapshot testing for complex outputs (pathfinding results, game states).
//    Test with realistic data sizes matching production workloads.
//    Best practice: Write integration tests that mirror real user workflows.
//
// Q: What are the best practices for grid-based game architecture?
// A: 1) Separation of Concerns: Grid (data) + Game Logic (rules) + Renderer (display)
//    2) Entity Component System (ECS): Store entity IDs in grid, attributes separately
//    3) Event-driven updates: Grid fires events on state changes for loose coupling
//    4) Chunk-based storage: For large worlds, load/unload grid regions dynamically
//    5) State management: Immutable grids + functional updates for time-travel/undo
//    6) Spatial indexing: Use HashMaps for sparse entities, grids for dense terrain
//    7) Query patterns: Provide `neighbors()`, `within_radius()`, `line_of_sight()`
//    8) Performance: Cache frequently-queried data, use dirty flags for updates
//    Architecture pattern: Grid<TileType> + HashMap<Coord, Entity> + RenderQueue
//    This separates static world data from dynamic entities for optimal performance.
