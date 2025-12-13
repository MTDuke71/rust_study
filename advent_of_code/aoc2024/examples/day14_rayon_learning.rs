// Example: Day 14 Rayon Learning Guide
//
// This file demonstrates rayon parallelization concepts using the robot simulation problem.

use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn position_at(&self, seconds: i32, width: i32, height: i32) -> (i32, i32) {
        let x = (self.px + self.vx * seconds).rem_euclid(width);
        let y = (self.py + self.vy * seconds).rem_euclid(height);
        (x, y)
    }
}

/// RAYON CONCEPT 1: par_iter() - Parallel Iterator
/// 
/// Convert any iterator to parallel with `.par_iter()`
fn example_par_iter() {
    let robots = vec![
        Robot { px: 0, py: 0, vx: 1, vy: 1 },
        Robot { px: 10, py: 10, vx: -1, vy: -1 },
        Robot { px: 50, py: 50, vx: 2, vy: 2 },
    ];
    
    // Serial version
    let positions_serial: Vec<_> = robots
        .iter()
        .map(|r| r.position_at(100, 101, 103))
        .collect();
    
    // Parallel version - just add `par_`!
    let positions_parallel: Vec<_> = robots
        .par_iter()  // <-- Only difference!
        .map(|r| r.position_at(100, 101, 103))
        .collect();
    
    assert_eq!(positions_serial, positions_parallel);
    println!("✅ Par_iter: Serial and parallel give identical results");
}

/// RAYON CONCEPT 2: reduce() - Parallel Reduction
/// 
/// Combine results from parallel computations
fn example_reduce() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Serial sum
    let sum_serial = numbers.iter().sum::<i32>();
    
    // Parallel reduce
    let sum_parallel = numbers
        .par_iter()
        .map(|&x| x)
        .reduce(
            || 0,           // Identity: starting value for each thread
            |a, b| a + b    // Combine: how to merge two results
        );
    
    assert_eq!(sum_serial, sum_parallel);
    println!("✅ Reduce: {} (serial) == {} (parallel)", sum_serial, sum_parallel);
}

/// RAYON CONCEPT 3: into_par_iter() - Consuming Parallel Iterator
/// 
/// Takes ownership and processes in parallel
fn example_into_par_iter() {
    let robots = vec![
        Robot { px: 0, py: 0, vx: 1, vy: 1 },
        Robot { px: 10, py: 10, vx: -1, vy: -1 },
    ];
    
    // Move robots and calculate final positions
    let final_positions: Vec<_> = robots
        .into_par_iter()  // Takes ownership
        .map(|r| r.position_at(1000, 101, 103))
        .collect();
    
    println!("✅ Into_par_iter: Processed {} robots", final_positions.len());
}

/// RAYON CONCEPT 4: find_first() - Early Exit
/// 
/// Stop as soon as first match is found
fn example_find_first() {
    use std::time::Instant;
    
    let start = Instant::now();
    
    // Find first number where complex_check returns true
    let result = (0..1_000_000)
        .into_par_iter()
        .find_first(|&n| complex_check(n));
    
    let elapsed = start.elapsed();
    
    println!("✅ Find_first: Found {:?} in {:?}", result, elapsed);
}

fn complex_check(n: i32) -> bool {
    // Simulate expensive check
    n > 500_000 && n % 12345 == 0
}

/// RAYON CONCEPT 5: When to Use Parallel
/// 
/// Parallel has overhead - use when work is "big enough"
fn example_when_to_parallelize() {
    use std::time::Instant;
    
    println!("\n📊 Parallelization Trade-offs:");
    
    // Small workload - serial wins
    let small_work = vec![1, 2, 3, 4, 5];
    
    let start = Instant::now();
    let _: i32 = small_work.iter().sum();
    let serial_small = start.elapsed();
    
    let start = Instant::now();
    let _: i32 = small_work.par_iter().sum();
    let parallel_small = start.elapsed();
    
    println!("Small (5 items):");
    println!("  Serial:   {:?}", serial_small);
    println!("  Parallel: {:?}", parallel_small);
    
    // Large workload - parallel wins
    let large_work: Vec<i64> = (0..1_000_000).collect();
    
    let start = Instant::now();
    let _: i64 = large_work.iter().map(|&x| (x % 1000) * 2 + 1).sum();
    let serial_large = start.elapsed();
    
    let start = Instant::now();
    let _: i64 = large_work.par_iter().map(|&x| (x % 1000) * 2 + 1).sum();
    let parallel_large = start.elapsed();
    
    println!("Large (1M items):");
    println!("  Serial:   {:?}", serial_large);
    println!("  Parallel: {:?}", parallel_large);
    println!("  Speedup:  {:.2}x", serial_large.as_secs_f64() / parallel_large.as_secs_f64());
}

/// RAYON CONCEPT 6: Parallel Collect
/// 
/// Build collections in parallel
fn example_par_extend() {
    let robots: Vec<Robot> = (0..1000)
        .into_par_iter()
        .map(|i| Robot {
            px: i % 101,
            py: i % 103,
            vx: (i * 17) % 11 - 5,
            vy: (i * 23) % 13 - 6,
        })
        .collect();
    
    println!("✅ Par_extend: Generated {} robots in parallel", robots.len());
}

fn main() {
    println!("🦀 Rayon Learning with Day 14 Robots\n");
    
    println!("1️⃣ Parallel Iteration:");
    example_par_iter();
    
    println!("\n2️⃣ Parallel Reduction:");
    example_reduce();
    
    println!("\n3️⃣ Consuming Parallel Iterator:");
    example_into_par_iter();
    
    println!("\n4️⃣ Early Exit with find_first:");
    example_find_first();
    
    println!("\n5️⃣ Parallelization Trade-offs:");
    example_when_to_parallelize();
    
    println!("\n6️⃣ Parallel Collection Building:");
    example_par_extend();
    
    println!("\n✨ Key Takeaways:");
    println!("  • par_iter() - Easy parallelization, just add 'par_'");
    println!("  • reduce() - Combine parallel results");
    println!("  • find_first() - Early exit for searches");
    println!("  • Parallel has overhead - use for large workloads");
    println!("  • Rule of thumb: >10k items or expensive per-item work");
}
