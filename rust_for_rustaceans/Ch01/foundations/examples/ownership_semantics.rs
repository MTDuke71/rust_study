//! Ownership Semantics Examples
//! 
//! Demonstrates move semantics, copy semantics, and dropping
//! from Section 2: Ownership

use std::mem;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  Section 2: Ownership                                    ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // === OWNERSHIP RULES ===
    {
        println!("📚 OWNERSHIP RULES");
        println!("   ────────────────");
        println!("   1. Each value has exactly ONE owner");
        println!("   2. Only ONE owner at a time");
        println!("   3. Value dropped when owner goes out of scope\n");
    }

    // === MOVE SEMANTICS ===
    {
        println!("📚 MOVE SEMANTICS: Transfer of Ownership");
        println!("   ──────────────────────────────────────");
        
        // String is NOT Copy
        let s1 = String::from("hello");
        println!("   s1: \"{}\"", s1);
        
        let s2 = s1;  // Ownership moves from s1 to s2
        println!("   s2: \"{}\" (moved from s1)", s2);
        // println!("{}", s1);  // ❌ Would fail - s1 no longer valid
        println!("   ✓ s1 invalidated after move\n");
    }

    {
        println!("📚 MOVE IN FUNCTIONS");
        println!("   ──────────────────");
        
        fn take_ownership(s: String) {
            println!("   Function received: \"{}\"", s);
        }  // s dropped here
        
        let text = String::from("world");
        take_ownership(text);  // text moved into function
        // println!("{}", text);  // ❌ Would fail - text was moved
        println!("   ✓ Ownership transferred to function\n");
    }

    {
        println!("📚 MOVE IN COLLECTIONS");
        println!("   ────────────────────");
        
        let vec1 = vec![1, 2, 3];
        let vec2 = vec1;  // Moves ownership
        println!("   vec2: {:?} (moved from vec1)", vec2);
        
        let box1 = Box::new(42);
        let box2 = box1;  // Moves ownership
        println!("   box2: {} (moved from box1)", box2);
        println!("   ✓ Heap-allocated data is moved\n");
    }

    // === COPY SEMANTICS ===
    {
        println!("📚 COPY SEMANTICS: Bit-for-Bit Duplication");
        println!("   ─────────────────────────────────────────");
        
        // i32 implements Copy
        let x = 42;
        let y = x;  // Copies value, x still valid
        println!("   x: {} (still valid)", x);
        println!("   y: {} (copied from x)", y);
        println!("   ✓ Both x and y are valid\n");
    }

    {
        println!("📚 COPY TYPES");
        println!("   ───────────");
        
        let a: i32 = 5;      // Copy
        let b: bool = true;  // Copy
        let c: char = 'A';   // Copy
        // Note: Changed from 3.14 to avoid clippy warning about approximating PI constant
        // The focus here is demonstrating f64 implements Copy, not mathematical constants
        let d: f64 = 2.5;    // Copy
        let e: (i32, i32) = (1, 2);  // Copy (if all elements are Copy)
        
        let _a2 = a; let _b2 = b; let _c2 = c; let _d2 = d; let _e2 = e;
        
        println!("   All primitives still valid after 'copy':");
        println!("   a: {}, b: {}, c: {}, d: {}, e: {:?}", a, b, c, d, e);
        println!("   ✓ Stack-only types implement Copy\n");
    }

    {
        println!("📚 WHY SOME TYPES ARE NOT COPY");
        println!("   ────────────────────────────────");
        println!("   String: Owns heap data, copying would duplicate");
        println!("   Vec<T>: Owns heap data, copying would duplicate");
        println!("   Box<T>: Owns heap data, would violate single owner");
        println!("   ✓ Types with heap allocation are NOT Copy\n");
    }

    {
        println!("📚 CLONE: EXPLICIT DEEP COPY");
        println!("   ──────────────────────────");
        
        let s1 = String::from("hello");
        let s2 = s1.clone();  // Explicit deep copy
        println!("   s1: \"{}\" (still valid)", s1);
        println!("   s2: \"{}\" (cloned)", s2);
        println!("   ✓ Both own independent heap data\n");
    }

    // === DROPPING ===
    {
        println!("📚 DROPPING: Automatic Cleanup");
        println!("   ────────────────────────────");
        
        struct Resource {
            id: i32,
        }
        
        impl Drop for Resource {
            fn drop(&mut self) {
                println!("   🗑️  Dropping Resource {}", self.id);
            }
        }
        
        {
            let r1 = Resource { id: 1 };
            let r2 = Resource { id: 2 };
            println!("   Created resources {} and {}", r1.id, r2.id);
        }  // r2 dropped first (LIFO), then r1
        
        println!("   ✓ Resources dropped in reverse order\n");
    }

    {
        println!("📚 DROP ORDER");
        println!("   ───────────");
        
        let outer = String::from("outer");
        {
            let _inner = String::from("inner");
            println!("   Both outer and inner in scope");
        }  // inner dropped here
        println!("   Only outer in scope: \"{}\"", outer);
        println!("   ✓ Drop when leaving scope\n");
    }

    {
        println!("📚 MANUAL DROP");
        println!("   ────────────");
        
        let s = String::from("manual");
        println!("   Created: \"{}\"", s);
        drop(s);  // Explicitly drop early
        // println!("{}", s);  // ❌ Would fail - s was dropped
        println!("   ✓ Can drop explicitly with std::mem::drop()\n");
    }

    // === ADVANCED OWNERSHIP ===
    {
        println!("📚 MEM::TAKE - Replace with Default");
        println!("   ───────────────────────────────────");
        
        let mut s = Some(String::from("hello"));
        let taken = mem::take(&mut s);  // s now None
        println!("   Taken: {:?}", taken);
        println!("   Original now: {:?}", s);
        println!("   ✓ Useful for moving out of &mut\n");
    }

    {
        println!("📚 MEM::REPLACE - Swap Values");
        println!("   ────────────────────────────");
        
        let mut s = String::from("old");
        let old = mem::replace(&mut s, String::from("new"));
        println!("   Old value: \"{}\"", old);
        println!("   New value: \"{}\"", s);
        println!("   ✓ Atomically replace and return old\n");
    }

    {
        println!("📚 MEM::SWAP - Exchange Values");
        println!("   ────────────────────────────");
        
        let mut x = Box::new(5);
        let mut y = Box::new(10);
        println!("   Before: x={}, y={}", x, y);
        mem::swap(&mut x, &mut y);
        println!("   After: x={}, y={}", x, y);
        println!("   ✓ Efficiently swap without moves\n");
    }

    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  Key Takeaways                                           ║");
    println!("╠══════════════════════════════════════════════════════════╣");
    println!("║  • Move: Transfers ownership, old owner invalid          ║");
    println!("║  • Copy: Duplicates value, both owners valid             ║");
    println!("║  • Drop: Automatic cleanup at end of scope               ║");
    println!("║  • Clone: Explicit deep copy for non-Copy types          ║");
    println!("╚══════════════════════════════════════════════════════════╝");
}
