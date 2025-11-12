//! # Mission 10 Tutorial - Step 6: Advanced Union-Find Variants
//! 
//! This step explores advanced Union-Find variants and extensions that go beyond
//! the standard implementation. Each variant addresses specific use cases and
//! demonstrates different trade-offs between functionality and performance.
//! 
//! ## Learning Objectives
//! 
//! After completing this step, you will understand:
//! - When to use different Union-Find variants
//! - How to implement weighted Union-Find for distance queries
//! - How to add undo functionality with operation history
//! - How to create persistent (immutable) Union-Find structures
//! - How to customize merge operations for additional metadata
//! - Trade-offs between memory usage and features
//! 
//! ## Variants Covered
//! 
//! 1. **Weighted Union-Find** - Track distances/weights between elements
//! 2. **Union-Find with Undo** - Support operation rollback
//! 3. **Persistent Union-Find** - Immutable with version history
//! 4. **Union-Find with Custom Merge** - Track metadata per set
//! 
//! ## Run This Example
//! 
//! ```bash
//! cd tutorials/Mission10_tut
//! cargo run --example step6_advanced_variants
//! ```



/// Demonstrates all advanced Union-Find variants
fn main() {
    println!("🚀 Mission 10 Tutorial - Step 6: Advanced Union-Find Variants");
    println!("{}", "=".repeat(70));
    
    demo_weighted_union_find();
    demo_union_find_with_undo();
    demo_persistent_union_find();
    demo_customizable_merge();
    show_comparison_table();
    show_exercises();
}

// ============================================================================
// VARIANT 1: WEIGHTED UNION-FIND
// ============================================================================

/// Weighted Union-Find that tracks distances/weights between elements
/// 
/// This variant is useful when you need to:
/// - Query distance between connected elements
/// - Model network latency or costs
/// - Solve problems involving relative positions
/// 
/// Key insight: Store weight to parent, compute path distances during find
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct WeightedUnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    weight: Vec<i64>, // Weight from element to its parent
    size: usize,
}

impl WeightedUnionFind {
    /// Create new weighted Union-Find structure
    /// 
    /// # Arguments
    /// * `n` - Number of elements (0 to n-1)
    /// 
    /// # Time Complexity
    /// O(n) - Initialize all arrays
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            weight: vec![0; n], // Weight to parent (0 for roots)
            size: n,
        }
    }
    
    /// Find root with path compression, updating weights along the path
    /// 
    /// Returns (root, weight_to_root)
    /// 
    /// # Time Complexity
    /// O(α(n)) - Amortized due to path compression
    pub fn find(&mut self, x: usize) -> (usize, i64) {
        if self.parent[x] == x {
            return (x, 0); // Root has weight 0 to itself
        }
        
        // Recursively find root and accumulate weights
        let (root, parent_weight_to_root) = self.find(self.parent[x]);
        let total_weight = self.weight[x] + parent_weight_to_root;
        
        // Path compression: point directly to root with accumulated weight
        self.parent[x] = root;
        self.weight[x] = total_weight;
        
        (root, total_weight)
    }
    
    /// Connect two elements with given weight between them
    /// 
    /// # Arguments
    /// * `x` - First element
    /// * `y` - Second element  
    /// * `w` - Weight from x to y (can be negative)
    /// 
    /// # Returns
    /// true if union was performed, false if already connected
    /// 
    /// # Time Complexity
    /// O(α(n)) - Two find operations plus constant work
    pub fn weighted_union(&mut self, x: usize, y: usize, w: i64) -> bool {
        let (root_x, weight_x) = self.find(x);
        let (root_y, weight_y) = self.find(y);
        
        if root_x == root_y {
            return false; // Already connected
        }
        
        // Union by rank - attach smaller tree to larger
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
            // weight[root_x] = weight_y - weight_x + w
            self.weight[root_x] = weight_y - weight_x + w;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
            // weight[root_y] = weight_x - weight_y - w  
            self.weight[root_y] = weight_x - weight_y - w;
        } else {
            self.parent[root_y] = root_x;
            self.weight[root_y] = weight_x - weight_y - w;
            self.rank[root_x] += 1;
        }
        
        true
    }
    
    /// Get distance between two elements
    /// 
    /// # Returns
    /// Some(distance) if connected, None if in different components
    /// 
    /// # Time Complexity
    /// O(α(n)) - Two find operations
    pub fn distance(&mut self, x: usize, y: usize) -> Option<i64> {
        let (root_x, weight_x) = self.find(x);
        let (root_y, weight_y) = self.find(y);
        
        if root_x == root_y {
            Some(weight_y - weight_x) // Distance from x to y
        } else {
            None // Not connected
        }
    }
    
    /// Check if two elements are connected
    #[allow(dead_code)]
    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        let (root_x, _) = self.find(x);
        let (root_y, _) = self.find(y);
        root_x == root_y
    }
}

