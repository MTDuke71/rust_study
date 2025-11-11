//! Chapter 15: Smart Pointers
//!
//! This library provides reusable smart pointer patterns and utilities
//! demonstrating concepts from Rust Book Chapter 15.

pub mod exercises;

// Re-export common smart pointer types for convenience
pub use std::boxed::Box;
pub use std::cell::{Cell, RefCell};
pub use std::rc::{Rc, Weak};
