//! Variance Examples
//!
//! Demonstrates covariance, invariance, and contravariance in Rust's type system
//! Advanced topic from Chapter 1

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  Variance in Rust                                        ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // === CONCEPT ===
    {
        println!("📚 VARIANCE: What Is It?");
        println!("   ──────────────────────");
        println!("   How subtyping works with generic types");
        println!("   Determines if Type<A> is subtype of Type<B>");
        println!("   when A is subtype of B");
        println!("   Three kinds: Covariant, Invariant, Contravariant\n");
    }

    // === LIFETIME SUBTYPING ===
    {
        println!("📚 LIFETIME SUBTYPING");
        println!("   ───────────────────");
        println!("   Longer lifetime is subtype of shorter lifetime");
        println!("   'static is subtype of any 'a");
        println!("   If 'a: 'b, then 'a is subtype of 'b");
        println!("   ✓ Longer lifetimes can substitute for shorter\n");
    }

    // === COVARIANCE ===
    {
        println!("📚 COVARIANCE: Subtyping Preserved");
        println!("   ─────────────────────────────────");
        println!("   If A is subtype of B, then Type<A> is subtype of Type<B>");
        println!("   Examples: &'a T, Box<T>, Vec<T>");

        // Lifetime example
        fn covariant_example() {
            let static_str: &'static str = "hello";

            fn takes_short_lifetime(s: &str) {
                println!("   Received: {}", s);
            }

            // 'static (longer) can be used where shorter lifetime expected
            takes_short_lifetime(static_str);
        }

        covariant_example();
        println!("   ✓ &'static str → &'a str (covariant over lifetime)\n");
    }

    {
        println!("📚 COVARIANCE: Shared References");
        println!("   ──────────────────────────────");

        fn print_slice(slice: &[i32]) {
            println!("   Slice: {:?}", slice);
        }

        let vec = vec![1, 2, 3];
        let slice: &[i32] = &vec; // 'vec lifetime
        print_slice(slice); // Works with shorter lifetime 'a

        println!("   ✓ &'long [T] can be used as &'short [T]\n");
    }

    // === INVARIANCE ===
    {
        println!("📚 INVARIANCE: No Subtyping");
        println!("   ─────────────────────────");
        println!("   Type<A> is NOT related to Type<B>");
        println!("   even if A is subtype of B");
        println!("   Examples: &mut T, UnsafeCell<T>\n");
    }

    {
        println!("📚 INVARIANCE: Mutable References");
        println!("   ────────────────────────────────");
        println!("   &mut 'a T is INVARIANT over 'a");
        println!("   Cannot substitute longer for shorter lifetime");
        println!("   Reason: Prevents lifetime shrinking bugs");

        // This demonstrates why invariance is necessary
        fn invariant_example() {
            println!("\n   Why invariance matters:");
            println!("   If &mut were covariant, this would be unsound:");
            println!("   1. Have &'static mut reference");
            println!("   2. Pass to function expecting &'a mut");
            println!("   3. Function stores shorter-lived reference");
            println!("   4. Original reference now points to freed memory!");
            println!("   ✓ Invariance prevents this bug\n");
        }

        invariant_example();
    }

    // === CONTRAVARIANCE ===
    {
        println!("📚 CONTRAVARIANCE: Subtyping Reversed");
        println!("   ────────────────────────────────────");
        println!("   If A is subtype of B, then Type<B> is subtype of Type<A>");
        println!("   Rare in Rust");
        println!("   Example: Function arguments (in theory)\n");
    }

    {
        println!("📚 CONTRAVARIANCE: Function Pointers");
        println!("   ───────────────────────────────────");

        // fn(&'a str) is contravariant over 'a
        // Shorter lifetime function can substitute for longer

        // Demonstrating contravariance concept
        println!("   fn(&'short) can be used where fn(&'long) expected");
        println!("   ✓ Contravariant over argument lifetimes\n");
    }

    // === VARIANCE TABLE ===
    {
        println!("╔══════════════════════════════════════════════════════════╗");
        println!("║  Variance Quick Reference                                ║");
        println!("╠══════════════════════════════════════════════════════════╣");
        println!("║  Type            | Variance over T  | Variance over 'a   ║");
        println!("╠══════════════════════════════════════════════════════════╣");
        println!("║  &'a T           | Covariant        | Covariant          ║");
        println!("║  &'a mut T       | Covariant        | Invariant          ║");
        println!("║  Box<T>          | Covariant        | N/A                ║");
        println!("║  Vec<T>          | Covariant        | N/A                ║");
        println!("║  UnsafeCell<T>   | Invariant        | N/A                ║");
        println!("║  Cell<T>         | Invariant        | N/A                ║");
        println!("║  fn(T) -> U      | Contravariant(T) | Covariant(U)       ║");
        println!("║  PhantomData<T>  | Covariant        | N/A                ║");
        println!("╚══════════════════════════════════════════════════════════╝\n");
    }

    // === PHANTOMDATA ===
    {
        println!("📚 PHANTOMDATA: Controlling Variance");
        println!("   ───────────────────────────────────");

        println!("   PhantomData<T> marks T as owned");
        println!("   Makes MyType<T> covariant over T");
        println!("   Without it, variance would be ambiguous");
        println!("   ✓ Used for type safety in unsafe code\n");
    }

    {
        println!("📚 PHANTOMDATA: Invariance Example");
        println!("   ─────────────────────────────────");

        println!("   PhantomData<&'a mut T> forces invariance");
        println!("   Even when we only hold &'a T");
        println!("   ✓ Useful for soundness in custom types\n");
    }

    // === REAL-WORLD IMPACT ===
    {
        println!("📚 REAL-WORLD IMPACT");
        println!("   ──────────────────");
        println!("   Why variance matters:");
        println!("   • Borrow checker uses variance rules");
        println!("   • Determines when types are compatible");
        println!("   • Prevents subtle lifetime bugs");
        println!("   • Mostly invisible (compiler handles it)");
        println!("   • Important for unsafe code authors\n");
    }

    {
        println!("📚 COMMON SCENARIOS");
        println!("   ─────────────────");

        // Covariant: Can use longer lifetime
        {
            fn accepts_any(s: &str) {
                println!("   Covariant: {}", s);
            }
            let static_s: &'static str = "hello";
            accepts_any(static_s); // ✓ Works
        }

        // Invariant: Must match exactly
        {
            fn needs_exact(r: &mut &str) {
                println!("   Invariant: {}", r);
            }
            let mut s = "world";
            needs_exact(&mut s); // ✓ Works with exact lifetime
                                 // Cannot pass longer or shorter lifetime
        }

        println!("   ✓ Variance affects what lifetimes work\n");
    }

    // === DEBUGGING VARIANCE ===
    {
        println!("📚 DEBUGGING VARIANCE ISSUES");
        println!("   ──────────────────────────");
        println!("   Error: \"lifetime mismatch\"");
        println!("   → Check if type is invariant (&mut, UnsafeCell)");
        println!("   → Invariant types need exact lifetime match");
        println!("   → Consider restructuring to avoid invariance");
        println!("   ✓ Understanding variance helps debug errors\n");
    }

    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  Key Takeaways                                           ║");
    println!("╠══════════════════════════════════════════════════════════╣");
    println!("║  • Covariant: Longer → Shorter lifetime OK (&T, Box)     ║");
    println!("║  • Invariant: Must match exactly (&mut T, UnsafeCell)    ║");
    println!("║  • Contravariant: Rare, function arguments               ║");
    println!("║  • Affects borrow checker and lifetime checking          ║");
    println!("║  • Use PhantomData to control variance in unsafe code    ║");
    println!("╚══════════════════════════════════════════════════════════╝");
}
