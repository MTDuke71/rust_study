# 🖥️ CLI Design Patterns in Rust

**Comprehensive guide to building robust command-line interfaces with argument parsing, error handling, and user experience best practices**

---

## 🎯 **CLI Philosophy in Rust**

### **Core Principles**
- **Fail fast** - Validate inputs early and provide clear error messages
- **Composability** - Design tools that work well with Unix pipes and other CLI tools
- **Predictable behavior** - Follow established CLI conventions and patterns
- **Rich help** - Provide comprehensive help and usage information
- **Error clarity** - Make errors actionable and user-friendly

### **Rust CLI Advantages**
- **Zero-cost performance** - Compiled binaries with minimal runtime overhead
- **Memory safety** - No segfaults or memory leaks in CLI tools
- **Cross-platform** - Single codebase works across Windows, macOS, Linux
- **Rich ecosystem** - Excellent crates for argument parsing, terminal output, etc.

---

## 🏗️ **Basic Argument Parsing**

### **Using `std::env`**
```rust
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [arguments...]", args[0]);
        std::process::exit(1);
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "help" | "--help" | "-h" => print_help(),
        "version" | "--version" | "-V" => print_version(),
        "process" => {
            if args.len() < 3 {
                return Err("process command requires a file argument".into());
            }
            process_file(&args[2])?;
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Use '{} help' for usage information", args[0]);
            std::process::exit(1);
        }
    }
    
    Ok(())
}

fn print_help() {
    println!("MyTool 1.0 - A sample CLI application

USAGE:
    mytool <COMMAND> [OPTIONS]

COMMANDS:
    process <FILE>    Process the specified file
    help             Show this help message
    version          Show version information

OPTIONS:
    -h, --help       Show help
    -V, --version    Show version");
}

fn print_version() {
    println!("mytool {}", env!("CARGO_PKG_VERSION"));
}

fn process_file(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Processing file: {}", filename);
    // Implementation here
    Ok(())
}
```

### **Environment Variable Integration**
```rust
use std::env;

#[derive(Debug)]
struct Config {
    debug: bool,
    output_dir: String,
    max_workers: usize,
    input_file: String,
}

impl Config {
    fn from_env_and_args() -> Result<Config, Box<dyn std::error::Error>> {
        let args: Vec<String> = env::args().collect();
        
        if args.len() < 2 {
            return Err("Missing input file argument".into());
        }
        
        let config = Config {
            debug: env::var("DEBUG").is_ok() || args.contains(&"--debug".to_string()),
            output_dir: env::var("OUTPUT_DIR").unwrap_or_else(|_| ".".to_string()),
            max_workers: env::var("MAX_WORKERS")
                .unwrap_or_else(|_| "4".to_string())
                .parse()
                .unwrap_or(4),
            input_file: args[1].clone(),
        };
        
        if config.debug {
            eprintln!("Debug mode enabled");
            eprintln!("Config: {:#?}", config);
        }
        
        Ok(config)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env_and_args()?;
    run_with_config(config)
}
```

---

## 📦 **Using `clap` for Advanced Parsing**

