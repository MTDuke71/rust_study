# Day 26 · Advanced Queues (Priority Queues & VecDeque Patterns)

> **Learning Context**: Day 26 explores advanced queue patterns including priority queues for weighted graphs (Dijkstra's algorithm), VecDeque's powerful double-ended operations, and sophisticated state management - essential for Mission 6's advanced pathfinding and complex AoC challenges.

**Cross-Track Integration:**
- **Mission Focus**: Priority queues enable Mission 6's Dijkstra implementation; VecDeque patterns power Mission 2's ring buffer
- **Daily Study**: Week 4, Day 5 - Advanced data structures building on queue foundations
- **Rust Book**: Chapter 8 (Collections - BinaryHeap, VecDeque) and Chapter 10 (Traits with Ord)

**Related Zettelkasten Notes:**
- [[../missions/Mission2/README|Mission2 - Ring Buffer Queue]]
- [[../missions/Mission6/README|Mission6 - Dijkstra's Algorithm]]
- [[Priority Queue Patterns]] - Heap-based algorithms
- [[zettel-index]] - Main learning hub

---

## 🎯 Core Concepts

### What is a Priority Queue?

A **priority queue** is a data structure where elements are served based on priority, not arrival order. Think of it as a queue where VIP members jump ahead.

**Key Characteristics:**
- **Not FIFO**: Highest priority element is served first
- **Dynamic ordering**: Priority determines position
- **Efficient operations**: Insert and remove-max in O(log n)
- **Heap-based**: Typically implemented with binary heaps

**Real-World Examples:**
- **Emergency Rooms**: Critical patients treated first
- **Task Schedulers**: High-priority jobs run before low-priority
- **Dijkstra's Algorithm**: Always process closest unvisited node
- **A* Pathfinding**: Explore most promising paths first
- **Event Simulation**: Process events in time order

### Rust's BinaryHeap

Rust provides `BinaryHeap<T>` in the standard library - a **max-heap** by default:

```rust
use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();
    
    heap.push(3);
    heap.push(1);
    heap.push(4);
    heap.push(1);
    heap.push(5);
    
    // Pops in descending order: 5, 4, 3, 1, 1
    while let Some(value) = heap.pop() {
        println!("{}", value); // 5, then 4, then 3, then 1, then 1
    }
}
```

**Why Max-Heap by Default?**
- Most common use case in algorithms
- Easy to convert to min-heap (negate values or use `Reverse`)
- Consistent with mathematical heap definition

---

## 🔢 Creating Min-Heaps

### Three Ways to Get Min-Heap Behavior

#### Method 1: Using `Reverse` Wrapper

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let mut min_heap = BinaryHeap::new();
    
    // Wrap values in Reverse to invert ordering
    min_heap.push(Reverse(3));
    min_heap.push(Reverse(1));
    min_heap.push(Reverse(4));
    
    // Pops in ascending order
    while let Some(Reverse(value)) = min_heap.pop() {
        println!("{}", value); // 1, then 3, then 4
    }
}
```

**How `Reverse` Works:**
```rust
// Reverse<T> inverts the Ord implementation
#[derive(PartialEq, Eq)]
struct Reverse<T>(T);

impl<T: Ord> Ord for Reverse<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Flip the comparison!
        other.0.cmp(&self.0)
    }
}
```

#### Method 2: Custom Type with Inverted Ord

```rust
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
struct MinValue(i32);

