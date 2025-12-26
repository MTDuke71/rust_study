// Example 5: Trait Bounds and Higher-Ranked Trait Bounds (HRTB)
// Demonstrates arbitrary bounds and for<'a> syntax

use rustaceans_ch02_types::utils::{print_section, print_subsection};
use std::fmt::{self, Debug, Formatter};

fn main() {
    print_section("5. TRAIT BOUNDS AND HRTB");
    
    arbitrary_bounds();
    hrtb_basics();
    hrtb_with_fn_traits();
    hrtb_with_iterables();
}

// --- Arbitrary Bounds ---
fn arbitrary_bounds() {
    print_subsection("a. Arbitrary Bounds");
    
    println!("Bounds don't need to reference the generic parameter:\n");
    
    // Bound on String, even though T is the generic parameter
    // Note: T is intentionally unused to demonstrate arbitrary bounds concept
    // Clippy warns about this, but it's pedagogically important to show
    #[allow(clippy::extra_unused_type_parameters)]
    fn example<T>() -> String 
    where
        String: Clone,  // Arbitrary bound - String is always Clone
    {
        "hello".to_string().clone()
    }
    
    let result = example::<i32>();
    println!("example::<i32>() = \"{}\"", result);
    
    println!("\n✅ where String: Clone is valid (even though unnecessary)");
    println!("Useful for:");
    println!("  - Conditional compilation (cfg)");
    println!("  - Complex trait interactions");
    println!("  - Associated type bounds");
}

// --- HRTB Basics ---
fn hrtb_basics() {
    print_subsection("b. Higher-Ranked Trait Bounds - for<'a>");
    
    println!("for<'a> means 'for ALL lifetimes':\n");
    
    // Regular lifetime bound
    // Note: Lifetime 'a is intentionally explicit here to contrast with HRTB below
    // Clippy suggests eliding it, but keeping it explicit demonstrates the difference
    #[allow(clippy::needless_lifetimes)]
    fn regular_bound<'a, T>(x: &'a T) -> &'a T 
    where
        T: Debug,
    {
        println!("Regular bound: {:?}", x);
        x
    }
    
    let val = 42;
    regular_bound(&val);
    
    println!("✅ Regular bound: requires specific lifetime 'a");
    
    // Higher-ranked bound
    fn hrtb_bound<T>(x: &T) -> &T
    where
        for<'a> &'a T: Debug,  // For ALL lifetimes 'a, &'a T must implement Debug
    {
        println!("HRTB bound: {:?}", x);
        x
    }
    
    hrtb_bound(&val);
    
    println!("✅ HRTB: for<'a> &'a T: Debug");
    println!("   Bound must hold for ALL possible lifetimes");
}

// --- HRTB with Fn Traits ---
fn hrtb_with_fn_traits() {
    print_subsection("c. HRTB with Fn Traits");
    
    println!("Common use case: Functions taking references\n");
    
    // Function that accepts a closure that works for ANY lifetime
    fn apply_to_ref<F>(f: F)
    where
        F: for<'a> Fn(&'a str) -> &'a str,  // Works for all lifetimes
    {
        let s1 = "hello";
        let result1 = f(s1);
        println!("  f(\"{}\") = \"{}\"", s1, result1);
        
        let s2 = "world";
        let result2 = f(s2);
        println!("  f(\"{}\") = \"{}\"", s2, result2);
    }
    
    // This closure works for any lifetime
    fn first_word(s: &str) -> &str {
        s.split_whitespace().next().unwrap_or(s)
    }
    
    println!("Function: first_word (extracts first word)");
    apply_to_ref(first_word);
    
    println!("\n✅ for<'a> Fn(&'a str) -> &'a str");
    println!("   Closure must work for ANY input lifetime");
    
    println!("\nWithout HRTB:");
    println!("  fn apply<'a, F>(f: F) where F: Fn(&'a str) -> &'a str");
    println!("  Problem: 'a is fixed at call site, not flexible");
}

// --- HRTB with Iterables ---
fn hrtb_with_iterables() {
    print_subsection("d. HRTB with IntoIterator");
    
    println!("Debug implementation for ANY iterable type:\n");
    
    // Generic type that can be iterated over
    struct AnyIterable<T>(T);
    
    // HRTB: For all lifetimes 'a, &'a T must be IntoIterator with Debug items
    impl<T> Debug for AnyIterable<T>
    where
        for<'a> &'a T: IntoIterator,
        for<'a> <&'a T as IntoIterator>::Item: Debug,
    {
        fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
            f.debug_list().entries(&self.0).finish()
        }
    }
    
    let vec = vec![1, 2, 3, 4, 5];
    let iterable = AnyIterable(vec);
    println!("AnyIterable(vec![1,2,3,4,5]) = {:?}", iterable);
    
    let array = [10, 20, 30];
    let iterable = AnyIterable(array);
    println!("AnyIterable([10,20,30]) = {:?}", iterable);
    
    println!("\n✅ Works for Vec, arrays, and any IntoIterator");
    println!("   for<'a> &'a T: IntoIterator");
    println!("   for<'a> <&'a T as IntoIterator>::Item: Debug");
    
    println!("\n📊 When you need HRTB:\n");
    println!("┌──────────────────────────────────────────────────────┐");
    println!("│ Lifetime not in scope                                │");
    println!("├──────────────────────────────────────────────────────┤");
    println!("│ • Bound must work for ALL possible lifetimes         │");
    println!("│ • Common with Fn traits: Fn(&'a T)                   │");
    println!("│ • Associated types with lifetimes                    │");
    println!("│ • Generic over references with any lifetime          │");
    println!("└──────────────────────────────────────────────────────┘");
    
    println!("\nSyntax: for<'a, 'b, ...> where lifetime appears");
}