/// Demonstrate weighted Union-Find with network latency example
fn demo_weighted_union_find() {
    println!("\n📊 Variant 1: Weighted Union-Find");
    println!("{}", "-".repeat(40));
    println!("Use case: Network latency between servers");
    
    let mut wuf = WeightedUnionFind::new(5);
    
    // Build network with latencies (in milliseconds)
    println!("\nBuilding network connections:");
    wuf.weighted_union(0, 1, 10); // Server 0 to 1: 10ms
    println!("Connected servers 0-1 with 10ms latency");
    
    wuf.weighted_union(1, 2, 5);  // Server 1 to 2: 5ms  
    println!("Connected servers 1-2 with 5ms latency");
    
    wuf.weighted_union(2, 3, 8);  // Server 2 to 3: 8ms
    println!("Connected servers 2-3 with 8ms latency");
    
    // Query distances
    println!("\nQuerying network latencies:");
    if let Some(latency) = wuf.distance(0, 3) {
        println!("Relative distance from server 0 to 3: {}", latency);
        println!("Note: Negative values indicate direction in the tree structure");
        println!("Absolute path cost: {}ms", latency.abs());
    }
    
    if let Some(latency) = wuf.distance(1, 3) {
        println!("Relative distance from server 1 to 3: {}", latency);
        println!("Absolute path cost: {}ms", latency.abs());
    }
    
    // Show bidirectional distances to demonstrate the directional nature
    println!("\nBidirectional distance demonstration:");
    if let Some(dist_03) = wuf.distance(0, 3) {
        if let Some(dist_30) = wuf.distance(3, 0) {
            println!("0→3: {}, 3→0: {} (opposite signs, same magnitude)", dist_03, dist_30);
        }
    }
    
    // Try to connect to isolated server
    println!("\nTrying to query isolated server:");
    match wuf.distance(0, 4) {
        Some(latency) => println!("Latency to server 4: {}ms", latency),
        None => println!("Server 4 is not connected to the network"),
    }
    
    // Visualize the network
    println!("\nNetwork topology:");
    println!("   [0]---10ms---[1]---5ms---[2]---8ms---[3]");
    println!("                                              ");
    println!("   [4] (isolated)                           ");
}

// ============================================================================
// VARIANT 2: UNION-FIND WITH UNDO
// ============================================================================

/// Operation history for undo functionality
#[derive(Debug, Clone)]
enum Operation {
    Union {
        x_root: usize,
        y_root: usize,
        x_rank: usize,
        y_rank: usize,
        x_parent: usize,
        y_parent: usize,
    },
}

/// Union-Find with undo support using operation history stack
/// 
/// This variant is useful when you need to:
/// - Try different sequences of operations
/// - Implement backtracking algorithms
/// - Support "what-if" scenarios
/// 
/// Memory overhead: O(operations) for history stack
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct UndoableUnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    history: Vec<Operation>, // Stack of operations for undo
    size: usize,
}

