use axum::{
    routing::{post, get, delete},
    Router,
    extract::{Path, State, Json, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use uuid::Uuid;
use std::collections::HashMap;
use serde_json::json;
use crate::{
    models::{CreateRequest, CreateResponse, UnionRequest, UnionResponse, FindRequest, FindResponse, ConnectedRequest, ConnectedResponse, StatsResponse, ErrorResponse, error_codes},
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/unionfind", post(create_instance))
        .route("/unionfind/:id/union", post(union_elements))
        .route("/unionfind/:id/find", get(find_element))
        .route("/unionfind/:id/connected", get(check_connected))
        .route("/unionfind/:id/stats", get(get_stats))
        .route("/unionfind/:id", delete(delete_instance))
}

// Semantic error codes with details
fn error_with_code(
    status: StatusCode,
    error_code: &str,
    message: impl Into<String>,
) -> Response {
    (
        status,
        Json(ErrorResponse::new(error_code, message)),
    ).into_response()
}

// Error with structured details
fn error_with_details(
    status: StatusCode,
    error_code: &str,
    message: impl Into<String>,
    details: HashMap<String, serde_json::Value>,
) -> Response {
    (
        status,
        Json(ErrorResponse::new(error_code, message).with_details(details)),
    ).into_response()
}

/// Create a new Union-Find instance with the specified number of elements. Each element starts in its own separate set.
///
/// Creates an isolated disjoint-set data structure with the specified capacity.
#[utoipa::path(
    post,
    path = "/api/v1/unionfind",
    request_body = CreateRequest,
    responses(
        (status = 201, 
         description = "Instance created successfully", 
         body = CreateResponse,
         example = json!({
             "id": "550e8400-e29b-41d4-a716-446655440000",
             "size": 10
         })),
        (status = 400, 
         description = "Invalid request parameters",
         body = ErrorResponse,
         examples(
             ("Zero size" = (value = json!({
                 "code": "INVALID_SIZE",
                 "message": "Size must be greater than 0",
                 "details": {
                     "provided": 0,
                     "minimum": 1
                 }
             }))),
             ("Too large" = (value = json!({
                 "code": "INVALID_SIZE",
                 "message": "Size exceeds maximum allowed",
                 "details": {
                     "provided": 200000,
                     "maximum": 100000
                 }
             })))
         )),
        (status = 422,
         description = "Unprocessable Entity - JSON deserialization failed",
         body = String,
         example = json!("Failed to deserialize the JSON body into the target type: size: invalid value: integer `-1`, expected usize at line 2 column 12")),
        (status = 500,
         description = "Internal server error",
         body = ErrorResponse,
         example = json!({
             "code": "INTERNAL_ERROR",
             "message": "Failed to create instance due to internal error"
         }))
    ),
    tag = "Union-Find Management"
)]
pub async fn create_instance(
    State(state): State<AppState>,
    Json(payload): Json<CreateRequest>,
) -> Response {
    // Validate size with structured error details
    if payload.size == 0 {
        return error_with_details(
            StatusCode::BAD_REQUEST,
            error_codes::INVALID_SIZE,
            "Size must be greater than 0",
            HashMap::from([
                ("provided".into(), json!(0)),
                ("minimum".into(), json!(1)),
            ]),
        );
    }
    if payload.size > 100_000 {
        return error_with_details(
            StatusCode::BAD_REQUEST,
            error_codes::INVALID_SIZE,
            "Size exceeds maximum allowed",
            HashMap::from([
                ("provided".into(), json!(payload.size)),
                ("maximum".into(), json!(100_000)),
            ]),
        );
    }

    let id = state.create_instance(payload.size);
    (
        StatusCode::CREATED,
        Json(CreateResponse {
            id,
            size: payload.size,
        }),
    ).into_response()
}

