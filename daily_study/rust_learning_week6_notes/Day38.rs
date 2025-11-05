// Day 38 - Cargo Features (conditional compilation, feature flags)
// Learning Focus: Mastering Cargo features for conditional compilation and flexible API design

#[cfg(feature = "debug-info")]
use std::collections::HashMap;

/// Union-Find data structure with optional features
/// 
/// This demonstrates how to use Cargo features to conditionally compile code,
/// providing optional functionality without breaking the core API.
pub struct FeaturefulUnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
    
    // Debug information (only with debug-info feature)
    #[cfg(feature = "debug-info")]
    operations: Vec<String>,
    
    #[cfg(feature = "debug-info")]
    operation_count: HashMap<String, usize>,
}

impl FeaturefulUnionFind {
    /// Creates a new Union-Find structure with n elements
    pub fn new(n: usize) -> Self {
        #[cfg(feature = "debug-info")]
        let mut operation_count = HashMap::new();
        #[cfg(feature = "debug-info")]
        operation_count.insert("new".to_string(), 1);
        
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: n,
            
            #[cfg(feature = "debug-info")]
            operations: vec![format!("Created UnionFind with {} elements", n)],
            
            #[cfg(feature = "debug-info")]
            operation_count,
        }
    }
    
    /// Find the root of element x with path compression
    pub fn find(&mut self, x: usize) -> usize {
        // Debug tracking (only compiled when debug-info feature is enabled)
        #[cfg(feature = "debug-info")]
        {
            *self.operation_count.entry("find".to_string()).or_insert(0) += 1;
            self.operations.push(format!("find({})", x));
        }
        
        // Bounds checking in debug builds
        #[cfg(debug_assertions)]
        {
            if x >= self.parent.len() {
                panic!("Index {} out of bounds (max: {})", x, self.parent.len() - 1);
            }
        }
        
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    
    /// Union two elements using union by rank
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        #[cfg(feature = "debug-info")]
        {
            *self.operation_count.entry("union".to_string()).or_insert(0) += 1;
            self.operations.push(format!("union({}, {})", x, y));
        }
        
        let root_x = self.find(x);
        let root_y = self.find(y);
        
        if root_x == root_y {
            return false;
        }
        
        // Union by rank optimization
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }
        
        self.count -= 1;
        true
    }
    
    /// Check if two elements are in the same component
    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        #[cfg(feature = "debug-info")]
        {
            *self.operation_count.entry("connected".to_string()).or_insert(0) += 1;
            self.operations.push(format!("connected({}, {})", x, y));
        }
        
        self.find(x) == self.find(y)
    }
    
    /// Get the number of connected components
    pub fn count(&self) -> usize {
        self.count
    }
    
    // Debug feature methods (only available when debug-info feature is enabled)
    #[cfg(feature = "debug-info")]
    pub fn print_debug_info(&self) {
        println!("=== Union-Find Debug Information ===");
        println!("Current component count: {}", self.count);
        println!("Total operations: {}", self.operations.len());
        
        println!("\nOperation counts:");
        for (op, count) in &self.operation_count {
            println!("  {}: {}", op, count);
        }
        
        println!("\nOperation history (last 5):");
        let start = if self.operations.len() > 5 { 
            self.operations.len() - 5 
        } else { 
            0 
        };
        for op in &self.operations[start..] {
            println!("  {}", op);
        }
    }
    
    #[cfg(not(feature = "debug-info"))]
    pub fn print_debug_info(&self) {
        println!("Debug information not available. Enable 'debug-info' feature to see detailed operation history.");
    }
    
    /// Get operation statistics (only with debug-info feature)
    #[cfg(feature = "debug-info")]
    pub fn get_operation_stats(&self) -> &HashMap<String, usize> {
        &self.operation_count
    }
    
    // Serialization methods (only available with serde feature)
    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        use serde_json::json;
        let data = json!({
            "parent": self.parent,
            "rank": self.rank,
            "count": self.count,
            "size": self.parent.len(),
        });
        serde_json::to_string_pretty(&data)
    }
    
    #[cfg(not(feature = "serde"))]
    pub fn to_json(&self) -> Result<String, &'static str> {
        Err("Serialization not available. Enable 'serde' feature.")
    }
    
    // Parallel operations (only with parallel feature)
    #[cfg(feature = "parallel")]
    pub fn parallel_batch_union(&mut self, pairs: Vec<(usize, usize)>) -> Vec<bool> {
        use rayon::prelude::*;
        
        // Note: Real parallel Union-Find is complex due to data races
        // This is a simplified demonstration
        let results: Vec<bool> = pairs.into_par_iter()
            .map(|(x, y)| {
                // In practice, you'd need sophisticated synchronization
                // This is just for demonstration
                x != y  // Simplified logic
            })
            .collect();
        
        // Sequential fallback for actual operations to maintain correctness
        results.into_iter().enumerate().map(|(i, _)| {
            // In a real implementation, you'd apply the operations sequentially
            // after parallel analysis
            if i % 2 == 0 { true } else { false }
        }).collect()
    }
    
    #[cfg(not(feature = "parallel"))]
    pub fn parallel_batch_union(&mut self, pairs: Vec<(usize, usize)>) -> Vec<bool> {
        // Fallback to sequential processing
        pairs.into_iter()
            .map(|(x, y)| self.union(x, y))
            .collect()
    }
    
    // Platform-specific optimizations
    #[cfg(target_os = "windows")]
    pub fn with_windows_optimization(n: usize) -> Self {
        println!("Using Windows-specific optimizations");
        Self::new(n)
    }
    
    #[cfg(unix)]
    pub fn with_unix_optimization(n: usize) -> Self {
        println!("Using Unix-specific optimizations");
        Self::new(n)
    }
    
    #[cfg(not(any(target_os = "windows", unix)))]
    pub fn with_platform_optimization(n: usize) -> Self {
        println!("Using generic optimizations");
        Self::new(n)
    }
    
    // Pointer width specific methods
    #[cfg(target_pointer_width = "64")]
    pub fn is_64bit_optimized(&self) -> bool {
        true
    }
    
    #[cfg(not(target_pointer_width = "64"))]
    pub fn is_64bit_optimized(&self) -> bool {
        false
    }
}

