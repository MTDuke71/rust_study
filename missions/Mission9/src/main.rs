/// Mission 9: Advanced Pathfinding Algorithms - Production CLI
///
/// REQ-7: Production-ready command-line interface with:
/// - Interactive pathfinding for various algorithms
/// - Graph loading from JSON, CSV formats
/// - Visualization output generation
/// - Batch processing capabilities
/// - Performance benchmarking
///
/// # Examples
///
/// Find a path using A*:
/// ```bash
/// cargo run -- find-path --graph data/graph.json --start 0 --goal 10 --algorithm astar
/// ```
///
/// Batch process multiple queries:
/// ```bash
/// cargo run -- batch --graph data/graph.json --queries queries.csv --output results.csv
/// ```
///
/// Generate a sample graph:
/// ```bash
/// cargo run -- generate --graph-type grid --nodes 100 --output grid.json
/// ```
use clap::Parser;
use mission9::*;
use std::path::Path;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.verbose {
        println!("🚀 Mission 9: Advanced Pathfinding Algorithms");
        println!("================================================\n");
    }

    match &cli.command {
        Commands::FindPath {
            ref graph,
            start,
            goal,
            algorithm,
            heuristic,
            ref visualize,
        } => handle_find_path(graph, *start, *goal, *algorithm, *heuristic, visualize.clone(), &cli)?,

        Commands::Batch {
            ref graph,
            ref queries,
            ref output,
            algorithm,
        } => handle_batch(graph, queries, output, *algorithm, &cli)?,

        Commands::Info { ref graph } => handle_info(graph, &cli)?,

        Commands::Benchmark {
            ref graph,
            start,
            goal,
            iterations,
        } => handle_benchmark(graph, *start, *goal, *iterations, &cli)?,

        Commands::Generate {
            graph_type,
            nodes,
            ref file,
            format,
        } => handle_generate(*graph_type, *nodes, file, *format, &cli)?,
    }

    Ok(())
}

