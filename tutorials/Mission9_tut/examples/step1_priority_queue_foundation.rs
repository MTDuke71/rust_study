//! # Step 1: Priority Queue Foundation
//!
//! **Time Estimate**: 25 minutes
//! **Requirements Addressed**: REQ-1 (Priority Queue Implementation)
//!
//! ## Learning Objectives
//! - Understand priority queue data structure and operations
//! - Implement custom priority comparison for pathfinding
//! - Practice with BinaryHeap and priority ordering
//! - Build foundation for Dijkstra and A* algorithms
//!
//! ## Key Concepts
//! - **Min-Heap**: Binary heap where parent ≤ children (smallest at root)
//! - **Priority Ordering**: Custom comparison for pathfinding costs
//! - **O(log n) Operations**: Efficient insert and extract-min
//! - **Generic Implementation**: Type-safe priority queue for any node/cost type
//!
//! ## Theory Background
//!
//! Priority queues are essential for pathfinding algorithms because they allow us to:
//! 1. **Always process the lowest-cost node next** (Dijkstra's guarantee)
//! 2. **Efficiently manage the frontier** of nodes to explore
//! 3. **Maintain algorithm optimality** through systematic exploration
//!
//! Rust's `BinaryHeap` is a max-heap, so we need to reverse ordering for min-heap behavior.

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::hash::Hash;

/// Ordered wrapper for floating-point costs
/// Required because f64 doesn't implement Ord due to NaN values
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrderedFloat(pub f64);

impl OrderedFloat {
    pub fn new(value: f64) -> Self {
        assert!(!value.is_nan(), "Cannot create OrderedFloat from NaN");
        Self(value)
    }

    pub fn get(self) -> f64 {
        self.0
    }
}

impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl Eq for OrderedFloat {}

impl From<f64> for OrderedFloat {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl std::fmt::Display for OrderedFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}

/// Priority item that wraps a value with its priority/cost
/// Implements custom ordering to convert BinaryHeap (max-heap) to min-heap behavior
#[derive(Debug, Clone, PartialEq)]
pub struct PriorityItem<T, P> {
    pub item: T,
    pub priority: P,
}

impl<T, P> PriorityItem<T, P> {
    pub fn new(item: T, priority: P) -> Self {
        Self { item, priority }
    }
}

// Implement ordering based on priority (reversed for min-heap)
impl<T: PartialEq, P: PartialOrd> PartialOrd for PriorityItem<T, P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Reverse ordering: smaller priority = higher precedence
        other.priority.partial_cmp(&self.priority)
    }
}

impl<T: PartialEq, P: Ord> Ord for PriorityItem<T, P> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering: smaller priority = higher precedence
        other.priority.cmp(&self.priority)
    }
}

impl<T: PartialEq, P: PartialEq> Eq for PriorityItem<T, P> {}

/// Pathfinding-optimized priority queue
/// Wrapper around BinaryHeap with pathfinding-specific operations
#[derive(Debug, Clone)]
pub struct PathfindingQueue<NodeId, Cost> {
    heap: BinaryHeap<PriorityItem<NodeId, Cost>>,
}

impl<NodeId, Cost> PathfindingQueue<NodeId, Cost>
where
    NodeId: Copy + Eq + Hash,
    Cost: Copy + PartialOrd + Ord,
{
    /// Create new empty priority queue
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    /// Create with initial capacity hint
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: BinaryHeap::with_capacity(capacity),
        }
    }

    /// Push node with priority/cost
    /// O(log n) complexity
    pub fn push(&mut self, node: NodeId, cost: Cost) {
        self.heap.push(PriorityItem::new(node, cost));
    }

    /// Pop node with lowest cost
    /// O(log n) complexity
    pub fn pop(&mut self) -> Option<(NodeId, Cost)> {
        self.heap.pop().map(|item| (item.item, item.priority))
    }

    /// Peek at lowest cost node without removing
    /// O(1) complexity
    pub fn peek(&self) -> Option<(&NodeId, &Cost)> {
        self.heap.peek().map(|item| (&item.item, &item.priority))
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Get current queue size
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Clear all items
    pub fn clear(&mut self) {
        self.heap.clear();
    }
}

impl<NodeId, Cost> Default for PathfindingQueue<NodeId, Cost>
where
    NodeId: Copy + Eq + Hash,
    Cost: Copy + PartialOrd + Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Exercise 1: Basic Priority Queue Operations
