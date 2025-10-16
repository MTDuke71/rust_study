//! # Chapter 10.1: Generic Data Types
//!
//! Demonstrates generic programming concepts from The Rust Book.
//! Shows how to write reusable code that works with multiple types.
//!
//! Run with: `cargo run`

fn example1_basic_generics() {
    println!("🦀 Example 1: Basic Generic Functions");
    println!("=====================================");

    // Generic function that works with any comparable type
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let string_list = vec!["hello", "world", "rust", "programming"];
    let result = largest(&string_list);
    println!("The largest string is {}", result);

    println!();
}

fn example2_generic_structs() {
    println!("🏗️  Example 2: Generic Structs");
    println!("===============================");

    // Generic struct with single type parameter
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    // Specific implementation for f32
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    println!("Integer point: x={}, y={}", integer_point.x(), integer_point.y);
    println!("Float point: x={}, y={}", float_point.x(), float_point.y);
    println!("Distance from origin: {}", float_point.distance_from_origin());

    // Generic struct with multiple type parameters
    struct Point2<T, U> {
        x: T,
        y: U,
    }

    let mixed_point = Point2 { x: 5, y: 4.0 };
    println!("Mixed point: x={}, y={}", mixed_point.x, mixed_point.y);

    println!();
}

fn example3_generic_enums() {
    println!("📦 Example 3: Generic Enums");
    println!("============================");

    // Custom generic enum example (avoiding std library conflicts)
    #[derive(Debug)]
    enum Maybe<T> {
        Just(T),
        Nothing,
    }

    #[derive(Debug)]
    enum MyResult<T, E> {
        Ok(T),
        Err(E),
    }

    // Using our custom Maybe enum
    let some_number = Maybe::Just(5);
    let some_string = Maybe::Just("a string");
    let absent_number: Maybe<i32> = Maybe::Nothing;

    println!("Some number: {:?}", some_number);
    println!("Some string: {:?}", some_string);
    println!("Absent number: {:?}", absent_number);

    // Using our custom Result enum
    let success: MyResult<i32, &str> = MyResult::Ok(42);
    let failure: MyResult<i32, &str> = MyResult::Err("Something went wrong");

    println!("Success: {:?}", success);
    println!("Failure: {:?}", failure);

    // Using standard library Option and Result
    let std_option: Option<i32> = Some(42);
    let std_result: Result<i32, &str> = Ok(42);

    println!("Standard Option: {:?}", std_option);
    println!("Standard Result: {:?}", std_result);

    println!();
}

fn example4_generic_methods() {
    println!("🔧 Example 4: Generic Methods and Impl Blocks");
    println!("==============================================");

    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("Mixed point: x={}, y={}", p3.x, p3.y);

    println!();
}

fn example5_performance_considerations() {
    println!("⚡ Example 5: Performance and Monomorphization");
    println!("===============================================");

    // Generic function that gets specialized at compile time
    fn identity<T>(x: T) -> T {
        x
    }

    // At compile time, Rust generates specific versions:
    // fn identity_i32(x: i32) -> i32 { x }
    // fn identity_string(x: String) -> String { x }
    // etc.

    let int_result = identity(5);
    let string_result = identity(String::from("hello"));

    println!("Integer identity: {}", int_result);
    println!("String identity: {}", string_result);

    // This is zero-cost abstraction - no runtime overhead!
    println!("No runtime cost for generics!");

    println!();
}

fn example6_trait_bounds_basics() {
    println!("🎯 Example 6: Basic Trait Bounds");
    println!("=================================");

    use std::fmt::Display;

    // Function that requires T to implement Display
    fn print_generic<T: Display>(item: T) {
        println!("Item: {}", item);
    }

    // Function with multiple trait bounds
    fn compare_and_print<T: PartialOrd + Display>(a: T, b: T) {
        if a > b {
            println!("{} is greater than {}", a, b);
        } else {
            println!("{} is not greater than {}", a, b);
        }
    }

    print_generic(42);
    print_generic("Hello, Rust!");
    print_generic(3.14);

    compare_and_print(10, 5);
    compare_and_print('z', 'a');

    println!();
}

fn main() {
    println!("📚 Chapter 10.1: Generic Data Types\n");

    example1_basic_generics();
    example2_generic_structs();
    example3_generic_enums();
    example4_generic_methods();
    example5_performance_considerations();
    example6_trait_bounds_basics();

    println!("✅ All examples completed!");
    println!("📖 Next: Read Chapter 10.2 (Traits) or run examples in ../traits/");
}
