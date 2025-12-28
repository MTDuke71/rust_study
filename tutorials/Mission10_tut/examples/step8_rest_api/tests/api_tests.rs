// Integration tests for Union-Find REST API
// Tests verify that error handling matches OpenAPI documentation

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; // for `oneshot`
use serde_json::Value;
use http_body_util::BodyExt; // for `collect`

// Import the app creation function
use step8_rest_api::create_app;

/// Helper to extract JSON body from response
async fn extract_json(response: axum::response::Response) -> Value {
    let body = response.into_body().collect().await.expect("Failed to read response body").to_bytes();
    serde_json::from_slice(&body).expect("Failed to parse JSON")
}

// ============================================================================
// CREATE INSTANCE TESTS
// ============================================================================

#[tokio::test]
async fn test_create_instance_success() {
    let app = create_app();
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/unionfind")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"size": 10}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    
    let body = extract_json(response).await;
    assert!(body["id"].is_string());
    assert_eq!(body["size"], 10);
}

#[tokio::test]
async fn test_create_instance_invalid_size_zero() {
    let app = create_app();
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/unionfind")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"size": 0}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    let body = extract_json(response).await;
    
    // Verify semantic error code
    assert_eq!(body["code"], "INVALID_SIZE");
    assert!(body["message"].as_str().unwrap().contains("greater than 0"));
    
    // Verify structured details
    assert!(body["details"].is_object());
    assert_eq!(body["details"]["provided"], 0);
    assert_eq!(body["details"]["minimum"], 1);
}

#[tokio::test]
async fn test_create_instance_size_too_large() {
    let app = create_app();
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/unionfind")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"size": 200000}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    let body = extract_json(response).await;
    assert_eq!(body["code"], "INVALID_SIZE");
    assert!(body["message"].as_str().unwrap().contains("exceeds maximum"));
    assert_eq!(body["details"]["provided"], 200000);
    assert_eq!(body["details"]["maximum"], 100000);
}

