//! # Mission 10: Union-Find Optimization Comparison Benchmarks
//! 
//! Compares performance with and without key optimizations:
//! - Path compression vs no path compression
//! - Union by rank vs naive union
//! - Combined optimizations vs naive implementation
//!
//! This demonstrates the dramatic performance improvements from optimizations.
//!
//! Run with: `cargo bench --bench comparison`

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

// ============================================================================
// Naive Union-Find Implementation (No Optimizations)
// ============================================================================

/// Naive Union-Find without path compression or union by rank
#[derive(Debug)]
pub struct NaiveUnionFind {
    parent: Vec<usize>,
    components: usize,
}

impl NaiveUnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            components: n,
        }
    }
    
    /// Find without path compression - O(n) worst case
    pub fn find(&self, x: usize) -> Result<usize, String> {
        if x >= self.parent.len() {
            return Err(format!("Element {} out of bounds", x));
        }
        
        let mut current = x;
        while self.parent[current] != current {
            current = self.parent[current];
        }
        Ok(current)
    }
    
    /// Union without rank consideration - O(n) worst case
    pub fn union(&mut self, x: usize, y: usize) -> Result<bool, String> {
        let root_x = self.find(x)?;
        let root_y = self.find(y)?;
        
        if root_x == root_y {
            return Ok(false);
        }
        
        // Always make x's root point to y's root (no rank consideration)
        self.parent[root_x] = root_y;
        self.components -= 1;
        Ok(true)
    }
    
    pub fn connected(&self, x: usize, y: usize) -> Result<bool, String> {
        Ok(self.find(x)? == self.find(y)?)
    }
    
    pub fn count(&self) -> usize {
        self.components
    }
    
    pub fn len(&self) -> usize {
        self.parent.len()
    }
}

// ============================================================================
// Path Compression Only Implementation
// ============================================================================

/// Union-Find with path compression but no union by rank
#[derive(Debug)]
pub struct PathCompressionUnionFind {
    parent: Vec<usize>,
    components: usize,
}

impl PathCompressionUnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            components: n,
        }
    }
    
    /// Find with path compression - O(log n) amortized
    pub fn find(&mut self, x: usize) -> Result<usize, String> {
        if x >= self.parent.len() {
            return Err(format!("Element {} out of bounds", x));
        }
        
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x])?; // Path compression
        }
        Ok(self.parent[x])
    }
    
    /// Union without rank consideration
    pub fn union(&mut self, x: usize, y: usize) -> Result<bool, String> {
        let root_x = self.find(x)?;
        let root_y = self.find(y)?;
        
        if root_x == root_y {
            return Ok(false);
        }
        
        self.parent[root_x] = root_y;
        self.components -= 1;
        Ok(true)
    }
    
    pub fn connected(&mut self, x: usize, y: usize) -> Result<bool, String> {
        Ok(self.find(x)? == self.find(y)?)
    }
    
    pub fn count(&self) -> usize {
        self.components
    }
    
    pub fn len(&self) -> usize {
        self.parent.len()
    }
}

// ============================================================================
// Union by Rank Only Implementation
// ============================================================================

/// Union-Find with union by rank but no path compression
#[derive(Debug)]
pub struct UnionByRankUnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    components: usize,
}

impl UnionByRankUnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            components: n,
        }
    }
    
    /// Find without path compression - O(log n) worst case
    pub fn find(&self, x: usize) -> Result<usize, String> {
        if x >= self.parent.len() {
            return Err(format!("Element {} out of bounds", x));
        }
        
        let mut current = x;
        while self.parent[current] != current {
            current = self.parent[current];
        }
        Ok(current)
    }
    
    /// Union with rank consideration - O(log n)
    pub fn union(&mut self, x: usize, y: usize) -> Result<bool, String> {
        let root_x = self.find(x)?;
        let root_y = self.find(y)?;
        
        if root_x == root_y {
            return Ok(false);
        }
        
        // Union by rank
        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => {
                self.parent[root_x] = root_y;
            }
            std::cmp::Ordering::Greater => {
                self.parent[root_y] = root_x;
            }
            std::cmp::Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
        
        self.components -= 1;
        Ok(true)
    }
    
    pub fn connected(&self, x: usize, y: usize) -> Result<bool, String> {
        Ok(self.find(x)? == self.find(y)?)
    }
    
    pub fn count(&self) -> usize {
        self.components
    }
    
    pub fn len(&self) -> usize {
        self.parent.len()
    }
}