### **Derive API (Recommended)**
```rust
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "pathfinder")]
#[command(about = "A pathfinding algorithm CLI tool")]
#[command(version = "1.0")]
struct Cli {
    /// Enable debug output
    #[arg(short, long)]
    debug: bool,
    
    /// Number of worker threads
    #[arg(short = 'j', long, default_value = "4")]
    threads: usize,
    
    /// Output format
    #[arg(short, long, default_value = "json")]
    format: OutputFormat,
    
    /// Input graph file
    #[arg(short, long)]
    input: Option<PathBuf>,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Find shortest path using Dijkstra's algorithm
    Dijkstra {
        /// Starting node ID
        #[arg(short, long)]
        start: u32,
        
        /// Goal node ID  
        #[arg(short, long)]
        goal: u32,
        
        /// Graph file (overrides global input)
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
    
    /// Find path using A* algorithm
    Astar {
        /// Starting node ID
        #[arg(short, long)]
        start: u32,
        
        /// Goal node ID
        #[arg(short, long)]
        goal: u32,
        
        /// Heuristic function to use
        #[arg(short = 'H', long, default_value = "manhattan")]
        heuristic: HeuristicType,
        
        /// Graph file (overrides global input)
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
    
    /// Analyze graph properties
    Analyze {
        /// Show detailed statistics
        #[arg(short, long)]
        detailed: bool,
        
        /// Export analysis to file
        #[arg(short, long)]
        export: Option<PathBuf>,
    },
}

#[derive(Clone, Debug, clap::ValueEnum)]
enum OutputFormat {
    Json,
    Yaml,
    Plain,
}

#[derive(Clone, Debug, clap::ValueEnum)]
enum HeuristicType {
    Manhattan,
    Euclidean,
    Chebyshev,
}

fn main() {
    let cli = Cli::parse();
    
    if cli.debug {
        eprintln!("Debug mode enabled");
        eprintln!("CLI args: {:#?}", cli);
    }
    
    let result = match cli.command {
        Commands::Dijkstra { start, goal, file } => {
            let input_file = file.or(cli.input).unwrap_or_else(|| {
                eprintln!("Error: No input file specified");
                std::process::exit(1);
            });
            
            run_dijkstra(start, goal, &input_file, &cli)
        }
        
        Commands::Astar { start, goal, heuristic, file } => {
            let input_file = file.or(cli.input).unwrap_or_else(|| {
                eprintln!("Error: No input file specified");
                std::process::exit(1);
            });
            
            run_astar(start, goal, heuristic, &input_file, &cli)
        }
        
        Commands::Analyze { detailed, export } => {
            let input_file = cli.input.unwrap_or_else(|| {
                eprintln!("Error: No input file specified for analysis");
                std::process::exit(1);
            });
            
            run_analysis(&input_file, detailed, export, &cli)
        }
    };
    
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
```

### **Validation and Custom Types**
```rust
use clap::{Parser, builder::TypedValueParser};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct NodeId(u32);

impl FromStr for NodeId {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id: u32 = s.parse().map_err(|_| "Node ID must be a positive integer")?;
        if id == 0 {
            return Err("Node ID cannot be zero".to_string());
        }
        Ok(NodeId(id))
    }
}

#[derive(Debug, Clone)]
struct WeightRange {
    min: f64,
    max: f64,
}

impl FromStr for WeightRange {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err("Weight range must be in format 'min-max'".to_string());
        }
        
        let min: f64 = parts[0].parse().map_err(|_| "Invalid minimum weight")?;
        let max: f64 = parts[1].parse().map_err(|_| "Invalid maximum weight")?;
        
        if min >= max {
            return Err("Minimum weight must be less than maximum".to_string());
        }
        
        if min < 0.0 {
            return Err("Weights must be non-negative".to_string());
        }
        
        Ok(WeightRange { min, max })
    }
}

#[derive(Parser)]
struct GraphGenerator {
    /// Number of nodes to generate
    #[arg(short, long, value_parser = validate_node_count)]
    nodes: u32,
    
    /// Edge density (0.0 to 1.0)
    #[arg(short, long, value_parser = validate_density)]
    density: f64,
    
    /// Weight range for edges
    #[arg(short, long, default_value = "1.0-10.0")]
    weights: WeightRange,
    
    /// Starting node (must be within node count)
    #[arg(short, long)]
    start: NodeId,
}

fn validate_node_count(s: &str) -> Result<u32, String> {
    let count: u32 = s.parse().map_err(|_| "Node count must be a positive integer")?;
    if count == 0 {
        return Err("Node count must be at least 1".to_string());
    }
    if count > 100_000 {
        return Err("Node count too large (maximum 100,000)".to_string());
    }
    Ok(count)
}

fn validate_density(s: &str) -> Result<f64, String> {
    let density: f64 = s.parse().map_err(|_| "Density must be a number")?;
    if density < 0.0 || density > 1.0 {
        return Err("Density must be between 0.0 and 1.0".to_string());
    }
    Ok(density)
}
```

---

## 🎨 **User Experience Patterns**

