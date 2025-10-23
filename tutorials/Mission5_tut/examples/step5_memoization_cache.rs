//! # Step 5: Memoization and Caching
//!
//! **Learning Objective**: Master HashMap-based caching for performance optimization
//!
//! **Time Estimate**: 30 minutes

#![allow(dead_code)]
//!
//! **What You'll Learn**:
//! - Implementing memoization for recursive functions
//! - Building LRU caches with HashMap
//! - Function result caching patterns
//! - Cache hit/miss ratio analysis
//! - Dynamic programming with HashMap storage

use std::collections::HashMap;

/// Example 1: Basic Function Memoization
/// The classic pattern for caching expensive function results
fn example_fibonacci_memoization() {
    println!("🦀 Example 1: Fibonacci Memoization");
    println!("===================================");

    // Memoized Fibonacci implementation
    fn fibonacci_memo(n: u32, cache: &mut HashMap<u32, u64>) -> u64 {
        if let Some(&cached_result) = cache.get(&n) {
            return cached_result;
        }

        let result = match n {
            0 => 0,
            1 => 1,
            _ => fibonacci_memo(n - 1, cache) + fibonacci_memo(n - 2, cache),
        };

        cache.insert(n, result);
        result
    }

    let mut cache = HashMap::new();

    println!("🔢 Computing Fibonacci numbers with memoization:");
    for i in 0..=10 {
        let fib = fibonacci_memo(i, &mut cache);
        println!("  fib({}) = {}", i, fib);
    }

    println!("📊 Cache contains {} entries", cache.len());
    println!("💾 Cache contents: {:?}", cache);

    // Demonstrate performance benefit
    println!("\n⚡ Performance test (fib(40)):");
    let start = std::time::Instant::now();
    let fib_40 = fibonacci_memo(40, &mut cache);
    let duration = start.elapsed();

    println!("  Result: {} (computed in {:?})", fib_40, duration);
    println!("  Cache now has {} entries", cache.len());

    // Compare with naive implementation (WARNING: This is SLOW!)
    fn fibonacci_naive(n: u32) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci_naive(n - 1) + fibonacci_naive(n - 2),
        }
    }

    println!("\n🐌 Performance comparison - Naive fib(40) WITHOUT caching:");
    println!("  ⚠️  WARNING: This will take several seconds...");
    let start_naive = std::time::Instant::now();
    let fib_40_naive = fibonacci_naive(40);
    let duration_naive = start_naive.elapsed();

    println!(
        "  Result: {} (computed in {:?})",
        fib_40_naive, duration_naive
    );

    let speedup = duration_naive.as_nanos() as f64 / duration.as_nanos() as f64;
    println!("\n🚀 MEMOIZATION PERFORMANCE IMPROVEMENT:");
    println!("  📊 Cached version: {:?}", duration);
    println!("  📊 Naive version:  {:?}", duration_naive);
    println!("  🏆 Speedup: {:.0}x faster with memoization!", speedup);

    println!();
}

/// Example 2: Generic Memoizer Struct
/// Reusable memoization wrapper for any function
#[derive(Debug)]
struct Memoizer<K, V> {
    cache: HashMap<K, V>,
    hits: usize,
    misses: usize,
}

impl<K, V> Memoizer<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    fn new() -> Self {
        Memoizer {
            cache: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    fn get_or_compute<F>(&mut self, key: K, compute_fn: F) -> V
    where
        F: FnOnce() -> V,
    {
        if let Some(cached_value) = self.cache.get(&key) {
            self.hits += 1;
            return cached_value.clone();
        }

        self.misses += 1;
        let computed_value = compute_fn();
        self.cache.insert(key.clone(), computed_value.clone());
        computed_value
    }

    fn hit_ratio(&self) -> f32 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as f32 / total as f32
    }

    fn cache_size(&self) -> usize {
        self.cache.len()
    }

    #[allow(dead_code)]
    fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }
}

