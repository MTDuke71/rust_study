use criterion::{black_box, criterion_group, criterion_main, Criterion};
use step8_rest_api::models::{ErrorResponse, error_codes};
use std::collections::HashMap;
use serde_json::json;

fn bench_simple_error(c: &mut Criterion) {
    c.bench_function("simple_error", |b| {
        b.iter(|| {
            ErrorResponse::new(
                black_box(error_codes::INVALID_SIZE),
                black_box("Test error"),
            )
        });
    });
}

fn bench_error_with_details(c: &mut Criterion) {
    c.bench_function("error_with_details", |b| {
        b.iter(|| {
            ErrorResponse::new(error_codes::INVALID_SIZE, "Test error")
                .with_details(HashMap::from([
                    ("provided".into(), json!(0)),
                    ("minimum".into(), json!(1)),
                ]))
        });
    });
}

fn bench_error_with_multiple_details(c: &mut Criterion) {
    c.bench_function("error_with_multiple_details", |b| {
        b.iter(|| {
            ErrorResponse::new(error_codes::ELEMENT_OUT_OF_BOUNDS, "Element out of bounds")
                .with_details(HashMap::from([
                    ("element".into(), json!(100)),
                    ("size".into(), json!(10)),
                    ("field".into(), json!("element1")),
                    ("operation".into(), json!("union")),
                ]))
        });
    });
}

fn bench_error_serialization(c: &mut Criterion) {
    c.bench_function("error_serialization", |b| {
        let error = ErrorResponse::new(error_codes::INVALID_SIZE, "Size must be greater than 0")
            .with_details(HashMap::from([
                ("provided".into(), json!(0)),
                ("minimum".into(), json!(1)),
            ]));
        
        b.iter(|| {
            serde_json::to_string(&black_box(&error)).unwrap()
        });
    });
}

fn bench_error_construction_and_serialization(c: &mut Criterion) {
    c.bench_function("error_construction_and_serialization", |b| {
        b.iter(|| {
            let error = ErrorResponse::new(
                black_box(error_codes::INSTANCE_NOT_FOUND),
                black_box("No Union-Find instance exists with the given ID")
            ).with_details(HashMap::from([
                ("id".into(), json!("550e8400-e29b-41d4-a716-446655440000")),
            ]));
            
            serde_json::to_string(&error).unwrap()
        });
    });
}

criterion_group!(
    benches,
    bench_simple_error,
    bench_error_with_details,
    bench_error_with_multiple_details,
    bench_error_serialization,
    bench_error_construction_and_serialization
);
criterion_main!(benches);
