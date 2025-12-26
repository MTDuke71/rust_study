// Example 3: Generic Traits vs Associated Types
// Demonstrates when to use generic type parameters vs associated types

use rustaceans_ch02_types::utils::{print_section, print_subsection};

fn main() {
    print_section("3. GENERIC TRAITS VS ASSOCIATED TYPES");
    
    generic_type_parameters();
    associated_types();
    comparison_example();
}

// --- Generic Type Parameters ---
fn generic_type_parameters() {
    print_subsection("a. Generic Type Parameters - Multiple Implementations");
    
    println!("Use trait Foo<T> when a type needs MULTIPLE implementations:\n");
    
    // From<T> is generic - can implement for many T
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl From<(i32, i32)> for Point {
        fn from((x, y): (i32, i32)) -> Self {
            Point { x, y }
        }
    }
    
    impl From<[i32; 2]> for Point {
        fn from([x, y]: [i32; 2]) -> Self {
            Point { x, y }
        }
    }
    
    impl From<i32> for Point {
        fn from(val: i32) -> Self {
            Point { x: val, y: val }
        }
    }
    
    let p1 = Point::from((3, 4));
    let p2 = Point::from([5, 6]);
    let p3 = Point::from(10);
    
    println!("Point::from((3, 4))    = Point {{ x: {}, y: {} }}", p1.x, p1.y);
    println!("Point::from([5, 6])    = Point {{ x: {}, y: {} }}", p2.x, p2.y);
    println!("Point::from(10)        = Point {{ x: {}, y: {} }}", p3.x, p3.y);
    
    println!("\n✅ Multiple impl From<T> for Point allowed");
    println!("❌ User must specify type: Point::from::<(i32, i32)>(...) if ambiguous");
}

// --- Associated Types ---
fn associated_types() {
    print_subsection("b. Associated Types - Single Implementation");
    
    println!("Use trait Foo {{ type Bar; }} when only ONE implementation per type:\n");
    
    // Iterator uses associated type - only one Item per iterator type
    struct Counter {
        count: u32,
        max: u32,
    }
    
    impl Counter {
        fn new(max: u32) -> Self {
            Counter { count: 0, max }
        }
    }
    
    impl Iterator for Counter {
        type Item = u32;  // Associated type - only one choice
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }
    
    let mut counter = Counter::new(5);
    print!("Counter yields: ");
    for val in counter {
        print!("{} ", val);
    }
    println!();
    
    println!("\n✅ Only one impl Iterator for Counter");
    println!("✅ User doesn't specify: just counter.next() (better ergonomics)");
    println!("❌ Cannot have multiple Iterator implementations for same type");
}

// --- Comparison Example ---
fn comparison_example() {
    print_subsection("c. Comparison - Generic vs Associated");
    
    println!("Generic Type Parameter (trait Add<Rhs>):\n");
    
    use std::ops::Add;
    
    #[derive(Debug, Clone, Copy)]
    struct Meters(f64);
    
    #[derive(Debug, Clone, Copy)]
    struct Feet(f64);
    
    // Can implement Add<Meters> for Meters
    impl Add<Meters> for Meters {
        type Output = Meters;
        fn add(self, other: Meters) -> Meters {
            Meters(self.0 + other.0)
        }
    }
    
    // Can ALSO implement Add<Feet> for Meters
    impl Add<Feet> for Meters {
        type Output = Meters;
        fn add(self, other: Feet) -> Meters {
            Meters(self.0 + other.0 * 0.3048)  // Convert feet to meters
        }
    }
    
    let m1 = Meters(10.0);
    let m2 = Meters(5.0);
    let f1 = Feet(3.28084);  // ~1 meter
    
    println!("Meters(10) + Meters(5) = {:?}", m1 + m2);
    println!("Meters(10) + Feet(3.28) = {:?}", m1 + f1);
    
    println!("\n✅ Two different Add implementations for Meters");
    println!("   - Add<Meters> for Meters");
    println!("   - Add<Feet> for Meters");
    
    println!("\nAssociated Type (Iterator::Item):\n");
    
    #[allow(dead_code)]
    struct WordCounter {
        words: Vec<String>,
        index: usize,
    }
    
    impl Iterator for WordCounter {
        type Item = String;  // Could be &str, String, etc., but pick ONE
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.words.len() {
                let word = self.words[self.index].clone();
                self.index += 1;
                Some(word)
            } else {
                None
            }
        }
    }
    
    println!("WordCounter has ONE Iterator::Item type: String");
    println!("Cannot have separate Item=String and Item=&str implementations");
    
    println!("\n📊 When to use which:\n");
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│ Generic Type Parameter (trait Foo<T>)                   │");
    println!("├─────────────────────────────────────────────────────────┤");
    println!("│ • Need multiple implementations per type               │");
    println!("│ • Example: From<T>, Add<Rhs>, Into<T>                  │");
    println!("│ • User may need to specify type                        │");
    println!("└─────────────────────────────────────────────────────────┘");
    println!();
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│ Associated Type (trait Foo {{ type Bar; }})               │");
    println!("├─────────────────────────────────────────────────────────┤");
    println!("│ • Only one implementation per type                     │");
    println!("│ • Example: Iterator::Item, Deref::Target               │");
    println!("│ • Better ergonomics (no type annotation needed)        │");
    println!("└─────────────────────────────────────────────────────────┘");
}