### **Progress Bars and Status Updates**
```rust
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use std::time::Duration;
use std::thread;

fn process_large_graph(nodes: u32) -> Result<(), Box<dyn std::error::Error>> {
    let multi = MultiProgress::new();
    
    // Overall progress
    let overall = multi.add(ProgressBar::new(100));
    overall.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")?
            .progress_chars("#>-")
    );
    overall.set_message("Processing graph");
    
    // Detailed progress for current operation
    let detail = multi.add(ProgressBar::new(nodes as u64));
    detail.set_style(
        ProgressStyle::default_bar()
            .template("  {msg} [{wide_bar:.green}] {pos}/{len}")?
            .progress_chars("█▉▊▋▌▍▎▏  ")
    );
    
    // Phase 1: Load graph
    detail.set_message("Loading nodes");
    for i in 0..nodes {
        thread::sleep(Duration::from_millis(1)); // Simulate work
        detail.inc(1);
    }
    detail.finish_with_message("Nodes loaded");
    overall.inc(30);
    
    // Phase 2: Build adjacency lists
    detail.reset();
    detail.set_length(nodes as u64 * 2); // Simulate more work
    detail.set_message("Building adjacency lists");
    for i in 0..(nodes * 2) {
        thread::sleep(Duration::from_millis(1));
        detail.inc(1);
    }
    detail.finish_with_message("Adjacency lists built");
    overall.inc(40);
    
    // Phase 3: Run algorithm
    detail.reset();
    detail.set_length(nodes as u64);
    detail.set_message("Running pathfinding");
    for i in 0..nodes {
        thread::sleep(Duration::from_millis(2)); // Simulate algorithm work
        detail.inc(1);
    }
    detail.finish_with_message("Pathfinding complete");
    overall.inc(30);
    
    overall.finish_with_message("Graph processing complete");
    
    Ok(())
}
```

### **Colored Output and Formatting**
```rust
use colored::*;
use std::fmt;

pub struct PathResult {
    pub path: Vec<u32>,
    pub cost: f64,
    pub nodes_explored: usize,
    pub algorithm: String,
}

impl fmt::Display for PathResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", "=== PATHFINDING RESULT ===".bold().cyan())?;
        writeln!(f, "{}: {}", "Algorithm".bold(), self.algorithm.green())?;
        writeln!(f, "{}: {}", "Path".bold(), 
                 self.path.iter()
                     .map(|n| n.to_string())
                     .collect::<Vec<_>>()
                     .join(" → ").yellow())?;
        writeln!(f, "{}: {:.2}", "Total Cost".bold(), self.cost.to_string().magenta())?;
        writeln!(f, "{}: {}", "Nodes Explored".bold(), self.nodes_explored.to_string().blue())?;
        
        // Color-code efficiency
        let efficiency_ratio = self.path.len() as f64 / self.nodes_explored as f64;
        let efficiency_msg = if efficiency_ratio > 0.1 {
            "Highly Efficient".green()
        } else if efficiency_ratio > 0.01 {
            "Moderately Efficient".yellow()
        } else {
            "Low Efficiency".red()
        };
        
        writeln!(f, "{}: {}", "Efficiency".bold(), efficiency_msg)?;
        Ok(())
    }
}

pub fn print_error(message: &str) {
    eprintln!("{} {}", "Error:".red().bold(), message);
}

pub fn print_warning(message: &str) {
    eprintln!("{} {}", "Warning:".yellow().bold(), message);
}

pub fn print_success(message: &str) {
    println!("{} {}", "✓".green().bold(), message);
}

pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".blue().bold(), message);
}
```

