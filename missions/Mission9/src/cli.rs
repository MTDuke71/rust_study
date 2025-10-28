/// Command-line interface for Mission 9 pathfinding algorithms
///
/// REQ-7: Production-ready CLI features including:
/// - Interactive command-line interface
/// - Graph loading from multiple formats
/// - Visualization output generation
/// - Batch processing capabilities
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "mission9",
    about = "Advanced Pathfinding Algorithms - Mission 9",
    long_about = "CLI for Dijkstra's and A* pathfinding algorithms with support for various graph formats and visualization",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Output format
    #[arg(short, long, value_enum, default_value = "text", global = true)]
    pub output: OutputFormat,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Find a path between two nodes using specified algorithm
    FindPath {
        /// Input graph file (JSON, CSV, or GraphML format)
        #[arg(short, long)]
        graph: PathBuf,

        /// Start node ID
        #[arg(short, long)]
        start: u32,

        /// Goal node ID
        #[arg(short = 'g', long)]
        goal: u32,

        /// Pathfinding algorithm to use
        #[arg(short, long, value_enum, default_value = "astar")]
        algorithm: AlgorithmType,

        /// Heuristic function (for A* algorithm)
        #[arg(short = 'H', long, value_enum)]
        heuristic: Option<HeuristicType>,

        /// Save visualization output to file
        #[arg(short = 'v', long)]
        visualize: Option<PathBuf>,
    },

    /// Run batch pathfinding on multiple queries
    Batch {
        /// Input graph file
        #[arg(short, long)]
        graph: PathBuf,

        /// Batch query file (CSV with start,goal columns)
        #[arg(short, long)]
        queries: PathBuf,

        /// Output file for results
        #[arg(short, long)]
        output: PathBuf,

        /// Algorithm to use for all queries
        #[arg(short, long, value_enum, default_value = "astar")]
        algorithm: AlgorithmType,
    },

    /// Display information about a graph
    Info {
        /// Input graph file
        #[arg(short, long)]
        graph: PathBuf,
    },

    /// Benchmark pathfinding algorithms
    Benchmark {
        /// Input graph file
        #[arg(short, long)]
        graph: PathBuf,

        /// Start node ID
        #[arg(short, long)]
        start: u32,

        /// Goal node ID
        #[arg(short = 'g', long)]
        goal: u32,

        /// Number of iterations for each algorithm
        #[arg(short, long, default_value = "100")]
        iterations: usize,
    },

    /// Generate a sample graph for testing
    Generate {
        /// Graph type to generate
        #[arg(short, long, value_enum)]
        graph_type: GraphType,

        /// Number of nodes
        #[arg(short, long, default_value = "100")]
        nodes: usize,

        /// Output file path
        #[arg(short = 'f', long)]
        file: PathBuf,

        /// Graph file format
        #[arg(long, value_enum, default_value = "json")]
        format: GraphFormat,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum AlgorithmType {
    /// Dijkstra's shortest path algorithm
    Dijkstra,
    /// A* pathfinding algorithm
    Astar,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum HeuristicType {
    /// Manhattan distance (L1 norm)
    Manhattan,
    /// Euclidean distance (L2 norm)
    Euclidean,
    /// Chebyshev distance (L∞ norm)
    Chebyshev,
    /// Zero heuristic (equivalent to Dijkstra)
    Zero,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    /// Plain text output
    Text,
    /// JSON output
    Json,
    /// CSV output
    Csv,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum GraphType {
    /// Random graph with specified density
    Random,
    /// Grid graph (useful for testing A*)
    Grid,
    /// Tree graph
    Tree,
    /// Complete graph
    Complete,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum GraphFormat {
    /// JSON format
    Json,
    /// CSV edge list format
    Csv,
}

impl std::fmt::Display for AlgorithmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlgorithmType::Dijkstra => write!(f, "Dijkstra"),
            AlgorithmType::Astar => write!(f, "A*"),
        }
    }
}

impl std::fmt::Display for HeuristicType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeuristicType::Manhattan => write!(f, "Manhattan"),
            HeuristicType::Euclidean => write!(f, "Euclidean"),
            HeuristicType::Chebyshev => write!(f, "Chebyshev"),
            HeuristicType::Zero => write!(f, "Zero"),
        }
    }
}