///
/// TODO: Complete the implementation and test the priority queue
fn exercise_1_basic_operations() {
    println!("🔧 Exercise 1: Basic Priority Queue Operations");

    let mut queue: PathfindingQueue<u32, OrderedFloat> = PathfindingQueue::new();

    // TODO: Add these nodes with their costs
    // Node 0: cost 5.0
    // Node 1: cost 2.0
    // Node 2: cost 8.0
    // Node 3: cost 1.0

    // YOUR CODE HERE:
    queue.push(0, OrderedFloat::new(5.0));
    queue.push(1, OrderedFloat::new(2.0));
    queue.push(2, OrderedFloat::new(8.0));
    queue.push(3, OrderedFloat::new(1.0));

    println!("Queue size after insertions: {}", queue.len());

    // TODO: Pop all nodes and verify they come out in cost order (lowest first)
    // Expected order: Node 3 (1.0), Node 1 (2.0), Node 0 (5.0), Node 2 (8.0)

    println!("Nodes in priority order:");
    while let Some((node, cost)) = queue.pop() {
        println!("  Node {}: cost {:.1}", node, cost);
    }

    // Verification
    assert!(queue.is_empty());
    println!("✅ Basic operations completed successfully!");
}

/// Exercise 2: Pathfinding Scenario
///
/// Simulate a pathfinding scenario where we're exploring nodes
/// and always want to process the closest (lowest cost) node next
fn exercise_2_pathfinding_simulation() {
    println!("\n🗺️ Exercise 2: Pathfinding Simulation");

    let mut frontier: PathfindingQueue<char, i32> = PathfindingQueue::new();

    // Starting at node 'A' with cost 0
    frontier.push('A', 0);

    // Simulate discovering neighbors with cumulative costs
    let discoveries = vec![
        ('B', 4), // A -> B costs 4
        ('C', 2), // A -> C costs 2
        ('D', 7), // A -> D costs 7
        ('E', 1), // A -> E costs 1
    ];

    println!("Discovering neighbors from A:");
    for (node, cost) in discoveries {
        frontier.push(node, cost);
        println!("  Found path to {}: total cost {}", node, cost);
    }

    // Process nodes in cost order (pathfinding exploration order)
    println!("\nProcessing nodes in exploration order:");
    while let Some((node, cost)) = frontier.pop() {
        println!("  Exploring node {}: cost {}", node, cost);

        // TODO: In real pathfinding, we would:
        // 1. Check if this is the goal
        // 2. Explore neighbors if not visited
        // 3. Add neighbors to frontier with updated costs
    }

    println!("✅ Pathfinding simulation completed!");
}

/// Exercise 3: Custom Node Types
///
/// Demonstrate priority queue with custom node types (coordinates)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn manhattan_distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn exercise_3_custom_node_types() {
    println!("\n📍 Exercise 3: Custom Node Types (Coordinates)");

    let mut queue: PathfindingQueue<Coordinate, OrderedFloat> = PathfindingQueue::new();

    let _start = Coordinate::new(0, 0);
    let goal = Coordinate::new(5, 5);

    // Add some coordinates with distances as priorities
    let coordinates = vec![
        Coordinate::new(1, 1),
        Coordinate::new(2, 0),
        Coordinate::new(0, 3),
        Coordinate::new(4, 1),
    ];

    println!("Adding coordinates with Manhattan distance to goal as priority:");
    for coord in coordinates {
        let distance = OrderedFloat::new(coord.manhattan_distance(&goal) as f64);
        queue.push(coord, distance);
        println!("  {:?}: distance {}", coord, distance);
    }

    println!("\nProcessing coordinates by distance to goal:");
    while let Some((coord, distance)) = queue.pop() {
        println!("  {:?}: distance {}", coord, distance);
    }

    println!("✅ Custom node types working correctly!");
}

/// Advanced Exercise: Priority Queue Performance Analysis
fn advanced_exercise_performance_analysis() {
    use std::time::Instant;

    println!("\n⚡ Advanced Exercise: Performance Analysis");

    let sizes = vec![100, 1000, 10000];

    for size in sizes {
        let mut queue: PathfindingQueue<usize, OrderedFloat> =
            PathfindingQueue::with_capacity(size);

        // Time insertions
        let start = Instant::now();
        for i in 0..size {
            // Insert in reverse order to test heap performance
            queue.push(i, OrderedFloat::new((size - i) as f64));
        }
        let insert_time = start.elapsed();

        // Time extractions
        let start = Instant::now();
        let mut extracted = 0;
        while !queue.is_empty() {
            if let Some((node, cost)) = queue.pop() {
                extracted += 1;
                // Verify ordering (first should be node size-1 with cost 1.0)
                if extracted == 1 {
                    assert_eq!(node, size - 1);
                    assert_eq!(cost, OrderedFloat::new(1.0));
                }
            }
        }
        let extract_time = start.elapsed();

        println!(
            "Size {}: Insert {:?}, Extract {:?}",
            size, insert_time, extract_time
        );
    }

    println!("✅ Performance analysis complete!");
}

