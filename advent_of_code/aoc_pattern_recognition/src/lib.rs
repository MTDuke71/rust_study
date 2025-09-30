//! # AoC Pattern Recognition Library
//! 
//! A comprehensive pattern recognition system for Advent of Code preparation.
//! This library teaches you to identify and implement the most common AoC problem patterns.
//!
//! ## Requirements Fulfilled
//! - **REQ-P1**: Grid navigation and manipulation patterns
//! - **REQ-P2**: Input parsing and transformation patterns  
//! - **REQ-P3**: State management and optimization patterns
//! - **REQ-P4**: Performance-aware pattern implementations
//! - **REQ-P5**: Pattern recognition training and validation
//!
//! ## Core Pattern Categories
//!
//! ### 🔲 Grid Patterns
//! Most AoC problems involving 2D navigation, flood fill, pathfinding
//! ```
//! use aoc_pattern_recognition::grid_patterns::*;
//! 
//! // Identify grid navigation problems
//! let grid = Grid::from_string("..#.\n.#..\n..#.");
//! let start = Coord { x: 0, y: 0 };
//! let end = Coord { x: 2, y: 2 };
//! // Use grid.shortest_path(start, end) or implement pathfinding algorithm
//! ```
//!
//! ### 📝 Parsing Patterns  
//! Efficient input transformation - the foundation of fast AoC solving
//! ```
//! use aoc_pattern_recognition::parsing_patterns::*;
//!
//! // Recognize parsing scenarios
//! let parser = InstructionParser;
//! let instruction = parser.parse_line("move 3 from 2 to 1").unwrap();
//! assert_eq!(instruction.command, "move");
//! assert_eq!(instruction.parameters, vec!["3", "from", "2", "to", "1"]);
//! ```
//!
//! ### 🗃️ State Management Patterns
//! Complex state tracking, memoization, optimization strategies
//! ```
//! use aoc_pattern_recognition::state_patterns::*;
//!
//! // Handle complex state problems
//! let mut cache = MemoizationCache::new();
//! cache.insert("key", 42);
//! let value = cache.get(&"key").unwrap();
//! assert_eq!(*value, 42);
//! ```

pub mod grid_patterns;
pub mod parsing_patterns; 
pub mod state_patterns;
pub mod pattern_trainer;

use std::fmt;

/// Common result type for pattern recognition operations
pub type PatternResult<T> = Result<T, PatternError>;

/// Errors that can occur during pattern recognition
#[derive(Debug, Clone, PartialEq)]
pub enum PatternError {
    InvalidInput(String),
    PatternNotFound(String),
    PerformanceThresholdExceeded(String),
    UnrecognizedStructure(String),
}

impl fmt::Display for PatternError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatternError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            PatternError::PatternNotFound(msg) => write!(f, "Pattern not found: {}", msg),
            PatternError::PerformanceThresholdExceeded(msg) => write!(f, "Performance threshold exceeded: {}", msg),
            PatternError::UnrecognizedStructure(msg) => write!(f, "Unrecognized structure: {}", msg),
        }
    }
}

impl std::error::Error for PatternError {}

/// Trait for AoC pattern recognition
pub trait AocPattern {
    type Input;
    type Output;
    
    /// Identify if the given input matches this pattern
    fn matches(&self, input: &Self::Input) -> bool;
    
    /// Apply the pattern to solve the problem
    fn solve(&self, input: Self::Input) -> PatternResult<Self::Output>;
    
    /// Get pattern complexity estimate (for performance awareness)
    fn complexity(&self) -> PatternComplexity;
    
    /// Get pattern name for debugging/learning
    fn pattern_name(&self) -> &'static str;
}

/// Pattern complexity categories for performance awareness
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternComplexity {
    /// O(1) or O(log n) - Very fast
    Constant,
    /// O(n) or O(n log n) - Reasonable for most inputs
    Linear,
    /// O(n²) or O(n³) - Acceptable for small/medium inputs
    Polynomial,
    /// O(2^n) or worse - Only for very small inputs
    Exponential,
}

impl fmt::Display for PatternComplexity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatternComplexity::Constant => write!(f, "O(1) - Very Fast"),
            PatternComplexity::Linear => write!(f, "O(n) - Fast"),  
            PatternComplexity::Polynomial => write!(f, "O(n²) - Moderate"),
            PatternComplexity::Exponential => write!(f, "O(2^n) - Slow"),
        }
    }
}