impl Ord for MinValue {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the natural ordering
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for MinValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Usage
let mut heap = BinaryHeap::new();
heap.push(MinValue(5));
heap.push(MinValue(1));
// Will pop 1 before 5
```

#### Method 3: Negating Integer Values

```rust
// For integers only - simple but limited
let mut min_heap = BinaryHeap::new();

min_heap.push(-3);
min_heap.push(-1);
min_heap.push(-4);

// Negate when retrieving
while let Some(value) = min_heap.pop() {
    println!("{}", -value); // 1, then 3, then 4
}
```

**Comparison:**

| Method | Pros | Cons |
|--------|------|------|
| `Reverse<T>` | ✅ Works with any Ord type<br>✅ Standard library | ❌ Verbose unwrapping |
| Custom Ord | ✅ Clean API for your type | ❌ Must implement 4 traits |
| Negation | ✅ Simple for integers | ❌ Only works with signed numbers<br>❌ Risk of overflow |

---

## 🗺️ Dijkstra's Algorithm with Priority Queue

### Why Priority Queue for Dijkstra?

**Dijkstra's algorithm** finds shortest paths in weighted graphs. It requires always processing the **closest unvisited node** - perfect for a min-heap!

**Algorithm Overview:**
1. Start with source at distance 0
2. Always process the node with minimum distance
3. Update neighbors' distances if we found a shorter path
4. Repeat until all reachable nodes processed

### Complete Dijkstra Implementation

```rust
use std::collections::{BinaryHeap, HashMap};
use std::cmp::{Ordering, Reverse};

/// Represents a node with its distance from source
/// 
/// The Ord implementation ensures the heap prioritizes nodes
/// with smaller distances (min-heap behavior via Reverse).
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Coord,
}

// The priority queue depends on `Ord`.
// We want min-heap, so we use Reverse ordering on cost.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reversed comparison for min-heap
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Dijkstra's shortest path algorithm for weighted grids
/// 
/// Finds shortest path from start to goal considering edge weights.
/// Unlike BFS (which assumes all edges cost 1), Dijkstra handles
/// arbitrary positive edge costs.
/// 
/// # Time Complexity
/// O((V + E) log V) where V = vertices, E = edges
/// 
/// # Space Complexity
/// O(V) for distances map and priority queue
/// 
/// # Returns
/// Some((distance, path)) if goal is reachable, None otherwise
pub fn dijkstra(
    grid: &Grid<usize>,  // Grid where cells contain movement costs
    start: Coord,
    goal: Coord,
) -> Option<(usize, Vec<Coord>)> {
    let mut dist: HashMap<Coord, usize> = HashMap::new();
    let mut parent: HashMap<Coord, Coord> = HashMap::new();
    let mut heap = BinaryHeap::new();
    
    // Start at distance 0
    dist.insert(start, 0);
    parent.insert(start, start);
    heap.push(State { cost: 0, position: start });
    
    while let Some(State { cost, position }) = heap.pop() {
        // Found the goal!
        if position == goal {
            let path = reconstruct_path(&parent, start, goal);
            return Some((cost, path));
        }
        
        // Skip if we've already found a better path
        if let Some(&best) = dist.get(&position) {
            if cost > best {
                continue;
            }
        }
        
        // Check all neighbors
        for neighbor in grid.neighbors_4(position) {
            // Get the cost to move to this neighbor
            let move_cost = grid.get_coord(neighbor).copied().unwrap_or(usize::MAX);
            if move_cost == usize::MAX {
                continue; // Impassable
            }
            
            let next_cost = cost + move_cost;
            
            // If we found a shorter path, update it
            let is_shorter = dist.get(&neighbor)
                .map_or(true, |&current| next_cost < current);
            
            if is_shorter {
                dist.insert(neighbor, next_cost);
                parent.insert(neighbor, position);
                heap.push(State {
                    cost: next_cost,
                    position: neighbor,
                });
            }
        }
    }
    
    None // Goal not reachable
}

fn reconstruct_path(
    parent: &HashMap<Coord, Coord>,
    start: Coord,
    goal: Coord,
) -> Vec<Coord> {
    let mut path = Vec::new();
    let mut current = goal;
    
    while current != start {
        path.push(current);
        current = parent[&current];
    }
    path.push(start);
    path.reverse();
    path
}
```

**Dijkstra vs BFS Comparison:**

| Aspect | BFS | Dijkstra |
|--------|-----|----------|
| **Graph Type** | Unweighted | Weighted |
| **Data Structure** | Queue (VecDeque) | Priority Queue (BinaryHeap) |
| **Correctness** | ✅ Optimal for unweighted | ✅ Optimal for weighted |
| **Time Complexity** | O(V + E) | O((V + E) log V) |
| **Space Complexity** | O(V) | O(V) |
| **Use Case** | All edges cost 1 | Edges have different costs |

### Dijkstra Visualization

**Example 1: When straight path is still optimal**
```
Grid with costs:           Dijkstra result:
S 1 1 1 G                  S * * * G
1 9 9 1 1                  1 9 9 1 1
1 1 1 1 1                  1 1 1 1 1

