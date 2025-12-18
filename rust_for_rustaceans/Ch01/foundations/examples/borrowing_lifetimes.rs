//! Borrowing and Lifetimes Examples
//! 
//! Demonstrates shared references, mutable references, borrow checker, and lifetimes
//! from Section 3: Borrowing and Lifetimes

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  Section 3: Borrowing and Lifetimes                     ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // === BORROWING RULES ===
    {
        println!("📚 BORROWING RULES");
        println!("   ────────────────");
        println!("   1. Multiple immutable borrows (&T) OR");
        println!("   2. ONE mutable borrow (&mut T)");
        println!("   3. References must always be valid\n");
    }

    // === SHARED REFERENCES ===
    {
        println!("📚 SHARED REFERENCES (&T): Immutable Access");
        println!("   ──────────────────────────────────────────");
        
        let x = 42;
        let r1 = &x;  // First immutable borrow
        let r2 = &x;  // Second immutable borrow
        let r3 = &x;  // Third immutable borrow
        
        println!("   Original: {}", x);
        println!("   r1: {}, r2: {}, r3: {}", r1, r2, r3);
        println!("   ✓ Multiple immutable borrows allowed\n");
    }

    {
        println!("📚 SHARED REFERENCES: Function Parameters");
        println!("   ─────────────────────────────────────────");
        
        fn calculate_length(s: &String) -> usize {
            s.len()  // Can read but not modify
        }
        
        let text = String::from("hello");
        let len = calculate_length(&text);
        println!("   Length of \"{}\": {}", text, len);
        println!("   ✓ text still owned by caller\n");
    }

    {
        println!("📚 SHARED REFERENCES: Compiler Assumptions");
        println!("   ─────────────────────────────────────────");
        
        fn cache(input: &i32, sum: &mut i32) {
            *sum = *input + *input;  // Compiler caches *input
            assert_eq!(*sum, 2 * *input);
        }
        
        let x = 10;
        let mut result = 0;
        cache(&x, &mut result);
        println!("   Input: {}, Cached sum: {}", x, result);
        println!("   ✓ Compiler optimizes: knows &i32 won't change\n");
    }

    // === MUTABLE REFERENCES ===
    {
        println!("📚 MUTABLE REFERENCES (&mut T): Exclusive Access");
        println!("   ───────────────────────────────────────────────");
        
        let mut x = 42;
        let r = &mut x;  // Only ONE mutable borrow
        *r += 10;
        println!("   Modified through &mut: {}", r);
        println!("   ✓ Exclusive access guarantees no data races\n");
    }

    {
        println!("📚 MUTABLE REFERENCES: No Aliasing");
        println!("   ──────────────────────────────────");
        
        fn noalias(input: &i32, output: &mut i32) {
            if *input == 1 {
                *output = 2;
            }
            if *input != 1 {  // Compiler optimizes: can combine conditions
                *output = 3;
            }
        }
        
        let inp = 1;
        let mut out = 0;
        noalias(&inp, &mut out);
        println!("   Input: {}, Output: {}", inp, out);
        println!("   ✓ No aliasing enables optimizations\n");
    }

    {
        println!("📚 MUTABLE REFERENCES: Cannot Coexist with Immutable");
        println!("   ─────────────────────────────────────────────────────");
        println!("   let mut x = 5;");
        println!("   let r1 = &x;      // OK");
        println!("   let r2 = &x;      // OK");
        println!("   // let r3 = &mut x;  // ❌ ERROR: cannot borrow as mutable");
        println!("   ✓ Prevents data races at compile time\n");
    }

    {
        println!("📚 MUTABLE REFERENCES: Must Leave Value Behind");
        println!("   ────────────────────────────────────────────");
        
        use std::mem;
        
        let mut s = Box::new(42);
        println!("   Before: {}", s);
        
        // Cannot move out of &mut directly
        // let x = *s;  // ❌ Would leave s empty
        
        // But can use mem functions
        let taken = std::mem::replace(&mut s, Box::new(0));  // Replace with default
        s = taken;  // Put it back (s is Box, not *s)
        
        let mut replacement = Box::new(84);
        mem::swap(&mut s, &mut replacement);
        
        println!("   After swap: {}", s);
        println!("   ✓ Used mem::take() and mem::swap()\n");
    }

    // === LIFETIMES ===
    {
        println!("📚 LIFETIMES: Reference Validity Tracking");
        println!("   ───────────────────────────────────────");
        
        let r;                // 'a lifetime begins
        {
            let x = 5;        // 'b lifetime begins
            r = &x;           // r borrows x with lifetime 'b
            println!("   r inside scope: {}", r);
        }                     // 'b ends, x dropped
        // println!("{}", r); // ❌ Would fail: 'b ended, r dangles
        println!("   ✓ Lifetime ensures reference validity\n");
    }

    {
        println!("📚 LIFETIME ANNOTATIONS: Function Signatures");
        println!("   ───────────────────────────────────────────");
        
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() { x } else { y }
        }
        
        let s1 = String::from("long string");
        let s2 = String::from("short");
        let result = longest(&s1, &s2);
        println!("   Longest: \"{}\"", result);
        println!("   ✓ 'a connects input and output lifetimes\n");
    }

    {
        println!("📚 LIFETIME ELISION: Implicit Rules");
        println!("   ──────────────────────────────────");
        
        // Compiler infers lifetime
        fn first_word(s: &str) -> &str {
            s.split_whitespace().next().unwrap_or("")
        }
        
        // Equivalent to:
        // fn first_word<'a>(s: &'a str) -> &'a str
        
        let sentence = "hello world";
        let word = first_word(sentence);
        println!("   First word: \"{}\"", word);
        println!("   ✓ Compiler infers lifetime in simple cases\n");
    }

    {
        println!("📚 NON-CONTIGUOUS LIFETIMES");
        println!("   ─────────────────────────");
        
        let mut x = Box::new(42);
        let r = &x;
        
        // Conditional usage creates gap in lifetime
        if true {
            *x = 84;  // Can modify: r not used after
        } else {
            println!("   r: {}", r);  // r's lifetime active here
        }
        
        println!("   ✓ Lifetimes can have gaps\n");
    }

    {
        println!("📚 LIFETIMES WITH HOLES");
        println!("   ──────────────────────");
        
        let mut x = Box::new(0);
        let mut z = &x;
        
        for i in 0..3 {
            println!("   Iteration {}: z = {}", i, z);
            *x = i + 1;  // x changes
            z = &x;  // New lifetime begins
        }
        
        println!("   ✓ Lifetime recreated each iteration\n");
    }

    // === BORROW CHECKER ===
    {
        println!("📚 BORROW CHECKER: Flow Analysis");
        println!("   ──────────────────────────────");
        
        let mut data = vec![1, 2, 3];
        let r1 = &data;
        println!("   r1: {:?}", r1);
        // r1 last used here
        
        data.push(4);  // OK: no active borrows
        println!("   Modified: {:?}", data);
        println!("   ✓ Borrow checker tracks usage, not scope\n");
    }

    {
        println!("📚 BORROW CHECKER: What It Prevents");
        println!("   ──────────────────────────────────");
        println!("   ✗ Use after free");
        println!("   ✗ Double free");
        println!("   ✗ Dangling pointers");
        println!("   ✗ Iterator invalidation");
        println!("   ✗ Data races");
        println!("   ✓ All caught at compile time!\n");
    }

    // === STRUCTS WITH LIFETIMES ===
    {
        println!("📚 STRUCTS WITH LIFETIME PARAMETERS");
        println!("   ──────────────────────────────────");
        
        struct Excerpt<'a> {
            part: &'a str,
        }
        
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().unwrap();
        let excerpt = Excerpt { part: first_sentence };
        
        println!("   Excerpt: \"{}\"", excerpt.part);
        println!("   ✓ Struct holds reference with lifetime 'a\n");
    }

    {
        println!("📚 MULTIPLE LIFETIME PARAMETERS");
        println!("   ──────────────────────────────────");
        
        #[allow(dead_code)]
        struct StrSplit<'s, 'p> {
            document: &'s str,
            delimiter: &'p str,
        }
        
        let doc = "hello,world";
        let delim = ",";
        let _splitter = StrSplit { document: doc, delimiter: delim };
        
        println!("   Document and delimiter have independent lifetimes");
        println!("   ✓ Generic over multiple lifetimes\n");
    }

    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  Key Takeaways                                           ║");
    println!("╠══════════════════════════════════════════════════════════╣");
    println!("║  • &T: Multiple immutable borrows allowed               ║");
    println!("║  • &mut T: ONE exclusive mutable borrow                  ║");
    println!("║  • Lifetimes: Ensure references always valid             ║");
    println!("║  • Borrow Checker: Prevents memory safety bugs           ║");
    println!("╚══════════════════════════════════════════════════════════╝");
}
