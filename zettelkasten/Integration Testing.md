# 🔗 Integration Testing

**Comprehensive guide to component interaction testing, end-to-end validation, and CSV-based dataset testing in Rust**

## 🎯 Overview

Integration testing in Rust focuses on testing how different components work together as a system. Unlike unit tests that test individual functions in isolation, integration tests verify that multiple components interact correctly, data flows properly between modules, and the system behaves as expected when components are combined.

## 🔧 Basic Integration Testing

### **Component Interaction Testing**
```rust
// tests/integration_tests.rs
use my_crate::{Stack, Queue, DataProcessor};

#[test]
fn test_stack_queue_integration() {
    let mut stack = Stack::new();
    let mut queue = Queue::new();
    
    // Test data flow between stack and queue
    stack.push(42);
    stack.push(24);
    
    // Move data from stack to queue
    while let Some(value) = stack.pop() {
        queue.enqueue(value);
    }
    
    // Verify data integrity through the transfer
    assert_eq!(queue.dequeue(), Some(42));
    assert_eq!(queue.dequeue(), Some(24));
    assert_eq!(queue.dequeue(), None);
}

#[test]
fn test_processor_pipeline() {
    let mut processor = DataProcessor::new();
    let input_data = vec![1, 2, 3, 4, 5];
    
    // Test complete processing pipeline
    let result = processor.process_data(input_data);
    
    assert!(result.is_ok());
    let processed = result.unwrap();
    
    // Verify pipeline transformations
    assert_eq!(processed.len(), 5);
    assert!(processed.iter().all(|&x| x > 0));
}
```

### **Module Integration Testing**
```rust
// tests/module_integration_tests.rs
use my_crate::{
    parser::parse_input,
    validator::validate_data,
    processor::process_data,
    exporter::export_results,
};

#[test]
fn test_complete_workflow() {
    let input = "42,24,36,48";
    
    // Step 1: Parse input
    let parsed = parse_input(input).expect("Failed to parse input");
    assert_eq!(parsed.len(), 4);
    
    // Step 2: Validate data
    let validation_result = validate_data(&parsed);
    assert!(validation_result.is_ok());
    
    // Step 3: Process data
    let processed = process_data(parsed).expect("Failed to process data");
    assert_eq!(processed.len(), 4);
    
    // Step 4: Export results
    let export_result = export_results(&processed);
    assert!(export_result.is_ok());
}
```

## 📊 CSV-Based Dataset Testing

### **CSV Validation Testing**
```rust
use csv::ReaderBuilder;
use std::fs::File;

#[test]
fn test_csv_dataset_validation() {
    let file = File::open("tests/data/test_dataset.csv")
        .expect("Failed to open test dataset");
    
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    
    let mut valid_rows = 0;
    let mut invalid_rows = 0;
    
    for result in reader.records() {
        match result {
            Ok(record) => {
                if validate_csv_record(&record) {
                    valid_rows += 1;
                } else {
                    invalid_rows += 1;
                }
            }
            Err(e) => {
                eprintln!("CSV parsing error: {}", e);
                invalid_rows += 1;
            }
        }
    }
    
    // Verify dataset quality
    assert!(valid_rows > 0, "No valid rows found in dataset");
    assert!(invalid_rows < valid_rows, "Too many invalid rows");
}

fn validate_csv_record(record: &csv::StringRecord) -> bool {
    // Validate that all required fields are present and valid
    if record.len() < 3 {
        return false;
    }
    
    // Validate numeric fields
    record[0].parse::<i32>().is_ok() &&
    record[1].parse::<f64>().is_ok() &&
    !record[2].is_empty()
}
```