Without weights (BFS): 4 steps
With weights (Dijkstra): Cost of 3 (1+1+1, straight path is optimal)
```

**Example 2: When going around is better**
```
Grid with costs:           Dijkstra result:
S 9 9 9 G                  S 9 9 9 G
1 1 1 1 1                  * * * * *
1 1 1 1 1                  1 1 1 1 1

Without weights (BFS): 4 steps
With weights (Dijkstra): Cost of 5 (going around: 1+1+1+1+1 vs straight: 9+9+9=27)
```

---

## 🔄 VecDeque Advanced Patterns

### Why VecDeque is Powerful

`VecDeque<T>` is a **double-ended queue** (deque, pronounced "deck") that allows efficient operations at both ends.

**Operations and Their Costs:**

| Operation | Time Complexity | Use Case |
|-----------|----------------|----------|
| `push_back(x)` | O(1) amortized | Enqueue (standard queue) |
| `pop_front()` | O(1) | Dequeue (standard queue) |
| `push_front(x)` | O(1) amortized | Add to front (deque) |
| `pop_back()` | O(1) | Remove from back (stack-like) |
| `front()`, `back()` | O(1) | Peek at ends |
| `[index]` | O(1) | Random access |
| `insert(i, x)` | O(n) | Insertion anywhere |
| `remove(i)` | O(n) | Removal anywhere |

### Pattern 1: Sliding Window

```rust
use std::collections::VecDeque;

/// Maintains a sliding window of maximum size k
/// 
/// Useful for problems like "maximum in sliding window" where you
/// need to efficiently track the most recent k elements.
pub struct SlidingWindow<T> {
    deque: VecDeque<T>,
    max_size: usize,
}

impl<T> SlidingWindow<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            deque: VecDeque::with_capacity(max_size),
            max_size,
        }
    }
    
    /// Add element, automatically removing oldest if at capacity
    pub fn push(&mut self, value: T) {
        if self.deque.len() == self.max_size {
            self.deque.pop_front(); // Remove oldest
        }
        self.deque.push_back(value); // Add newest
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.deque.iter()
    }
    
    pub fn len(&self) -> usize {
        self.deque.len()
    }
}

// Example: Moving average
fn moving_average(data: &[f64], window_size: usize) -> Vec<f64> {
    let mut window = SlidingWindow::new(window_size);
    let mut averages = Vec::new();
    
    for &value in data {
        window.push(value);
        
        if window.len() == window_size {
            let sum: f64 = window.iter().sum();
            averages.push(sum / window_size as f64);
        }
    }
    
    averages
}
```

### Pattern 2: Monotonic Queue

```rust
/// Maintains a monotonic (always increasing or decreasing) queue
/// 
/// Useful for problems like "sliding window maximum" where you need
/// to efficiently track the maximum value in a sliding window.
pub struct MonotonicQueue {
    deque: VecDeque<(usize, i32)>, // (index, value)
}

impl MonotonicQueue {
    pub fn new() -> Self {
        Self {
            deque: VecDeque::new(),
        }
    }
    
    /// Add element, removing smaller elements from back
    /// Maintains decreasing order (front = largest)
    pub fn push(&mut self, index: usize, value: i32) {
        // Remove all smaller values from back
        while let Some(&(_, back_val)) = self.deque.back() {
            if back_val < value {
                self.deque.pop_back();
            } else {
                break;
            }
        }
        self.deque.push_back((index, value));
    }
    