// Test module for step validation
#[cfg(test)]
mod step1_tests {
    use super::*;

    #[test]
    fn test_priority_queue_ordering() {
        let mut queue: PathfindingQueue<i32, OrderedFloat> = PathfindingQueue::new();

        // Insert in random order
        queue.push(1, OrderedFloat::new(5.0));
        queue.push(2, OrderedFloat::new(1.0));
        queue.push(3, OrderedFloat::new(3.0));
        queue.push(4, OrderedFloat::new(2.0));

        // Should come out in cost order
        assert_eq!(queue.pop(), Some((2, OrderedFloat::new(1.0))));
        assert_eq!(queue.pop(), Some((4, OrderedFloat::new(2.0))));
        assert_eq!(queue.pop(), Some((3, OrderedFloat::new(3.0))));
        assert_eq!(queue.pop(), Some((1, OrderedFloat::new(5.0))));
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn test_queue_operations() {
        let mut queue: PathfindingQueue<char, i32> = PathfindingQueue::new();

        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);

        queue.push('A', 10);
        queue.push('B', 5);

        assert!(!queue.is_empty());
        assert_eq!(queue.len(), 2);

        assert_eq!(queue.peek(), Some((&'B', &5)));
        assert_eq!(queue.len(), 2); // Peek doesn't remove

        assert_eq!(queue.pop(), Some(('B', 5)));
        assert_eq!(queue.len(), 1);

        queue.clear();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_coordinate_priority_queue() {
        let mut queue: PathfindingQueue<Coordinate, i32> = PathfindingQueue::new();

        let coords = vec![
            (Coordinate::new(0, 0), 10),
            (Coordinate::new(1, 1), 5),
            (Coordinate::new(2, 2), 15),
        ];

        for (coord, priority) in coords {
            queue.push(coord, priority);
        }

        // Should come out in priority order
        assert_eq!(queue.pop().unwrap().1, 5); // Lowest priority first
        assert_eq!(queue.pop().unwrap().1, 10);
        assert_eq!(queue.pop().unwrap().1, 15);
    }
}

/// Main function to run all exercises
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Mission 9 Tutorial - Step 1: Priority Queue Foundation");
    println!("═══════════════════════════════════════════════════════════");

    exercise_1_basic_operations();
    exercise_2_pathfinding_simulation();
    exercise_3_custom_node_types();
    advanced_exercise_performance_analysis();

    // Show solutions if feature is enabled
    #[cfg(feature = "solutions")]
    {
        println!("\n🔍 Solutions Available:");
        solutions::exercise_1_solution();
        println!();
        solutions::pathfinding_explanation();
    }

    println!("\n🎉 Step 1 Complete!");
    println!("Next: cargo run --example step2_dijkstra_basics");

    Ok(())
}

// Solution section (feature-gated)
#[cfg(feature = "solutions")]
mod solutions {
    use super::*;

    /// Complete solution for Exercise 1
    pub fn exercise_1_solution() {
        println!("💡 Exercise 1 Solution:");
        println!("The key insight is that Rust's BinaryHeap is a max-heap,");
        println!("but pathfinding needs a min-heap (process lowest cost first).");
        println!("We reverse the ordering in our PriorityItem implementation.");

        let mut queue: PathfindingQueue<u32, OrderedFloat> = PathfindingQueue::new();

        // Add nodes with costs
        queue.push(0, OrderedFloat::new(5.0));
        queue.push(1, OrderedFloat::new(2.0));
        queue.push(2, OrderedFloat::new(8.0));
        queue.push(3, OrderedFloat::new(1.0));

        // Pop in min-cost order
        while let Some((node, cost)) = queue.pop() {
            println!("  Process node {} with cost {}", node, cost);
        }
    }

    /// Explanation of priority queue in pathfinding context
    pub fn pathfinding_explanation() {
        println!("📚 Why Priority Queues in Pathfinding?");
        println!("1. Dijkstra's algorithm requires processing nodes in cost order");
        println!("2. A* needs to process nodes by f(n) = g(n) + h(n) order");
        println!("3. Priority queue ensures optimal exploration order");
        println!("4. O(log n) operations maintain efficiency for large graphs");
    }
}