### **End-to-End CSV Processing**
```rust
#[test]
fn test_csv_processing_pipeline() {
    let input_file = "tests/data/input.csv";
    let output_file = "tests/data/output.csv";
    
    // Test complete CSV processing workflow
    let result = process_csv_file(input_file, output_file);
    assert!(result.is_ok());
    
    // Verify output file was created
    assert!(std::path::Path::new(output_file).exists());
    
    // Verify output file contents
    let output_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(output_file)
        .expect("Failed to read output file");
    
    let mut row_count = 0;
    for result in output_reader.records() {
        let record = result.expect("Failed to parse output record");
        validate_processed_record(&record);
        row_count += 1;
    }
    
    assert!(row_count > 0, "No rows in output file");
    
    // Clean up test output
    std::fs::remove_file(output_file).ok();
}

fn validate_processed_record(record: &csv::StringRecord) {
    // Validate that processing transformed data correctly
    assert_eq!(record.len(), 4, "Incorrect number of columns");
    
    // Validate transformed values
    let original_value: i32 = record[0].parse().expect("Invalid original value");
    let processed_value: i32 = record[1].parse().expect("Invalid processed value");
    let validation_status: &str = &record[2];
    let timestamp: &str = &record[3];
    
    assert!(processed_value > 0, "Processed value should be positive");
    assert!(["valid", "invalid"].contains(&validation_status));
    assert!(!timestamp.is_empty(), "Timestamp should not be empty");
}
```

## 🔄 API Integration Testing

### **HTTP API Testing**
```rust
use reqwest::Client;
use serde_json::json;

#[tokio::test]
async fn test_api_integration() {
    let client = Client::new();
    let base_url = "http://localhost:3000";
    
    // Test GET endpoint
    let response = client
        .get(&format!("{}/api/users/42", base_url))
        .send()
        .await
        .expect("Failed to send GET request");
    
    assert_eq!(response.status(), 200);
    
    let user: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse JSON response");
    
    assert_eq!(user["id"], json!(42));
    assert!(user["name"].is_string());
    
    // Test POST endpoint
    let new_user = json!({
        "name": "Alice",
        "email": "alice@example.com"
    });
    
    let response = client
        .post(&format!("{}/api/users", base_url))
        .json(&new_user)
        .send()
        .await
        .expect("Failed to send POST request");
    
    assert_eq!(response.status(), 201);
    
    let created_user: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse created user");
    
    assert_eq!(created_user["name"], "Alice");
    assert!(created_user["id"].is_number());
}
```

### **Database Integration Testing**
```rust
use sqlx::{PgPool, Row};

#[tokio::test]
async fn test_database_integration() {
    let pool = PgPool::connect("postgresql://test:test@localhost/testdb")
        .await
        .expect("Failed to connect to test database");
    
    // Test database operations
    let user_id = create_test_user(&pool, "Alice", "alice@example.com").await;
    
    // Verify user was created
    let user = get_user_by_id(&pool, user_id).await
        .expect("Failed to retrieve user");
    
    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, "alice@example.com");
    
    // Test user update
    update_user(&pool, user_id, "Alice Updated", "alice.updated@example.com").await
        .expect("Failed to update user");
    
    let updated_user = get_user_by_id(&pool, user_id).await
        .expect("Failed to retrieve updated user");
    
    assert_eq!(updated_user.name, "Alice Updated");
    assert_eq!(updated_user.email, "alice.updated@example.com");
    
    // Clean up
    delete_user(&pool, user_id).await
        .expect("Failed to delete test user");
}

async fn create_test_user(pool: &PgPool, name: &str, email: &str) -> i32 {
    let row = sqlx::query(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id"
    )
    .bind(name)
    .bind(email)
    .fetch_one(pool)
    .await
    .expect("Failed to create user");
    
    row.get("id")
}
```

## 🎯 Mission Integration Testing

### **Mission1: Stack Integration**
```rust
#[test]
fn test_stack_integration_with_parser() {
    let mut stack = Stack::new();
    let input = "1,2,3,4,5";
    
    // Parse input and push to stack
    let numbers: Vec<i32> = input
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    
    for num in numbers {
        stack.push(num);
    }
    
    // Verify stack contains all numbers in reverse order
    assert_eq!(stack.len(), 5);
    
    let mut popped_values = Vec::new();
    while let Some(value) = stack.pop() {
        popped_values.push(value);
    }
    
    // Stack should pop in reverse order (LIFO)
    assert_eq!(popped_values, vec![5, 4, 3, 2, 1]);
}
```