    /// Remove elements outside the window
    pub fn pop_outside_window(&mut self, left_bound: usize) {
        while let Some(&(idx, _)) = self.deque.front() {
            if idx < left_bound {
                self.deque.pop_front();
            } else {
                break;
            }
        }
    }
    
    /// Get current maximum (front element)
    pub fn max(&self) -> Option<i32> {
        self.deque.front().map(|(_, val)| *val)
    }
}

/// Sliding window maximum problem
fn max_sliding_window(nums: &[i32], k: usize) -> Vec<i32> {
    let mut mq = MonotonicQueue::new();
    let mut result = Vec::new();
    
    for i in 0..nums.len() {
        // Add current element
        mq.push(i, nums[i]);
        
        // Remove elements outside window
        if i >= k {
            mq.pop_outside_window(i - k + 1);
        }
        
        // Record maximum when window is full
        if i >= k - 1 {
            result.push(mq.max().unwrap());
        }
    }
    
    result
}
```

### Pattern 3: Work Stealing Queue

```rust
/// Double-ended work queue for task scheduling
/// 
/// Workers push/pop from their own end, but can "steal" from other end
/// when their queue is empty. Common pattern in parallel task schedulers.
pub struct WorkStealingQueue<T> {
    deque: VecDeque<T>,
}

impl<T> WorkStealingQueue<T> {
    pub fn new() -> Self {
        Self {
            deque: VecDeque::new(),
        }
    }
    
    /// Owner pushes tasks to their end
    pub fn push_task(&mut self, task: T) {
        self.deque.push_back(task);
    }
    
    /// Owner pops tasks from their end (LIFO - good for cache locality)
    pub fn pop_task(&mut self) -> Option<T> {
        self.deque.pop_back()
    }
    
    /// Other workers steal from opposite end (FIFO)
    pub fn steal_task(&mut self) -> Option<T> {
        self.deque.pop_front()
    }
    
    pub fn is_empty(&self) -> bool {
        self.deque.is_empty()
    }
}
```

### Pattern 4: Palindrome Checker

```rust
/// Check if a string is a palindrome using VecDeque
/// 
/// Demonstrates bi-directional iteration - compare characters
/// from both ends simultaneously.
pub fn is_palindrome(s: &str) -> bool {
    let mut deque: VecDeque<char> = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    
    while deque.len() > 1 {
        if deque.pop_front() != deque.pop_back() {
            return false;
        }
    }
    
    true
}
```

---

## 🚀 Complete Runnable Example

```rust
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::cmp::{Ordering, Reverse};
use std::fmt;