// ============================================================================
// Comparison Benchmarks
// ============================================================================

use mission10::UnionFind as OptimizedUnionFind;

const COMPARISON_SIZES: &[usize] = &[100, 500, 1_000, 5_000, 10_000];

fn bench_find_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("find_comparison");
    
    for &size in COMPARISON_SIZES {
        group.throughput(Throughput::Elements(size as u64));
        
        // Setup: Create worst-case chain structure for all implementations
        
        // Naive implementation
        let mut naive_uf = NaiveUnionFind::new(size);
        for i in 1..size {
            naive_uf.union(i - 1, i).unwrap();
        }
        
        group.bench_with_input(
            BenchmarkId::new("naive", size),
            &size,
            |b, &s| {
                b.iter(|| {
                    // Find deepest element multiple times (no path compression benefit)
                    for _ in 0..10 {
                        black_box(naive_uf.find(s - 1).unwrap());
                    }
                });
            },
        );
        
        // Path compression only
        let mut pc_uf = PathCompressionUnionFind::new(size);
        for i in 1..size {
            pc_uf.union(i - 1, i).unwrap();
        }
        
        group.bench_with_input(
            BenchmarkId::new("path_compression", size),
            &size,
            |b, &s| {
                b.iter(|| {
                    // Find deepest element multiple times (benefits from path compression)
                    for _ in 0..10 {
                        black_box(pc_uf.find(s - 1).unwrap());
                    }
                });
            },
        );
        
        // Union by rank only
        let mut rank_uf = UnionByRankUnionFind::new(size);
        for i in 1..size {
            rank_uf.union(i - 1, i).unwrap();
        }
        
        group.bench_with_input(
            BenchmarkId::new("union_by_rank", size),
            &size,
            |b, &s| {
                b.iter(|| {
                    for _ in 0..10 {
                        black_box(rank_uf.find(s - 1).unwrap());
                    }
                });
            },
        );
        
        // Optimized (both optimizations)
        let mut opt_uf = OptimizedUnionFind::new(size);
        for i in 1..size {
            opt_uf.union(i - 1, i).unwrap();
        }
        
        group.bench_with_input(
            BenchmarkId::new("optimized", size),
            &size,
            |b, &s| {
                b.iter(|| {
                    for _ in 0..10 {
                        black_box(opt_uf.find(s - 1).unwrap());
                    }
                });
            },
        );
    }
    
    group.finish();
}

fn bench_union_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("union_comparison");
    
    for &size in COMPARISON_SIZES {
        group.throughput(Throughput::Elements(size as u64));
        
        // Naive union (creates unbalanced trees)
        group.bench_with_input(
            BenchmarkId::new("naive", size),
            &size,
            |b, &s| {
                b.iter(|| {
                    let mut uf = NaiveUnionFind::new(s);
                    for i in 1..s {
                        black_box(uf.union(i - 1, i).unwrap());
                    }
                });
            },
        );
        
        // Path compression only
        group.bench_with_input(
            BenchmarkId::new("path_compression", size),
            &size,
            |b, &s| {
                b.iter(|| {
                    let mut uf = PathCompressionUnionFind::new(s);
                    for i in 1..s {
                        black_box(uf.union(i - 1, i).unwrap());
                    }
                });
            },
        );
        
        // Union by rank only
        group.bench_with_input(
            BenchmarkId::new("union_by_rank", size),
            &size,
            |b, &s| {
                b.iter(|| {
                    let mut uf = UnionByRankUnionFind::new(s);
                    for i in 1..s {
                        black_box(uf.union(i - 1, i).unwrap());
                    }
                });
            },
        );
        
        // Optimized implementation
        group.bench_with_input(
            BenchmarkId::new("optimized", size),
            &size,
            |b, &s| {
                b.iter(|| {
                    let mut uf = OptimizedUnionFind::new(s);
                    for i in 1..s {
                        black_box(uf.union(i - 1, i).unwrap());
                    }
                });
            },
        );
    }
    
    group.finish();
}