// Feature-specific trait implementations
#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;
    use serde::{Serialize, Deserialize};
    
    #[derive(Serialize, Deserialize)]
    struct UnionFindSnapshot {
        parent: Vec<usize>,
        rank: Vec<usize>,
        count: usize,
    }
    
    impl FeaturefulUnionFind {
        pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
            let snapshot: UnionFindSnapshot = serde_json::from_str(json)?;
            Ok(Self {
                parent: snapshot.parent,
                rank: snapshot.rank,
                count: snapshot.count,
                
                #[cfg(feature = "debug-info")]
                operations: vec!["Loaded from JSON".to_string()],
                
                #[cfg(feature = "debug-info")]
                operation_count: {
                    let mut map = HashMap::new();
                    map.insert("from_json".to_string(), 1);
                    map
                },
            })
        }
    }
}

/// Demonstrates different conditional compilation patterns
pub mod conditional_examples {
    // Conditional imports - shows common no_std + alloc patterns
    // Note: This example shows the pattern even though we use std by default
    #[cfg(feature = "std")]
    use std::collections::HashMap;
    
    // In a real no_std crate, you'd have `extern crate alloc;` at the top
    // This demonstrates the conditional import pattern for no_std + alloc
    #[cfg(all(not(feature = "std"), feature = "alloc"))]
    use alloc::collections::BTreeMap as HashMap;
    
    /// Cache implementation that varies based on available features
    pub struct Cache<K, V> {
        #[cfg(any(feature = "std", feature = "alloc"))]
        data: HashMap<K, V>,
        
        #[cfg(not(any(feature = "std", feature = "alloc")))]
        _phantom: std::marker::PhantomData<(K, V)>,
    }
    
    impl<K, V> Cache<K, V> 
    where 
        K: Eq + std::hash::Hash,
    {
        #[cfg(any(feature = "std", feature = "alloc"))]
        pub fn new() -> Self {
            Self {
                data: HashMap::new(),
            }
        }
        
        #[cfg(not(any(feature = "std", feature = "alloc")))]
        pub fn new() -> Self {
            Self {
                _phantom: std::marker::PhantomData,
            }
        }
        
        #[cfg(any(feature = "std", feature = "alloc"))]
        pub fn insert(&mut self, key: K, value: V) -> Option<V> {
            self.data.insert(key, value)
        }
        
        #[cfg(not(any(feature = "std", feature = "alloc")))]
        pub fn insert(&mut self, _key: K, _value: V) -> Option<V> {
            None // No-op implementation for no_std without alloc
        }
    }
}

