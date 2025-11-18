use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Request to create a new Union-Find instance
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateRequest {
    /// Number of elements in the set
    #[schema(example = 10, minimum = 1)]
    pub size: usize,
}

/// Response for instance creation
#[derive(Debug, Serialize, ToSchema)]
pub struct CreateResponse {
    /// Unique identifier for the instance
    pub id: Uuid,
    /// Number of elements
    pub size: usize,
}

/// Request to union two elements
#[derive(Debug, Deserialize, ToSchema)]
pub struct UnionRequest {
    /// First element index
    pub element1: usize,
    /// Second element index
    pub element2: usize,
}

/// Response for union operation
#[derive(Debug, Serialize, ToSchema)]
pub struct UnionResponse {
    /// Whether the elements were successfully merged (false if already connected)
    pub merged: bool,
    /// The new root of the set
    pub root: usize,
}

/// Response for find operation
#[derive(Debug, Serialize, ToSchema)]
pub struct FindResponse {
    /// The element queried
    pub element: usize,
    /// The representative (root) of the set
    pub root: usize,
}

/// Request for find operation (query parameters)
#[derive(Debug, Deserialize, ToSchema)]
pub struct FindRequest {
    /// Element to find root for
    pub element: usize,
}

/// Response for connectivity check
#[derive(Debug, Serialize, ToSchema)]
pub struct ConnectedResponse {
    /// Whether the elements are in the same set
    pub connected: bool,
}

/// Request for connectivity check (query parameters)
#[derive(Debug, Deserialize, ToSchema)]
pub struct ConnectedRequest {
    /// First element
    pub element1: usize,
    /// Second element
    pub element2: usize,
}

/// Response for statistics endpoint
#[derive(Debug, Serialize, ToSchema)]
pub struct StatsResponse {
    /// Total number of elements
    pub total_elements: usize,
    /// Number of disjoint sets
    pub num_components: usize,
}

/// Standard error response
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
}