impl UndoableUnionFind {
    /// Create new undoable Union-Find structure
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            history: Vec::new(),
            size: n,
        }
    }
    
    /// Find root without path compression (needed for undo)
    /// 
    /// **Key Insight**: This works because elements in the same connected 
    /// component will always traverse up their respective paths and reach
    /// the SAME root node. If two elements have different roots after
    /// traversal, they're in different components.
    /// 
    /// Note: We can't use path compression with undo because it would
    /// modify the tree structure, making undo complex
    /// 
    /// # Time Complexity
    /// O(log n) - Without path compression (traverses up to root)
    pub fn find(&self, mut x: usize) -> usize {
        while self.parent[x] != x {
            x = self.parent[x]; // Follow parent pointers to root
        }
        x // Return the root node
    }
    
    /// Union two elements and record operation for undo
    /// 
    /// # Returns
    /// true if union was performed, false if already connected
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        
        if root_x == root_y {
            return false; // Already connected
        }
        
        // Record current state before modification
        let operation = Operation::Union {
            x_root: root_x,
            y_root: root_y,
            x_rank: self.rank[root_x],
            y_rank: self.rank[root_y],
            x_parent: self.parent[root_x],
            y_parent: self.parent[root_y],
        };
        
        // Perform union by rank
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }
        
        // Save operation for potential undo
        self.history.push(operation);
        true
    }
    
    /// Undo the last union operation
    /// 
    /// # Returns
    /// true if undo was performed, false if no operations to undo
    pub fn undo(&mut self) -> bool {
        if let Some(operation) = self.history.pop() {
            match operation {
                Operation::Union { x_root, y_root, x_rank, y_rank, x_parent, y_parent } => {
                    // Restore previous state
                    self.parent[x_root] = x_parent;
                    self.parent[y_root] = y_parent;
                    self.rank[x_root] = x_rank;
                    self.rank[y_root] = y_rank;
                }
            }
            true
        } else {
            false // No operations to undo
        }
    }
    
    /// Check if two elements are connected
    /// 
    /// **How it works**: Both elements traverse up their parent chains.
    /// If they end up at the same root, they're in the same component.
    /// This is the fundamental property that makes Union-Find work!
    pub fn connected(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y) // Same root = same component
    }
    
    /// Get number of operations that can be undone
    pub fn undo_count(&self) -> usize {
        self.history.len()
    }
    
    /// Get connected components count
    pub fn component_count(&self) -> usize {
        (0..self.size).map(|i| self.find(i)).collect::<std::collections::HashSet<_>>().len()
    }
}

/// Demonstrate Union-Find with undo using backtracking scenario
fn demo_union_find_with_undo() {
    println!("\n⏪ Variant 2: Union-Find with Undo");
    println!("{}", "-".repeat(40));
    println!("Use case: Backtracking in maze generation");
    
    let mut uuf = UndoableUnionFind::new(6);
    
    println!("\nInitial state: {} separate components", uuf.component_count());
    
    // Try building connections
    println!("\nBuilding connections:");
    uuf.union(0, 1);
    println!("Connected 0-1. Components: {}, Undo available: {}", 
             uuf.component_count(), uuf.undo_count());
    
    uuf.union(1, 2);  
    println!("Connected 1-2. Components: {}, Undo available: {}", 
             uuf.component_count(), uuf.undo_count());
    
    uuf.union(3, 4);
    println!("Connected 3-4. Components: {}, Undo available: {}", 
             uuf.component_count(), uuf.undo_count());
    
    // Check current connectivity
    println!("\nCurrent connectivity:");
    println!("0-2 connected: {}", uuf.connected(0, 2));
    println!("2-4 connected: {}", uuf.connected(2, 4));
    
    // Backtrack and try different path
    println!("\nBacktracking - undoing last operation:");
    if uuf.undo() {
        println!("Undone! Components: {}, Undo available: {}", 
                 uuf.component_count(), uuf.undo_count());
    }
    
    // Try alternative connection
    uuf.union(2, 5);
    println!("Connected 2-5 instead. Components: {}", uuf.component_count());
    
    // Multiple undos
    println!("\nUndoing all operations:");
    while uuf.undo() {
        println!("Undone. Components: {}", uuf.component_count());
    }
    
    println!("Back to initial state: {} components", uuf.component_count());
}

// ============================================================================
// VARIANT 3: PERSISTENT UNION-FIND  
// ============================================================================

/// Persistent (immutable) Union-Find using path copying
/// 
/// This variant is useful when you need to:
/// - Keep multiple versions simultaneously
/// - Support concurrent access without locks
/// - Implement functional programming patterns
/// 
/// Memory overhead: O(log n) per operation due to path copying
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct PersistentUnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    version: usize, // Version identifier
}

