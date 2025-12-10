//! Chapter 20.1: Unsafe Rust
//!
//! This example demonstrates the five unsafe superpowers:
//! 1. Dereference raw pointers
//! 2. Call unsafe functions
//! 3. Access/modify mutable static variables
//! 4. Implement unsafe traits
//! 5. Access union fields

fn main() {
    println!("=== Chapter 20.1: Unsafe Rust ===\n");

    demonstrate_raw_pointers();
    demonstrate_unsafe_functions();
    demonstrate_safe_abstraction();
    demonstrate_static_variables();
    demonstrate_unsafe_traits();
    demonstrate_unions();

    println!("\n✅ All unsafe Rust concepts demonstrated!");
    println!("💡 Remember: unsafe gives you superpowers, not safety!");
}

/// 1. Raw Pointers
///
/// **Book Reference**: Section 20.1 - Dereferencing Raw Pointers
/// **Key Learning**: Raw pointers (*const T, *mut T) can bypass borrow checker
fn demonstrate_raw_pointers() {
    println!("--- 1. Raw Pointers ---");

    let mut num = 5;

    // Creating raw pointers (safe)
    let r1 = &raw const num; // *const i32
    let r2 = &raw mut num; // *mut i32

    println!("  Created raw pointers (safe):");
    println!("    r1 (*const i32) points to address {:?}", r1);
    println!("    r2 (*mut i32) points to address {:?}", r2);

    // Dereferencing requires unsafe
    unsafe {
        println!("  Dereferencing (unsafe):");
        println!("    *r1 = {}", *r1);
        println!("    *r2 = {}", *r2);

        // Can modify through mutable raw pointer
        *r2 = 10;
        println!("    After modification: *r2 = {}", *r2);
    }

    // Creating raw pointer from arbitrary address (dangerous!)
    let address = 0x012345usize;
    let r = address as *const i32;
    println!("  Created pointer from arbitrary address: {:?}", r);
    println!("    ⚠️ Dereferencing this would be undefined behavior!");

    println!("💡 Raw pointers: power without safety guarantees");
    println!();
}

/// 2. Unsafe Functions
///
/// **Book Reference**: Section 20.1 - Calling Unsafe Functions
/// **Key Learning**: Mark functions that have safety requirements
fn demonstrate_unsafe_functions() {
    println!("--- 2. Unsafe Functions ---");

    /// Unsafe function requires caller to uphold safety contract
    unsafe fn dangerous() {
        println!("  Inside dangerous() - caller must ensure safety!");
    }

    // Must call in unsafe block
    unsafe {
        dangerous();
    }

    // Extern functions from C
    unsafe extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        let result = abs(-3);
        println!("  C's abs(-3) = {}", result);
    }

    println!("💡 Unsafe functions: explicit safety contracts");
    println!();
}

/// 3. Safe Abstraction Over Unsafe Code
///
/// **Book Reference**: Section 20.1 - Safe Abstractions
/// **Key Learning**: Wrap unsafe code in safe interfaces
fn demonstrate_safe_abstraction() {
    println!("--- 3. Safe Abstraction ---");

    /// Safe function that uses unsafe code internally
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        use std::slice;

        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len, "Index out of bounds");

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (a, b) = split_at_mut(&mut v, 3);

    println!("  Split [1,2,3,4,5,6] at index 3:");
    println!("    First half: {:?}", a);
    println!("    Second half: {:?}", b);

    a[0] = 10;
    b[0] = 40;
    println!("  After modification: {:?}", v);

    println!("💡 Safe abstraction: unsafe implementation, safe interface");
    println!();
}

/// 4. Static Variables
///
/// **Book Reference**: Section 20.1 - Static Variables
/// **Key Learning**: Mutable static variables require unsafe access
fn demonstrate_static_variables() {
    println!("--- 4. Static Variables ---");

    // Immutable static (safe)
    static HELLO_WORLD: &str = "Hello, world!";
    println!("  Immutable static (safe): {}", HELLO_WORLD);

    // Mutable static (unsafe)
    static mut COUNTER: u32 = 0;

    /// SAFETY: This example is single-threaded
    unsafe fn increment_counter(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    unsafe {
        increment_counter(3);
        increment_counter(5);

        // Read through raw pointer (safer than direct reference)
        let count = COUNTER;
        println!("  Mutable static (unsafe): COUNTER = {}", count);
    }

    println!("💡 Mutable statics: global state requires unsafe");
    println!();
}

/// 5. Unsafe Traits
///
/// **Book Reference**: Section 20.1 - Unsafe Traits
/// **Key Learning**: Traits with safety invariants compiler can't verify
fn demonstrate_unsafe_traits() {
    println!("--- 5. Unsafe Traits ---");

    unsafe trait UnsafeTrait {
        fn method(&self);
    }

    struct SafeType;

    unsafe impl UnsafeTrait for SafeType {
        fn method(&self) {
            println!("  SafeType implements UnsafeTrait");
        }
    }

    let instance = SafeType;
    instance.method(); // Calling method is safe

    println!("💡 Unsafe traits: promises compiler can't verify");
    println!();
}

/// 6. Union Access
///
/// **Book Reference**: Section 20.1 - Unions
/// **Key Learning**: Unions allow overlapping storage for different types
fn demonstrate_unions() {
    println!("--- 6. Unions ---");

    #[repr(C)]
    union MyUnion {
        i: i32,
        f: f32,
    }

    let u = MyUnion { i: 42 };

    unsafe {
        println!("  Union as integer: {}", u.i);

        // Reading f after writing i is technically valid but
        // interprets i32 bits as f32 (type punning)
        println!("  Union as float (reinterpret): {}", u.f);
    }

    println!("💡 Unions: primarily for C FFI");
    println!();
}

/// Bonus: Demonstrate Safety Documentation
fn documented_unsafe_example() {
    /// Reads a value from a raw pointer
    ///
    /// # Safety
    ///
    /// - `ptr` must be properly aligned for type `T`
    /// - `ptr` must point to a valid, initialized value of type `T`
    /// - `ptr` must be valid for reads
    /// - No other mutable references to the same location while this read occurs
    unsafe fn read_ptr<T>(ptr: *const T) -> T
    where
        T: Copy,
    {
        // SAFETY: Caller guarantees ptr validity
        unsafe { *ptr }
    }

    let value = 42;
    let ptr = &value as *const i32;

    unsafe {
        // SAFETY: ptr points to valid i32 value from reference
        let result = read_ptr(ptr);
        println!("Read through pointer: {}", result);
    }
}
