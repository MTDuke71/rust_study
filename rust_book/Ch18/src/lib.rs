//! Chapter 18: Object-Oriented Programming Features in Rust
//!
//! This library provides reusable OOP patterns and implementations
//! for learning object-oriented concepts in Rust.

// Re-export main types for easy access
pub use averaged_collection::AveragedCollection;
pub use gui::{Button, Draw, Screen, SelectBox, TextField};
pub use post::{DraftPost, PendingReviewPost, Post};
pub use state_pattern::{Post as StatePost, State};

/// Module for encapsulation examples
pub mod averaged_collection {
    /// AveragedCollection demonstrates encapsulation
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }

    impl AveragedCollection {
        pub fn new() -> Self {
            AveragedCollection {
                list: vec![],
                average: 0.0,
            }
        }

        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }
                None => None,
            }
        }

        pub fn average(&self) -> f64 {
            self.average
        }

        pub fn len(&self) -> usize {
            self.list.len()
        }

        pub fn is_empty(&self) -> bool {
            self.list.is_empty()
        }

        fn update_average(&mut self) {
            if self.list.is_empty() {
                self.average = 0.0;
            } else {
                let total: i32 = self.list.iter().sum();
                self.average = total as f64 / self.list.len() as f64;
            }
        }
    }

    impl Default for AveragedCollection {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// Module for trait objects and GUI examples
pub mod gui {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("Drawing Button: {}", self.label);
        }
    }

    pub struct SelectBox {
        pub width: u32,
        pub height: u32,
        pub options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            println!("Drawing SelectBox with {} options", self.options.len());
        }
    }

    pub struct TextField {
        pub width: u32,
        pub placeholder: String,
        pub value: String,
    }

    impl Draw for TextField {
        fn draw(&self) {
            println!(
                "Drawing TextField: {}",
                if self.value.is_empty() {
                    &self.placeholder
                } else {
                    &self.value
                }
            );
        }
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn new() -> Self {
            Screen { components: vec![] }
        }

        pub fn add_component(&mut self, component: Box<dyn Draw>) {
            self.components.push(component);
        }

        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    impl Default for Screen {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// Module for OOP state pattern
pub mod state_pattern {
    pub trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, _post: &'a Post) -> &'a str {
            ""
        }
    }

    pub struct Draft {}
    pub struct PendingReview {}
    pub struct Published {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }
    }

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }

    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }

        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }

        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
    }

    impl Default for Post {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// Module for type-based state pattern
pub mod post {
    pub struct DraftPost {
        content: String,
    }

    pub struct PendingReviewPost {
        content: String,
    }

    pub struct Post {
        content: String,
    }

    impl Post {
        #[allow(clippy::new_ret_no_self)]
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }

        pub fn content(&self) -> &str {
            &self.content
        }
    }

    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }

    impl PendingReviewPost {
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }
    }
}

/// Module containing exercises
pub mod exercises;
