//! Chapter 20: Advanced Features - Library
//!
//! Supporting types and functions for Chapter 20 examples

use std::fmt;

/// Point struct for trait examples
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Color enum for advanced trait examples
#[derive(Debug)]
pub enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

/// Message enum for pattern examples
#[derive(Debug)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

/// Event enum for nested destructuring
#[derive(Debug)]
pub enum Event {
    Quit,
    Move(Point),
    ChangeColor(Color),
}

/// Millimeters newtype for operator overloading
#[derive(Debug, Clone, Copy)]
pub struct Millimeters(pub u32);

/// Meters newtype for operator overloading
#[derive(Debug, Clone, Copy)]
pub struct Meters(pub u32);

/// Trait for animal shelter example
pub trait Animal {
    fn baby_name() -> String;
}

/// Dog struct for trait disambiguation
pub struct Dog;

impl Dog {
    pub fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

/// Pilot trait for method disambiguation
pub trait Pilot {
    fn fly(&self);
}

/// Wizard trait for method disambiguation
pub trait Wizard {
    fn fly(&self);
}

/// Human struct for multiple trait implementations
pub struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    pub fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
