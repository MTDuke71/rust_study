//! # Mission 9 Tutorial: Pathfinding Algorithms
//!
//! This library contains step-by-step tutorial examples for learning
//! advanced pathfinding algorithms including Dijkstra's algorithm and A*.
//!
//! ## Tutorial Structure
//!
//! The tutorial is organized into progressive steps:
//!
//! 1. **Priority Queue Foundation** - Learn the core data structure
//! 2. **Dijkstra's Algorithm** - Implement shortest path finding
//! 3. **A* Search** - Add heuristic-guided pathfinding
//! 4. **Performance Optimizations** - Bidirectional search and improvements
//! 5. **Advanced Heuristics** - Multi-objective and custom heuristics
//! 6. **Hierarchical Methods** - Large-scale pathfinding techniques
//! 7. **Real-World Applications** - CLI tools and practical integration
//!
//! ## Running Tutorial Steps
//!
//! Each step can be run as an example:
//!
//! ```bash
//! cargo run --bin step1_priority_queue
//! ```
//!
//! ## Learning Objectives
//!
//! By completing this tutorial, you will understand:
//! - Priority queue implementations and their role in pathfinding
//! - Core pathfinding algorithms (Dijkstra, A*)
//! - Heuristic design and admissibility concepts
//! - Performance optimization techniques
//! - Real-world application patterns

/// Common utilities and helper functions for tutorial examples
pub mod utils {
    use std::time::Instant;

    /// Simple timer for measuring tutorial example performance
    pub struct Timer {
        start: Instant,
    }

    impl Timer {
        pub fn start() -> Self {
            Self {
                start: Instant::now(),
            }
        }

        pub fn elapsed_ms(&self) -> u64 {
            self.start.elapsed().as_millis() as u64
        }

        pub fn elapsed_micros(&self) -> u64 {
            self.start.elapsed().as_micros() as u64
        }
    }

    /// Print a formatted section header for tutorial output
    pub fn print_section(title: &str) {
        println!("\n📚 {}", title);
        println!("{}", "=".repeat(title.len() + 3));
    }

    /// Print a formatted exercise header
    pub fn print_exercise(number: usize, title: &str) {
        println!("\n🔧 Exercise {}: {}", number, title);
        println!("{}", "-".repeat(title.len() + 15));
    }

    /// Print test results with formatting
    pub fn print_result(test_name: &str, passed: bool, details: Option<&str>) {
        let icon = if passed { "✅" } else { "❌" };
        println!("{} {}", icon, test_name);

        if let Some(details) = details {
            println!("   {}", details);
        }
    }
}