fn example_generic_memoizer() {
    println!("⚡ Example 2: Generic Memoizer");
    println!("==============================");

    let mut expensive_function_cache = Memoizer::new();

    // Simulate an expensive computation (factorial)
    let expensive_factorial = |n: u32| -> u64 {
        println!("    Computing factorial({}) - expensive operation!", n);
        (1..=n as u64).product()
    };

    println!("🧮 Computing factorials with memoization:");

    // First computation - cache miss
    let fact_5 = expensive_function_cache.get_or_compute(5, || expensive_factorial(5));
    println!("  5! = {}", fact_5);

    // Second computation - cache hit
    let fact_5_again = expensive_function_cache.get_or_compute(5, || expensive_factorial(5));
    println!("  5! = {} (cached)", fact_5_again);

    // More computations
    let fact_6 = expensive_function_cache.get_or_compute(6, || expensive_factorial(6));
    println!("  6! = {}", fact_6);

    let fact_7 = expensive_function_cache.get_or_compute(7, || expensive_factorial(7));
    println!("  7! = {}", fact_7);

    // Check cache again
    let _fact_5_third = expensive_function_cache.get_or_compute(5, || expensive_factorial(5));

    println!("\n📊 Cache Statistics:");
    println!(
        "  Cache size: {} entries",
        expensive_function_cache.cache_size()
    );
    println!(
        "  Hit ratio: {:.1}%",
        expensive_function_cache.hit_ratio() * 100.0
    );
    println!(
        "  Hits: {}, Misses: {}",
        expensive_function_cache.hits, expensive_function_cache.misses
    );
    println!(
        "  💡 Note: Cache contains 3 unique values (5!, 6!, 7!) - duplicates reuse cached results"
    );
    println!("  🔍 Key insight: Unlike Fibonacci, factorial doesn't store intermediate steps");
    println!("     Fibonacci fib(40) → 41 cache entries (0,1,2,...,40)");
    println!("     Factorial fact(7) → 1 cache entry (just 7!)");

    // Performance comparison with recursive factorial
    fn factorial_recursive_memo(n: u32, cache: &mut HashMap<u32, u64>) -> u64 {
        if let Some(&cached_result) = cache.get(&n) {
            return cached_result;
        }

        let result = match n {
            0 | 1 => 1,
            _ => n as u64 * factorial_recursive_memo(n - 1, cache),
        };

        cache.insert(n, result);
        result
    }

    fn factorial_recursive_naive(n: u32) -> u64 {
        match n {
            0 | 1 => 1,
            _ => n as u64 * factorial_recursive_naive(n - 1),
        }
    }

    println!("\n⚡ Factorial Performance Comparison:");
    println!("  Computing multiple factorials to show memoization benefit...");

    // Test multiple calculations where memoization helps
    let test_values = [10, 15, 18, 20, 12, 16, 14, 19];

    let start_multi_memo = std::time::Instant::now();
    let mut multi_cache = HashMap::new();
    for &n in &test_values {
        let _result = factorial_recursive_memo(n, &mut multi_cache);
    }
    let duration_multi_memo = start_multi_memo.elapsed();

    let start_multi_naive = std::time::Instant::now();
    for &n in &test_values {
        let _result = factorial_recursive_naive(n);
    }
    let duration_multi_naive = start_multi_naive.elapsed();

    println!(
        "  📊 Memoized approach: {:?} ({} cache entries built)",
        duration_multi_memo,
        multi_cache.len()
    );
    println!(
        "  📊 Naive approach:    {:?} (recalculated everything)",
        duration_multi_naive
    );

    if duration_multi_naive.as_nanos() > 0 {
        let speedup =
            duration_multi_naive.as_nanos() as f64 / duration_multi_memo.as_nanos() as f64;
        println!("  🚀 Speedup: {:.1}x faster with memoization!", speedup);
    }

    println!("  💡 Key insight: Recursive factorial DOES store intermediate values!");
    println!("  💡 Computing 20! stores 1!, 2!, 3!, ..., 20! (20 cache entries)");
    println!("  🔍 Compare to earlier: iterative factorial stored only final answers");
    println!("     - Iterative: fact(5), fact(6), fact(7) → 3 cache entries");
    println!("     - Recursive: fact(20) → 20 cache entries (1 through 20)");

    println!();
}

