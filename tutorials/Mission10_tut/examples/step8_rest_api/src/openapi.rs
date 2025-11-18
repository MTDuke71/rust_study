use utoipa::OpenApi;
use crate::models::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::create_instance,
        crate::handlers::union_elements,
        // Add other handlers here
    ),
    components(
        schemas(
            CreateRequest,
            CreateResponse,
            UnionRequest,
            UnionResponse,
            FindResponse,
            ConnectedResponse,
            ErrorResponse
        )
    ),
    tags(
        (name = "Union-Find Management", description = "Lifecycle operations for Union-Find instances"),
        (name = "Operations", description = "Core Union-Find operations (union, find, connected)")
    ),
    info(
        title = "Union-Find REST API",
        version = "1.0.0",
        description = "A production-ready REST API for the Union-Find data structure"
    )
)]
pub struct ApiDoc;
