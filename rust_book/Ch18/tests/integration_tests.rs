//! Integration tests for Chapter 18: Real-world OOP patterns

use ch18::*;

// ============================================================================
// Test 1: Complete GUI Application Workflow
// ============================================================================

#[test]
fn test_gui_application_with_multiple_screens() {
    // Login screen
    let mut login_screen = Screen::new();
    login_screen.add_component(Box::new(TextField {
        width: 30,
        placeholder: String::from("Username"),
        value: String::from(""),
    }));
    login_screen.add_component(Box::new(TextField {
        width: 30,
        placeholder: String::from("Password"),
        value: String::from(""),
    }));
    login_screen.add_component(Box::new(Button {
        width: 15,
        height: 2,
        label: String::from("Login"),
    }));
    
    assert_eq!(login_screen.components.len(), 3);
    login_screen.run();
    
    // Settings screen
    let mut settings_screen = Screen::new();
    settings_screen.add_component(Box::new(SelectBox {
        width: 40,
        height: 5,
        options: vec![
            String::from("Light Mode"),
            String::from("Dark Mode"),
            String::from("Auto"),
        ],
    }));
    settings_screen.add_component(Box::new(Button {
        width: 15,
        height: 2,
        label: String::from("Save"),
    }));
    settings_screen.add_component(Box::new(Button {
        width: 15,
        height: 2,
        label: String::from("Cancel"),
    }));
    
    assert_eq!(settings_screen.components.len(), 3);
    settings_screen.run();
}

// ============================================================================
// Test 2: Blog Publishing Workflow (State Pattern)
// ============================================================================

#[test]
fn test_blog_publishing_complete_workflow() {
    // Article 1: Draft -> Review -> Published
    let mut article1 = StatePost::new();
    article1.add_text("Introduction to Rust");
    article1.add_text("\n\nRust is a systems programming language...");
    
    assert_eq!(article1.content(), ""); // Not published yet
    
    article1.request_review();
    assert_eq!(article1.content(), ""); // Still in review
    
    article1.approve();
    assert!(!article1.content().is_empty()); // Now published
    
    // Article 2: Draft -> Rejected -> Re-draft -> Published
    // (This would require reject() method which is shown in examples)
}

// ============================================================================
// Test 3: Type-Safe Document Workflow
// ============================================================================

#[test]
fn test_document_type_safe_workflow() {
    // Create and edit document
    let mut draft1 = Post::new();
    draft1.add_text("Chapter 1: Introduction");
    
    // Submit for review
    let pending1 = draft1.request_review();
    
    // Approve and publish
    let published1 = pending1.approve();
    assert_eq!(published1.content(), "Chapter 1: Introduction");
    
    // Create another document
    let mut draft2 = Post::new();
    draft2.add_text("Chapter 2: Getting Started");
    
    let pending2 = draft2.request_review();
    let published2 = pending2.approve();
    assert_eq!(published2.content(), "Chapter 2: Getting Started");
}

// ============================================================================
// Test 4: Data Analysis with Encapsulated Statistics
// ============================================================================

#[test]
fn test_data_analysis_workflow() {
    // Collect test scores
    let mut scores = AveragedCollection::new();
    
    // First batch of tests
    scores.add(85);
    scores.add(92);
    scores.add(78);
    scores.add(95);
    scores.add(88);
    
    let first_average = scores.average();
    assert!((first_average - 87.6).abs() < 0.1);
    
    // Remove outlier
    scores.remove(); // Remove 88
    
    let adjusted_average = scores.average();
    assert!((adjusted_average - 87.5).abs() < 0.1);
    
    // Add more scores
    scores.add(90);
    scores.add(82);
    
    assert_eq!(scores.len(), 6);
    let final_average = scores.average();
    assert!((final_average - 87.0).abs() < 0.1);
}

// ============================================================================
// Test 5: Multi-Component Form Validation
// ============================================================================

#[test]
fn test_form_validation_with_trait_objects() {
    // Create a registration form
    let mut form = Screen::new();
    
    form.add_component(Box::new(TextField {
        width: 40,
        placeholder: String::from("First Name"),
        value: String::from("John"),
    }));
    
    form.add_component(Box::new(TextField {
        width: 40,
        placeholder: String::from("Last Name"),
        value: String::from("Doe"),
    }));
    
    form.add_component(Box::new(TextField {
        width: 40,
        placeholder: String::from("Email"),
        value: String::from("john.doe@example.com"),
    }));
    
    form.add_component(Box::new(SelectBox {
        width: 40,
        height: 4,
        options: vec![
            String::from("United States"),
            String::from("Canada"),
            String::from("United Kingdom"),
            String::from("Other"),
        ],
    }));
    
    form.add_component(Box::new(Button {
        width: 20,
        height: 3,
        label: String::from("Submit"),
    }));
    
    form.add_component(Box::new(Button {
        width: 20,
        height: 3,
        label: String::from("Cancel"),
    }));
    
    assert_eq!(form.components.len(), 6);
    form.run(); // Render all components
}