/// Example 3: Dynamic Programming with HashMap
/// Using HashMap for bottom-up dynamic programming solutions
fn example_dynamic_programming() {
    println!("🎯 Example 3: Dynamic Programming");
    println!("=================================");

    // Coin change problem: minimum coins needed to make amount
    fn min_coins(amount: u32, coins: &[u32], memo: &mut HashMap<u32, Option<u32>>) -> Option<u32> {
        if amount == 0 {
            return Some(0);
        }

        if let Some(&cached_result) = memo.get(&amount) {
            return cached_result;
        }

        let mut min_count = None;

        for &coin in coins {
            if coin <= amount {
                if let Some(sub_result) = min_coins(amount - coin, coins, memo) {
                    let current_count = sub_result + 1;
                    min_count =
                        Some(min_count.map_or(current_count, |min: u32| min.min(current_count)));
                }
            }
        }

        memo.insert(amount, min_count);
        min_count
    }

    let coins = vec![1, 5, 10, 25]; // penny, nickel, dime, quarter
    let mut memo = HashMap::new();

    println!("💰 Coin Change Problem (coins: {:?}):", coins);

    let test_amounts = vec![11, 15, 27, 30, 43];
    for amount in test_amounts {
        match min_coins(amount, &coins, &mut memo) {
            Some(min_count) => println!("  {}¢ requires {} coins minimum", amount, min_count),
            None => println!("  {}¢ cannot be made with available coins", amount),
        }
    }

    println!("📊 Memoization table size: {} entries", memo.len());

    println!();
}

/// Example 4: LRU Cache Implementation
/// Limited-size cache with Least Recently Used eviction policy
#[derive(Debug)]
struct LRUCache<K, V> {
    capacity: usize,
    cache: HashMap<K, (V, usize)>, // value + access_time
    access_counter: usize,
}

impl<K, V> LRUCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            cache: HashMap::new(),
            access_counter: 0,
        }
    }

    fn get(&mut self, key: &K) -> Option<V> {
        if let Some((value, _)) = self.cache.get(key) {
            let value = value.clone();
            self.access_counter += 1;
            self.cache
                .insert(key.clone(), (value.clone(), self.access_counter));
            Some(value)
        } else {
            None
        }
    }

    fn put(&mut self, key: K, value: V) {
        self.access_counter += 1;

        if self.cache.len() >= self.capacity && !self.cache.contains_key(&key) {
            // Evict least recently used item
            if let Some(lru_key) = self
                .cache
                .iter()
                .min_by_key(|(_, (_, access_time))| *access_time)
                .map(|(k, _)| k.clone())
            {
                self.cache.remove(&lru_key);
            }
        }

        self.cache.insert(key, (value, self.access_counter));
    }

    fn size(&self) -> usize {
        self.cache.len()
    }
}

fn example_lru_cache() {
    println!("🗂️  Example 4: LRU Cache");
    println!("========================");

    let mut lru = LRUCache::new(3); // Capacity of 3

    println!("📝 Adding items to LRU cache (capacity: 3):");

    lru.put("A", "Data A");
    println!("  Added A -> Cache size: {}", lru.size());

    lru.put("B", "Data B");
    println!("  Added B -> Cache size: {}", lru.size());

    lru.put("C", "Data C");
    println!("  Added C -> Cache size: {}", lru.size());

    // Access A to make it recently used
    if let Some(value) = lru.get(&"A") {
        println!("  Accessed A: {}", value);
    }

    // Add D - should evict B (least recently used)
    lru.put("D", "Data D");
    println!(
        "  Added D -> Cache size: {} (evicted least recent)",
        lru.size()
    );

    // Try to access B (should be None - was evicted)
    match lru.get(&"B") {
        Some(value) => println!("  B still in cache: {}", value),
        None => println!("  B was evicted from cache"),
    }

    // Access remaining items
    for key in ["A", "C", "D"] {
        match lru.get(&key) {
            Some(value) => println!("  {}: {}", key, value),
            None => println!("  {}: Not in cache", key),
        }
    }

    println!();
}

/// Example 5: Real-World Application - Web Page Cache
/// Practical caching system for web resources
#[derive(Debug, Clone)]
struct WebPage {
    url: String,
    content: String,
    fetch_time: std::time::Instant,
}

#[derive(Debug)]
struct WebPageCache {
    cache: HashMap<String, WebPage>,
    max_age: std::time::Duration,
    cache_hits: usize,
    cache_misses: usize,
}

impl WebPageCache {
    fn new(max_age_seconds: u64) -> Self {
        WebPageCache {
            cache: HashMap::new(),
            max_age: std::time::Duration::from_secs(max_age_seconds),
            cache_hits: 0,
            cache_misses: 0,
        }
    }