fn main() {
    println!("=== Day 26: Advanced Queues Demo ===\n");
    
    // 1. Priority Queue Basics
    println!("🔷 1. Priority Queue (Max-Heap)");
    println!("================================");
    
    let mut max_heap = BinaryHeap::new();
    let values = vec![3, 1, 4, 1, 5, 9, 2, 6];
    
    println!("Inserting: {:?}", values);
    for val in values {
        max_heap.push(val);
    }
    
    print!("Popping (descending): ");
    while let Some(val) = max_heap.pop() {
        print!("{} ", val);
    }
    println!("\n");
    
    // 2. Min-Heap with Reverse
    println!("🔷 2. Min-Heap using Reverse");
    println!("============================");
    
    let mut min_heap = BinaryHeap::new();
    let values = vec![3, 1, 4, 1, 5, 9, 2, 6];
    
    println!("Inserting: {:?}", values);
    for val in values {
        min_heap.push(Reverse(val));
    }
    
    print!("Popping (ascending): ");
    while let Some(Reverse(val)) = min_heap.pop() {
        print!("{} ", val);
    }
    println!("\n");
    
    // 3. Dijkstra's Algorithm
    println!("🔷 3. Dijkstra's Shortest Path");
    println!("==============================");
    
    // Grid where each cell contains movement cost
    let cost_grid = Grid::from_costs(vec![
        vec![1, 1, 1, 1, 1],
        vec![1, 9, 9, 9, 1],
        vec![1, 9, 1, 9, 1],
        vec![1, 1, 1, 9, 1],
        vec![1, 1, 1, 1, 1],
    ]);
    
    println!("Cost grid:");
    println!("{}", cost_grid);
    
    let start = Coord::new(0, 0);
    let goal = Coord::new(4, 4);
    
    if let Some((cost, path)) = dijkstra(&cost_grid, start, goal) {
        println!("Shortest path found!");
        println!("  Total cost: {}", cost);
        println!("  Path length: {} steps", path.len());
        println!("  Path: {:?}", path);
    }
    
    // 4. VecDeque: Sliding Window
    println!("\n🔷 4. Sliding Window Pattern");
    println!("============================");
    
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let window_size = 3;
    
    println!("Data: {:?}", data);
    println!("Window size: {}", window_size);
    
    let averages = moving_average(&data, window_size);
    println!("Moving averages: {:?}", averages);
    
    // 5. Monotonic Queue: Sliding Window Maximum
    println!("\n🔷 5. Sliding Window Maximum");
    println!("============================");
    
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    
    println!("Array: {:?}", nums);
    println!("Window size: {}", k);
    
    let maximums = max_sliding_window(&nums, k);
    println!("Window maximums: {:?}", maximums);
    
    // 6. Work Stealing Queue
    println!("\n🔷 6. Work Stealing Queue Pattern");
    println!("==================================");
    
    let mut worker_queue = WorkStealingQueue::new();
    
    println!("Worker adds tasks: 1, 2, 3, 4, 5");
    for task in 1..=5 {
        worker_queue.push_task(task);
    }
    
    println!("Worker pops (LIFO): {:?}", worker_queue.pop_task());
    println!("Worker pops (LIFO): {:?}", worker_queue.pop_task());
    
    println!("Thief steals (FIFO): {:?}", worker_queue.steal_task());
    println!("Thief steals (FIFO): {:?}", worker_queue.steal_task());
    
    // 7. Palindrome Checker
    println!("\n🔷 7. Palindrome Detection");
    println!("==========================");
    
    let test_strings = vec![
        "A man a plan a canal Panama",
        "race a car",
        "Was it a rat I saw",
        "hello world",
    ];
    
    for s in test_strings {
        let result = if is_palindrome(s) { "✓ Yes" } else { "✗ No" };
        println!("  '{}' → {}", s, result);
    }
    
    // 8. Priority Queue Application: Task Scheduler
    println!("\n🔷 8. Task Scheduler (Priority Queue)");
    println!("=====================================");
    
    #[derive(Eq, PartialEq)]
    struct Task {
        priority: u8,
        name: String,
    }
    
    impl Ord for Task {
        fn cmp(&self, other: &Self) -> Ordering {
            self.priority.cmp(&other.priority)
        }
    }
    
    impl PartialOrd for Task {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    
    let mut task_queue = BinaryHeap::new();
    
    task_queue.push(Task { priority: 5, name: "Critical bug fix".to_string() });
    task_queue.push(Task { priority: 2, name: "Documentation".to_string() });
    task_queue.push(Task { priority: 8, name: "Production outage".to_string() });
    task_queue.push(Task { priority: 3, name: "Code review".to_string() });
    
    println!("Processing tasks by priority:");
    while let Some(task) = task_queue.pop() {
        println!("  [Priority {}] {}", task.priority, task.name);
    }
}

// === Supporting Code ===

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coord {
    pub row: isize,
    pub col: isize,
}

impl Coord {
    pub fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
}

pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl Grid<usize> {
    pub fn from_costs(costs: Vec<Vec<usize>>) -> Self {
        let height = costs.len();
        let width = costs[0].len();
        let mut data = Vec::with_capacity(width * height);
        
        for row in costs {
            for cost in row {
                data.push(cost);
            }
        }
        
        Self { data, width, height }
    }
}

impl<T: Clone> Grid<T> {
    pub fn contains(&self, coord: Coord) -> bool {
        coord.row >= 0 && coord.row < self.height as isize &&
        coord.col >= 0 && coord.col < self.width as isize
    }
    
    pub fn get_coord(&self, coord: Coord) -> Option<&T> {
        if self.contains(coord) {
            Some(&self[(coord.row as usize, coord.col as usize)])
        } else {
            None
        }
    }
    
    pub fn neighbors_4(&self, coord: Coord) -> Vec<Coord> {
        let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        offsets.iter()
            .map(|(dr, dc)| Coord::new(coord.row + dr, coord.col + dc))
            .filter(|&pos| self.contains(pos))
            .collect()
    }
}

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * self.width + col]
    }
}