#[tokio::test]
async fn test_create_instance_invalid_json() {
    let app = create_app();
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/unionfind")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"size": -1}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    // Negative size fails JSON deserialization (usize can't be negative)
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

// ============================================================================
// INSTANCE NOT FOUND TESTS
// ============================================================================

#[tokio::test]
async fn test_get_stats_instance_not_found() {
    let app = create_app();
    
    let fake_uuid = "550e8400-e29b-41d4-a716-446655440000";
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/unionfind/{}/stats", fake_uuid))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    
    let body = extract_json(response).await;
    assert_eq!(body["code"], "INSTANCE_NOT_FOUND");
    assert!(body["message"].as_str().unwrap().contains("No Union-Find instance"));
}

#[tokio::test]
async fn test_union_instance_not_found() {
    let app = create_app();
    
    let fake_uuid = "550e8400-e29b-41d4-a716-446655440000";
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/unionfind/{}/union", fake_uuid))
                .header("content-type", "application/json")
                .body(Body::from(r#"{"element1": 0, "element2": 1}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    
    let body = extract_json(response).await;
    assert_eq!(body["code"], "INSTANCE_NOT_FOUND");
    assert!(body["details"].is_object());
    assert_eq!(body["details"]["id"], fake_uuid);
}

#[tokio::test]
async fn test_delete_instance_not_found() {
    let app = create_app();
    
    let fake_uuid = "550e8400-e29b-41d4-a716-446655440000";
    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/unionfind/{}", fake_uuid))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    
    let body = extract_json(response).await;
    assert_eq!(body["code"], "INSTANCE_NOT_FOUND");
    assert_eq!(body["details"]["id"], fake_uuid);
}

// ============================================================================
// ELEMENT OUT OF BOUNDS TESTS
// ============================================================================

#[tokio::test]
async fn test_find_element_out_of_bounds() {
    let app = create_app();
    
    // First create an instance
    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/unionfind")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"size": 10}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    
    let create_body = extract_json(create_response).await;
    let id = create_body["id"].as_str().unwrap();
    
    // Try to find element out of bounds
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/unionfind/{}/find?element=100", id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    let body = extract_json(response).await;
    assert_eq!(body["code"], "ELEMENT_OUT_OF_BOUNDS");
    assert!(body["message"].as_str().unwrap().contains("exceeds instance size"));
    assert!(body["details"].is_object());
    assert_eq!(body["details"]["element"], 100);
}

#[tokio::test]
async fn test_union_element_out_of_bounds() {
    let app = create_app();
    
    // Create instance
    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/unionfind")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"size": 10}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    
    let create_body = extract_json(create_response).await;
    let id = create_body["id"].as_str().unwrap();
    
    // Try union with out-of-bounds element
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/unionfind/{}/union", id))
                .header("content-type", "application/json")
                .body(Body::from(r#"{"element1": 0, "element2": 100}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    let body = extract_json(response).await;
    assert_eq!(body["code"], "ELEMENT_OUT_OF_BOUNDS");
    assert!(body["details"].is_object());
}

#[tokio::test]
async fn test_connected_element_out_of_bounds() {
    let app = create_app();
    
    // Create instance
    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/unionfind")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"size": 10}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    
    let create_body = extract_json(create_response).await;
    let id = create_body["id"].as_str().unwrap();
    
    // Try connected with out-of-bounds element
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/unionfind/{}/connected?element1=0&element2=100", id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    let body = extract_json(response).await;
    assert_eq!(body["code"], "ELEMENT_OUT_OF_BOUNDS");
    assert!(body["message"].as_str().unwrap().contains("exceed instance size"));
}

// ============================================================================
// SUCCESSFUL OPERATIONS TESTS
// ============================================================================

#[tokio::test]
async fn test_union_success() {
    let app = create_app();
    
    // Create instance
    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/unionfind")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"size": 10}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    
    let create_body = extract_json(create_response).await;
    let id = create_body["id"].as_str().unwrap();
    
    // Union elements
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/unionfind/{}/union", id))
                .header("content-type", "application/json")
                .body(Body::from(r#"{"element1": 3, "element2": 7}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    
    let body = extract_json(response).await;
    assert_eq!(body["merged"], true);
    assert!(body["root"].is_number());
}

#[tokio::test]
async fn test_find_success() {
    let app = create_app();
    
    // Create instance
    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/unionfind")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"size": 10}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    
    let create_body = extract_json(create_response).await;
    let id = create_body["id"].as_str().unwrap();
    
    // Find element
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/unionfind/{}/find?element=5", id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    
    let body = extract_json(response).await;
    assert_eq!(body["element"], 5);
    assert_eq!(body["root"], 5); // Initially each element is its own root
}

#[tokio::test]
async fn test_connected_success() {
    let app = create_app();
    
    // Create instance
    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/unionfind")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"size": 10}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    
    let create_body = extract_json(create_response).await;
    let id = create_body["id"].as_str().unwrap();
    
    // Check connected (should be false initially)
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/unionfind/{}/connected?element1=3&element2=7", id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    
    let body = extract_json(response).await;
    assert_eq!(body["connected"], false);
}

#[tokio::test]
async fn test_get_stats_success() {
    let app = create_app();
    
    // Create instance
    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/unionfind")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"size": 10}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    
    let create_body = extract_json(create_response).await;
    let id = create_body["id"].as_str().unwrap();
    
    // Get stats
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/unionfind/{}/stats", id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    
    let body = extract_json(response).await;
    assert_eq!(body["total_elements"], 10);
    assert_eq!(body["num_components"], 10); // Initially all separate
}

#[tokio::test]
async fn test_delete_instance_success() {
    let app = create_app();
    
    // Create instance
    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/unionfind")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"size": 10}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    
    let create_body = extract_json(create_response).await;
    let id = create_body["id"].as_str().unwrap();
    
    // Delete instance
    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/unionfind/{}", id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

// ============================================================================
// END-TO-END WORKFLOW TEST
// ============================================================================

#[tokio::test]
async fn test_complete_workflow() {
    let app = create_app();
    
    // 1. Create instance
    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/unionfind")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"size": 10}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(create_response.status(), StatusCode::CREATED);
    let create_body = extract_json(create_response).await;
    let id = create_body["id"].as_str().unwrap().to_string();
    
    // 2. Check initial stats
    let stats_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/unionfind/{}/stats", id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    let stats_body = extract_json(stats_response).await;
    assert_eq!(stats_body["num_components"], 10);
    
    // 3. Union some elements
    let union_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/unionfind/{}/union", id))
                .header("content-type", "application/json")
                .body(Body::from(r#"{"element1": 0, "element2": 1}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    
    let union_body = extract_json(union_response).await;
    assert_eq!(union_body["merged"], true);
    
    // 4. Check they're connected
    let connected_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/unionfind/{}/connected?element1=0&element2=1", id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    let connected_body = extract_json(connected_response).await;
    assert_eq!(connected_body["connected"], true);
    
    // 5. Check updated stats
    let stats2_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/unionfind/{}/stats", id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    let stats2_body = extract_json(stats2_response).await;
    assert_eq!(stats2_body["num_components"], 9); // One less component
    
    // 6. Delete instance
    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/unionfind/{}", id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);
    
    // 7. Verify it's gone
    let final_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/unionfind/{}/stats", id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(final_response.status(), StatusCode::NOT_FOUND);
    let final_body = extract_json(final_response).await;
    assert_eq!(final_body["code"], "INSTANCE_NOT_FOUND");
}
