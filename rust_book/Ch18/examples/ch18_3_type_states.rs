//! Chapter 18.3: Type System Alternative to State Pattern
//!
//! This example demonstrates:
//! - Encoding states as types
//! - Compile-time state validation
//! - Type system preventing invalid transitions
//! - Zero-cost abstractions

/// Draft post - can add text, can request review
pub struct DraftPost {
    content: String,
}

/// Pending review post - cannot modify, can approve or reject
pub struct PendingReviewPost {
    content: String,
}

/// Published post - can only read content
pub struct Post {
    content: String,
}

impl Post {
    /// Create a new post - starts in Draft state
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    /// Get the content of a published post
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    /// Add text to the draft (only drafts can be edited)
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    /// Request review - consumes draft, returns pending review
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }

    /// Get content for debugging (draft content not normally visible)
    pub fn preview(&self) -> &str {
        &self.content
    }
}

impl PendingReviewPost {
    /// Approve the post - consumes pending review, returns published post
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }

    /// Reject the post - returns to draft state
    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

// Enhanced version with two-approval requirement
pub struct PendingReviewPostV2 {
    content: String,
    approvals: u32,
}

impl DraftPost {
    pub fn request_review_v2(self) -> PendingReviewPostV2 {
        PendingReviewPostV2 {
            content: self.content,
            approvals: 0,
        }
    }
}

impl PendingReviewPostV2 {
    /// Add an approval - requires two approvals to publish
    pub fn approve(mut self) -> Result<Post, PendingReviewPostV2> {
        self.approvals += 1;

        if self.approvals >= 2 {
            Ok(Post {
                content: self.content,
            })
        } else {
            Err(self) // Return self with updated approval count
        }
    }

    /// Get approval count
    pub fn approval_count(&self) -> u32 {
        self.approvals
    }

