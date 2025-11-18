use utoipa::OpenApi;
use crate::models::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::create_instance,
        crate::handlers::union_elements,
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
        (name = "Operations", description = "Core Union-Find operations")
    ),
    info(
        title = "Mission 10 Union-Find API",
        version = "1.0.0",
        description = "REST API for Mission 10 Union-Find implementation"
    )
)]
pub struct ApiDoc;