/// Handle the find-path command
fn handle_find_path(
    graph_path: &Path,
    start: u32,
    goal: u32,
    algorithm: AlgorithmType,
    heuristic: Option<HeuristicType>,
    visualize_path: Option<std::path::PathBuf>,
    cli: &Cli,
) -> Result<(), Box<dyn std::error::Error>> {
    if cli.verbose {
        println!("� Loading graph from: {}", graph_path.display());
    }

    // Load graph based on file extension
    let (graph, context) = if graph_path.extension().and_then(|s| s.to_str()) == Some("json") {
        load_graph_json(graph_path)?
    } else {
        let g = load_graph_csv(graph_path)?;
        (g, HeuristicContext::new())
    };

    if cli.verbose {
        println!("✅ Graph loaded: {} nodes, {} edges", graph.node_count(), graph.edge_count());
        println!("🎯 Finding path: {} → {}", start, goal);
        println!("🔧 Algorithm: {}", algorithm);
    }

    // Find path using specified algorithm
    let result = match algorithm {
        AlgorithmType::Dijkstra => {
            let pathfinder = DijkstraPathfinder::new();
            pathfinder.find_path(&graph, start, goal)?
        }
        AlgorithmType::Astar => {
            let heuristic_type = heuristic.unwrap_or(HeuristicType::Euclidean);
            if cli.verbose {
                println!("📐 Heuristic: {}", heuristic_type);
            }

            let pathfinder = AstarPathfinder::new(EuclideanHeuristic);
            pathfinder.find_path_with_context(&graph, start, goal, &context)?
        }
    };

    // Output results based on format
    match cli.output {
        OutputFormat::Text => {
            println!("\n📊 Results:");
            println!("  Path: {:?}", result.path);
            println!("  Cost: {:.2}", result.cost);
            println!("  Nodes explored: {}", result.nodes_explored);
            println!("  Search time: {:.2}ms", result.search_time_ms);
        }
        OutputFormat::Json => {
            let json = serde_json::json!({
                "path": result.path,
                "cost": result.cost,
                "nodes_explored": result.nodes_explored,
                "search_time_ms": result.search_time_ms,
            });
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        OutputFormat::Csv => {
            println!("path,cost,nodes_explored,search_time_ms");
            println!(
                "\"{:?}\",{},{},{}",
                result.path, result.cost, result.nodes_explored, result.search_time_ms
            );
        }
    }

    // Generate visualization if requested
    if let Some(viz_path) = visualize_path {
        if cli.verbose {
            println!("\n📊 Generating visualization: {}", viz_path.display());
        }
        save_visualization(&viz_path, &graph, Some(&result.path))?;
        println!("✅ Visualization saved to: {}", viz_path.display());
        println!("   (Render with: dot -Tpng {} -o output.png)", viz_path.display());
    }

    Ok(())
}

/// Handle the batch processing command
fn handle_batch(
    graph_path: &Path,
    queries_path: &Path,
    output_path: &Path,
    algorithm: AlgorithmType,
    cli: &Cli,
) -> Result<(), Box<dyn std::error::Error>> {
    if cli.verbose {
        println!("📂 Loading graph from: {}", graph_path.display());
    }

    let (graph, context) = if graph_path.extension().and_then(|s| s.to_str()) == Some("json") {
        load_graph_json(graph_path)?
    } else {
        let g = load_graph_csv(graph_path)?;
        (g, HeuristicContext::new())
    };

    if cli.verbose {
        println!("✅ Graph loaded: {} nodes, {} edges", graph.node_count(), graph.edge_count());
        println!("📂 Loading queries from: {}", queries_path.display());
    }

    let queries = load_queries_csv(queries_path)?;

    if cli.verbose {
        println!("✅ Loaded {} queries", queries.len());
        println!("🔧 Algorithm: {}", algorithm);
        println!("⏳ Processing queries...");
    }

    let mut results = Vec::new();
    let total_start = Instant::now();

    for (idx, (start, goal)) in queries.iter().enumerate() {
        if cli.verbose && idx % 100 == 0 {
            println!("  Progress: {}/{}", idx, queries.len());
        }

        let result = match algorithm {
            AlgorithmType::Dijkstra => {
                let pathfinder = DijkstraPathfinder::new();
                pathfinder.find_path(&graph, *start, *goal)
            }
            AlgorithmType::Astar => {
                let pathfinder = AstarPathfinder::new(EuclideanHeuristic);
                pathfinder.find_path_with_context(&graph, *start, *goal, &context)
            }
        };

        results.push((*start, *goal, result));
    }

    let total_time = total_start.elapsed();

    if cli.verbose {
        println!("✅ All queries processed in {:.2}s", total_time.as_secs_f64());
        println!("📝 Writing results to: {}", output_path.display());
    }

    // Write results to CSV
    let mut writer = csv::Writer::from_path(output_path)?;
    writer.write_record(["start", "goal", "cost", "path_length", "nodes_explored", "time_ms"])?;

    for (start, goal, result) in results {
        match result {
            Ok(path_result) => {
                writer.write_record(&[
                    start.to_string(),
                    goal.to_string(),
                    format!("{:.2}", path_result.cost),
                    path_result.path.len().to_string(),
                    path_result.nodes_explored.to_string(),
                    format!("{:.2}", path_result.search_time_ms),
                ])?;
            }
            Err(_) => {
                writer.write_record(&[
                    start.to_string(),
                    goal.to_string(),
                    "N/A".to_string(),
                    "0".to_string(),
                    "0".to_string(),
                    "0".to_string(),
                ])?;
            }
        }
    }

    writer.flush()?;

    println!("✅ Batch processing complete!");
    println!("   Results saved to: {}", output_path.display());

    Ok(())
}

/// Handle the info command
fn handle_info(graph_path: &Path, cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    if cli.verbose {
        println!("📂 Loading graph from: {}", graph_path.display());
    }

    let (graph, context) = if graph_path.extension().and_then(|s| s.to_str()) == Some("json") {
        load_graph_json(graph_path)?
    } else {
        let g = load_graph_csv(graph_path)?;
        (g, HeuristicContext::new())
    };

    println!("\n📊 Graph Information:");
    println!("  File: {}", graph_path.display());
    println!("  Nodes: {}", graph.node_count());
    println!("  Edges: {}", graph.edge_count());

    // Calculate degree statistics
    let mut degrees = Vec::new();
    for node in 0..graph.node_count() {
        degrees.push(graph.neighbors(node as u32).len());
    }

    if !degrees.is_empty() {
        degrees.sort_unstable();
        let min_degree = degrees[0];
        let max_degree = degrees[degrees.len() - 1];
        let avg_degree = degrees.iter().sum::<usize>() as f64 / degrees.len() as f64;

        println!("\n📈 Degree Statistics:");
        println!("  Min degree: {}", min_degree);
        println!("  Max degree: {}", max_degree);
        println!("  Avg degree: {:.2}", avg_degree);
    }

    // Check for coordinates
    let has_coords = (0..graph.node_count())
        .any(|n| context.get_coordinates(n as u32).is_some());

    println!("\n📐 Spatial Information:");
    if has_coords {
        println!("  ✅ Graph has coordinate data (suitable for A* with geometric heuristics)");
    } else {
        println!("  ❌ No coordinate data (A* will use zero heuristic)");
    }

    Ok(())
}

/// Handle the benchmark command
fn handle_benchmark(
    graph_path: &Path,
    start: u32,
    goal: u32,
    iterations: usize,
    cli: &Cli,
) -> Result<(), Box<dyn std::error::Error>> {
    if cli.verbose {
        println!("📂 Loading graph from: {}", graph_path.display());
    }

    let (graph, context) = if graph_path.extension().and_then(|s| s.to_str()) == Some("json") {
        load_graph_json(graph_path)?
    } else {
        let g = load_graph_csv(graph_path)?;
        (g, HeuristicContext::new())
    };

    if cli.verbose {
        println!("✅ Graph loaded: {} nodes, {} edges", graph.node_count(), graph.edge_count());
        println!("🎯 Benchmarking: {} → {}", start, goal);
        println!("🔄 Iterations: {}", iterations);
    }

    println!("\n⏱️  Running benchmarks...\n");

    // Benchmark Dijkstra
    let dijkstra = DijkstraPathfinder::new();
    let dijkstra_start = Instant::now();
    for _ in 0..iterations {
        let _ = dijkstra.find_path(&graph, start, goal)?;
    }
    let dijkstra_time = dijkstra_start.elapsed();

    // Benchmark A*
    let astar = AstarPathfinder::new(EuclideanHeuristic);
    let astar_start = Instant::now();
    for _ in 0..iterations {
        let _ = astar.find_path_with_context(&graph, start, goal, &context)?;
    }
    let astar_time = astar_start.elapsed();

    // Display results
    println!("📊 Benchmark Results ({} iterations):\n", iterations);
    println!("  Algorithm                     | Total Time | Avg Time/Query");
    println!("  ------------------------------|------------|---------------");
    println!(
        "  Dijkstra                      | {:>8.2}ms | {:>8.2}ms",
        dijkstra_time.as_secs_f64() * 1000.0,
        dijkstra_time.as_secs_f64() * 1000.0 / iterations as f64
    );
    println!(
        "  A* (Euclidean)                | {:>8.2}ms | {:>8.2}ms",
        astar_time.as_secs_f64() * 1000.0,
        astar_time.as_secs_f64() * 1000.0 / iterations as f64
    );

    // Calculate speedup
    let dijkstra_avg = dijkstra_time.as_secs_f64() / iterations as f64;
    let astar_speedup = dijkstra_avg / (astar_time.as_secs_f64() / iterations as f64);

    println!("\n🏆 Performance Comparison (vs Dijkstra):\n");
    println!("  A* speedup:                   {:.2}x", astar_speedup);

    Ok(())
}

/// Handle the generate command
fn handle_generate(
    graph_type: GraphType,
    nodes: usize,
    output_path: &Path,
    format: GraphFormat,
    cli: &Cli,
) -> Result<(), Box<dyn std::error::Error>> {
    if cli.verbose {
        println!("🏗️  Generating {} graph with {} nodes", graph_type_str(graph_type), nodes);
    }

    let (graph, context) = generate_graph(graph_type_str(graph_type), nodes)?;

    if cli.verbose {
        println!("✅ Generated: {} nodes, {} edges", graph.node_count(), graph.edge_count());
        println!("💾 Saving to: {}", output_path.display());
    }

    match format {
        GraphFormat::Json => save_graph_json(output_path, &graph, Some(&context))?,
        GraphFormat::Csv => save_graph_csv(output_path, &graph)?,
    }

    println!("✅ Graph generated successfully!");
    println!("   Output: {}", output_path.display());
    println!("   Format: {:?}", format);

    Ok(())
}

fn graph_type_str(graph_type: GraphType) -> &'static str {
    match graph_type {
        GraphType::Random => "random",
        GraphType::Grid => "grid",
        GraphType::Tree => "tree",
        GraphType::Complete => "complete",
    }
}
