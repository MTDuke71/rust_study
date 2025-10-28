/// Mission 9: Grid-based Pathfinding CLI
///
/// REQ-7: Production-ready CLI for bidirectional algorithms
/// 
/// This CLI works with grid-based pathfinding algorithms that use
/// coordinate-based representations (BidirectionalDijkstra, BidirectionalAstar).

use clap::Parser;
use mission9::*;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(
    name = "mission9-grid",
    about = "Grid-based Pathfinding - Bidirectional Algorithms",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Find a path in a grid using bidirectional search
    FindPath {
        /// Grid width
        #[arg(short, long)]
        width: usize,

        /// Grid height
        #[arg(short = 'H', long)]
        height: usize,

        /// Start position (x,y)
        #[arg(short, long, value_parser = parse_coord)]
        start: (usize, usize),

        /// Goal position (x,y)
        #[arg(short, long, value_parser = parse_coord)]
        goal: (usize, usize),

        /// Algorithm to use
        #[arg(short, long, value_enum, default_value = "bidirectional-astar")]
        algorithm: Algorithm,

        /// Obstacles (x,y coordinates, comma-separated)
        #[arg(short, long, value_parser = parse_coord, num_args = 0..)]
        obstacles: Vec<(usize, usize)>,
    },

    /// Benchmark bidirectional algorithms
    Benchmark {
        /// Grid width
        #[arg(short, long, default_value = "100")]
        width: usize,

        /// Grid height
        #[arg(short = 'H', long, default_value = "100")]
        height: usize,

        /// Number of iterations
        #[arg(short, long, default_value = "100")]
        iterations: usize,
    },
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum Algorithm {
    BidirectionalDijkstra,
    BidirectionalAstar,
}

fn parse_coord(s: &str) -> Result<(usize, usize), String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err(format!("Expected format: x,y but got: {}", s));
    }
    
    let x = parts[0].parse::<usize>()
        .map_err(|_| format!("Invalid x coordinate: {}", parts[0]))?;
    let y = parts[1].parse::<usize>()
        .map_err(|_| format!("Invalid y coordinate: {}", parts[1]))?;
    
    Ok((x, y))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.verbose {
        println!("🚀 Mission 9: Grid-based Pathfinding");
        println!("=====================================\n");
    }

    match cli.command {
        Commands::FindPath {
            width,
            height,
            start,
            goal,
            algorithm,
            obstacles,
        } => handle_find_path(width, height, start, goal, algorithm, obstacles, cli.verbose)?,

        Commands::Benchmark {
            width,
            height,
            iterations,
        } => handle_benchmark(width, height, iterations, cli.verbose)?,
    }

    Ok(())
}

fn handle_find_path(
    width: usize,
    height: usize,
    start: (usize, usize),
    goal: (usize, usize),
    algorithm: Algorithm,
    obstacles: Vec<(usize, usize)>,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if verbose {
        println!("🗺️  Grid: {}x{}", width, height);
        println!("🎯 Path: {:?} → {:?}", start, goal);
        println!("🚧 Obstacles: {}", obstacles.len());
    }

    // Create obstacle set
    let obstacle_set: std::collections::HashSet<_> = obstacles.into_iter().collect();

    // Define neighbors function (4-directional movement)
    let neighbors = |pos: (usize, usize)| -> Vec<((usize, usize), u32)> {
        let (x, y) = pos;
        let mut result = Vec::new();

        // Right
        if x + 1 < width {
            let next = (x + 1, y);
            if !obstacle_set.contains(&next) {
                result.push((next, 1));
            }
        }

        // Left
        if x > 0 {
            let next = (x - 1, y);
            if !obstacle_set.contains(&next) {
                result.push((next, 1));
            }
        }

        // Down
        if y + 1 < height {
            let next = (x, y + 1);
            if !obstacle_set.contains(&next) {
                result.push((next, 1));
            }
        }

        // Up
        if y > 0 {
            let next = (x, y - 1);
            if !obstacle_set.contains(&next) {
                result.push((next, 1));
            }
        }

        result
    };

    // Manhattan distance heuristic
    let manhattan = |a: (usize, usize), b: (usize, usize)| -> u32 {
        let dx = a.0.abs_diff(b.0);
        let dy = a.1.abs_diff(b.1);
        (dx + dy) as u32
    };

    let start_time = std::time::Instant::now();

    let path = match algorithm {
        Algorithm::BidirectionalDijkstra => {
            let mut pathfinder = BidirectionalDijkstra::new(width * height);
            pathfinder.find_path(start, goal, neighbors)
        }
        Algorithm::BidirectionalAstar => {
            let mut pathfinder = BidirectionalAstar::new(width * height);
            pathfinder.find_path(start, goal, manhattan, neighbors)
        }
    };

    let elapsed = start_time.elapsed();

    match path {
        Some(p) => {
            println!("\n✅ Path found!");
            println!("  Length: {} steps", p.len());
            println!("  Search time: {:.2}ms", elapsed.as_secs_f64() * 1000.0);
            
            if verbose {
                println!("\n📍 Path coordinates:");
                for (i, pos) in p.iter().enumerate() {
                    println!("  {}: {:?}", i, pos);
                }
            }

            // Draw grid
            if width <= 50 && height <= 50 {
                println!("\n📊 Grid visualization:");
                draw_grid(width, height, &p, &obstacle_set, start, goal);
            }
        }
        None => {
            println!("\n❌ No path found!");
            println!("  Check if goal is reachable or blocked by obstacles");
        }
    }

    Ok(())
}

