use utoipa::OpenApi;
use crate::models::*;

/// OpenAPI specification with advanced documentation features
/// Includes security schemes, detailed examples, and comprehensive metadata
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::create_instance,
        crate::handlers::union_elements,
        crate::handlers::find_element,
        crate::handlers::check_connected,
        crate::handlers::get_stats,
        crate::handlers::delete_instance,
    ),
    components(
        schemas(
            CreateRequest,
            CreateResponse,
            UnionRequest,
            UnionResponse,
            FindRequest,
            FindResponse,
            ConnectedRequest,
            ConnectedResponse,
            StatsResponse,
            ErrorResponse
        )
    ),
    tags(
        (name = "Union-Find Management", description = "Lifecycle operations for Union-Find instances"),
        (name = "Operations", description = "Core Union-Find operations")
    ),
    info(
        title = "Mission 10 Union-Find API",
        version = "1.0.0",
        description = "REST API for Mission 10 Union-Find implementation with path compression and union by rank optimizations.\n\n**Features:**\n- Path compression for O(α(n)) amortized find operations\n- Union by rank for balanced tree structures\n- Thread-safe concurrent access\n- Complete OpenAPI 3.0 documentation"
    )
)]
pub struct ApiDoc;