### **Interactive Prompts**
```rust
use dialoguer::{Confirm, Select, Input, MultiSelect};

fn interactive_pathfinding() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "🗺️  Interactive Pathfinding Tool".bold().cyan());
    
    // Algorithm selection
    let algorithms = &["Dijkstra", "A* (Manhattan)", "A* (Euclidean)", "BFS (unweighted)"];
    let algorithm_index = Select::new()
        .with_prompt("Choose pathfinding algorithm")
        .items(algorithms)
        .default(0)
        .interact()?;
    
    // Input method selection
    let input_methods = &["Load from file", "Generate random graph", "Manual input"];
    let input_method = Select::new()
        .with_prompt("How would you like to provide the graph?")
        .items(input_methods)
        .default(0)
        .interact()?;
    
    match input_method {
        0 => {
            // File input
            let filename: String = Input::new()
                .with_prompt("Graph file path")
                .default("graph.txt".to_string())
                .interact_text()?;
            
            if !std::path::Path::new(&filename).exists() {
                print_error(&format!("File '{}' does not exist", filename));
                return Ok(());
            }
        }
        1 => {
            // Random graph generation
            let node_count: u32 = Input::new()
                .with_prompt("Number of nodes")
                .default(100)
                .interact()?;
            
            let density: f64 = Input::new()
                .with_prompt("Edge density (0.0 to 1.0)")
                .default(0.1)
                .validate_with(|input: &f64| -> Result<(), &str> {
                    if *input >= 0.0 && *input <= 1.0 {
                        Ok(())
                    } else {
                        Err("Density must be between 0.0 and 1.0")
                    }
                })
                .interact()?;
        }
        2 => {
            print_info("Manual input mode - enter edges in format 'from to weight'");
            print_info("Enter empty line when done");
            // Manual input implementation
        }
        _ => unreachable!(),
    }
    
    // Start and goal nodes
    let start_node: u32 = Input::new()
        .with_prompt("Start node ID")
        .default(0)
        .interact()?;
    
    let goal_node: u32 = Input::new()
        .with_prompt("Goal node ID")
        .default(10)
        .interact()?;
    
    // Additional options
    let options = &["Show detailed steps", "Export result to file", "Benchmark performance"];
    let selected_options = MultiSelect::new()
        .with_prompt("Additional options (use space to select)")
        .items(options)
        .interact()?;
    
    // Confirmation
    let proceed = Confirm::new()
        .with_prompt("Proceed with pathfinding?")
        .default(true)
        .interact()?;
    
    if proceed {
        print_success("Starting pathfinding...");
        // Run the selected algorithm
    } else {
        print_info("Operation cancelled");
    }
    
    Ok(())
}
```

---

## 🏆 **Mission-Specific CLI Patterns**

### **Pathfinding CLI (Mission 9)**
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pathfinder")]
#[command(about = "Advanced pathfinding algorithms for graph analysis")]
struct PathfinderCli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Input graph format
    #[arg(long, default_value = "adjacency-list")]
    input_format: GraphFormat,
    
    /// Output format for results
    #[arg(short, long, default_value = "human")]
    output: OutputFormat,
    
    #[command(subcommand)]
    command: PathfinderCommands,
}

#[derive(Subcommand)]
enum PathfinderCommands {
    /// Run Dijkstra's shortest path algorithm
    Dijkstra {
        /// Graph file path
        graph: PathBuf,
        
        /// Start node
        #[arg(short, long)]
        start: u32,
        
        /// Goal node (optional, finds shortest paths to all nodes if omitted)
        #[arg(short, long)]
        goal: Option<u32>,
        
        /// Show algorithm steps
        #[arg(long)]
        trace: bool,
    },
    
    /// Run A* pathfinding with heuristic
    Astar {
        /// Graph file path
        graph: PathBuf,
        
        /// Start node
        #[arg(short, long)]
        start: u32,
        
        /// Goal node
        #[arg(short, long)]
        goal: u32,
        
        /// Heuristic function
        #[arg(short = 'H', long, default_value = "manhattan")]
        heuristic: HeuristicType,
        
        /// Show algorithm steps
        #[arg(long)]
        trace: bool,
    },
    
    /// Compare multiple algorithms on the same graph
    Compare {
        /// Graph file path
        graph: PathBuf,
        
        /// Start node
        #[arg(short, long)]
        start: u32,
        
        /// Goal node
        #[arg(short, long)]
        goal: u32,
        
        /// Algorithms to compare
        #[arg(long, default_values = ["dijkstra", "astar"])]
        algorithms: Vec<String>,
        
        /// Number of benchmark runs
        #[arg(long, default_value = "10")]
        runs: usize,
    },
    
    /// Analyze graph properties
    Info {
        /// Graph file path
        graph: PathBuf,
        
        /// Show detailed analysis
        #[arg(short, long)]
        detailed: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = PathfinderCli::parse();
    
    match cli.command {
        PathfinderCommands::Dijkstra { graph, start, goal, trace } => {
            if cli.verbose {
                print_info(&format!("Loading graph from {:?}", graph));
            }
            
            let graph_data = load_graph(&graph, cli.input_format)?;
            
            match goal {
                Some(goal_node) => {
                    let result = run_dijkstra_single(&graph_data, start, goal_node, trace)?;
                    output_result(&result, &cli.output);
                }
                None => {
                    let results = run_dijkstra_all(&graph_data, start, trace)?;
                    output_all_paths(&results, &cli.output);
                }
            }
        }
        
        PathfinderCommands::Compare { graph, start, goal, algorithms, runs } => {
            let graph_data = load_graph(&graph, cli.input_format)?;
            let comparison = benchmark_algorithms(&graph_data, start, goal, &algorithms, runs)?;
            output_comparison(&comparison, &cli.output);
        }
        
        // Other command implementations...
        _ => todo!(),
    }
    
    Ok(())
}
```

### **Graph Analysis CLI (Mission 7)**
```rust
#[derive(Parser)]
#[command(name = "graph-analyzer")]
#[command(about = "Graph structure analysis and visualization tools")]
struct GraphAnalyzer {
    /// Input graph file
    graph: PathBuf,
    
