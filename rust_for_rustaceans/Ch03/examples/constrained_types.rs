//! Chapter 3: Constrained Interfaces - Type Modifications
//!
//! Topics: Type Modifications, Trait Implementations, Sealed Traits

fn main() {
    println!("=== Constrained Interfaces: Type Modifications ===\n");

    non_exhaustive_demo();
    sealed_traits_demo();
    blanket_implementations();
    handle_status(&Status::Active);
    handle_status(&Status::Inactive);
}

/// non_exhaustive: Allow adding fields/variants without breaking changes
fn non_exhaustive_demo() {
    println!("--- #[non_exhaustive] ---");

    // ✅ Can construct with builder pattern
    let config = Config::builder().timeout(30).build();

    println!("  Timeout: {}", config.timeout());

    // ❌ COMPILER ERROR: Cannot construct directly
    // let config = Config { timeout: 30 };

    // ❌ COMPILER ERROR: Cannot match exhaustively
    // match config {
    //     Config { timeout } => {}  // Error: non-exhaustive
    // }

    println!("  Future-proof: Can add fields without breaking API\n");
}

/// Sealed Traits: Prevent downstream implementations
fn sealed_traits_demo() {
    println!("--- Sealed Traits ---");

    let circle = Circle;
    let rect = Rectangle;

    println!("  Circle area: {}", circle.area());
    println!("  Rectangle area: {}", rect.area());

    println!("\n  Sealed trait prevents downstream users from implementing");
    println!("  Allows adding methods without breaking changes\n");
}

/// Blanket Implementations: Be careful with coherence
fn blanket_implementations() {
    println!("--- Blanket Implementations ---");

    println!("  ⚠️  Adding blanket impls is a BREAKING CHANGE");
    println!("  Example: impl<T> Display for Vec<T>");
    println!("  Violates coherence if downstream already implemented it");
    println!("  Solution: Only add blanket impls in initial release\n");
}

// === non_exhaustive Example ===

/// Config struct that can evolve without breaking changes
#[non_exhaustive]
pub struct Config {
    timeout: u32,
    // Future: Can add fields here without breaking existing code
    // max_retries: u32,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    pub fn timeout(&self) -> u32 {
        self.timeout
    }
}

#[derive(Default)]
pub struct ConfigBuilder {
    timeout: Option<u32>,
}

impl ConfigBuilder {
    pub fn timeout(mut self, timeout: u32) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn build(self) -> Config {
        Config {
            timeout: self.timeout.unwrap_or(60),
        }
    }
}

// === Sealed Trait Pattern ===

/// Private module ensures trait cannot be implemented outside
mod sealed {
    pub trait Sealed {}
}

/// Public trait that cannot be implemented downstream
pub trait Shape: sealed::Sealed {
    fn area(&self) -> f64;

    // Can add methods here without breaking changes
    // fn perimeter(&self) -> f64 { 0.0 }
}

pub struct Circle;
impl sealed::Sealed for Circle {}
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * 5.0 * 5.0
    }
}

pub struct Rectangle;
impl sealed::Sealed for Rectangle {}
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        10.0 * 20.0
    }
}

// === non_exhaustive Enum Example ===

#[non_exhaustive]
pub enum Status {
    Active,
    Inactive,
    // Future: Can add variants without breaking match statements
    // Suspended,
}

impl Status {
    pub fn is_active(&self) -> bool {
        matches!(self, Status::Active)
    }
}

// Downstream code must use wildcard
#[allow(unreachable_patterns)] // Pattern demonstrates non_exhaustive requirement
fn handle_status(status: &Status) {
    match status {
        Status::Active => println!("Active"),
        Status::Inactive => println!("Inactive"),
        _ => println!("Other"), // Required for non_exhaustive (currently unreachable, but would catch future variants)
    }
}