    /// Reject - back to draft
    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

fn main() {
    println!("=== Chapter 18.3: Type System State Pattern ===");

    println!("1. Basic Workflow with Type States:");
    let mut post = Post::new();
    println!("   Created new post (type: DraftPost)");

    post.add_text("I ate a salad for lunch today");
    println!("   Added text: 'I ate a salad for lunch today'");
    println!("   Preview: '{}'", post.preview());

    let post = post.request_review();
    println!("\n   Requested review (type: PendingReviewPost)");
    println!("   Cannot call add_text() - method doesn't exist!");
    println!("   Cannot call content() - method doesn't exist!");

    let post = post.approve();
    println!("\n   Approved (type: Post)");
    println!("   Content: '{}'", post.content());
    println!("   Cannot call add_text() - method doesn't exist!");

    // 2. Compile-time safety
    println!("\n2. Compile-Time Safety:");
    let mut draft = Post::new();
    draft.add_text("Rust is awesome");

    println!("   The following WON'T COMPILE:");
    println!("   draft.approve()          // ✗ DraftPost has no approve()");
    println!("   draft.content()          // ✗ DraftPost has no content()");

    let pending = draft.request_review();
    println!("\n   pending.add_text()       // ✗ PendingReviewPost has no add_text()");
    println!("   pending.request_review() // ✗ Already in review");

    let _published = pending.approve();
    println!("\n   published.add_text()     // ✗ Post has no add_text()");
    println!("   published.approve()      // ✗ Already published");
    println!("\n   ✓ Invalid operations prevented at COMPILE TIME!");

    // 3. Type transitions enforce workflow
    println!("\n3. Type Transitions:");
    println!("   DraftPost --request_review()--> PendingReviewPost");
    println!("   PendingReviewPost --approve()--> Post");
    println!("   PendingReviewPost --reject()--> DraftPost");
    println!("\n   Each transition CONSUMES the old state");
    println!("   Old state cannot be used after transition");

    // 4. Rejection workflow
    println!("\n4. Rejection and Revision:");
    let mut draft2 = Post::new();
    draft2.add_text("Initial content");

    let pending2 = draft2.request_review();
    println!("   Submitted for review");

    let mut draft3 = pending2.reject();
    println!("   Rejected - back to draft");

    draft3.add_text(" (revised)");
    println!("   Added revision");

    let pending3 = draft3.request_review();
    let final_post = pending3.approve();
    println!("   Re-submitted and approved");
    println!("   Final: '{}'", final_post.content());

    // 5. Two-approval requirement version
    println!("\n5. Enhanced: Two-Approval Requirement:");
    let mut draft4 = Post::new();
    draft4.add_text("Important document");

    let pending4 = draft4.request_review_v2();
    println!("   Submitted for review (needs 2 approvals)");

    let pending5 = match pending4.approve() {
        Ok(_post) => {
            println!("   First approval - but this won't happen!");
            return;
        }
        Err(still_pending) => {
            println!(
                "   First approval (count: {})",
                still_pending.approval_count()
            );
            still_pending
        }
    };

    let final_post2 = match pending5.approve() {
        Ok(post) => {
            println!("   Second approval - published!");
            post
        }
        Err(_) => {
            println!("   Still pending!");
            return;
        }
    };

    println!("   Content: '{}'", final_post2.content());

    // 6. Comparison: Type States vs OOP State Pattern
    println!("\n6. Type States vs OOP State Pattern:");
    println!("   Type States (this example):");
    println!("   ✓ Compile-time safety");
    println!("   ✓ Zero runtime cost");
    println!("   ✓ Invalid states IMPOSSIBLE");
    println!("   ✓ No trait objects or dynamic dispatch");
    println!("   ✗ Less flexible (compile-time only)");
    println!("   ✗ Value is consumed on transition");

    println!("\n   OOP State Pattern:");
    println!("   ✓ Runtime flexibility");
    println!("   ✓ Can query current state");
    println!("   ✓ Reference stays valid");
    println!("   ✗ Runtime cost");
    println!("   ✗ Invalid states possible");
    println!("   ✗ Methods can be called in wrong state");

    // 7. When to use each approach
    println!("\n7. When to Use Type States:");
    println!("   ✓ Workflow is known at compile time");
    println!("   ✓ Want maximum safety guarantees");
    println!("   ✓ Performance is critical");
    println!("   ✓ State transitions are clear and linear");

    println!("\n   When to Use OOP State Pattern:");
    println!("   ✓ States determined at runtime");
    println!("   ✓ Need to query/inspect current state");
    println!("   ✓ Complex state graphs with loops");
    println!("   ✓ Familiar pattern for team");

    println!("\n8. Real-World Type State Examples:");
    println!("   • File handles (Open → Reading → Closed)");
    println!("   • Network sockets (Unbound → Bound → Listening → Connected)");
    println!("   • Builder patterns (Required fields as types)");
    println!("   • Database transactions (Begin → Active → Committed)");
    println!("   • HTTP requests (Building → Sent → Response)");

    println!("\n=== End of Chapter 18.3 (Type System) ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draft_can_add_text() {
        let mut draft = Post::new();
        draft.add_text("Hello");
        draft.add_text(" World");
        assert_eq!(draft.preview(), "Hello World");
    }

    #[test]
    fn test_published_post_has_content() {
        let mut post = Post::new();
        post.add_text("Test content");

        let post = post.request_review();
        let post = post.approve();

        assert_eq!(post.content(), "Test content");
    }

    #[test]
    fn test_reject_returns_to_draft() {
        let mut draft = Post::new();
        draft.add_text("Initial");

        let pending = draft.request_review();
        let mut draft2 = pending.reject();

        draft2.add_text(" Revised");
        assert_eq!(draft2.preview(), "Initial Revised");
    }

    #[test]
    fn test_two_approvals_required() {
        let mut draft = Post::new();
        draft.add_text("Test");

        let pending = draft.request_review_v2();
        assert_eq!(pending.approval_count(), 0);

        let pending = match pending.approve() {
            Ok(_) => panic!("Should need 2 approvals"),
            Err(p) => p,
        };
        assert_eq!(pending.approval_count(), 1);

        let post = match pending.approve() {
            Ok(p) => p,
            Err(_) => panic!("Second approval should succeed"),
        };

        assert_eq!(post.content(), "Test");
    }

    #[test]
    fn test_full_workflow_type_safety() {
        let mut draft = Post::new();
        draft.add_text("Rust");

        // Type transitions
        let pending = draft.request_review(); // DraftPost -> PendingReviewPost
        let post = pending.approve(); // PendingReviewPost -> Post

        assert_eq!(post.content(), "Rust");
    }

    // These tests demonstrate COMPILE-TIME safety
    // Uncomment to see compiler errors:

    // #[test]
    // fn test_cannot_add_text_to_pending() {
    //     let mut draft = Post::new();
    //     let pending = draft.request_review();
    //     pending.add_text("Won't compile");  // ERROR: no method `add_text`
    // }

    // #[test]
    // fn test_cannot_get_content_of_draft() {
    //     let draft = Post::new();
    //     let _ = draft.content();  // ERROR: no method `content`
    // }

    // #[test]
    // fn test_cannot_use_after_transition() {
    //     let mut draft = Post::new();
    //     draft.add_text("Test");
    //     let pending = draft.request_review();
    //     draft.add_text("More");  // ERROR: value moved
    // }
}
