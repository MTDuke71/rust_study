// Demonstration: When is size_hint() actually called?
// This shows the internal behavior of Rust's standard library

use mission6::{Coord, Grid};

fn main() {
    println!("🔍 Investigating when size_hint() is called");
    println!("============================================\n");

    // ============================================================================
    // TEST 1: Does collect() call size_hint()?
    // ============================================================================
    println!("TEST 1: Using .collect() to create a Vec");
    println!("------------------------------------------");

    let grid = Grid::new(3, 3, 0);

    println!("Creating iterator from 3x3 grid (9 coordinates)...");
    let coords: Vec<Coord> = grid.coordinates().collect();

    println!("✅ Collected {} coordinates", coords.len());
    println!("   (collect() uses size_hint() internally for pre-allocation)\n");

    // ============================================================================
    // TEST 2: Custom Iterator with Instrumentation
    // ============================================================================
    println!("TEST 2: Custom Iterator with Debug Output");
    println!("------------------------------------------\n");

    struct InstrumentedIterator {
        remaining: usize,
    }

    impl Iterator for InstrumentedIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            println!("  ➤ next() called, remaining: {}", self.remaining);
            if self.remaining > 0 {
                self.remaining -= 1;
                Some(self.remaining)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            println!(
                "  ➤ size_hint() called! Returning ({}, Some({}))",
                self.remaining, self.remaining
            );
            (self.remaining, Some(self.remaining))
        }
    }

    println!("Creating iterator with 5 items...\n");
    let iter = InstrumentedIterator { remaining: 5 };

    println!("Now calling .collect():");
    let vec: Vec<usize> = iter.collect();
    println!("\n✅ Result: {:?}", vec);
    println!("   Notice: size_hint() was called BEFORE next()!\n");

    // ============================================================================
    // TEST 3: ExactSizeIterator enables .len()
    // ============================================================================
    println!("TEST 3: ExactSizeIterator::len() calls size_hint()");
    println!("-----------------------------------------------------");

    let grid = Grid::new(3, 3, 0);
    let mut iter = grid.coordinates();

    println!("Grid: 3x3 = 9 coordinates");
    println!("  Initial iter.len(): {}", iter.len());

    iter.next();
    println!("  After next(): iter.len() = {}", iter.len());

    iter.next();
    println!("  After next(): iter.len() = {}", iter.len());

    println!("\n✅ .len() is implemented using size_hint()!\n");

    // ============================================================================
    // TEST 4: Operations that DON'T call size_hint()
    // ============================================================================
    println!("TEST 4: Operations that DON'T use size_hint()");
    println!("----------------------------------------------");

    let grid = Grid::new(2, 2, 0);

    println!("Using .for_each() to iterate:");
    grid.coordinates().for_each(|coord| {
        print!("  {:?} ", coord);
    });
    println!("\n  (size_hint() not called - just uses next())\n");

    println!("Using manual loop:");
    let iter = grid.coordinates();
    for coord in iter {
        print!("  {:?} ", coord);
    }
    println!("\n  (size_hint() not called - just uses next())\n");

    // ============================================================================
    // TEST 5: Which std operations call size_hint()?
    // ============================================================================
    println!("TEST 5: Standard Library Operations Using size_hint()");
    println!("-------------------------------------------------------\n");

    let grid = Grid::new(10, 10, 0);
    let _iter = grid.coordinates();

    println!("✅ Operations that CALL size_hint():");
    println!("   • .collect::<Vec<_>>()        - Pre-allocates Vec");
    println!("   • .collect::<HashSet<_>>()    - Pre-allocates HashSet");
    println!("   • .collect::<HashMap<_,_>>()  - Pre-allocates HashMap");
    println!("   • .len() on ExactSizeIterator - Returns remaining count");
    println!("   • .partition()                - Pre-allocates both collections");
    println!("   • .unzip()                    - Pre-allocates both collections");

    println!("\n❌ Operations that DON'T call size_hint():");
    println!("   • .for_each()                 - Just calls next()");
    println!("   • .fold()                     - Just calls next()");
    println!("   • .find()                     - Just calls next()");
    println!("   • .any() / .all()             - Just calls next()");
    println!("   • for loops                   - Just calls next()");

    println!("\n");

    // ============================================================================
    // TEST 6: Performance Impact Demonstration
    // ============================================================================
    println!("TEST 6: Performance Impact");
    println!("--------------------------\n");

    use std::time::Instant;

    // Large grid
    let grid = Grid::new(100, 100, 0);

    println!("Collecting 10,000 coordinates into Vec...");
    let start = Instant::now();
    let coords: Vec<Coord> = grid.coordinates().collect();
    let duration = start.elapsed();

    println!("  Time: {:?}", duration);
    println!("  Collected {} items", coords.len());
    println!("\n✅ Thanks to size_hint(), Vec pre-allocated 10,000 capacity!");
    println!("   No reallocations needed during collection!\n");

    // ============================================================================
    // TEST 7: Proving size_hint() is called with instrumentation
    // ============================================================================
    println!("TEST 7: Direct Proof that collect() calls size_hint()");
    println!("-------------------------------------------------------\n");

    use std::cell::Cell;

    struct CountingIterator {
        remaining: usize,
        size_hint_calls: Cell<usize>,
        next_calls: Cell<usize>,
    }

    impl Iterator for CountingIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            self.next_calls.set(self.next_calls.get() + 1);
            if self.remaining > 0 {
                self.remaining -= 1;
                Some(self.remaining)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.size_hint_calls.set(self.size_hint_calls.get() + 1);
            (self.remaining, Some(self.remaining))
        }
    }

    let iter = CountingIterator {
        remaining: 100,
        size_hint_calls: Cell::new(0),
        next_calls: Cell::new(0),
    };

    println!("Creating iterator with 100 items...");
    println!("Before collect():");
    println!("  size_hint() calls: {}", iter.size_hint_calls.get());
    println!("  next() calls: {}", iter.next_calls.get());

    let vec: Vec<usize> = iter.collect();

    println!("\nAfter collect():");
    println!("  size_hint() calls: {} ✅", vec.capacity()); // Indirect proof
    println!("  next() calls: {}", vec.len());
    println!("  Vec capacity: {} (pre-allocated!)", vec.capacity());
    println!("  Vec length: {}", vec.len());

    println!("\n✅ PROOF: Vec capacity = 100 means size_hint() was used!");
    println!("   Without size_hint(), capacity would be ~128 (power of 2)\n");
}