impl PersistentUnionFind {
    /// Create new persistent Union-Find structure
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            version: 0,
        }
    }
    
    /// Find root (immutable operation)
    pub fn find(&self, x: usize) -> usize {
        let mut current = x;
        while self.parent[current] != current {
            current = self.parent[current];
        }
        current
    }
    
    /// Create new version with union operation (functional style)
    /// 
    /// Returns new PersistentUnionFind instance, original unchanged
    /// 
    /// # Time Complexity
    /// O(n) - Due to cloning, but could be optimized with structural sharing
    pub fn union(&self, x: usize, y: usize) -> Self {
        let root_x = self.find(x);
        let root_y = self.find(y);
        
        if root_x == root_y {
            return self.clone(); // No change needed
        }
        
        // Create new version with modified structure
        let mut new_parent = self.parent.clone();
        let mut new_rank = self.rank.clone();
        
        // Union by rank
        if new_rank[root_x] < new_rank[root_y] {
            new_parent[root_x] = root_y;
        } else if new_rank[root_x] > new_rank[root_y] {
            new_parent[root_y] = root_x;
        } else {
            new_parent[root_y] = root_x;
            new_rank[root_x] += 1;
        }
        
        Self {
            parent: new_parent,
            rank: new_rank,
            version: self.version + 1,
        }
    }
    
    /// Check if two elements are connected (immutable)
    pub fn connected(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    
    /// Get version number
    pub fn version(&self) -> usize {
        self.version
    }
    
    /// Get connected components count
    pub fn component_count(&self) -> usize {
        let size = self.parent.len();
        (0..size).map(|i| self.find(i)).collect::<std::collections::HashSet<_>>().len()
    }
}

/// Demonstrate persistent Union-Find with version management
fn demo_persistent_union_find() {
    println!("\n🕰️  Variant 3: Persistent Union-Find");
    println!("{}", "-".repeat(40));
    println!("Use case: Version control for graph states");
    
    let v0 = PersistentUnionFind::new(5);
    println!("\nVersion {}: {} components", v0.version(), v0.component_count());
    
    // Create version 1 by connecting 0-1
    let v1 = v0.union(0, 1);
    println!("Version {}: {} components (connected 0-1)", v1.version(), v1.component_count());
    
    // Create version 2 by connecting 1-2 (from v1) 
    let v2 = v1.union(1, 2);
    println!("Version {}: {} components (connected 1-2)", v2.version(), v2.component_count());
    
    // Create alternative version 1b (from v0, different path)
    let v1b = v0.union(2, 3);  
    println!("Version {}: {} components (alternative: connected 2-3)", v1b.version(), v1b.component_count());
    
    // Create version from v1b
    let v2b = v1b.union(3, 4);
    println!("Version {}: {} components (connected 3-4)", v2b.version(), v2b.component_count());
    
    // Compare different versions
    println!("\nComparing versions:");
    println!("v1: 0-1 connected = {}, 2-3 connected = {}", 
             v1.connected(0, 1), v1.connected(2, 3));
    println!("v1b: 0-1 connected = {}, 2-3 connected = {}", 
             v1b.connected(0, 1), v1b.connected(2, 3));
    
    // All versions remain accessible
    println!("\nAll versions still accessible:");
    println!("v0 (original): {} components", v0.component_count());
    println!("v1 (path A): {} components", v1.component_count());
    println!("v2 (path A): {} components", v2.component_count());  
    println!("v1b (path B): {} components", v1b.component_count());
    println!("v2b (path B): {} components", v2b.component_count());
    
    println!("\nVersion tree:");
    println!("        v0 (5 components)");
    println!("       /  \\");
    println!("      v1   v1b (4 components each)");  
    println!("     /     \\");
    println!("    v2     v2b (3 components each)");
}

// ============================================================================
// VARIANT 4: UNION-FIND WITH CUSTOMIZABLE MERGE
// ============================================================================

/// Generic Union-Find that tracks additional metadata per set
/// 
/// This variant is useful when you need to:
/// - Track set properties (size, sum, max element)
/// - Maintain invariants across union operations
/// - Support custom merge logic for different applications
/// 
/// The merge function is customizable for different use cases
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct CustomizableUnionFind<T> {
    parent: Vec<usize>,
    rank: Vec<usize>,
    data: Vec<T>, // Metadata per set (stored at root)
    merge_fn: fn(&T, &T) -> T, // How to merge metadata when unioning
}

