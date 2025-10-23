//! Error types for pathfinding operations

use thiserror::Error;

/// Pathfinding-specific error types
#[derive(Error, Debug, Clone, PartialEq)]
pub enum PathfindingError {
    #[error("No path exists from {start} to {goal}")]
    NoPathExists { start: u32, goal: u32 },

    #[error("Invalid start node: {node}")]
    InvalidStartNode { node: u32 },

    #[error("Invalid goal node: {node}")]
    InvalidGoalNode { node: u32 },

    #[error("Graph contains negative edge weights (not supported by Dijkstra)")]
    NegativeWeights,

    #[error("Heuristic function returned invalid value: {value}")]
    InvalidHeuristic { value: f64 },

    #[error("Priority queue error: {message}")]
    QueueError { message: String },

    #[error("Graph is empty or disconnected")]
    InvalidGraph,

    #[error("Algorithm timeout after {seconds} seconds")]
    Timeout { seconds: u64 },
}

impl PathfindingError {
    /// Check if the error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            PathfindingError::NoPathExists { .. }
                | PathfindingError::InvalidHeuristic { .. }
                | PathfindingError::Timeout { .. }
        )
    }

    /// Get error severity level
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            PathfindingError::NoPathExists { .. } => ErrorSeverity::Warning,
            PathfindingError::InvalidStartNode { .. }
            | PathfindingError::InvalidGoalNode { .. } => ErrorSeverity::Fatal,
            PathfindingError::NegativeWeights | PathfindingError::InvalidGraph => {
                ErrorSeverity::Fatal
            }
            PathfindingError::InvalidHeuristic { .. } => ErrorSeverity::Warning,
            PathfindingError::QueueError { .. } => ErrorSeverity::Fatal,
            PathfindingError::Timeout { .. } => ErrorSeverity::Recoverable,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    Warning,     // Log and continue with alternative
    Recoverable, // Retry with different parameters
    Fatal,       // Stop execution
}
