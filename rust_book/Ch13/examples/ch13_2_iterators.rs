// Chapter 13.2: Processing a Series of Items with Iterators
// Run with: cargo run --example ch13_2_iterators

fn main() {
    println!("╔═══════════════════════════════════════════════╗");
    println!("║  Chapter 13.2: Iterators                      ║");
    println!("╚═══════════════════════════════════════════════╝\n");

    // Part 1: Iterator Basics
    println!("=== Part 1: Iterator Basics ===");
    iterator_basics();

    // Part 2: Iterator Adaptors
    println!("\n=== Part 2: Iterator Adaptors ===");
    iterator_adaptors();

    // Part 3: Consuming Adaptors
    println!("\n=== Part 3: Consuming Adaptors ===");
    consuming_adaptors();

    // Part 4: Custom Iterators
    println!("\n=== Part 4: Custom Iterators ===");
    custom_iterators();

    // Part 5: Iterator Performance
    println!("\n=== Part 5: Iterator Performance vs Loops ===");
    performance_comparison();
}

fn iterator_basics() {
    // The Iterator trait
    let v = vec![1, 2, 3];
    
    // iter() - borrows each element
    println!("iter() - immutable references:");
    for item in v.iter() {
        print!("{} ", item);
    }
    println!();
    
    // Can still use v
    println!("Original vector: {:?}", v);
    
    // into_iter() - takes ownership
    let v2 = vec![4, 5, 6];
    println!("\ninto_iter() - takes ownership:");
    for item in v2.into_iter() {
        print!("{} ", item);
    }
    println!();
    // v2 is moved, can't use it anymore
    
    // iter_mut() - mutable references
    let mut v3 = vec![1, 2, 3];
    println!("\niter_mut() - mutable references:");
    for item in v3.iter_mut() {
        *item *= 2;
    }
    println!("Modified vector: {:?}", v3);
}

fn iterator_adaptors() {
    let v = vec![1, 2, 3, 4, 5, 6];
    
    // map() - transform each element
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("map (doubled): {:?}", doubled);
    
    // filter() - select elements
    let evens: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
    println!("filter (evens): {:?}", evens);
    
    // Chain adaptors together
    let result: Vec<i32> = v.iter()
        .filter(|x| *x % 2 == 0)    // Keep evens
        .map(|x| x * 2)              // Double them
        .collect();
    println!("filter + map: {:?}", result);
    
    // take() and skip()
    let first_three: Vec<&i32> = v.iter().take(3).collect();
    println!("take(3): {:?}", first_three);
    
    let skip_two: Vec<&i32> = v.iter().skip(2).collect();
    println!("skip(2): {:?}", skip_two);
    
    // enumerate() - add indices
    println!("\nenumerate():");
    for (i, val) in v.iter().enumerate() {
        println!("  Index {}: {}", i, val);
    }
    
    // zip() - combine two iterators
    let letters = vec!['a', 'b', 'c'];
    let zipped: Vec<_> = v.iter().zip(letters.iter()).collect();
    println!("\nzip: {:?}", zipped);
}

fn consuming_adaptors() {
    let v = vec![1, 2, 3, 4, 5];
    
    // collect() - build a collection
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("collect(): {:?}", doubled);
    
    // sum() - add all elements
    let sum: i32 = v.iter().sum();
    println!("sum(): {}", sum);
    
    // product() - multiply all elements
    let product: i32 = v.iter().product();
    println!("product(): {}", product);
    
    // count() - count elements
    let count = v.iter().filter(|x| *x % 2 == 0).count();
    println!("count (evens): {}", count);
    
    // min() and max()
    let min = v.iter().min();
    let max = v.iter().max();
    println!("min: {:?}, max: {:?}", min, max);
    
    // any() and all()
    let has_even = v.iter().any(|x| x % 2 == 0);
    let all_positive = v.iter().all(|x| *x > 0);
    println!("any even: {}, all positive: {}", has_even, all_positive);
    
    // find() - first matching element
    let first_even = v.iter().find(|x| *x % 2 == 0);
    println!("first even: {:?}", first_even);
    
    // fold() - reduce to single value
    let sum_fold = v.iter().fold(0, |acc, x| acc + x);
    println!("fold (sum): {}", sum_fold);
    
    let product_fold = v.iter().fold(1, |acc, x| acc * x);
    println!("fold (product): {}", product_fold);
}

