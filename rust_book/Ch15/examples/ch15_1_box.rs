//! Chapter 15.1: Using Box<T> to Point to Data on the Heap
//!
//! This example demonstrates Box<T> for heap allocation, recursive types,
//! and trait objects.

fn main() {
    println!("=== Chapter 15.1: Box<T> - Heap Allocation ===\n");
    
    basic_box_usage();
    recursive_list_example();
    large_data_transfer();
    trait_object_example();
    
    println!("\n✅ Chapter 15.1 Box<T> concepts demonstrated!");
}

/// Demonstrates basic Box<T> heap allocation
/// 
/// **Book Reference**: Section 15.1 - Using Box<T>
/// **Key Learning**: Box allocates on heap, deallocates when dropped
fn basic_box_usage() {
    println!("--- Basic Box Usage ---");
    
    let b = Box::new(5);
    println!("Box contains: {}", b);
    
    // Box is automatically deallocated here
    println!("✓ Box will be deallocated at end of scope\n");
}

/// Shows how Box enables recursive types with known size
///
/// **Book Reference**: Section 15.1 - Enabling Recursive Types
/// **Integration**: Used in [[mission-3]] for Binary Search Tree nodes
fn recursive_list_example() {
    println!("--- Recursive Cons List with Box ---");
    
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    
    use List::{Cons, Nil};
    
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Cons list: {:?}", list);
    println!("✓ Recursive type has known size due to Box indirection\n");
}

/// Demonstrates efficient large data transfer using Box
fn large_data_transfer() {
    println!("--- Large Data Transfer ---");
    
    struct LargeData {
        data: [u8; 1024], // 1KB array
    }
    
    let large = Box::new(LargeData { data: [0; 1024] });
    println!("Created large boxed data: {} bytes", std::mem::size_of_val(&*large));
    
    // Only pointer is moved, not the data itself
    let transferred = large;
    println!("✓ Only 8-byte pointer moved, not 1KB of data\n");
}

/// Shows Box for trait objects (dynamic dispatch)
fn trait_object_example() {
    println!("--- Box for Trait Objects ---");
    
    trait Draw {
        fn draw(&self);
    }
    
    struct Circle {
        radius: f64,
    }
    
    impl Draw for Circle {
        fn draw(&self) {
            println!("Drawing circle with radius {}", self.radius);
        }
    }
    
    struct Rectangle {
        width: f64,
        height: f64,
    }
    
    impl Draw for Rectangle {
        fn draw(&self) {
            println!("Drawing rectangle {}x{}", self.width, self.height);
        }
    }
    
    // Vec of trait objects
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 10.0, height: 20.0 }),
    ];
    
    for shape in shapes {
        shape.draw();
    }
    
    println!("✓ Trait objects enable heterogeneous collections\n");
}