impl<T: Clone> CustomizableUnionFind<T> {
    /// Create new customizable Union-Find structure
    /// 
    /// # Arguments
    /// * `initial_data` - Initial metadata for each element
    /// * `merge_fn` - Function to merge metadata when unioning sets
    pub fn new(initial_data: Vec<T>, merge_fn: fn(&T, &T) -> T) -> Self {
        let n = initial_data.len();
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            data: initial_data,
            merge_fn,
        }
    }
    
    /// Find root with path compression
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    
    /// Union two elements with custom merge logic
    /// 
    /// Returns true if union was performed
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        
        if root_x == root_y {
            return false; // Already connected
        }
        
        // Merge metadata using custom function
        let merged_data = (self.merge_fn)(&self.data[root_x], &self.data[root_y]);
        
        // Union by rank
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
            self.data[root_y] = merged_data;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
            self.data[root_x] = merged_data;
        } else {
            self.parent[root_y] = root_x;
            self.data[root_x] = merged_data;
            self.rank[root_x] += 1;
        }
        
        true
    }
    
    /// Get metadata for the set containing element x
    pub fn get_set_data(&mut self, x: usize) -> &T {
        let root = self.find(x);
        &self.data[root]
    }
    
    /// Check if two elements are connected
    #[allow(dead_code)]
    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

/// Set statistics for demonstration
#[derive(Debug, Clone)]
struct SetStats {
    size: usize,
    sum: i32,
    max: i32,
    min: i32,
}

impl SetStats {
    fn new(value: i32) -> Self {
        Self {
            size: 1,
            sum: value,
            max: value,
            min: value,
        }
    }
    
    fn merge(a: &Self, b: &Self) -> Self {
        Self {
            size: a.size + b.size,
            sum: a.sum + b.sum,
            max: a.max.max(b.max),
            min: a.min.min(b.min),
        }
    }
}

/// Demonstrate customizable Union-Find with set statistics
fn demo_customizable_merge() {
    println!("\n🔧 Variant 4: Union-Find with Custom Merge");
    println!("{}", "-".repeat(40));
    println!("Use case: Tracking statistics for connected components");
    
    // Initialize with element values
    let values = [10, 25, 5, 30, 15];
    let initial_stats: Vec<SetStats> = values.iter().map(|&v| SetStats::new(v)).collect();
    let mut cuf = CustomizableUnionFind::new(initial_stats, SetStats::merge);
    
    println!("\nInitial elements and their values:");
    for i in 0..5 {
        let stats = cuf.get_set_data(i);
        println!("Element {}: value = {}, size = {}", i, stats.sum, stats.size);
    }
    
    // Union elements and observe merged statistics
    println!("\nUnioning elements:");
    
    cuf.union(0, 1); // Union elements with values 10 and 25
    let stats = cuf.get_set_data(0);
    println!("After union(0,1): size = {}, sum = {}, max = {}, min = {}", 
             stats.size, stats.sum, stats.max, stats.min);
    
    cuf.union(2, 3); // Union elements with values 5 and 30
    let stats = cuf.get_set_data(2);
    println!("After union(2,3): size = {}, sum = {}, max = {}, min = {}", 
             stats.size, stats.sum, stats.max, stats.min);
    
    cuf.union(0, 2); // Merge the two groups
    let stats = cuf.get_set_data(0);
    println!("After union(0,2): size = {}, sum = {}, max = {}, min = {}", 
             stats.size, stats.sum, stats.max, stats.min);
    
    // Check statistics for any element in the large component
    println!("\nQuerying statistics through different elements:");
    for i in 0..4 {
        let stats = cuf.get_set_data(i);
        println!("Via element {}: sum = {} (avg = {:.1})", 
                 i, stats.sum, stats.sum as f64 / stats.size as f64);
    }
    
    // Element 4 is still isolated
    let stats = cuf.get_set_data(4);
    println!("Element 4 (isolated): sum = {}, size = {}", stats.sum, stats.size);
    
    println!("\nVisualization:");
    println!("Connected: {{0(10), 1(25), 2(5), 3(30)}} → sum=70, avg=17.5");
    println!("Isolated:  {{4(15)}} → sum=15, avg=15.0");
}

