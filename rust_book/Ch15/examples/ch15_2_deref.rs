//! Chapter 15.2: Treating Smart Pointers Like Regular References with Deref
//!
//! This example demonstrates the Deref trait and deref coercion for
//! ergonomic smart pointer APIs.

use std::ops::Deref;

fn main() {
    println!("=== Chapter 15.2: Deref Trait - Smart Pointer References ===\n");
    
    basic_deref_example();
    custom_smart_pointer();
    deref_coercion_demo();
    multiple_deref_coercions();
    
    println!("\n✅ Chapter 15.2 Deref trait concepts demonstrated!");
}

/// Demonstrates basic dereferencing with Box
/// 
/// **Book Reference**: Section 15.2 - Following the Pointer to the Value
/// **Key Learning**: * operator calls deref() method
fn basic_deref_example() {
    println!("--- Basic Dereferencing ---");
    
    let x = 5;
    let y = Box::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y);  // *y is equivalent to *(y.deref())
    
    println!("x = {}", x);
    println!("*y = {}", *y);
    println!("✓ Box<T> can be dereferenced with * operator\n");
}

/// Shows custom smart pointer with Deref implementation
///
/// **Book Reference**: Section 15.2 - Treating a Type Like a Reference
/// **Integration**: Pattern used in custom smart pointers across missions
fn custom_smart_pointer() {
    println!("--- Custom Smart Pointer (MyBox) ---");
    
    struct MyBox<T>(T);
    
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    
    impl<T> Deref for MyBox<T> {
        type Target = T;
        
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    
    let x = 5;
    let y = MyBox::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y);
    
    println!("Created MyBox with value: {}", *y);
    println!("✓ Custom smart pointer works like built-in references\n");
}

/// Demonstrates deref coercion for ergonomic APIs
fn deref_coercion_demo() {
    println!("--- Deref Coercion ---");
    
    struct MyBox<T>(T);
    
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    
    impl<T> Deref for MyBox<T> {
        type Target = T;
        
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    
    fn hello(name: &str) {
        println!("Hello, {name}!");
    }
    
    let m = MyBox::new(String::from("Rust"));
    
    // Deref coercion: &MyBox<String> -> &String -> &str
    hello(&m);
    
    // Without deref coercion, we'd need:
    // hello(&(*m)[..]);
    
    println!("✓ Deref coercion automatically converts &MyBox<String> to &str\n");
}

/// Shows multiple deref coercion steps
fn multiple_deref_coercions() {
    println!("--- Multiple Deref Coercions ---");
    
    struct Wrapper<T>(T);
    
    impl<T> Deref for Wrapper<T> {
        type Target = T;
        
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    
    fn print_length(s: &str) {
        println!("String length: {}", s.len());
    }
    
    let wrapped = Wrapper(Box::new(String::from("Multiple coercions")));
    
    // Coercion chain: &Wrapper<Box<String>> -> &Box<String> -> &String -> &str
    print_length(&wrapped);
    
    println!("Deref coercion chain:");
    println!("  &Wrapper<Box<String>>");
    println!("  -> &Box<String>");
    println!("  -> &String");
    println!("  -> &str");
    println!("✓ Rust automatically applies multiple deref coercions\n");
}

/// Demonstrates the three deref coercion rules
#[allow(dead_code)]
fn deref_coercion_rules() {
    println!("--- Deref Coercion Rules ---");
    println!("1. &T -> &U when T: Deref<Target=U>");
    println!("2. &mut T -> &mut U when T: DerefMut<Target=U>");
    println!("3. &mut T -> &U when T: Deref<Target=U>");
    println!("✓ Rule 3 allows coercion from &mut to & (but not reverse)\n");
}