### **Mission2: Queue Integration**
```rust
#[test]
fn test_queue_integration_with_processor() {
    let mut queue = RingBufferQueue::with_capacity(10);
    let data = vec!["task1", "task2", "task3", "task4", "task5"];
    
    // Enqueue all tasks
    for task in data {
        queue.enqueue(task.to_string()).unwrap();
    }
    
    // Process tasks in FIFO order
    let mut processed_tasks = Vec::new();
    while let Some(task) = queue.dequeue() {
        processed_tasks.push(task);
    }
    
    // Verify FIFO order is maintained
    assert_eq!(processed_tasks, vec![
        "task1".to_string(),
        "task2".to_string(),
        "task3".to_string(),
        "task4".to_string(),
        "task5".to_string(),
    ]);
}
```

### **Mission5: HashMap Integration**
```rust
#[test]
fn test_hashmap_integration_with_parser() {
    let mut map = HashMap::new();
    let config_data = r#"
        server.port=8080
        server.host=localhost
        database.url=jdbc:postgresql://localhost:5432/mydb
        database.username=admin
    "#;
    
    // Parse configuration and store in HashMap
    for line in config_data.lines() {
        if let Some((key, value)) = parse_config_line(line) {
            map.insert(key, value);
        }
    }
    
    // Verify configuration was parsed correctly
    assert_eq!(map.get("server.port"), Some(&"8080".to_string()));
    assert_eq!(map.get("server.host"), Some(&"localhost".to_string()));
    assert_eq!(map.get("database.url"), Some(&"jdbc:postgresql://localhost:5432/mydb".to_string()));
    assert_eq!(map.get("database.username"), Some(&"admin".to_string()));
    
    // Test configuration access in application code
    let server_port = map.get("server.port")
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(3000);
    
    assert_eq!(server_port, 8080);
}

fn parse_config_line(line: &str) -> Option<(String, String)> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    
    if let Some((key, value)) = line.split_once('=') {
        Some((key.trim().to_string(), value.trim().to_string()))
    } else {
        None
    }
}
```

## 🔧 Integration Test Patterns

### **Test Fixtures and Setup**
```rust
struct TestEnvironment {
    database: PgPool,
    api_client: Client,
    temp_dir: TempDir,
}

impl TestEnvironment {
    async fn new() -> Self {
        let database = setup_test_database().await;
        let api_client = Client::new();
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        
        Self {
            database,
            api_client,
            temp_dir,
        }
    }
    
    async fn cleanup(&self) {
        cleanup_test_database(&self.database).await;
    }
}

#[tokio::test]
async fn test_with_fixture() {
    let env = TestEnvironment::new().await;
    
    // Run integration tests
    test_user_workflow(&env.database, &env.api_client).await;
    
    env.cleanup().await;
}
```

### **Parallel Integration Testing**
```rust
#[tokio::test]
async fn test_parallel_operations() {
    let pool = setup_test_database().await;
    
    // Test concurrent database operations
    let tasks = (0..10).map(|i| {
        let pool = pool.clone();
        tokio::spawn(async move {
            let user_id = create_test_user(&pool, &format!("user{}", i), &format!("user{}@test.com", i)).await;
            let user = get_user_by_id(&pool, user_id).await.unwrap();
            assert_eq!(user.name, format!("user{}", i));
            user_id
        })
    });
    
    let results = futures::future::join_all(tasks).await;
    
    // Verify all operations completed successfully
    for result in results {
        assert!(result.is_ok());
        let user_id = result.unwrap();
        assert!(user_id > 0);
    }
    
    cleanup_test_database(&pool).await;
}
```

## 📊 Performance Integration Testing

### **End-to-End Performance Testing**
```rust
#[tokio::test]
async fn test_system_performance() {
    let start = std::time::Instant::now();
    
    // Test complete system workflow
    let input_data = generate_large_dataset(10000);
    let processed_data = process_large_dataset(input_data).await;
    let results = analyze_results(processed_data);
    
    let duration = start.elapsed();
    
    // Verify performance requirements
    assert!(duration.as_millis() < 5000, "System too slow: {:?}", duration);
    assert!(results.len() > 0, "No results generated");
    
    // Verify data integrity through the pipeline
    assert!(results.iter().all(|r| r.is_valid()));
}

async fn process_large_dataset(data: Vec<DataItem>) -> Vec<ProcessedItem> {
    // Simulate complex processing pipeline
    let mut results = Vec::new();
    
    for item in data {
        let processed = process_item(item).await;
        results.push(processed);
    }
    
    results
}
```