fn bench_connected_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("connected_comparison");
    
    for &size in &[100, 500, 1_000, 2_000] {
        group.throughput(Throughput::Elements((size * size / 4) as u64));
        
        // Create chain structure and test many connectivity queries
        
        // Naive
        let mut naive_uf = NaiveUnionFind::new(size);
        for i in 1..size {
            naive_uf.union(i - 1, i).unwrap();
        }
        
        group.bench_with_input(
            BenchmarkId::new("naive", size),
            &size,
            |b, &s| {
                b.iter(|| {
                    for i in 0..(s / 4) {
                        for j in (i + 1)..(s / 4) {
                            black_box(naive_uf.connected(i, j).unwrap());
                        }
                    }
                });
            },
        );
        
        // Path compression only
        let mut pc_uf = PathCompressionUnionFind::new(size);
        for i in 1..size {
            pc_uf.union(i - 1, i).unwrap();
        }
        
        group.bench_with_input(
            BenchmarkId::new("path_compression", size),
            &size,
            |b, &s| {
                b.iter(|| {
                    for i in 0..(s / 4) {
                        for j in (i + 1)..(s / 4) {
                            black_box(pc_uf.connected(i, j).unwrap());
                        }
                    }
                });
            },
        );
        
        // Union by rank only
        let mut rank_uf = UnionByRankUnionFind::new(size);
        for i in 1..size {
            rank_uf.union(i - 1, i).unwrap();
        }
        
        group.bench_with_input(
            BenchmarkId::new("union_by_rank", size),
            &size,
            |b, &s| {
                b.iter(|| {
                    for i in 0..(s / 4) {
                        for j in (i + 1)..(s / 4) {
                            black_box(rank_uf.connected(i, j).unwrap());
                        }
                    }
                });
            },
        );
        
        // Optimized
        let mut opt_uf = OptimizedUnionFind::new(size);
        for i in 1..size {
            opt_uf.union(i - 1, i).unwrap();
        }
        
        group.bench_with_input(
            BenchmarkId::new("optimized", size),
            &size,
            |b, &s| {
                b.iter(|| {
                    for i in 0..(s / 4) {
                        for j in (i + 1)..(s / 4) {
                            black_box(opt_uf.connected(i, j).unwrap());
                        }
                    }
                });
            },
        );
    }
    
    group.finish();
}

fn bench_worst_case_scenario(c: &mut Criterion) {
    let mut group = c.benchmark_group("worst_case_scenarios");
    
    // Deep chain traversal - shows dramatic difference
    let size = 1000;
    group.throughput(Throughput::Elements(size as u64));
    
    // Naive: O(n) every find
    group.bench_function("naive_deep_find", |b| {
        b.iter(|| {
            let mut uf = NaiveUnionFind::new(size);
            // Create deep chain
            for i in 1..size {
                uf.union(i - 1, i).unwrap();
            }
            // Find deepest element 100 times (no optimization)
            for _ in 0..100 {
                black_box(uf.find(size - 1).unwrap());
            }
        });
    });
    
    // Path compression: O(n) first, O(1) subsequent
    group.bench_function("path_compression_deep_find", |b| {
        b.iter(|| {
            let mut uf = PathCompressionUnionFind::new(size);
            for i in 1..size {
                uf.union(i - 1, i).unwrap();
            }
            // Find deepest element 100 times (benefits from path compression)
            for _ in 0..100 {
                black_box(uf.find(size - 1).unwrap());
            }
        });
    });
    
    // Union by rank: O(log n) consistently
    group.bench_function("union_by_rank_deep_find", |b| {
        b.iter(|| {
            let mut uf = UnionByRankUnionFind::new(size);
            for i in 1..size {
                uf.union(i - 1, i).unwrap();
            }
            for _ in 0..100 {
                black_box(uf.find(size - 1).unwrap());
            }
        });
    });
    
    // Optimized: O(α(n)) amortized
    group.bench_function("optimized_deep_find", |b| {
        b.iter(|| {
            let mut uf = OptimizedUnionFind::new(size);
            for i in 1..size {
                uf.union(i - 1, i).unwrap();
            }
            for _ in 0..100 {
                black_box(uf.find(size - 1).unwrap());
            }
        });
    });
    
    group.finish();
}