    fn get_page(&mut self, url: &str) -> WebPage {
        // Check if we have a fresh cached version
        if let Some(cached_page) = self.cache.get(url) {
            if cached_page.fetch_time.elapsed() < self.max_age {
                self.cache_hits += 1;
                println!("  📄 Cache HIT for {}", url);
                return cached_page.clone();
            } else {
                println!("  ⏰ Cache EXPIRED for {}", url);
            }
        }

        // Cache miss - "fetch" the page
        self.cache_misses += 1;
        println!("  🌐 Cache MISS - fetching {}", url);

        let page = WebPage {
            url: url.to_string(),
            content: format!("Content of {}", url),
            fetch_time: std::time::Instant::now(),
        };

        self.cache.insert(url.to_string(), page.clone());
        page
    }

    fn stats(&self) -> (usize, usize, f32) {
        let total = self.cache_hits + self.cache_misses;
        let hit_ratio = if total > 0 {
            self.cache_hits as f32 / total as f32
        } else {
            0.0
        };
        (self.cache_hits, self.cache_misses, hit_ratio)
    }
}

fn example_web_cache() {
    println!("🌐 Example 5: Web Page Cache");
    println!("============================");

    let mut web_cache = WebPageCache::new(2); // 2-second cache expiry

    println!("🔄 Simulating web page requests:");

    // First requests - cache misses
    let page1 = web_cache.get_page("https://example.com");
    println!(
        "  📄 Retrieved: {} (content length: {})",
        page1.url,
        page1.content.len()
    );

    let page2 = web_cache.get_page("https://rust-lang.org");
    println!(
        "  📄 Retrieved: {} (content length: {})",
        page2.url,
        page2.content.len()
    );

    // Immediate re-requests - cache hits
    let page1_again = web_cache.get_page("https://example.com");
    println!(
        "  📄 Cached content for {}: {}",
        page1_again.url, page1_again.content
    );

    let _page2_again = web_cache.get_page("https://rust-lang.org");

    // New page
    let page3 = web_cache.get_page("https://github.com");
    println!("  📄 Retrieved: {} (content: {})", page3.url, page3.content);

    // Wait for cache to expire
    println!("  ⏳ Waiting for cache expiry...");
    std::thread::sleep(std::time::Duration::from_secs(3));

    // Request expired page - should be cache miss
    let page1_expired = web_cache.get_page("https://example.com");
    println!(
        "  📄 Fresh content for {}: {}",
        page1_expired.url, page1_expired.content
    );

    let (hits, misses, hit_ratio) = web_cache.stats();
    println!("\n📊 Cache Statistics:");
    println!("  Hits: {}, Misses: {}", hits, misses);
    println!("  Hit ratio: {:.1}%", hit_ratio * 100.0);
    println!("  Cached pages: {}", web_cache.cache.len());

    println!();
}

// 🧪 EXERCISE TIME!

/// EXERCISE 1: Recursive Function Optimizer
///
/// **Task**: Implement memoization for expensive recursive calculations
/// **Challenge**: Compare performance with and without memoization
fn exercise_recursive_optimization() {
    println!("🧪 EXERCISE 1: Recursive Optimization");
    println!("=====================================");

    // Expensive recursive function that benefits from memoization
    fn expensive_calculation_naive(n: u32) -> u64 {
        if n <= 1 {
            return 1;
        }
        expensive_calculation_naive(n - 1) + expensive_calculation_naive(n - 2) + n as u64
    }

    // Memoized version using HashMap<u32, u64>
    fn expensive_calculation_memo(n: u32, cache: &mut HashMap<u32, u64>) -> u64 {
        if let Some(&cached_result) = cache.get(&n) {
            return cached_result;
        }

        let result = if n <= 1 {
            1
        } else {
            expensive_calculation_memo(n - 1, cache)
                + expensive_calculation_memo(n - 2, cache)
                + n as u64
        };

        cache.insert(n, result);
        result
    }

    let test_input = 30u32;
    println!(
        "  Computing expensive_calculation({}) with and without memoization",
        test_input
    );

    // Time memoized version
    let start_memo = std::time::Instant::now();
    let mut cache = HashMap::new();
    let result_memo = expensive_calculation_memo(test_input, &mut cache);
    let duration_memo = start_memo.elapsed();

    // Time naive version with same input
    let start_naive = std::time::Instant::now();
    let result_naive = expensive_calculation_naive(test_input);
    let duration_naive = start_naive.elapsed();

    println!(
        "  📊 Memoized version (n={}): result={}, time={:?}, cache_size={}",
        test_input,
        result_memo,
        duration_memo,
        cache.len()
    );
    println!(
        "  📊 Naive version (n={}): result={}, time={:?}",
        test_input, result_naive, duration_naive
    );

    // Verify both versions produce the same result
    assert_eq!(
        result_memo, result_naive,
        "Memoized and naive versions should produce identical results!"
    );

    let speedup = if duration_memo.as_nanos() > 0 {
        duration_naive.as_nanos() as f64 / duration_memo.as_nanos() as f64
    } else {
        0.0
    };
    println!(
        "  🚀 Actual speedup: {:.0}x faster with memoization!",
        speedup
    );

    println!("✅ Exercise 1 complete!\n");
}

