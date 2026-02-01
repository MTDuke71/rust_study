//! Chapter 1: Full Chapter Examples
//!
//! All listings from Chapter 1 of "Rust for Rustaceans" demonstrating
//! foundations of memory, ownership, and borrowing.

// --- Helper Functions and Structs defined in the Chapter ---

// Listing 1-4: Rust assumes that shared references are immutable. [1]
fn cache(input: &i32, sum: &mut i32) {
    *sum = *input + *input;
    assert_eq!(*sum, 2 * *input);
}

// Listing 1-5: Rust assumes that mutable references are exclusive. [2]
fn noalias(input: &i32, output: &mut i32) {
    if *input == 1 {
        *output = 2;
    }
    if *input != 1 {
        *output = 3;
    }
}

// Listing 1-7: Access through a mutable reference must leave a value behind. [3]
fn replace_with_84(s: &mut Box<i32>) {
    // this is not okay, as *s would be empty:
    // let was = *s;

    // but this is:
    let was = std::mem::take(s); // [3]
                                 // so is this:
    *s = was; // [3]

    // we can exchange values behind &mut:
    let mut r = Box::new(84);
    std::mem::swap(s, &mut r); // [3]
    assert_ne!(*r, 84);
}

// Listing 1-10: A type that needs to be generic over multiple lifetimes. [4]
#[allow(dead_code)]
struct StrSplit<'s, 'p> {
    delimiter: &'p str,
    document: &'s str,
}

impl<'s, 'p> Iterator for StrSplit<'s, 'p> {
    type Item = &'s str;

    fn next(&mut self) -> Option<Self::Item> {
        // Source uses todo!(), returning None here to make the example runnable.
        None
    }
}

// Listing 1-10 Helper function [4]
fn str_before(s: &str, c: char) -> Option<&str> {
    StrSplit {
        document: s,
        delimiter: &c.to_string(),
    }
    .next()
}

// Listing 1-11: A type that needs to be generic over multiple lifetimes. [5]
struct MutStr<'a, 'b> {
    s: &'a mut &'b str,
}