    /// Analysis depth
    #[arg(short, long, default_value = "basic")]
    analysis: AnalysisLevel,
    
    /// Export results to file
    #[arg(short, long)]
    export: Option<PathBuf>,
    
    /// Generate visualization
    #[arg(short, long)]
    visualize: bool,
    
    #[command(subcommand)]
    command: Option<AnalysisCommands>,
}

#[derive(Subcommand)]
enum AnalysisCommands {
    /// Find connected components
    Components,
    
    /// Detect cycles
    Cycles {
        /// Algorithm to use for cycle detection
        #[arg(short, long, default_value = "dfs")]
        algorithm: CycleDetectionAlgorithm,
    },
    
    /// Calculate graph metrics
    Metrics {
        /// Include expensive metrics
        #[arg(long)]
        expensive: bool,
    },
    
    /// Generate graph visualization
    Visualize {
        /// Output format
        #[arg(short, long, default_value = "dot")]
        format: VisualizationFormat,
        
        /// Layout algorithm
        #[arg(short, long, default_value = "spring")]
        layout: LayoutAlgorithm,
    },
}
```

---

## 🔗 **Integration with Learning System**

### **Mission Integration**
- **[[Mission9 Overview]]** - Pathfinding CLI tools for Dijkstra and A* algorithms
- **[[Mission7 Overview]]** - Graph analysis CLI for structure analysis and visualization
- **[[Mission12 Overview]]** - Parser CLI tools for custom input format processing

### **Pattern Integration**
- **[[Custom Error Types]]** - CLI error handling with domain-specific error types
- **[[API Design Patterns]]** - Command pattern implementation in CLI tools
- **[[Testing Patterns]]** - CLI testing strategies and integration testing

### **Rust Book Integration**
- **[[Rust Book MOC]]** - Chapter 12 I/O project with CLI argument handling
- **Command line parsing** with `std::env` and advanced crate integration
- **Error handling** patterns specific to CLI applications

### **Daily Study Integration**
- **[[Daily Study MOC]]** - Week 5 CLI development and user interface design
- **Environment variable integration** for configuration management
- **Cross-platform considerations** for CLI tool deployment

---

## 📚 **External Resources**

### **Essential Crates**
- **[clap](https://docs.rs/clap/)** - Command line argument parsing (most popular)
- **[structopt](https://docs.rs/structopt/)** - Deprecated, use clap's derive API instead
- **[colored](https://docs.rs/colored/)** - Terminal color output
- **[indicatif](https://docs.rs/indicatif/)** - Progress bars and spinners
- **[dialoguer](https://docs.rs/dialoguer/)** - Interactive CLI prompts

### **Advanced CLI Features**
- **[termion](https://docs.rs/termion/)** - Low-level terminal control
- **[crossterm](https://docs.rs/crossterm/)** - Cross-platform terminal manipulation
- **[tui-rs](https://docs.rs/tui/)** - Terminal user interfaces
- **[console](https://docs.rs/console/)** - Terminal utilities and styling

### **Best Practices**
- **[Command Line Interface Guidelines](https://clig.dev/)** - General CLI design principles
- **[The Rust CLI Book](https://rust-cli.github.io/book/)** - Comprehensive CLI development guide
- **[CLI Testing Strategies](https://rust-cli.github.io/book/tutorial/testing.html)** - Testing CLI applications

---

*Tags: #cli #command-line #clap #argument-parsing #user-experience #mission9 #pathfinding #rust #terminal*
*Links: [[zettel-index]] | [[rust-book-ch9-12-review]] | [[Standard Error and Stream Separation]] | [[Custom Error Types]] | [[API Design Patterns]] | [[Testing Patterns]] | [[Mission9 Overview]] | [[Mission7 Overview]] | [[Rust Book MOC]] | [[Daily Study MOC]]*