/// EXERCISE 2: Database Query Cache
///
/// **Task**: Build a smart caching system for database queries
/// **Challenge**: Implement cache invalidation and query result expiration
fn exercise_database_cache() {
    println!("🧪 EXERCISE 2: Database Query Cache");
    println!("===================================");

    #[derive(Debug, Clone)]
    struct QueryResult {
        data: String,
        timestamp: std::time::Instant,
    }

    #[derive(Debug)]
    struct DatabaseCache {
        cache: HashMap<String, QueryResult>,
        max_age: std::time::Duration,
        hits: usize,
        misses: usize,
    }

    impl DatabaseCache {
        fn new(max_age_seconds: u64) -> Self {
            DatabaseCache {
                cache: HashMap::new(),
                max_age: std::time::Duration::from_secs(max_age_seconds),
                hits: 0,
                misses: 0,
            }
        }

        fn query(&mut self, sql: &str) -> String {
            // Check if we have a cached result that's still valid
            if let Some(result) = self.cache.get(sql) {
                if result.timestamp.elapsed() < self.max_age {
                    self.hits += 1;
                    println!("  📄 Cache HIT for: {}", sql);
                    return result.data.clone();
                } else {
                    println!("  ⏰ Cache EXPIRED for: {}", sql);
                    self.cache.remove(sql); // Remove expired entry
                }
            }

            // Cache miss - simulate database query
            self.misses += 1;
            println!("  🗃️  Cache MISS - executing: {}", sql);

            // Simulate query execution time
            std::thread::sleep(std::time::Duration::from_millis(10));

            let result_data = format!("Result for: {}", sql);
            let result = QueryResult {
                data: result_data.clone(),
                timestamp: std::time::Instant::now(),
            };

            self.cache.insert(sql.to_string(), result);
            result_data
        }

        fn invalidate_table(&mut self, table_name: &str) {
            let keys_to_remove: Vec<String> = self
                .cache
                .keys()
                .filter(|query| query.contains(table_name))
                .cloned()
                .collect();

            for key in keys_to_remove {
                self.cache.remove(&key);
                println!(
                    "  🗑️  Invalidated cache for table '{}': {}",
                    table_name, key
                );
            }
        }

        fn stats(&self) -> (usize, usize, f32) {
            let total = self.hits + self.misses;
            let hit_ratio = if total == 0 {
                0.0
            } else {
                self.hits as f32 / total as f32
            };
            (self.hits, self.misses, hit_ratio)
        }
    }

    // Simulate database queries with caching
    let mut db_cache = DatabaseCache::new(5); // 5-second cache expiration

    let queries = [
        "SELECT * FROM users WHERE id = 1",
        "SELECT * FROM posts WHERE user_id = 1",
        "SELECT * FROM users WHERE id = 2",
        "SELECT * FROM users WHERE id = 1", // Repeated query - should be cache hit
        "SELECT COUNT(*) FROM posts",
    ];

    println!("  🔄 Executing database queries with caching:");
    for query in &queries {
        let _result = db_cache.query(query);
    }

    // Test cache expiration
    println!("\n  ⏳ Waiting for cache expiration (6 seconds)...");
    std::thread::sleep(std::time::Duration::from_secs(6));

    let _expired_result = db_cache.query("SELECT * FROM users WHERE id = 1");

    // Test cache invalidation
    println!("\n  🔄 Testing cache invalidation:");
    let _result1 = db_cache.query("SELECT * FROM products WHERE category = 'electronics'");
    let _result2 = db_cache.query("SELECT COUNT(*) FROM products");

    println!(
        "  💾 Before invalidation - cache size: {}",
        db_cache.cache.len()
    );
    db_cache.invalidate_table("products");
    println!(
        "  💾 After invalidation - cache size: {}",
        db_cache.cache.len()
    );

    let (hits, misses, hit_ratio) = db_cache.stats();
    println!("\n  📊 Cache Statistics:");
    println!("    Hits: {}, Misses: {}", hits, misses);
    println!("    Hit ratio: {:.1}%", hit_ratio * 100.0);

    println!("✅ Exercise 2 complete!\n");
}