fn handle_benchmark(
    width: usize,
    height: usize,
    iterations: usize,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if verbose {
        println!("🗺️  Grid: {}x{}", width, height);
        println!("🔄 Iterations: {}", iterations);
    }

    let start = (0, 0);
    let goal = (width - 1, height - 1);

    // Simple neighbors function
    let neighbors = |pos: (usize, usize)| -> Vec<((usize, usize), u32)> {
        let (x, y) = pos;
        let mut result = Vec::new();

        if x + 1 < width {
            result.push(((x + 1, y), 1));
        }
        if x > 0 {
            result.push(((x - 1, y), 1));
        }
        if y + 1 < height {
            result.push(((x, y + 1), 1));
        }
        if y > 0 {
            result.push(((x, y - 1), 1));
        }

        result
    };

    let manhattan = |a: (usize, usize), b: (usize, usize)| -> u32 {
        let dx = a.0.abs_diff(b.0);
        let dy = a.1.abs_diff(b.1);
        (dx + dy) as u32
    };

    println!("\n⏱️  Running benchmarks...\n");

    // Benchmark Bidirectional Dijkstra
    let mut bidir_dijkstra = BidirectionalDijkstra::new(width * height);
    let dijkstra_start = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = bidir_dijkstra.find_path(start, goal, neighbors);
    }
    let dijkstra_time = dijkstra_start.elapsed();

    // Benchmark Bidirectional A*
    let mut bidir_astar = BidirectionalAstar::new(width * height);
    let astar_start = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = bidir_astar.find_path(start, goal, manhattan, neighbors);
    }
    let astar_time = astar_start.elapsed();

    println!("📊 Benchmark Results ({} iterations):\n", iterations);
    println!("  Algorithm                     | Total Time | Avg Time/Query");
    println!("  ------------------------------|------------|---------------");
    println!(
        "  Bidirectional Dijkstra        | {:>8.2}ms | {:>8.2}ms",
        dijkstra_time.as_secs_f64() * 1000.0,
        dijkstra_time.as_secs_f64() * 1000.0 / iterations as f64
    );
    println!(
        "  Bidirectional A*              | {:>8.2}ms | {:>8.2}ms",
        astar_time.as_secs_f64() * 1000.0,
        astar_time.as_secs_f64() * 1000.0 / iterations as f64
    );

    let dijkstra_avg = dijkstra_time.as_secs_f64() / iterations as f64;
    let astar_speedup = dijkstra_avg / (astar_time.as_secs_f64() / iterations as f64);

    println!("\n🏆 Performance Comparison (vs Bidirectional Dijkstra):\n");
    println!("  Bidirectional A* speedup:     {:.2}x", astar_speedup);

    Ok(())
}

fn draw_grid(
    width: usize,
    height: usize,
    path: &[(usize, usize)],
    obstacles: &std::collections::HashSet<(usize, usize)>,
    start: (usize, usize),
    goal: (usize, usize),
) {
    let path_set: HashMap<_, _> = path.iter().enumerate().map(|(i, &pos)| (pos, i)).collect();

    for y in 0..height {
        print!("  ");
        for x in 0..width {
            let pos = (x, y);
            if pos == start {
                print!("S ");
            } else if pos == goal {
                print!("G ");
            } else if obstacles.contains(&pos) {
                print!("█ ");
            } else if path_set.contains_key(&pos) {
                print!("• ");
            } else {
                print!("· ");
            }
        }
        println!();
    }
    println!("\n  Legend: S=Start, G=Goal, •=Path, █=Obstacle, ·=Empty");
}
