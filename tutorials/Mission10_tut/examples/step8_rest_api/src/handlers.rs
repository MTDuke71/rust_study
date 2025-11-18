use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use uuid::Uuid;
use crate::{
    models::{CreateRequest, CreateResponse, UnionRequest, UnionResponse, ErrorResponse},
    state::AppState,
};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/unionfind", post(create_instance))
        .route("/unionfind/:id/union", post(union_elements))
}

// Helper to convert errors to JSON response
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
) -> impl IntoResponse {
    let id = state.create_instance(payload.size);
    (
        StatusCode::CREATED,
        Json(CreateResponse {
            id,
            size: payload.size,
        }),
    )
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
    let result = state.get_instance(id, |uf| {
        uf.union(payload.element1, payload.element2)
    });

    match result {
        Some(Ok(merged)) => (
            StatusCode::OK,
            Json(UnionResponse {
                merged,
                root: 0, // TODO: Get actual root if needed, or update UnionResponse
            }),
        ).into_response(),
        Some(Err(e)) => error_response(StatusCode::BAD_REQUEST, &e),
        None => error_response(StatusCode::NOT_FOUND, "Instance not found"),
    }
}

// TODO: Implement find, connected, and delete handlers