fn bench_tree_structure_evolution(c: &mut Criterion) {
    let mut group = c.benchmark_group("tree_structure_evolution");
    
    let size = 1000;
    
    // Show how tree structure affects performance over time
    group.bench_function("naive_progressive_finds", |b| {
        b.iter(|| {
            let mut uf = NaiveUnionFind::new(size);
            
            // Progressive union and find operations
            for i in 1..size {
                uf.union(i - 1, i).unwrap();
                // Tree gets deeper, finds get slower
                black_box(uf.find(i).unwrap());
            }
        });
    });
    
    group.bench_function("optimized_progressive_finds", |b| {
        b.iter(|| {
            let mut uf = OptimizedUnionFind::new(size);
            
            // Progressive union and find operations  
            for i in 1..size {
                uf.union(i - 1, i).unwrap();
                // Path compression keeps finds fast
                black_box(uf.find(i).unwrap());
            }
        });
    });
    
    group.finish();
}

fn bench_memory_usage_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    
    for &size in &[1_000, 10_000, 100_000] {
        group.throughput(Throughput::Elements(size as u64));
        
        // Naive: only parent array
        group.bench_with_input(
            BenchmarkId::new("naive_creation", size),
            &size,
            |b, &s| {
                b.iter(|| {
                    black_box(NaiveUnionFind::new(s));
                });
            },
        );
        
        // Union by rank: parent + rank arrays
        group.bench_with_input(
            BenchmarkId::new("rank_creation", size),
            &size,
            |b, &s| {
                b.iter(|| {
                    black_box(UnionByRankUnionFind::new(s));
                });
            },
        );
        
        // Optimized: parent + rank arrays (same as union by rank)
        group.bench_with_input(
            BenchmarkId::new("optimized_creation", size),
            &size,
            |b, &s| {
                b.iter(|| {
                    black_box(OptimizedUnionFind::new(s));
                });
            },
        );
    }
    
    group.finish();
}

// ============================================================================
// Performance Summary Benchmark
// ============================================================================

fn bench_performance_summary(c: &mut Criterion) {
    let mut group = c.benchmark_group("performance_summary");
    
    let size = 5000;
    let operations = 10000;
    
    group.throughput(Throughput::Elements(operations as u64));
    
    // Mixed workload comparison showing overall performance difference
    group.bench_function("naive_mixed_workload", |b| {
        b.iter(|| {
            let mut uf = NaiveUnionFind::new(size);
            let mut rng_state = 12345u64;
            
            for _ in 0..operations {
                rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
                let op_type = rng_state % 10;
                
                rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
                let a = (rng_state % size as u64) as usize;
                
                rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
                let b = (rng_state % size as u64) as usize;
                
                if a != b {
                    match op_type {
                        0..=3 => { black_box(uf.union(a, b).unwrap()); }
                        4..=8 => { black_box(uf.connected(a, b).unwrap()); }
                        9 => { /* count is O(1) for all */ black_box(uf.count()); }
                        _ => unreachable!(),
                    }
                }
            }
        });
    });
    
    group.bench_function("optimized_mixed_workload", |b| {
        b.iter(|| {
            let mut uf = OptimizedUnionFind::new(size);
            let mut rng_state = 12345u64;
            
            for _ in 0..operations {
                rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
                let op_type = rng_state % 10;
                
                rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
                let a = (rng_state % size as u64) as usize;
                
                rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
                let b = (rng_state % size as u64) as usize;
                
                if a != b {
                    match op_type {
                        0..=3 => { black_box(uf.union(a, b).unwrap()); }
                        4..=8 => { black_box(uf.connected(a, b).unwrap()); }
                        9 => { black_box(uf.count()); }
                        _ => unreachable!(),
                    }
                }
            }
        });
    });
    
    group.finish();
}

// ============================================================================
// Benchmark Groups
// ============================================================================

criterion_group!(
    comparison_benches,
    bench_find_comparison,
    bench_union_comparison,
    bench_connected_comparison
);

criterion_group!(
    optimization_impact_benches,
    bench_worst_case_scenario,
    bench_tree_structure_evolution,
    bench_memory_usage_comparison
);

criterion_group!(
    summary_benches,
    bench_performance_summary
);

criterion_main!(
    comparison_benches,
    optimization_impact_benches,
    summary_benches
);