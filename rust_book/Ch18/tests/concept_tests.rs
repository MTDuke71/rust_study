//! Concept tests for Chapter 18: Object-Oriented Programming

use ch18::*;

// ============================================================================
// Section 18.1: Encapsulation Tests
// ============================================================================

#[test]
fn test_encapsulation_averaged_collection() {
    let mut collection = AveragedCollection::new();
    
    collection.add(5);
    collection.add(10);
    collection.add(15);
    
    assert_eq!(collection.average(), 10.0);
    assert_eq!(collection.len(), 3);
}

#[test]
fn test_encapsulation_maintains_invariant() {
    let mut collection = AveragedCollection::new();
    
    collection.add(10);
    assert_eq!(collection.average(), 10.0);
    
    collection.add(20);
    assert_eq!(collection.average(), 15.0);
    
    collection.remove();
    assert_eq!(collection.average(), 10.0);
    
    collection.remove();
    assert_eq!(collection.average(), 0.0);
}

#[test]
fn test_encapsulation_empty_collection() {
    let collection = AveragedCollection::new();
    assert!(collection.is_empty());
    assert_eq!(collection.len(), 0);
    assert_eq!(collection.average(), 0.0);
}

// ============================================================================
// Section 18.2: Trait Objects Tests
// ============================================================================

#[test]
fn test_trait_objects_heterogeneous_collection() {
    let mut screen = Screen::new();
    
    screen.add_component(Box::new(Button {
        width: 50,
        height: 10,
        label: String::from("OK"),
    }));
    
    screen.add_component(Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![
            String::from("Yes"),
            String::from("No"),
        ],
    }));
    
    screen.add_component(Box::new(TextField {
        width: 60,
        placeholder: String::from("Enter text"),
        value: String::from(""),
    }));
    
    assert_eq!(screen.components.len(), 3);
    screen.run(); // Should not panic
}

#[test]
fn test_trait_objects_button_implements_draw() {
    let button = Button {
        width: 10,
        height: 2,
        label: String::from("Test"),
    };
    
    // If this compiles, Button implements Draw
    let _drawable: &dyn Draw = &button;
}

#[test]
fn test_trait_objects_selectbox_implements_draw() {
    let selectbox = SelectBox {
        width: 10,
        height: 3,
        options: vec![String::from("A"), String::from("B")],
    };
    
    let _drawable: &dyn Draw = &selectbox;
}

#[test]
fn test_trait_objects_function_parameter() {
    fn render(component: &dyn Draw) {
        component.draw();
    }
    
    let button = Button {
        width: 5,
        height: 1,
        label: String::from("Hi"),
    };
    
    render(&button); // Should not panic
}

// ============================================================================
// Section 18.3: OOP State Pattern Tests
// ============================================================================

#[test]
fn test_state_pattern_draft_has_no_content() {
    let mut post = StatePost::new();
    post.add_text("Test content");
    assert_eq!(post.content(), "");
}

#[test]
fn test_state_pattern_pending_review_has_no_content() {
    let mut post = StatePost::new();
    post.add_text("Test content");
    post.request_review();
    assert_eq!(post.content(), "");
}

#[test]
fn test_state_pattern_approved_post_has_content() {
    let mut post = StatePost::new();
    post.add_text("Test content");
    post.request_review();
    post.approve();
    assert_eq!(post.content(), "Test content");
}

#[test]
fn test_state_pattern_full_workflow() {
    let mut post = StatePost::new();
    
    // Draft state
    post.add_text("I ate a salad for lunch today");
    assert_eq!(post.content(), "");
    
    // Request review
    post.request_review();
    assert_eq!(post.content(), "");
    
    // Approve
    post.approve();
    assert_eq!(post.content(), "I ate a salad for lunch today");
}

#[test]
fn test_state_pattern_approve_before_review_does_nothing() {
    let mut post = StatePost::new();
    post.add_text("Test");
    
    // Try to approve without requesting review
    post.approve();
    assert_eq!(post.content(), "");
    
    // Proper workflow
    post.request_review();
    post.approve();
    assert_eq!(post.content(), "Test");
}

// ============================================================================
// Section 18.3: Type State Pattern Tests
// ============================================================================

#[test]
fn test_type_states_draft_to_published() {
    let mut post = Post::new();
    post.add_text("Test content");
    
    let post = post.request_review();
    let post = post.approve();
    
    assert_eq!(post.content(), "Test content");
}

#[test]
fn test_type_states_workflow() {
    let mut draft = Post::new();
    draft.add_text("I ate a salad for lunch today");
    
    let pending = draft.request_review();
    let published = pending.approve();
    
    assert_eq!(published.content(), "I ate a salad for lunch today");
}

// The following tests demonstrate compile-time safety
// They are commented out because they SHOULD NOT compile

// #[test]
// fn test_type_states_cannot_approve_draft() {
//     let draft = Post::new();
//     let _published = draft.approve(); // ERROR: no method `approve` on DraftPost
// }

// #[test]
// fn test_type_states_cannot_get_draft_content() {
//     let draft = Post::new();
//     let _content = draft.content(); // ERROR: no method `content` on DraftPost
// }

// #[test]
// fn test_type_states_cannot_use_after_move() {
//     let mut draft = Post::new();
//     draft.add_text("Test");
//     let _pending = draft.request_review();
//     draft.add_text("More"); // ERROR: value moved
// }

// ============================================================================
// Comparison Tests
// ============================================================================

#[test]
fn test_compare_oop_vs_type_states() {
    // OOP State Pattern
    let mut oop_post = StatePost::new();
    oop_post.add_text("OOP content");
    oop_post.request_review();
    oop_post.approve();
    assert_eq!(oop_post.content(), "OOP content");
    
    // Type State Pattern
    let mut type_post = Post::new();
    type_post.add_text("Type state content");
    let type_post = type_post.request_review();
    let type_post = type_post.approve();
    assert_eq!(type_post.content(), "Type state content");
    
    // Both achieve the same result with different trade-offs
}

#[test]
fn test_encapsulation_vs_public_fields() {
    // Encapsulated version
    let mut encapsulated = AveragedCollection::new();
    encapsulated.add(10);
    encapsulated.add(20);
    assert_eq!(encapsulated.average(), 15.0);
    
    // Cannot access internal state directly:
    // let _ = encapsulated.list; // ERROR: field `list` is private
    // let _ = encapsulated.average; // ERROR: field `average` is private
    
    // This ensures the average is always consistent with the list
}
