// Step 3: Iterator Patterns
//
// Learning Objectives:
// - Implement custom Iterator trait
// - Understand lazy evaluation benefits
// - Use extension traits for ergonomic APIs
// - Master iterator composition

fn main() {
    println!("=== Step 3: Iterator Patterns ===\n");

    // Section 1: Why Iterators?
    println!("📖 Section 1: Iterator Benefits\n");
    println!("Iterators provide:");
    println!("✓ Lazy evaluation - compute on demand");
    println!("✓ Zero-cost abstraction - compiles to loops");
    println!("✓ Composability - chain operations");
    println!("✓ Memory efficiency - no intermediate collections\n");

    // Section 2: Custom RangeIter
    println!("📖 Section 2: Custom Range Iterator\n");

    let data = [1, 2, 2, 3, 4, 5, 5, 5, 6, 7];
    println!("Data: {:?}\n", data);

    // Find all occurrences of 5
    let fives: Vec<_> = find_all_equal(&data, &5).collect();
    println!("All 5s at indices: {:?}", fives);

    // Find range [3, 6)
    let range: Vec<_> = find_range(&data, &3, &6).collect();
    println!("Values in range [3,6): {:?}", range);

    // Section 3: Extension Trait Pattern
    println!("\n📖 Section 3: Extension Traits for Ergonomics\n");

    use SearchExt;

    // Method-style calls feel more natural
    let fives: Vec<_> = data.find_all_equal(&5).collect();
    println!("Using extension: {:?}", fives);

    // Chainable operations
    let sum: i32 = data.find_range(&2, &6).map(|&x| x * 2).sum();
    println!("Sum of doubled range: {}", sum);

    // Section 4: Lazy Evaluation Demo
    println!("\n📖 Section 4: Lazy Evaluation Benefits\n");
    demonstrate_lazy_evaluation();

    // Section 5: Iterator Combinators
    println!("\n📖 Section 5: Iterator Composition\n");

    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Complex query: find even numbers > 3, squared
    let result: Vec<_> = data
        .find_range(&4, &10)
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();

    println!("Even squares in range [4,10): {:?}", result);

    println!("\n✅ Self-Assessment:");
    println!("   □ I can implement the Iterator trait");
    println!("   □ I understand lazy vs eager evaluation");
    println!("   □ I can use iterator combinators");
    println!("\nNext: `cargo run --example step4_custom_ordering`");
}

/// Custom iterator over a range of indices
struct RangeIter<'a, T> {
    slice: &'a [T],
    start: usize,
    end: usize,
}

impl<'a, T> RangeIter<'a, T> {
    fn new(slice: &'a [T], start: usize, end: usize) -> Self {
        RangeIter {
            slice,
            start,
            end: end.min(slice.len()),
        }
    }
}

/// Implement Iterator trait
impl<'a, T> Iterator for RangeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let item = &self.slice[self.start];
            self.start += 1;
            Some(item)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end.saturating_sub(self.start);
        (remaining, Some(remaining))
    }
}

/// Find all equal elements
fn find_all_equal<'a, T: Ord>(slice: &'a [T], target: &T) -> RangeIter<'a, T> {
    // Binary search for first occurrence
    let start = match slice.binary_search(target) {
        Ok(mut pos) => {
            while pos > 0 && slice[pos - 1] == *target {
                pos -= 1;
            }
            pos
        }
        Err(_) => return RangeIter::new(slice, 0, 0),
    };

    // Find last occurrence
    let mut end = start;
    while end < slice.len() && slice[end] == *target {
        end += 1;
    }

    RangeIter::new(slice, start, end)
}

/// Find all elements in range [low, high)
fn find_range<'a, T: Ord>(slice: &'a [T], low: &T, high: &T) -> RangeIter<'a, T> {
    let start = match slice.binary_search(low) {
        Ok(pos) => pos,
        Err(pos) => pos,
    };

    let end = match slice.binary_search(high) {
        Ok(pos) => pos,
        Err(pos) => pos,
    };

    RangeIter::new(slice, start, end)
}

/// Extension trait for ergonomic API
trait SearchExt<T> {
    fn find_all_equal(&self, target: &T) -> RangeIter<'_, T>;
    fn find_range(&self, low: &T, high: &T) -> RangeIter<'_, T>;
}

impl<T: Ord> SearchExt<T> for [T] {
    fn find_all_equal(&self, target: &T) -> RangeIter<'_, T> {
        find_all_equal(self, target)
    }

    fn find_range(&self, low: &T, high: &T) -> RangeIter<'_, T> {
        find_range(self, low, high)
    }
}

fn demonstrate_lazy_evaluation() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    println!("Creating iterator (no work done yet)...");
    let iter = data.find_range(&3, &8);

    println!("Iterator created - data not yet accessed!");
    println!("Now calling .take(2) to get first 2 elements...");

    let first_two: Vec<_> = iter.take(2).collect();
    println!("Result: {:?}", first_two);
    println!("Notice: Only needed elements were accessed!");
}