impl fmt::Display for Grid<usize> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                write!(f, "{} ", self[(row, col)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// Dijkstra implementation
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Coord,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(
    grid: &Grid<usize>,
    start: Coord,
    goal: Coord,
) -> Option<(usize, Vec<Coord>)> {
    let mut dist: HashMap<Coord, usize> = HashMap::new();
    let mut parent: HashMap<Coord, Coord> = HashMap::new();
    let mut heap = BinaryHeap::new();
    
    dist.insert(start, 0);
    parent.insert(start, start);
    heap.push(State { cost: 0, position: start });
    
    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            let path = reconstruct_path(&parent, start, goal);
            return Some((cost, path));
        }
        
        if let Some(&best) = dist.get(&position) {
            if cost > best {
                continue;
            }
        }
        
        for neighbor in grid.neighbors_4(position) {
            let move_cost = grid.get_coord(neighbor).copied().unwrap_or(usize::MAX);
            if move_cost == usize::MAX {
                continue;
            }
            
            let next_cost = cost + move_cost;
            let is_shorter = dist.get(&neighbor)
                .map_or(true, |&current| next_cost < current);
            
            if is_shorter {
                dist.insert(neighbor, next_cost);
                parent.insert(neighbor, position);
                heap.push(State { cost: next_cost, position: neighbor });
            }
        }
    }
    
    None
}

fn reconstruct_path(
    parent: &HashMap<Coord, Coord>,
    start: Coord,
    goal: Coord,
) -> Vec<Coord> {
    let mut path = Vec::new();
    let mut current = goal;
    
    while current != start {
        path.push(current);
        current = parent[&current];
    }
    path.push(start);
    path.reverse();
    path
}

// VecDeque patterns
struct SlidingWindow<T> {
    deque: VecDeque<T>,
    max_size: usize,
}

impl<T> SlidingWindow<T> {
    fn new(max_size: usize) -> Self {
        Self {
            deque: VecDeque::with_capacity(max_size),
            max_size,
        }
    }
    
    fn push(&mut self, value: T) {
        if self.deque.len() == self.max_size {
            self.deque.pop_front();
        }
        self.deque.push_back(value);
    }
    
    fn iter(&self) -> impl Iterator<Item = &T> {
        self.deque.iter()
    }
    
    fn len(&self) -> usize {
        self.deque.len()
    }
}

fn moving_average(data: &[f64], window_size: usize) -> Vec<f64> {
    let mut window = SlidingWindow::new(window_size);
    let mut averages = Vec::new();
    
    for &value in data {
        window.push(value);
        
        if window.len() == window_size {
            let sum: f64 = window.iter().sum();
            averages.push(sum / window_size as f64);
        }
    }
    
    averages
}

struct MonotonicQueue {
    deque: VecDeque<(usize, i32)>,
}

impl MonotonicQueue {
    fn new() -> Self {
        Self { deque: VecDeque::new() }
    }
    
    fn push(&mut self, index: usize, value: i32) {
        while let Some(&(_, back_val)) = self.deque.back() {
            if back_val < value {
                self.deque.pop_back();
            } else {
                break;
            }
        }
        self.deque.push_back((index, value));
    }
    
    fn pop_outside_window(&mut self, left_bound: usize) {
        while let Some(&(idx, _)) = self.deque.front() {
            if idx < left_bound {
                self.deque.pop_front();
            } else {
                break;
            }
        }
    }
    
