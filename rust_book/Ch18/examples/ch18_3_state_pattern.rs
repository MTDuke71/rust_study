//! Chapter 18.3: Implementing the State Pattern (OOP Approach)
//!
//! This example demonstrates:
//! - Traditional OOP state pattern
//! - State objects implementing common trait
//! - Runtime state transitions
//! - Behavior changes based on internal state

/// State trait defines behavior for all states
pub trait State {
    /// Request review - transition from Draft to PendingReview
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    /// Approve - transition from PendingReview to Published
    fn approve(self: Box<Self>) -> Box<dyn State>;

    /// Reject - transition back to Draft
    fn reject(self: Box<Self>) -> Box<dyn State>;

    /// Get content - only Published state returns content
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    /// Check if can add text - only Draft allows editing
    fn can_add_text(&self) -> bool {
        false
    }
}

/// Draft state - initial state of a post
pub struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // Can't approve a draft directly
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        // Already in draft state
        self
    }

    fn can_add_text(&self) -> bool {
        true
    }
}

/// PendingReview state - waiting for approval
pub struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // Already in review
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

/// Published state - post is live
pub struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // Already published
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // Already published
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        // Can't reject published post
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

/// Post with internal state
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    /// Create a new post in Draft state
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    /// Add text to the post (only works in Draft state)
    pub fn add_text(&mut self, text: &str) {
        if self.state.as_ref().unwrap().can_add_text() {
            self.content.push_str(text);
        }
    }

    /// Get the content (returns empty string unless Published)
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    /// Request review - move to PendingReview state
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    /// Approve the post - move to Published state
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    /// Reject the post - move back to Draft
    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }

    /// Get current state name for debugging
    pub fn state_name(&self) -> &str {
        // This is a simplified version - real implementation would
        // add a method to State trait
        if self.content().is_empty() && !self.content.is_empty() {
            "Draft"
        } else if self.content().is_empty() {
            "PendingReview"
        } else {
            "Published"
        }
    }
}

impl Default for Post {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    println!("=== Chapter 18.3: State Pattern (OOP Approach) ===");

    println!("1. Basic Workflow:");
    let mut post = Post::new();
    println!("   Created new post (Draft state)");

    post.add_text("I ate a salad for lunch today");
    println!("   Added text: 'I ate a salad for lunch today'");
    println!("   Content visible: '{}'", post.content());
    println!("   (Empty because not yet published)");

    post.request_review();
    println!("\n   Requested review (now in PendingReview state)");
    println!("   Content visible: '{}'", post.content());
    println!("   (Still empty - not published yet)");

    post.approve();
    println!("\n   Approved post (now in Published state)");
    println!("   Content visible: '{}'", post.content());
    println!("   (Now shows content!)");

    // 2. Demonstrating state-specific behavior
    println!("\n2. State-Specific Behavior:");
    let mut post2 = Post::new();

    println!("   Draft state:");
    post2.add_text("First paragraph. ");
    println!("   ✓ Can add text");

    post2.request_review();
    println!("\n   PendingReview state:");
    post2.add_text("This won't be added. ");
    println!("   ✗ Cannot add text");
    println!("   ✗ Content not visible");

    post2.approve();
    println!("\n   Published state:");
    println!("   ✓ Content visible: '{}'", post2.content());
    post2.add_text("This also won't be added. ");
    println!("   ✗ Cannot modify published post");

    // 3. Rejection workflow
    println!("\n3. Rejection Workflow:");
    let mut post3 = Post::new();
    post3.add_text("Draft article about Rust");
    println!("   Created draft: '{}'", "Draft article about Rust");

    post3.request_review();
    println!("   Requested review");

    post3.reject();
    println!("   Rejected - back to Draft");

    post3.add_text(" (revised)");
    println!("   Added revision: 'Draft article about Rust (revised)'");

    post3.request_review();
    post3.approve();
    println!("   Re-submitted and approved");
    println!("   Final content: '{}'", post3.content());

    // 4. Invalid transitions
    println!("\n4. Invalid State Transitions:");
    let mut post4 = Post::new();
    post4.add_text("Test post");

    println!("   Draft state:");
    post4.approve();
    println!("   ✗ approve() in Draft - no effect");
    println!("   Content: '{}'", post4.content());

    post4.request_review();
    println!("\n   PendingReview state:");
    post4.request_review();
    println!("   ✗ request_review() again - no effect");

    post4.approve();
    println!("\n   Published state:");
    post4.approve();
    println!("   ✗ approve() again - no effect");
    post4.request_review();
    println!("   ✗ request_review() - can't unpublish");

    // 5. State pattern benefits
    println!("\n5. State Pattern Benefits:");
    println!("   ✓ Encapsulates state-specific behavior");
    println!("   ✓ Easy to add new states");
    println!("   ✓ State transitions in one place");
    println!("   ✓ Open/Closed Principle - extend without modifying");

    // 6. Trade-offs
    println!("\n6. Trade-offs of OOP State Pattern:");
    println!("   Pros:");
    println!("   ✓ Familiar to OOP developers");
    println!("   ✓ Runtime state changes");
    println!("   ✓ Dynamic behavior modification");

    println!("\n   Cons:");
    println!("   ✗ Runtime cost (trait objects)");
    println!("   ✗ Can't prevent invalid states at compile-time");
    println!("   ✗ Boilerplate (Option<Box<dyn State>>)");
    println!("   ✗ Can call methods in wrong state (they just no-op)");

    println!("\n7. Real-World Applications:");
    println!("   • Document workflow systems");
    println!("   • Order processing (Draft → Submitted → Confirmed → Shipped)");
    println!("   • User account states (Pending → Active → Suspended)");
    println!("   • Game character states (Idle → Running → Jumping → Falling)");
    println!("   • Network connection states (Disconnected → Connecting → Connected)");

    println!("\n=== End of Chapter 18.3 (OOP) ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draft_post_has_no_content() {
        let mut post = Post::new();
        post.add_text("Test content");
        assert_eq!(post.content(), "");
    }

    #[test]
    fn test_pending_review_has_no_content() {
        let mut post = Post::new();
        post.add_text("Test content");
        post.request_review();
        assert_eq!(post.content(), "");
    }

    #[test]
    fn test_approved_post_has_content() {
        let mut post = Post::new();
        post.add_text("Test content");
        post.request_review();
        post.approve();
        assert_eq!(post.content(), "Test content");
    }

    #[test]
    fn test_cannot_add_text_after_review() {
        let mut post = Post::new();
        post.add_text("Initial content");
        post.request_review();
        post.add_text(" More content");
        post.approve();
        assert_eq!(post.content(), "Initial content");
    }

    #[test]
    fn test_reject_returns_to_draft() {
        let mut post = Post::new();
        post.add_text("Initial");
        post.request_review();
        post.reject();
        post.add_text(" Revised");
        assert_eq!(post.content(), ""); // Still not published

        post.request_review();
        post.approve();
        assert_eq!(post.content(), "Initial Revised");
    }

    #[test]
    fn test_approve_draft_does_nothing() {
        let mut post = Post::new();
        post.add_text("Test");
        post.approve();
        assert_eq!(post.content(), "");
    }

    #[test]
    fn test_full_workflow() {
        let mut post = Post::new();

        // Draft
        post.add_text("I ate a salad for lunch today");
        assert_eq!(post.content(), "");

        // Pending review
        post.request_review();
        assert_eq!(post.content(), "");

        // Published
        post.approve();
        assert_eq!(post.content(), "I ate a salad for lunch today");
    }
}