## 🔍 Error Integration Testing

### **Error Propagation Testing**
```rust
#[test]
fn test_error_propagation_through_layers() {
    // Test that errors propagate correctly through multiple layers
    let invalid_input = "invalid,data,format";
    
    let result = process_complete_workflow(invalid_input);
    
    assert!(result.is_err());
    
    // Verify error contains context from multiple layers
    let error = result.unwrap_err();
    assert!(error.to_string().contains("parsing"));
    assert!(error.to_string().contains("validation"));
}

fn process_complete_workflow(input: &str) -> Result<ProcessedResult, WorkflowError> {
    // Layer 1: Parsing
    let parsed = parse_input(input)
        .map_err(|e| WorkflowError::ParsingError(e))?;
    
    // Layer 2: Validation
    let validated = validate_data(&parsed)
        .map_err(|e| WorkflowError::ValidationError(e))?;
    
    // Layer 3: Processing
    let processed = process_data(validated)
        .map_err(|e| WorkflowError::ProcessingError(e))?;
    
    Ok(processed)
}
```

## 🔗 Related Concepts

### **Testing Integration**
- **[[Unit Testing]]** - Individual component testing
- **[[Testing Strategies]]** - Comprehensive testing approaches
- **[[TDD (Test-Driven Development)]]** - Test-first development

### **Mission Applications**
- **[[mission-1]]** - Stack integration with parsing systems
- **[[mission-2]]** - Queue integration with task processing
- **[[mission-5]]** - HashMap integration with configuration systems
- **[[mission-6]]** - Grid integration with pathfinding systems

### **Data Processing**
- **[[CSV Processing]]** - Dataset validation and processing
- **[[API Integration]]** - HTTP service integration testing
- **[[Database Integration]]** - Database operation testing

### **Development Methodology**
- **[[REQ-1 Test Strategy]]** - Requirement-based integration testing
- **[[V-Cycle Methodology]]** - Systematic integration testing
- **[[Debugging Lessons]]** - Using integration tests for debugging

## 🎯 Integration Testing Best Practices

### **DO:**
- Test real component interactions
- Use realistic test data
- Test error propagation through layers
- Verify data integrity across components
- Test performance under realistic loads
- Clean up test resources properly

### **DON'T:**
- Mock all dependencies (defeats the purpose)
- Use production data in tests
- Ignore test cleanup and teardown
- Test implementation details
- Make tests dependent on external services
- Ignore performance in integration tests

## 📋 Integration Testing Checklist

### **Before Writing Tests:**
- [ ] Identify component boundaries and interfaces
- [ ] Plan test data and scenarios
- [ ] Set up test environment and fixtures
- [ ] Design error propagation test cases

### **While Writing Tests:**
- [ ] Test component interactions
- [ ] Verify data flow integrity
- [ ] Test error handling across layers
- [ ] Validate performance characteristics
- [ ] Test concurrent operations

### **After Writing Tests:**
- [ ] Run tests in realistic environments
- [ ] Verify test cleanup and teardown
- [ ] Monitor test execution time
- [ ] Review integration test coverage

---

*Tags: #integration-testing #component-testing #end-to-end-testing #csv-testing #api-testing #database-testing #performance-testing*
*Links: [[zettel-index]] | [[rust-book-ch9-12-review]] | [[Unit Testing]] | [[Testing Strategies]] | [[TDD (Test-Driven Development)]] | [[REQ-1 Test Strategy]] | [[V-Cycle Methodology]] | [[Debugging Lessons]] | [[mission-1]] | [[mission-2]] | [[mission-5]] | [[mission-6]] | [[CSV Processing]] | [[API Integration]] | [[Database Integration]]*
