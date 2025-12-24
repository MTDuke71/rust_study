use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Request body for creating a new Union-Find instance
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateRequest {
    /// Number of elements in the set (must be > 0)
    #[schema(example = 10, minimum = 1)]
    pub size: usize,
}

/// Response after creating a Union-Find instance
#[derive(Debug, Serialize, ToSchema)]
pub struct CreateResponse {
    /// Unique identifier for the instance
    pub id: Uuid,
    
    /// Number of elements in the set
    pub size: usize,
}

/// Request body for union operation
#[derive(Debug, Deserialize, ToSchema)]
pub struct UnionRequest {
    /// First element to union
    #[schema(example = 3, minimum = 0)]
    pub element1: usize,
    
    /// Second element to union
    #[schema(example = 7, minimum = 0)]
    pub element2: usize,
}

/// Result of union operation
#[derive(Debug, Serialize, ToSchema)]
pub struct UnionResponse {
    /// Whether the elements were successfully merged (false if already connected)
    pub merged: bool,
    
    /// The new root of the set
    pub root: usize,
}

/// Result of find operation
#[derive(Debug, Serialize, ToSchema)]
pub struct FindResponse {
    /// The element that was queried
    #[schema(example = 5)]
    pub element: usize,
    
    /// Root element of the set
    #[schema(example = 3)]
    pub root: usize,
}

/// Query parameters for find operation
#[derive(Debug, Deserialize, ToSchema)]
pub struct FindRequest {
    /// Element to find the root of
    #[schema(example = 5, minimum = 0)]
    pub element: usize,
}

/// Result of connected check
#[derive(Debug, Serialize, ToSchema)]
pub struct ConnectedResponse {
    /// Whether elements are in the same set
    pub connected: bool,
}

/// Query parameters for connected check
#[derive(Debug, Deserialize, ToSchema)]
pub struct ConnectedRequest {
    /// First element to check
    #[schema(example = 3, minimum = 0)]
    pub element1: usize,
    
    /// Second element to check
    #[schema(example = 7, minimum = 0)]
    pub element2: usize,
}

/// Statistics about a Union-Find instance
#[derive(Debug, Serialize, ToSchema)]
pub struct StatsResponse {
    /// Total number of elements
    pub total_elements: usize,
    
    /// Number of disjoint sets
    pub num_components: usize,
}

/// Error response structure
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Error code
    pub code: String,
    
    /// Human-readable error message
    pub message: String,
}