/// EXERCISE 3: Pathfinding with Memoization
///
/// **Task**: Optimize pathfinding algorithms using memoization
/// **Challenge**: Cache shortest paths between nodes in a graph
fn exercise_pathfinding_cache() {
    println!("🧪 EXERCISE 3: Pathfinding Cache");
    println!("=================================");

    use std::collections::{HashSet, VecDeque};

    #[derive(Debug)]
    struct PathfindingCache {
        path_cache: HashMap<(String, String), Vec<String>>,
        hits: usize,
        misses: usize,
    }

    impl PathfindingCache {
        fn new() -> Self {
            PathfindingCache {
                path_cache: HashMap::new(),
                hits: 0,
                misses: 0,
            }
        }

        fn find_path_cached(
            &mut self,
            graph: &HashMap<String, Vec<String>>,
            start: &str,
            end: &str,
        ) -> Vec<String> {
            let cache_key = (start.to_string(), end.to_string());

            // Check cache first
            if let Some(cached_path) = self.path_cache.get(&cache_key) {
                self.hits += 1;
                println!("  📄 Cache HIT for path: {} → {}", start, end);
                return cached_path.clone();
            }

            // Cache miss - compute path using BFS
            self.misses += 1;
            println!("  🔍 Cache MISS - computing path: {} → {}", start, end);

            let path = self.bfs_shortest_path(graph, start, end);

            // Cache the result (cache both directions for efficiency)
            self.path_cache.insert(cache_key, path.clone());
            if !path.is_empty() && path.len() > 1 {
                let reverse_path: Vec<String> = path.iter().rev().cloned().collect();
                let reverse_key = (end.to_string(), start.to_string());
                self.path_cache.insert(reverse_key, reverse_path);
            }

            path
        }

        fn bfs_shortest_path(
            &self,
            graph: &HashMap<String, Vec<String>>,
            start: &str,
            end: &str,
        ) -> Vec<String> {
            if start == end {
                return vec![start.to_string()];
            }

            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();
            let mut parent = HashMap::new();

            queue.push_back(start.to_string());
            visited.insert(start.to_string());

            while let Some(current) = queue.pop_front() {
                if let Some(neighbors) = graph.get(&current) {
                    for neighbor in neighbors {
                        if !visited.contains(neighbor) {
                            visited.insert(neighbor.clone());
                            parent.insert(neighbor.clone(), current.clone());
                            queue.push_back(neighbor.clone());

                            if neighbor == end {
                                // Reconstruct path
                                let mut path = Vec::new();
                                let mut node = end.to_string();

                                while let Some(p) = parent.get(&node) {
                                    path.push(node.clone());
                                    node = p.clone();
                                }
                                path.push(start.to_string());
                                path.reverse();
                                return path;
                            }
                        }
                    }
                }
            }

            vec![] // No path found
        }

        fn stats(&self) -> (usize, usize, f32) {
            let total = self.hits + self.misses;
            let hit_ratio = if total == 0 {
                0.0
            } else {
                self.hits as f32 / total as f32
            };
            (self.hits, self.misses, hit_ratio)
        }
    }

    // Build adjacency list from graph data
    let graph_data = vec![
        ("A", vec!["B", "C"]),
        ("B", vec!["A", "D", "E"]),
        ("C", vec!["A", "F"]),
        ("D", vec!["B"]),
        ("E", vec!["B", "F"]),
        ("F", vec!["C", "E"]),
    ];

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for (node, neighbors) in graph_data {
        graph.insert(
            node.to_string(),
            neighbors.iter().map(|s| s.to_string()).collect(),
        );
    }

    println!("  🗺️  Graph: {:?}", graph);

    let mut pathfinder = PathfindingCache::new();

    // Test pathfinding with multiple queries
    let path_queries = [
        ("A", "F"),
        ("B", "F"),
        ("A", "E"),
        ("A", "F"), // Repeated query - should be cache hit
        ("F", "A"), // Reverse path - should also be cache hit due to bidirectional caching
        ("D", "C"),
        ("B", "F"), // Another repeat - cache hit
    ];

    println!("\n  🔄 Testing pathfinding with caching:");
    for (start, end) in &path_queries {
        let path = pathfinder.find_path_cached(&graph, start, end);
        if path.is_empty() {
            println!("    No path from {} to {}", start, end);
        } else {
            println!("    Path from {} to {}: {}", start, end, path.join(" → "));
        }
    }

    let (hits, misses, hit_ratio) = pathfinder.stats();
    println!("\n  📊 Pathfinding Cache Statistics:");
    println!("    Hits: {}, Misses: {}", hits, misses);
    println!("    Hit ratio: {:.1}%", hit_ratio * 100.0);
    println!("    Cached paths: {}", pathfinder.path_cache.len());

    println!("✅ Exercise 3 complete!\n");
}