/// Union two elements in the Union-Find instance, merging their sets if they are not already connected.
///
/// Connects two elements by merging their representative sets using union-by-rank optimization.
#[utoipa::path(
    post,
    path = "/api/v1/unionfind/{id}/union",
    params(
        ("id" = Uuid, Path, description = "Instance ID")
    ),
    request_body(content = UnionRequest, examples(
        ("Connect adjacent pixels" = (
            summary = "Image processing: connect adjacent pixels",
            description = "Union adjacent pixels in a 10x10 grid for image segmentation",
            value = json!({
                "element1": 45,
                "element2": 46
            })
        )),
        ("Network connection" = (
            summary = "Network: establish router link",
            description = "Create connection between network routers in topology graph",
            value = json!({
                "element1": 3,
                "element2": 7
            })
        )),
        ("Graph edge" = (
            summary = "Graph: add edge to spanning tree",
            description = "Add edge for Kruskal's minimum spanning tree algorithm",
            value = json!({
                "element1": 0,
                "element2": 9
            })
        ))
    )),
    responses(
        (status = 200, 
         description = "Union operation successful", 
         body = UnionResponse,
         examples(
             ("New connection" = (value = json!({
                 "merged": true,
                 "root": 3
             }))),
             ("Already connected" = (value = json!({
                 "merged": false,
                 "root": 3
             })))
         )),
        (status = 404,
         description = "Instance not found",
         body = ErrorResponse,
         example = json!({
             "code": "INSTANCE_NOT_FOUND",
             "message": "No Union-Find instance exists with the given ID",
             "details": {
                 "id": "550e8400-e29b-41d4-a716-446655440000"
             }
         })),
        (status = 400,
         description = "Invalid element indices",
         body = ErrorResponse,
         examples(
             ("Element1 out of bounds" = (value = json!({
                 "code": "ELEMENT_OUT_OF_BOUNDS",
                 "message": "element1 index exceeds instance size",
                 "details": {
                     "element": 100,
                     "size": 10,
                     "field": "element1"
                 }
             }))),
             ("Element2 out of bounds" = (value = json!({
                 "code": "ELEMENT_OUT_OF_BOUNDS",
                 "message": "element2 index exceeds instance size",
                 "details": {
                     "element": 50,
                     "size": 10,
                     "field": "element2"
                 }
             }))),
             ("Same element" = (value = json!({
                 "code": "INVALID_UNION",
                 "message": "Cannot union element with itself",
                 "details": {
                     "element1": 5,
                     "element2": 5
                 }
             })))
         ))
    ),
    tag = "Operations"
)]
pub async fn union_elements(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UnionRequest>,
) -> Response {
    // Get the union result and the actual root after the operation
    let result = state.get_instance(id, |uf| {
        uf.union(payload.element1, payload.element2)
            .and_then(|merged| {
                // After union, get the actual root
                uf.find(payload.element1).map(|root| (merged, root))
            })
    });

    match result {
        Some(Ok((merged, root))) => (
            StatusCode::OK,
            Json(UnionResponse {
                merged,
                root,
            }),
        ).into_response(),
        Some(Err(e)) => {
            // Parse error message to determine specific error
            if e.contains("out of bounds") || e.contains("exceeds") {
                error_with_details(
                    StatusCode::BAD_REQUEST,
                    error_codes::ELEMENT_OUT_OF_BOUNDS,
                    e,
                    HashMap::from([
                        ("element1".into(), json!(payload.element1)),
                        ("element2".into(), json!(payload.element2)),
                    ]),
                )
            } else {
                error_with_code(StatusCode::BAD_REQUEST, error_codes::INVALID_UNION, e)
            }
        },
        None => error_with_details(
            StatusCode::NOT_FOUND,
            error_codes::INSTANCE_NOT_FOUND,
            "No Union-Find instance exists with the given ID",
            HashMap::from([
                ("id".into(), json!(id.to_string())),
            ]),
        ),
    }
}

/// Find the root representative of the set containing the specified element. Uses path compression for O(α(n)) amortized time complexity.
///
/// Returns the canonical representative of the set, applying path compression to optimize future queries.
#[utoipa::path(
    get,
    path = "/api/v1/unionfind/{id}/find",
    params(
        ("id" = Uuid, Path, description = "Instance ID"),
        ("element" = usize, Query, description = "Element to find root for")
    ),
    responses(
        (status = 200, 
         description = "Find operation successful", 
         body = FindResponse,
         example = json!({
             "element": 5,
             "root": 3
         })),
        (status = 404, 
         description = "Instance not found",
         body = ErrorResponse,
         example = json!({
             "code": "INSTANCE_NOT_FOUND",
             "message": "No Union-Find instance exists with the given ID"
         })),
        (status = 400, 
         description = "Invalid element index",
         body = ErrorResponse,
         example = json!({
             "code": "ELEMENT_OUT_OF_BOUNDS",
             "message": "Element index exceeds instance size",
             "details": {
                 "element": 100,
                 "size": 10
             }
         }))
    ),
    tag = "Operations"
)]
pub async fn find_element(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(params): Query<FindRequest>,
) -> Response {
    let result = state.get_instance(id, |uf| {
        uf.find(params.element)
    });

    match result {
        Some(Ok(root)) => (
            StatusCode::OK,
            Json(FindResponse {
                element: params.element,
                root,
            }),
        ).into_response(),
        Some(Err(_e)) => error_with_details(
            StatusCode::BAD_REQUEST,
            error_codes::ELEMENT_OUT_OF_BOUNDS,
            "Element index exceeds instance size",
            HashMap::from([
                ("element".into(), json!(params.element)),
            ]),
        ),
        None => error_with_code(
            StatusCode::NOT_FOUND,
            error_codes::INSTANCE_NOT_FOUND,
            "No Union-Find instance exists with the given ID",
        ),
    }
}