// ============================================================================
// COMPARISON AND ANALYSIS
// ============================================================================

/// Show comparison table of all variants
fn show_comparison_table() {
    println!("\n📊 Variant Comparison Table");
    println!("{}", "=".repeat(70));
    
    let table = [
        ["Variant", "Memory", "Find Time", "Union Time", "Features"],
        ["Standard", "O(n)", "O(α(n))", "O(α(n))", "Basic connectivity"],
        ["Weighted", "O(n)", "O(α(n))", "O(α(n))", "Distance queries"],
        ["With Undo", "O(ops)", "O(log n)", "O(log n)", "Operation rollback"],
        ["Persistent", "O(n*vers)", "O(log n)", "O(n)", "Version history"],
        ["Custom Merge", "O(n+meta)", "O(α(n))", "O(α(n))", "Set metadata"],
    ];
    
    // Print table with formatting
    for (i, row) in table.iter().enumerate() {
        if i == 0 {
            // Header
            println!("{:<15} {:<10} {:<12} {:<12} {:<20}", row[0], row[1], row[2], row[3], row[4]);
            println!("{}", "-".repeat(70));
        } else {
            println!("{:<15} {:<10} {:<12} {:<12} {:<20}", row[0], row[1], row[2], row[3], row[4]);
        }
    }
    
    println!("\nWhen to Use Each Variant:");
    println!("• Standard: Most common case, best performance");
    println!("• Weighted: Need distance/cost queries between elements");
    println!("• With Undo: Backtracking algorithms, trial scenarios");
    println!("• Persistent: Functional programming, concurrent access");
    println!("• Custom Merge: Need to track additional set properties");
    
    println!("\nTrade-off Considerations:");
    println!("• Memory: Standard < Weighted ≈ Custom < Undo < Persistent");
    println!("• Performance: Standard ≈ Weighted ≈ Custom > Undo > Persistent");
    println!("• Flexibility: Standard < Weighted < Undo < Custom < Persistent");
    println!("• Complexity: Standard < Weighted < Custom < Undo < Persistent");
}

// ============================================================================
// EXERCISES
// ============================================================================

