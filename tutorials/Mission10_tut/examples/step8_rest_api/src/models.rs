use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use std::collections::HashMap;

/// Request body for creating a new Union-Find instance
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateRequest {
    /// Number of elements in the set (must be > 0)
    /// 
    /// Common use cases:
    /// - 10: Small network
    /// - 100: Medium network or 10x10 percolation grid
    /// - 1000: Large network
    /// - 1920: HD image segments
    /// 
    /// Larger values may impact performance.
    #[schema(example = 100, minimum = 1)]
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

/// Standard error response structure
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Machine-readable error code
    #[schema(example = "INVALID_SIZE")]
    pub code: String,
    
    /// Human-readable error message
    #[schema(example = "Size must be greater than 0")]
    pub message: String,
    
    /// Optional additional error details
    #[schema(nullable = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<HashMap<String, serde_json::Value>>,
    
    /// Optional field-level validation errors
    #[schema(nullable = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_errors: Option<HashMap<String, String>>,
}

impl ErrorResponse {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: None,
            field_errors: None,
        }
    }
    
    pub fn with_details(mut self, details: HashMap<String, serde_json::Value>) -> Self {
        self.details = Some(details);
        self
    }
    
    #[allow(dead_code)] // Used in OpenAPI documentation examples
    pub fn with_field_errors(mut self, errors: HashMap<String, String>) -> Self {
        self.field_errors = Some(errors);
        self
    }
}

/// Common error codes for OpenAPI documentation
pub mod error_codes {
    pub const INVALID_SIZE: &str = "INVALID_SIZE";
    pub const INVALID_UNION: &str = "INVALID_UNION";
    pub const ELEMENT_OUT_OF_BOUNDS: &str = "ELEMENT_OUT_OF_BOUNDS";
    pub const INSTANCE_NOT_FOUND: &str = "INSTANCE_NOT_FOUND";
    
    // Reserved for future use. Currently, invalid UUID format fails at Axum's
    // Path<Uuid> extractor before reaching handler code, resulting in a generic
    // 400 error. Custom validation middleware would be needed to use this code.
    #[allow(dead_code)]
    pub const INVALID_UUID: &str = "INVALID_UUID";
    
    #[allow(dead_code)] // Reserved for internal error handling
    pub const INTERNAL_ERROR: &str = "INTERNAL_ERROR";
    
    #[allow(dead_code)] // Reserved for union operation optimization
    pub const ELEMENTS_ALREADY_CONNECTED: &str = "ELEMENTS_ALREADY_CONNECTED";
}
