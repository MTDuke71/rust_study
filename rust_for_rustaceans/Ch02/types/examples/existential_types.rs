// Example 7: Existential Types - impl Trait
// Demonstrates impl Trait for zero-cost abstraction

use rustaceans_ch02_types::utils::{print_section, print_subsection};

fn main() {
    print_section("7. EXISTENTIAL TYPES - impl Trait");
    
    impl_trait_basics();
    iterator_chains();
    returning_closures();
    comparison_with_dyn();
}

// --- impl Trait Basics ---
fn impl_trait_basics() {
    print_subsection("a. impl Trait in Return Position");
    
    println!("Return 'some type that implements Trait' without naming it:\n");
    
    // Return concrete type without naming it
    fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x + n
    }
    
    let add5 = make_adder(5);
    println!("add5(10) = {}", add5(10));
    println!("add5(20) = {}", add5(20));
    
    println!("\n✅ Compiler knows concrete type (optimization possible)");
    println!("✅ Caller sees only Fn(i32) -> i32 interface");
    println!("✅ No heap allocation (unlike Box<dyn Fn>)");
    
    // Can't name the closure type
    // let add5: ??? = make_adder(5);  // What type?
    // impl Trait solves this!
}

// --- Complex Iterator Chains ---
fn iterator_chains() {
    print_subsection("b. Complex Iterator Chains");
    
    println!("impl Iterator avoids naming complex iterator types:\n");
    
    // Without impl Trait, type would be:
    // Filter<Map<IntoIter<i32>, impl Fn(i32) -> i32>, impl Fn(&i32) -> bool>
    fn process_numbers(nums: Vec<i32>) -> impl Iterator<Item = i32> {
        nums.into_iter()
            .map(|x| x * 2)
            .filter(|x| *x > 10)
    }
    
    let numbers = vec![1, 5, 10, 15, 20];
    let result: Vec<_> = process_numbers(numbers).collect();
    
    println!("Input:  [1, 5, 10, 15, 20]");
    println!("Output: {:?} (doubled, filtered > 10)", result);
    
    // More complex example
    fn fibonacci(limit: u32) -> impl Iterator<Item = u32> {
        let mut a = 0;
        let mut b = 1;
        
        std::iter::from_fn(move || {
            if a > limit {
                return None;
            }
            let next = a + b;
            a = b;
            b = next;
            Some(a)
        })
    }
    
    let fibs: Vec<_> = fibonacci(50).collect();
    println!("\nFibonacci up to 50: {:?}", fibs);
    
    println!("\n✅ Avoids complex, unnameable type signatures");
    println!("✅ Encapsulates implementation details");
    println!("✅ Still allows inlining and optimization");
}

// --- Returning Closures ---
fn returning_closures() {
    print_subsection("c. Returning Closures");
    
    println!("Closures have unique, unnameable types:\n");
    
    // Return different closures based on condition
    fn make_transformer(uppercase: bool) -> impl Fn(&str) -> String {
        if uppercase {
            |s: &str| s.to_uppercase()
        } else {
            |s: &str| s.to_lowercase()
        }
    }
    
    let upper = make_transformer(true);
    let lower = make_transformer(false);
    
    println!("upper(\"Hello\") = {}", upper("Hello"));
    println!("lower(\"Hello\") = {}", lower("Hello"));
    
    println!("\n✅ Each closure has unique type");
    println!("✅ impl Trait allows returning without boxing");
    println!("✅ Zero-cost abstraction");
    
    // Composable transformers
    fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
    where
        F: Fn(A) -> B,
        G: Fn(B) -> C,
    {
        move |x| g(f(x))
    }
    
    let double = |x: i32| x * 2;
    let add_one = |x: i32| x + 1;
    let composed = compose(double, add_one);
    
    println!("\nComposed (double ∘ add_one):");
    println!("  composed(5) = {}", composed(5));  // (5 * 2) + 1 = 11
}

// --- Comparison with dyn Trait ---
fn comparison_with_dyn() {
    print_subsection("d. impl Trait vs dyn Trait");
    
    println!("Comparing two approaches to type erasure:\n");
    
    // impl Trait - static dispatch
    fn static_version(n: i32) -> impl Iterator<Item = i32> {
        (0..n).map(|x| x * 2)
    }
    
    // dyn Trait - dynamic dispatch
    fn dynamic_version(n: i32) -> Box<dyn Iterator<Item = i32>> {
        Box::new((0..n).map(|x| x * 2))
    }
    
    let static_iter = static_version(5);
    let dynamic_iter = dynamic_version(5);
    
    println!("Static:  {:?}", static_iter.collect::<Vec<_>>());
    println!("Dynamic: {:?}", dynamic_iter.collect::<Vec<_>>());
    
    println!("\n┌─────────────────────┬──────────────┬──────────────┐");
    println!("│ Feature             │ impl Trait   │ dyn Trait    │");
    println!("├─────────────────────┼──────────────┼──────────────┤");
    println!("│ Dispatch            │ Static       │ Dynamic      │");
    println!("│ Heap allocation     │ No           │ Yes (Box)    │");
    println!("│ Inlining            │ Possible     │ No           │");
    println!("│ Performance         │ Fast         │ Slower       │");
    println!("│ Binary size         │ Larger       │ Smaller      │");
    println!("│ Return type varies  │ No*          │ Yes          │");
    println!("│ Trait object safe   │ N/A          │ Required     │");
    println!("└─────────────────────┴──────────────┴──────────────┘");
    
    println!("\n*impl Trait: all return paths must return SAME concrete type");
    
    // Example of limitation
    fn must_be_same_type(flag: bool) -> impl Iterator<Item = i32> {
        let iter: Box<dyn Iterator<Item = i32>> = if flag {
            Box::new(0..5)
        } else {
            Box::new(vec![10, 20, 30].into_iter())
        };
        iter.into_iter().map(|x| x)  // Workaround: wrap in another iterator
    }
    
    println!("\nWorkaround for different types:");
    println!("  true:  {:?}", must_be_same_type(true).collect::<Vec<_>>());
    println!("  false: {:?}", must_be_same_type(false).collect::<Vec<_>>());
    
    println!("\n📊 When to use each:\n");
    println!("impl Trait:");
    println!("  ✅ Returning iterators, closures");
    println!("  ✅ Performance critical code");
    println!("  ✅ Avoid complex type names");
    println!("  ❌ Need runtime polymorphism");
    
    println!("\ndyn Trait:");
    println!("  ✅ Collections of different types");
    println!("  ✅ Runtime plugin systems");
    println!("  ✅ Reduce binary size");
    println!("  ❌ Performance critical paths");
}
