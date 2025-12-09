//! Chapter 20.2: Advanced Traits
//!
//! Demonstrates:
//! - Associated types
//! - Default type parameters
//! - Method disambiguation
//! - Supertraits
//! - Newtype pattern

use rust_book_ch20::*;
use std::fmt;
use std::ops::Add;

fn main() {
    println!("=== Chapter 20.2: Advanced Traits ===\n");

    demonstrate_associated_types();
    demonstrate_default_type_parameters();
    demonstrate_method_disambiguation();
    demonstrate_supertraits();
    demonstrate_newtype_pattern();

    println!("\n✅ All advanced trait concepts demonstrated!");
}

/// 1. Associated Types
fn demonstrate_associated_types() {
    println!("--- 1. Associated Types ---");

    struct Counter {
        count: u32,
    }

    impl Iterator for Counter {
        type Item = u32; // Associated type

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    let mut counter = Counter { count: 0 };
    println!("  Counter iterator:");
    while let Some(n) = counter.next() {
        print!("    {}", n);
    }
    println!();

    println!("💡 Associated types: one implementation per type");
    println!();
}

/// 2. Default Type Parameters
fn demonstrate_default_type_parameters() {
    println!("--- 2. Default Type Parameters ---");

    // Add<Rhs=Self> has default parameter
    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point::new(self.x + other.x, self.y + other.y)
        }
    }

    // Custom Rhs type
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    let p1 = Point::new(1, 2);
    let p2 = Point::new(3, 4);
    let p3 = p1 + p2;
    println!("  Point + Point: {}", p3);

    let mm = Millimeters(1000);
    let m = Meters(2);
    let total = mm + m;
    println!("  Millimeters + Meters: {:?}", total);

    println!("💡 Default parameters: customizable with sensible defaults");
    println!();
}

/// 3. Method Disambiguation
fn demonstrate_method_disambiguation() {
    println!("--- 3. Disambiguating Methods ---");

    let person = Human;

    // Direct implementation
    person.fly();

    // Trait implementations
    Pilot::fly(&person);
    Wizard::fly(&person);

    // Fully qualified syntax for associated functions
    println!("  Dog::baby_name() = {}", Dog::baby_name());
    println!("  <Dog as Animal>::baby_name() = {}", <Dog as Animal>::baby_name());

    println!("💡 Disambiguation: specify which trait's method to call");
    println!();
}

/// 4. Supertraits
fn demonstrate_supertraits() {
    println!("--- 4. Supertraits ---");

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    impl OutlinePrint for Point {}

    let point = Point::new(3, 7);
    println!("  OutlinePrint requires Display:");
    point.outline_print();

    println!("💡 Supertraits: require another trait as dependency");
    println!();
}

/// 5. Newtype Pattern
fn demonstrate_newtype_pattern() {
    println!("--- 5. Newtype Pattern ---");

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("  Wrapper around Vec: {}", w);

    println!("💡 Newtype: bypass orphan rule, add type safety");
    println!();
}