// Mock function for Listing 1-8 to simulate randomness
fn rand() -> f64 {
    0.6
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  Rust for Rustaceans - Chapter 1: Foundations            ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // Listing 1-1: Values, variables, and pointers [6]
    {
        println!("📌 Listing 1-1: Values, Variables, and Pointers");
        println!("   ─────────────────────────────────────────────");
        let x = 42;
        let y = 43;
        let var1 = &x;
        let mut var2 = &x;
        println!(
            "   x: {}, y: {}, var1: {}, var2 (before): {}",
            x, y, var1, var2
        );
        var2 = &y;
        println!("   var2 (after reassignment): {}", var2);
        println!("   ✓ Variables hold values, pointers hold addresses\n");
    }

    // String example [7]
    {
        println!("📌 String Example: Stack vs Heap");
        println!("   ──────────────────────────────");
        let string = "Hello world"; // &str on stack, points to static memory
        println!("   String literal: \"{}\"", string);
        println!("   ✓ &str is a pointer to UTF-8 data\n");
    }

    // Listing 1-2: Illegal flows that the borrow checker will catch [8]
    {
        println!("📌 Listing 1-2: Borrow Checker Flow Analysis");
        println!("   ──────────────────────────────────────────");
        println!("   The following code would NOT compile:");
        println!("   ┌─────────────────────────────────────┐");
        println!("   │ let mut x;                          │");
        println!("   │ // assert_eq!(x, 42); // ❌ No flow │");
        println!("   │ x = 42;                             │");
        println!("   │ let y = &x;                         │");
        println!("   │ x = 43; // ❌ Conflicts with y      │");
        println!("   │ assert_eq!(*y, 42);                 │");
        println!("   └─────────────────────────────────────┘");
        println!("   ✓ Borrow checker prevents use-after-move\n");
    }

    // Low-Level Model Example [9]
    {
        println!("📌 Low-Level Model: Uninitialized Memory");
        println!("   ───────────────────────────────────────");

        let x: usize = 6;
        println!("   x value slot initialized to: {}", x);
        println!("   ✓ Variable is invalid until assignment\n");
    }

    // Listing 1-3: Moving and copying semantics [10]
    {
        println!("📌 Listing 1-3: Move vs Copy Semantics");
        println!("   ────────────────────────────────────");
        let x1 = 42;
        let y1 = Box::new(84);
        {
            let z = (x1, y1);
            println!("   Inside scope, z: {:?}", z);
        } // z dropped here

        let x2 = x1; // x1 is Copy
        println!("   x1 still valid (Copy trait): {}", x2);
        // let y2 = y1; // ❌ Would fail - y1 was moved
        println!("   ✓ Copy types: stack data (i32, bool, etc.)");
        println!("   ✓ Move types: heap data (Box, String, Vec)\n");
    }

    // Listing 1-4: Rust assumes that shared references are immutable [1]
    {
        println!("📌 Listing 1-4: Shared References are Immutable");
        println!("   ──────────────────────────────────────────────");
        let x = 42;
        let mut sum = 0;
        cache(&x, &mut sum);
        println!("   Input: {}, Cached sum: {}", x, sum);
        println!("   ✓ Compiler optimizes knowing &i32 won't change\n");
    }

    // Listing 1-5: Rust assumes that mutable references are exclusive [2]
    {
        println!("📌 Listing 1-5: Mutable References are Exclusive");
        println!("   ─────────────────────────────────────────────");
        let input = 1;
        let mut output = 0;
        noalias(&input, &mut output);
        println!("   Input: {}, Output: {}", input, output);
        println!("   ✓ No aliasing allows compiler optimizations\n");
    }

    // Listing 1-6: Mutability applies only to the immediately referenced memory [11]
    {
        println!("📌 Listing 1-6: Mutability of References");
        println!("   ─────────────────────────────────────");
        let x = 42;
        let mut y = &x; // y is of type &i32
        let z = &mut y; // z is of type &mut &i32
        println!("   x = {}, y = &x, z = &mut y", x);
        println!("   **z = {}", **z);
        println!("   ✓ Can mutate what y points to, not x itself\n");
    }

    // Listing 1-7: Access through a mutable reference must leave a value behind [3]
    {
        println!("📌 Listing 1-7: Mutable References Must Leave Values");
        println!("   ───────────────────────────────────────────────────");
        let mut s = Box::new(42);
        println!("   Before: s = {}", s);
        replace_with_84(&mut s);
        println!("   After: s = {}", s);
        println!("   ✓ Used mem::take() and mem::swap()\n");
    }

    // Listing 1-8: Lifetimes do not need to be contiguous [12]
    {
        println!("📌 Listing 1-8: Non-Contiguous Lifetimes");
        println!("   ─────────────────────────────────────");
        let mut x = Box::new(42);
        let r = &x; // lifetime 'a starts

        if rand() > 0.5 {
            *x = 84;
        } else {
            println!("   r: {}", r); // lifetime 'a used here
        }
        println!("   ✓ Lifetime 'a has gap in the middle\n");
    }

    // Listing 1-9: Lifetimes can have holes [13, 14]
    {
        println!("📌 Listing 1-9: Lifetimes with Holes");
        println!("   ──────────────────────────────────");
        let mut x = Box::new(42);
        let mut z = &x; // lifetime 'a

        for i in 0..5 {
            println!("   Iteration {}: z = {}", i, z);
            *x = i;
            z = &x; // new lifetime 'a starts
        }
        println!("   Final z: {}", z);
        println!("   ✓ Lifetime 'a recreated each iteration\n");
    }

    // Listing 1-10: Generic Lifetimes Usage [4]
    {
        println!("📌 Listing 1-10: Generic Lifetimes in Structs");
        println!("   ───────────────────────────────────────────");
        let s = "hello world";
        let c = ' ';
        let result = str_before(s, c);
        println!("   str_before('{}', '{}'): {:?}", s, c, result);
        println!("   ✓ StrSplit<'s, 'p> generic over two lifetimes\n");
    }

    // Listing 1-11: A type that needs to be generic over multiple lifetimes [5]
    {
        println!("📌 Listing 1-11: Multiple Lifetime Parameters");
        println!("   ───────────────────────────────────────────");
        let mut s = "hello";
        println!("   Before: s = \"{}\"", s);
        *MutStr { s: &mut s }.s = "world";
        println!("   After: s = \"{}\"", s);
        println!("   ✓ MutStr<'a, 'b> with &'a mut &'b str\n");
    }

    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  Chapter 1 Complete!                                     ║");
    println!("╚══════════════════════════════════════════════════════════╝");
}