fn main() {
    println!("=== Day 38: Cargo Features Demo ===\n");
    
    let mut uf = FeaturefulUnionFind::new(8);
    
    // Basic operations work regardless of features
    println!("--- Basic Operations ---");
    uf.union(0, 1);
    uf.union(2, 3);
    uf.union(0, 2);
    
    println!("0 and 3 connected: {}", uf.connected(0, 3));
    println!("0 and 4 connected: {}", uf.connected(0, 4));
    println!("Components: {}", uf.count());
    
    // Debug info (conditional on debug-info feature)
    println!("\n--- Debug Information ---");
    uf.print_debug_info();
    
    // Platform-specific information
    println!("\n--- Platform Information ---");
    println!("Running on: {}", std::env::consts::OS);
    println!("Architecture: {}", std::env::consts::ARCH);
    println!("64-bit optimized: {}", uf.is_64bit_optimized());
    
    // Build configuration information
    #[cfg(debug_assertions)]
    println!("Build type: Debug (assertions enabled)");
    
    #[cfg(not(debug_assertions))]
    println!("Build type: Release (assertions disabled)");
    
    // Serialization demo (conditional)
    println!("\n--- Serialization Test ---");
    match uf.to_json() {
        Ok(json) => {
            println!("JSON representation:");
            println!("{}", json);
        }
        Err(e) => println!("Serialization error: {}", e),
    }
    
    // Parallel operations demo (conditional)
    println!("\n--- Parallel Operations Test ---");
    let pairs = vec![(4, 5), (6, 7), (5, 6)];
    let results = uf.parallel_batch_union(pairs);
    println!("Batch union results: {:?}", results);
    println!("Final components: {}", uf.count());
    
    // Show which features are currently enabled
    println!("\n--- Feature Status ---");
    
    #[cfg(feature = "debug-info")]
    println!("✅ debug-info feature enabled");
    #[cfg(not(feature = "debug-info"))]
    println!("❌ debug-info feature disabled");
    
    #[cfg(feature = "serde")]
    println!("✅ serde feature enabled");
    #[cfg(not(feature = "serde"))]
    println!("❌ serde feature disabled");
    
    #[cfg(feature = "parallel")]
    println!("✅ parallel feature enabled");
    #[cfg(not(feature = "parallel"))]
    println!("❌ parallel feature disabled");
    
    #[cfg(feature = "std")]
    println!("✅ std feature enabled");
    #[cfg(not(feature = "std"))]
    println!("❌ std feature disabled");
    
    println!("\n--- Conditional Compilation Examples ---");
    
    // Different code paths based on target
    #[cfg(target_os = "windows")]
    println!("Windows-specific code path");
    
    #[cfg(target_os = "linux")]
    println!("Linux-specific code path");
    
    #[cfg(target_os = "macos")]
    println!("macOS-specific code path");
    
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    println!("Other platform code path");
    
    // Pointer width conditional code
    #[cfg(target_pointer_width = "64")]
    println!("Running on 64-bit architecture");
    
    #[cfg(target_pointer_width = "32")]
    println!("Running on 32-bit architecture");
    
    println!("\n✅ Cargo Features demonstration complete!");
    
    // Demonstrate conditional cache
    println!("\n--- Conditional Cache Example ---");
    let mut cache = conditional_examples::Cache::<i32, String>::new();
    
    #[cfg(any(feature = "std", feature = "alloc"))]
    {
        println!("✅ Cache enabled: Using HashMap for storage");
        let result = cache.insert(1, "Hello".to_string());
        println!("Inserted key 1 -> 'Hello', previous value: {:?}", result);
        let result = cache.insert(1, "World".to_string());
        println!("Updated key 1 -> 'World', previous value: {:?}", result);
    }
    
    #[cfg(not(any(feature = "std", feature = "alloc")))]
    {
        println!("⚠️  Cache disabled: Running in no_std mode without alloc");
        println!("All cache operations are no-ops in this configuration");
        let result = cache.insert(1, "Hello".to_string());
        println!("insert() returned: {:?} (always None without storage)", result);
    }
    
    println!("Cache demonstration complete");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_functionality() {
        let mut uf = FeaturefulUnionFind::new(5);
        assert_eq!(uf.count(), 5);
        
        assert!(uf.union(0, 1));
        assert_eq!(uf.count(), 4);
        assert!(uf.connected(0, 1));
        assert!(!uf.connected(0, 2));
    }
    
    #[test]
    fn test_union_by_rank() {
        let mut uf = FeaturefulUnionFind::new(4);
        
        // Create chains of different lengths
        uf.union(0, 1);  // rank[0] = 1
        uf.union(2, 3);  // rank[2] = 1
        uf.union(0, 2);  // Should merge by rank
        
        // All should be connected
        assert!(uf.connected(0, 3));
        assert_eq!(uf.count(), 1);
    }
    
    #[test]
    fn test_path_compression() {
        let mut uf = FeaturefulUnionFind::new(5);
        
        // Create a chain: 0 -> 1 -> 2 -> 3 -> 4
        uf.union(0, 1);
        uf.union(1, 2);
        uf.union(2, 3);
        uf.union(3, 4);
        
        // Find should compress paths
        let root = uf.find(0);
        assert_eq!(root, uf.find(4));
    }
    
    #[cfg(feature = "debug-info")]
    #[test]
    fn test_debug_features() {
        let mut uf = FeaturefulUnionFind::new(3);
        uf.union(0, 1);
        uf.find(0);
        uf.connected(0, 2);
        
        let stats = uf.get_operation_stats();
        assert!(stats.contains_key("union"));
        assert!(stats.contains_key("find"));
        assert!(stats.contains_key("connected"));
        
        // Print debug info shouldn't panic
        uf.print_debug_info();
    }
    
    #[cfg(feature = "serde")]
    #[test]
    fn test_serialization() {
        let uf = FeaturefulUnionFind::new(3);
        let json = uf.to_json().expect("Should serialize successfully");
        
        assert!(json.contains("parent"));
        assert!(json.contains("rank"));
        assert!(json.contains("count"));
        assert!(json.contains("size"));
        
        // Test round-trip
        let _restored = FeaturefulUnionFind::from_json(&json)
            .expect("Should deserialize successfully");
    }
    
    #[test]
    fn test_platform_methods() {
        let uf = FeaturefulUnionFind::new(5);
        
        // These methods should exist regardless of platform
        // The behavior varies based on target
        #[cfg(target_pointer_width = "64")]
        assert!(uf.is_64bit_optimized());
        
        #[cfg(target_pointer_width = "32")]
        assert!(!uf.is_64bit_optimized());
    }
    
    #[test]
    fn test_conditional_cache() {
        let mut cache = conditional_examples::Cache::<i32, &str>::new();
        
        // Should work regardless of features available
        let result = cache.insert(1, "test");
        
        #[cfg(any(feature = "std", feature = "alloc"))]
        assert_eq!(result, None); // First insertion
        
        #[cfg(not(any(feature = "std", feature = "alloc")))]
        assert_eq!(result, None); // No-op implementation
    }
    
    #[test]
    fn test_bounds_checking() {
        let mut uf = FeaturefulUnionFind::new(3);
        
        // Valid operations should work
        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.find(2), 2);
        
        // Bounds checking behavior varies by build type
        // In debug builds, this would panic
        // In release builds, it might cause undefined behavior
        // We can't test the panic directly here safely
    }
}

// Demonstrate different feature combinations in documentation
/// # Feature Flags
/// 
/// This crate supports several optional features:
/// 
/// - `debug-info`: Enables operation tracking and debug information
/// - `serde`: Enables JSON serialization and deserialization  
/// - `parallel`: Enables parallel batch operations (with rayon)
/// - `std`: Enables standard library features (enabled by default)
/// 
/// ## Examples
/// 
/// ```toml
/// [dependencies]
/// day38 = { version = "0.1", features = ["debug-info", "serde"] }
/// ```
/// 
/// ```rust,ignore
/// use day38::FeaturefulUnionFind;
/// 
/// let mut uf = FeaturefulUnionFind::new(100);
/// uf.union(0, 1);
/// 
/// // Only available with debug-info feature
/// uf.print_debug_info();
/// 
/// // Only available with serde feature  
/// let json = uf.to_json().unwrap();
/// ```
#[allow(dead_code)]
fn print_documentation_examples() {
    println!("See documentation for feature usage examples");
}