/// Check whether two elements belong to the same connected component. Returns true if they share the same root.
///
/// Determines connectivity by comparing the root representatives of both elements.
#[utoipa::path(
    get,
    path = "/api/v1/unionfind/{id}/connected",
    params(
        ("id" = Uuid, Path, description = "Instance ID"),
        ("element1" = usize, Query, description = "First element"),
        ("element2" = usize, Query, description = "Second element")
    ),
    responses(
        (status = 200, 
         description = "Connectivity check successful", 
         body = ConnectedResponse,
         examples(
             ("Connected" = (value = json!({"connected": true}))),
             ("Not connected" = (value = json!({"connected": false})))
         )),
        (status = 404, 
         description = "Instance not found",
         body = ErrorResponse,
         example = json!({
             "code": "INSTANCE_NOT_FOUND",
             "message": "No Union-Find instance exists with the given ID"
         })),
        (status = 400, 
         description = "Invalid element indices",
         body = ErrorResponse,
         example = json!({
             "code": "ELEMENT_OUT_OF_BOUNDS",
             "message": "One or more element indices exceed instance size",
             "details": {
                 "element1": 5,
                 "element2": 100,
                 "size": 10
             }
         }))
    ),
    tag = "Operations"
)]
pub async fn check_connected(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(params): Query<ConnectedRequest>,
) -> Response {
    let result = state.get_instance(id, |uf| {
        uf.connected(params.element1, params.element2)
    });

    match result {
        Some(Ok(connected)) => (
            StatusCode::OK,
            Json(ConnectedResponse { connected }),
        ).into_response(),
        Some(Err(_e)) => error_with_details(
            StatusCode::BAD_REQUEST,
            error_codes::ELEMENT_OUT_OF_BOUNDS,
            "One or more element indices exceed instance size",
            HashMap::from([
                ("element1".into(), json!(params.element1)),
                ("element2".into(), json!(params.element2)),
            ]),
        ),
        None => error_with_code(
            StatusCode::NOT_FOUND,
            error_codes::INSTANCE_NOT_FOUND,
            "No Union-Find instance exists with the given ID",
        ),
    }
}

/// Retrieve statistics including total number of elements and number of disjoint connected components.
///
/// Returns structural information about the current state of the Union-Find instance.
#[utoipa::path(
    get,
    path = "/api/v1/unionfind/{id}/stats",
    params(
        ("id" = Uuid, Path, description = "Instance ID")
    ),
    responses(
        (status = 200, 
         description = "Statistics retrieved successfully", 
         body = StatsResponse,
         examples(
             ("Initial state" = (
                 summary = "Newly created instance",
                 description = "All elements are in separate sets immediately after creation",
                 value = json!({
                     "total_elements": 10,
                     "num_components": 10
                 })
             )),
             ("Partially connected" = (
                 summary = "After some union operations",
                 description = "Several unions performed, forming 3 connected components",
                 value = json!({
                     "total_elements": 10,
                     "num_components": 3
                 })
             )),
             ("Fully connected" = (
                 summary = "All elements in one set",
                 description = "All elements unioned into a single connected component",
                 value = json!({
                     "total_elements": 10,
                     "num_components": 1
                 })
             ))
         )),
        (status = 404, 
         description = "Instance not found",
         body = ErrorResponse,
         example = json!({
             "code": "INSTANCE_NOT_FOUND",
             "message": "No Union-Find instance exists with the given ID"
         }))
    ),
    tag = "Operations"
)]
pub async fn get_stats(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Response {
    let result = state.get_instance(id, |uf| {
        StatsResponse {
            total_elements: uf.len(),
            num_components: uf.count(),
        }
    });

    match result {
        Some(stats) => (
            StatusCode::OK,
            Json(stats),
        ).into_response(),
        None => error_with_code(
            StatusCode::NOT_FOUND,
            error_codes::INSTANCE_NOT_FOUND,
            "No Union-Find instance exists with the given ID",
        ),
    }
}

/// Permanently delete a Union-Find instance and free its resources. This operation cannot be undone.
///
/// Removes the instance from memory. All subsequent operations on this ID will fail.
#[utoipa::path(
    delete,
    path = "/api/v1/unionfind/{id}",
    params(
        ("id" = Uuid, Path, description = "Instance ID")
    ),
    responses(
        (status = 204, description = "Instance deleted successfully"),
        (status = 401, 
         description = "Unauthorized - Missing or invalid credentials",
         body = ErrorResponse,
         example = json!({
             "code": "UNAUTHORIZED",
             "message": "Valid API key or bearer token required"
         })),
        (status = 403,
         description = "Forbidden - Insufficient permissions",
         body = ErrorResponse,
         example = json!({
             "code": "FORBIDDEN",
             "message": "delete:unionfind scope required"
         })),
        (status = 404, 
         description = "Instance not found",
         body = ErrorResponse,
         example = json!({
             "code": "INSTANCE_NOT_FOUND",
             "message": "No Union-Find instance exists with the given ID",
             "details": {
                 "id": "550e8400-e29b-41d4-a716-446655440000"
             }
         }))
    ),
    security(
        ("api_key" = []),
        ("bearer_auth" = []),
        ("oauth2" = ["delete:unionfind"])
    ),
    tag = "Union-Find Management"
)]
pub async fn delete_instance(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Response {
    if state.delete_instance(id) {
        StatusCode::NO_CONTENT.into_response()
    } else {
        error_with_details(
            StatusCode::NOT_FOUND,
            error_codes::INSTANCE_NOT_FOUND,
            "No Union-Find instance exists with the given ID",
            HashMap::from([
                ("id".into(), json!(id.to_string())),
            ]),
        )
    }
}