fn custom_iterators() {
    // Implementing Iterator trait
    struct Counter {
        count: u32,
        max: u32,
    }
    
    impl Counter {
        fn new(max: u32) -> Counter {
            Counter { count: 0, max }
        }
    }
    
    impl Iterator for Counter {
        type Item = u32;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }
    
    println!("Custom Counter iterator (1-5):");
    let counter = Counter::new(5);
    for num in counter {
        print!("{} ", num);
    }
    println!();
    
    // Using iterator methods on custom iterator
    let sum: u32 = Counter::new(10).sum();
    println!("Sum of Counter(1-10): {}", sum);
    
    let evens: Vec<u32> = Counter::new(10)
        .filter(|x| x % 2 == 0)
        .collect();
    println!("Even numbers from Counter(1-10): {:?}", evens);
    
    // Chaining custom iterators
    let result: Vec<_> = Counter::new(5)
        .zip(Counter::new(5).skip(1))
        .map(|(a, b)| a * b)
        .collect();
    println!("zip + map: {:?}", result);
}

fn performance_comparison() {
    let data: Vec<i32> = (1..=1000).collect();
    
    // Imperative approach with loop
    let mut sum_loop = 0;
    for &item in &data {
        if item % 2 == 0 {
            sum_loop += item * 2;
        }
    }
    println!("Loop result: {}", sum_loop);
    
    // Functional approach with iterators
    let sum_iter: i32 = data.iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * 2)
        .sum();
    println!("Iterator result: {}", sum_iter);
    
    println!("\nIterators are:");
    println!("  • Zero-cost abstractions (compile to same code as loops)");
    println!("  • More expressive and composable");
    println!("  • Lazily evaluated (only computed when needed)");
    println!("  • Safe from off-by-one errors");
}

// Additional examples
#[allow(dead_code)]
fn advanced_iterator_patterns() {
    let v = vec![1, 2, 3, 4, 5];
    
    // flat_map() - map and flatten
    let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
    let flattened: Vec<i32> = nested.iter()
        .flat_map(|inner| inner.iter())
        .copied()
        .collect();
    println!("flat_map: {:?}", flattened);
    
    // partition() - split into two collections
    let (evens, odds): (Vec<_>, Vec<_>) = v.iter()
        .partition(|x| *x % 2 == 0);
    println!("partition - evens: {:?}, odds: {:?}", evens, odds);
    
    // inspect() - peek at values during iteration
    let doubled: Vec<i32> = v.iter()
        .inspect(|x| println!("  Before: {}", x))
        .map(|x| x * 2)
        .inspect(|x| println!("  After: {}", x))
        .collect();
    println!("inspect result: {:?}", doubled);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_iterator_sum() {
        let v = vec![1, 2, 3, 4, 5];
        let sum: i32 = v.iter().sum();
        assert_eq!(sum, 15);
    }
    
    #[test]
    fn test_iterator_filter_map() {
        let v = vec![1, 2, 3, 4, 5, 6];
        let result: Vec<i32> = v.iter()
            .filter(|x| *x % 2 == 0)
            .map(|x| x * 2)
            .collect();
        assert_eq!(result, vec![4, 8, 12]);
    }
    
    #[test]
    fn test_custom_iterator() {
        struct Counter { count: u32, max: u32 }
        
        impl Counter {
            fn new(max: u32) -> Counter {
                Counter { count: 0, max }
            }
        }
        
        impl Iterator for Counter {
            type Item = u32;
            
            fn next(&mut self) -> Option<Self::Item> {
                if self.count < self.max {
                    self.count += 1;
                    Some(self.count)
                } else {
                    None
                }
            }
        }
        
        let sum: u32 = Counter::new(5).sum();
        assert_eq!(sum, 15); // 1 + 2 + 3 + 4 + 5
    }
}