    fn max(&self) -> Option<i32> {
        self.deque.front().map(|(_, val)| *val)
    }
}

fn max_sliding_window(nums: &[i32], k: usize) -> Vec<i32> {
    let mut mq = MonotonicQueue::new();
    let mut result = Vec::new();
    
    for i in 0..nums.len() {
        mq.push(i, nums[i]);
        
        if i >= k {
            mq.pop_outside_window(i - k + 1);
        }
        
        if i >= k - 1 {
            result.push(mq.max().unwrap());
        }
    }
    
    result
}

struct WorkStealingQueue<T> {
    deque: VecDeque<T>,
}

impl<T> WorkStealingQueue<T> {
    fn new() -> Self {
        Self { deque: VecDeque::new() }
    }
    
    fn push_task(&mut self, task: T) {
        self.deque.push_back(task);
    }
    
    fn pop_task(&mut self) -> Option<T> {
        self.deque.pop_back()
    }
    
    fn steal_task(&mut self) -> Option<T> {
        self.deque.pop_front()
    }
}

fn is_palindrome(s: &str) -> bool {
    let mut deque: VecDeque<char> = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    
    while deque.len() > 1 {
        if deque.pop_front() != deque.pop_back() {
            return false;
        }
    }
    
    true
}
```

### 🛠️ How to Run This Code:

1. **Online**: Copy to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `day26_demo.rs` and run `rustc day26_demo.rs && ./day26_demo`
3. **In this workspace**: Create a new cargo project and paste into `src/main.rs`

---

## 💡 Key Takeaways

### Priority Queue Selection Guide

**Use BinaryHeap (Priority Queue) when:**
- ✅ Need to always process highest/lowest priority item
- ✅ Implementing Dijkstra's algorithm
- ✅ Scheduling tasks by priority
- ✅ Finding k-th largest/smallest elements
- ✅ Merging sorted streams

**Use VecDeque (Double-Ended Queue) when:**
- ✅ Need efficient operations at both ends
- ✅ Implementing BFS (queue)
- ✅ Sliding window algorithms
- ✅ Palindrome checking
- ✅ Work-stealing patterns

### Performance Characteristics

| Structure | Insert | Remove Min/Max | Peek | Access |
|-----------|--------|----------------|------|--------|
| BinaryHeap | O(log n) | O(log n) | O(1) | ❌ |
| VecDeque | O(1)* | O(1) | O(1) | O(1) |
| Vec (as queue) | O(1)* | O(n) ❌ | O(1) | O(1) |

*Amortized

### Common Patterns

**Pattern: Top K Elements**
```rust
let mut heap = BinaryHeap::new();
for val in values {
    heap.push(Reverse(val));
    if heap.len() > k {
        heap.pop();
    }
}
// heap now contains k largest elements
```

**Pattern: Merge K Sorted Lists**
```rust
let mut heap = BinaryHeap::new();
// Add first element from each list
for (list_idx, list) in lists.iter().enumerate() {
    heap.push(Reverse((list[0], list_idx, 0)));
}
// Process and add next elements...
```

---

## 🔗 Related Topics

### Tomorrow's Preview: Day 27 - String Parsing
- Split methods and patterns
- Regex for complex parsing
- Custom parser implementations
- AoC input handling strategies

### Mission Integration
- **Mission 2**: Ring buffer VecDeque implementation
- **Mission 6**: Dijkstra's algorithm with BinaryHeap

### AoC Applications
- **Pathfinding**: Dijkstra for weighted grids
- **Scheduling**: Priority-based task processing
- **Window Problems**: Sliding window maximum/minimum
- **Stream Processing**: Efficient queue operations

---

*Tags: #priority-queue #binary-heap #vecdeque #dijkstra #advanced-queues #mission2 #mission6 #graph-algorithms #aoc-patterns*
*Links: [[daily-study/Day25]] ← | [[../missions/Mission6/README|Mission6]] | [[zettel-index]] | [[daily-study/Day27]] →*