// ============================================================================
// Test 6: Streaming Data with Continuous Averaging
// ============================================================================

#[test]
fn test_streaming_data_analysis() {
    let mut sensor_data = AveragedCollection::new();
    
    // Simulate sensor readings over time
    let readings = vec![23, 25, 24, 26, 25, 27, 26, 28, 27, 29];
    
    for reading in readings {
        sensor_data.add(reading);
        
        // Average should be continuously updated
        let current_avg = sensor_data.average();
        assert!(current_avg > 0.0);
        assert!(current_avg <= 29.0);
    }
    
    // Final average
    let final_avg = sensor_data.average();
    assert!((final_avg - 26.0).abs() < 0.1);
}

// ============================================================================
// Test 7: OOP vs Type State Performance Comparison
// ============================================================================

#[test]
fn test_performance_comparison() {
    use std::time::Instant;
    
    // OOP State Pattern (runtime dispatch)
    let start_oop = Instant::now();
    for _ in 0..1000 {
        let mut post = StatePost::new();
        post.add_text("Test");
        post.request_review();
        post.approve();
        let _ = post.content();
    }
    let oop_duration = start_oop.elapsed();
    
    // Type State Pattern (compile-time)
    let start_type = Instant::now();
    for _ in 0..1000 {
        let mut post = Post::new();
        post.add_text("Test");
        let post = post.request_review();
        let post = post.approve();
        let _ = post.content();
    }
    let type_duration = start_type.elapsed();
    
    println!("\nPerformance Comparison (1000 iterations):");
    println!("OOP State Pattern:  {:?}", oop_duration);
    println!("Type State Pattern: {:?}", type_duration);
    
    // Type state should be faster (zero-cost abstraction)
    // But both should be very fast for such simple operations
}

// ============================================================================
// Test 8: Complex State Transitions
// ============================================================================

#[test]
fn test_complex_state_workflow() {
    // Multiple articles in different states
    let mut draft1 = StatePost::new();
    draft1.add_text("Article 1");
    
    let mut draft2 = StatePost::new();
    draft2.add_text("Article 2");
    
    let mut draft3 = StatePost::new();
    draft3.add_text("Article 3");
    
    // Different workflows
    draft1.request_review();
    draft1.approve();
    assert_eq!(draft1.content(), "Article 1"); // Published
    
    draft2.request_review();
    assert_eq!(draft2.content(), ""); // Still in review
    
    assert_eq!(draft3.content(), ""); // Still draft
}

// ============================================================================
// Test 9: Encapsulation Invariants Under Stress
// ============================================================================

#[test]
fn test_encapsulation_invariants() {
    let mut collection = AveragedCollection::new();
    
    // Add and remove in random pattern
    for i in 1..=100 {
        collection.add(i);
    }
    
    for _ in 0..50 {
        collection.remove();
    }
    
    // Average should still be consistent
    let expected_sum: i32 = (1..=50).sum();
    let expected_avg = expected_sum as f64 / 50.0;
    
    assert!((collection.average() - expected_avg).abs() < 0.001);
    assert_eq!(collection.len(), 50);
}

// ============================================================================
// Test 10: Heterogeneous Collections with Many Types
// ============================================================================

#[test]
fn test_large_heterogeneous_collection() {
    let mut screen = Screen::new();
    
    // Add many different components
    for i in 0..10 {
        screen.add_component(Box::new(Button {
            width: 20,
            height: 2,
            label: format!("Button {}", i),
        }));
        
        screen.add_component(Box::new(TextField {
            width: 30,
            placeholder: format!("Field {}", i),
            value: String::from(""),
        }));
        
        screen.add_component(Box::new(SelectBox {
            width: 25,
            height: 3,
            options: vec![
                format!("Option {}A", i),
                format!("Option {}B", i),
            ],
        }));
    }
    
    assert_eq!(screen.components.len(), 30);
    screen.run(); // Should handle many components
}