// 📝 SOLUTION SECTION

#[cfg(feature = "solutions")]
mod solutions {
    use super::*;

    fn exercise_recursive_optimization_solution() {
        fn expensive_calculation_memo(
            n: u32,
            cache: &mut HashMap<u32, u64>,
            calls: &mut usize,
        ) -> u64 {
            *calls += 1;

            if let Some(&cached) = cache.get(&n) {
                return cached;
            }

            let result = if n <= 1 {
                1
            } else {
                expensive_calculation_memo(n - 1, cache, calls)
                    + expensive_calculation_memo(n - 2, cache, calls)
                    + n as u64
            };

            cache.insert(n, result);
            result
        }

        let test_input = 30;
        let mut cache = HashMap::new();
        let mut memo_calls = 0;

        let start = std::time::Instant::now();
        let memo_result = expensive_calculation_memo(test_input, &mut cache, &mut memo_calls);
        let memo_time = start.elapsed();

        println!("🚀 Memoized version:");
        println!("  Result: {}", memo_result);
        println!("  Time: {:?}", memo_time);
        println!("  Function calls: {}", memo_calls);
        println!("  Cache size: {}", cache.len());
    }
}

/// Main function - runs all examples and exercises
fn main() {
    println!("🦀 Step 5: Memoization and Caching");
    println!("===================================\n");

    // Run all examples
    example_fibonacci_memoization();
    example_generic_memoizer();
    example_dynamic_programming();
    example_lru_cache();
    example_web_cache();

    // Exercise time!
    println!("🎯 Now it's your turn! Complete these exercises:");
    println!("================================================\n");

    exercise_recursive_optimization();
    exercise_database_cache();
    exercise_pathfinding_cache();

    println!("🎉 Congratulations! You've completed Step 5!");
    println!("🏆 You've mastered all HashMap and HashSet patterns!");
    println!("📚 Ready for the final challenge? Run: cargo run --example final_project");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memoizer() {
        let mut memo = Memoizer::new();

        let expensive_fn = || {
            std::thread::sleep(std::time::Duration::from_millis(1));
            42
        };

        // First call - cache miss
        let result1 = memo.get_or_compute("key", expensive_fn);
        assert_eq!(result1, 42);
        assert_eq!(memo.hits, 0);
        assert_eq!(memo.misses, 1);

        // Second call - cache hit
        let result2 = memo.get_or_compute("key", expensive_fn);
        assert_eq!(result2, 42);
        assert_eq!(memo.hits, 1);
        assert_eq!(memo.misses, 1);
    }

    #[test]
    fn test_lru_cache() {
        let mut lru = LRUCache::new(2);

        lru.put("A", 1);
        lru.put("B", 2);

        assert_eq!(lru.get(&"A"), Some(1));
        assert_eq!(lru.get(&"B"), Some(2));

        // Add C, should evict least recently used
        lru.put("C", 3);

        // A should still be there (was accessed recently)
        assert_eq!(lru.get(&"A"), Some(1));
        // B should be evicted
        assert_eq!(lru.get(&"B"), None);
        // C should be there
        assert_eq!(lru.get(&"C"), Some(3));
    }
}
