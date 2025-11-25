use crate::{
    models::{
        ConnectedRequest, ConnectedResponse, CreateRequest, CreateResponse, ErrorResponse,
        FindRequest, FindResponse, StatsResponse, UnionRequest, UnionResponse,
    },
    state::AppState,
};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Router,
};
use uuid::Uuid;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/unionfind", post(create_instance))
        .route("/unionfind/:id/union", post(union_elements))
        .route("/unionfind/:id/find", get(find_element))
        .route("/unionfind/:id/connected", get(check_connected))
        .route("/unionfind/:id/stats", get(get_stats))
        .route("/unionfind/:id", delete(delete_instance))
}

fn error_response(code: StatusCode, message: &str) -> Response {
    (
        code,
        Json(ErrorResponse {
            code: code.to_string(),
            message: message.to_string(),
        }),
    )
        .into_response()
}

/// Create a new Union-Find instance
#[utoipa::path(
    post,
    path = "/api/v1/unionfind",
    request_body = CreateRequest,
    responses(
        (status = 201, description = "Instance created", body = CreateResponse),
        (status = 400, description = "Invalid input")
    ),
    tag = "Union-Find Management"
)]
pub async fn create_instance(
    State(state): State<AppState>,
    Json(payload): Json<CreateRequest>,
) -> Response {
    // Validate size
    if payload.size == 0 {
        return error_response(StatusCode::BAD_REQUEST, "Size must be at least 1");
    }
    if payload.size > 100_000 {
        return error_response(StatusCode::BAD_REQUEST, "Size exceeds maximum of 100,000");
    }

    let id = state.create_instance(payload.size);
    (
        StatusCode::CREATED,
        Json(CreateResponse {
            id,
            size: payload.size,
        }),
    )
        .into_response()
}

/// Union two elements
#[utoipa::path(
    post,
    path = "/api/v1/unionfind/{id}/union",
    params(
        ("id" = Uuid, Path, description = "Instance ID")
    ),
    request_body = UnionRequest,
    responses(
        (status = 200, description = "Union operation successful", body = UnionResponse),
        (status = 404, description = "Instance not found"),
        (status = 400, description = "Invalid element indices")
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
        Some(Ok((merged, root))) => {
            (StatusCode::OK, Json(UnionResponse { merged, root })).into_response()
        }
        Some(Err(e)) => error_response(StatusCode::BAD_REQUEST, &e),
        None => error_response(StatusCode::NOT_FOUND, "Instance not found"),
    }
}

/// Find the root of an element
#[utoipa::path(
    get,
    path = "/api/v1/unionfind/{id}/find",
    params(
        ("id" = Uuid, Path, description = "Instance ID"),
        ("element" = usize, Query, description = "Element to find root for")
    ),
    responses(
        (status = 200, description = "Find operation successful", body = FindResponse),
        (status = 404, description = "Instance not found"),
        (status = 400, description = "Invalid element index")
    ),
    tag = "Operations"
)]
pub async fn find_element(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(params): Query<FindRequest>,
) -> Response {
    let result = state.get_instance(id, |uf| uf.find(params.element));

    match result {
        Some(Ok(root)) => (
            StatusCode::OK,
            Json(FindResponse {
                element: params.element,
                root,
            }),
        )
            .into_response(),
        Some(Err(e)) => error_response(StatusCode::BAD_REQUEST, &e),
        None => error_response(StatusCode::NOT_FOUND, "Instance not found"),
    }
}

/// Check if two elements are connected
#[utoipa::path(
    get,
    path = "/api/v1/unionfind/{id}/connected",
    params(
        ("id" = Uuid, Path, description = "Instance ID"),
        ("element1" = usize, Query, description = "First element"),
        ("element2" = usize, Query, description = "Second element")
    ),
    responses(
        (status = 200, description = "Connectivity check successful", body = ConnectedResponse),
        (status = 404, description = "Instance not found"),
        (status = 400, description = "Invalid element indices")
    ),
    tag = "Operations"
)]
pub async fn check_connected(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(params): Query<ConnectedRequest>,
) -> Response {
    let result = state.get_instance(id, |uf| uf.connected(params.element1, params.element2));

    match result {
        Some(Ok(connected)) => {
            (StatusCode::OK, Json(ConnectedResponse { connected })).into_response()
        }
        Some(Err(e)) => error_response(StatusCode::BAD_REQUEST, &e),
        None => error_response(StatusCode::NOT_FOUND, "Instance not found"),
    }
}

/// Get statistics about the Union-Find instance
#[utoipa::path(
    get,
    path = "/api/v1/unionfind/{id}/stats",
    params(
        ("id" = Uuid, Path, description = "Instance ID")
    ),
    responses(
        (status = 200, description = "Statistics retrieved", body = StatsResponse),
        (status = 404, description = "Instance not found")
    ),
    tag = "Operations"
)]
pub async fn get_stats(State(state): State<AppState>, Path(id): Path<Uuid>) -> Response {
    let result = state.get_instance(id, |uf| StatsResponse {
        total_elements: uf.len(),
        num_components: uf.count(),
    });

    match result {
        Some(stats) => (StatusCode::OK, Json(stats)).into_response(),
        None => error_response(StatusCode::NOT_FOUND, "Instance not found"),
    }
}

/// Delete a Union-Find instance
#[utoipa::path(
    delete,
    path = "/api/v1/unionfind/{id}",
    params(
        ("id" = Uuid, Path, description = "Instance ID")
    ),
    responses(
        (status = 204, description = "Instance deleted successfully"),
        (status = 404, description = "Instance not found")
    ),
    tag = "Union-Find Management"
)]
pub async fn delete_instance(State(state): State<AppState>, Path(id): Path<Uuid>) -> Response {
    if state.delete_instance(id) {
        StatusCode::NO_CONTENT.into_response()
    } else {
        error_response(StatusCode::NOT_FOUND, "Instance not found")
    }
}