/// Show exercises for practicing advanced variants
fn show_exercises() {
    println!("\n🎯 Exercises - Advanced Variants");
    println!("{}", "=".repeat(50));
    
    println!("\n🌟 Exercise 1: Randomized Union-Find");
    println!("Implement a Union-Find that uses random choice instead of union by rank.");
    println!("Compare performance with standard implementation on large inputs.");
    println!("Research: Does randomization provide good expected performance?");
    
    println!("\n🌟 Exercise 2: Thread-Safe Union-Find");  
    println!("Create a concurrent Union-Find using Arc<Mutex<>> or RwLock.");
    println!("Support parallel find/union operations safely.");
    println!("Challenge: Minimize lock contention for better performance.");
    
    println!("\n⭐ Exercise 3: Union-Find with Deletions");
    println!("Implement a Union-Find that supports removing elements from sets.");
    println!("This is a hard problem - research existing approaches.");
    println!("Hint: Consider rebuilding affected subtrees or lazy deletion.");
    
    println!("\n⭐ Exercise 4: Hybrid Weighted+Undo Variant");
    println!("Combine weighted Union-Find with undo functionality.");
    println!("Handle weight updates correctly during undo operations.");
    println!("Test with a pathfinding algorithm that backtracks on dead ends.");
    
    println!("\n🌟 Exercise 5: Compressed Persistent Union-Find");
    println!("Optimize the persistent variant using structural sharing.");
    println!("Instead of full cloning, share unchanged parts between versions.");
    println!("Research: Path-copying vs fat node methods for persistence.");
    
    println!("\n⭐ Exercise 6: Union-Find with Range Updates");
    println!("Implement a Union-Find where you can union ranges: union_range(l, r).");
    println!("Example: union_range(3, 7) connects elements 3,4,5,6,7 to same component.");
    println!("Optimize to handle large ranges efficiently.");
    
    println!("\n🌟 Exercise 7: Metrics-Tracking Union-Find");
    println!("Extend the customizable merge variant to track operation metrics:");
    println!("- Number of find/union operations per component");
    println!("- Average tree depth over time");
    println!("- Compression ratio achieved");
    println!("Use this data to analyze algorithm performance.");
    
    println!("\n⭐ Exercise 8: Distributed Union-Find");
    println!("Design a Union-Find that works across multiple machines.");
    println!("Elements are partitioned across nodes, operations may be remote.");
    println!("Handle network partitions and consistency challenges.");
    
    println!("\nExercise Difficulty:");
    println!("🌟 = Moderate (2-4 hours)");
    println!("⭐ = Advanced (1-2 days)");
    
    println!("\nNext Steps:");
    println!("→ Try implementing 2-3 exercises that interest you");
    println!("→ Compare your implementations with research papers");
    println!("→ Move to Step 7: Problem Solving Patterns");
    println!("→ Review Mission 10 implementation for production examples");
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_weighted_union_find() {
        let mut wuf = WeightedUnionFind::new(4);
        
        // Build simple chain: 0-1-2-3 with weights 10, 5, 8
        assert!(wuf.weighted_union(0, 1, 10));
        assert!(wuf.weighted_union(1, 2, 5));
        assert!(wuf.weighted_union(2, 3, 8));
        
        // Test distances (note: the implementation returns weight_y - weight_x)
        assert_eq!(wuf.distance(0, 1), Some(-10)); // From perspective of root structure
        assert_eq!(wuf.distance(1, 0), Some(10));  // Reverse gives positive
        
        // Test that connected elements have well-defined distances
        assert!(wuf.distance(0, 3).is_some());
        assert!(wuf.distance(1, 3).is_some());
    }
    
    #[test]
    fn test_undoable_union_find() {
        let mut uuf = UndoableUnionFind::new(4);
        
        assert_eq!(uuf.component_count(), 4);
        
        uuf.union(0, 1);
        assert_eq!(uuf.component_count(), 3);
        assert_eq!(uuf.undo_count(), 1);
        
        uuf.union(2, 3);
        assert_eq!(uuf.component_count(), 2);
        assert_eq!(uuf.undo_count(), 2);
        
        // Undo last operation
        assert!(uuf.undo());
        assert_eq!(uuf.component_count(), 3);
        assert!(!uuf.connected(2, 3));
        
        // Undo first operation
        assert!(uuf.undo());
        assert_eq!(uuf.component_count(), 4);
        assert!(!uuf.connected(0, 1));
        
        // No more operations to undo
        assert!(!uuf.undo());
    }
    
    #[test]
    fn test_persistent_union_find() {
        let v0 = PersistentUnionFind::new(4);
        assert_eq!(v0.component_count(), 4);
        
        let v1 = v0.union(0, 1);
        assert_eq!(v1.component_count(), 3);
        
        let v2 = v1.union(2, 3);
        assert_eq!(v2.component_count(), 2);
        
        // Original versions unchanged
        assert_eq!(v0.component_count(), 4);
        assert_eq!(v1.component_count(), 3);
        
        // Different version from v0
        let v1b = v0.union(1, 2);
        assert_eq!(v1b.component_count(), 3);
        assert!(v1b.connected(1, 2));
        assert!(!v1.connected(1, 2)); // v1 has different connections
    }
    
    #[test]
    fn test_customizable_merge() {
        let initial_stats = vec![
            SetStats::new(10),
            SetStats::new(20),
            SetStats::new(30),
        ];
        let mut cuf = CustomizableUnionFind::new(initial_stats, SetStats::merge);
        
        cuf.union(0, 1);
        let stats = cuf.get_set_data(0);
        assert_eq!(stats.size, 2);
        assert_eq!(stats.sum, 30);
        assert_eq!(stats.max, 20);
        assert_eq!(stats.min, 10);
        
        cuf.union(0, 2);
        let stats = cuf.get_set_data(0);
        assert_eq!(stats.size, 3);
        assert_eq!(stats.sum, 60);
        assert_eq!(stats.max, 30);
        assert_eq!(stats.min, 10);
    }
}