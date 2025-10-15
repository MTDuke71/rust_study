// Day 26: Advanced Queues (Priority Queues & VecDeque Patterns)
// Complete runnable example demonstrating priority queues, Dijkstra's algorithm,
// and VecDeque advanced patterns

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
        vec![1, 1, 1, 1, 1, 1],
        vec![1, 9, 9, 9, 9, 9],
        vec![1, 9, 1, 9, 9, 9],
        vec![1, 9, 1, 1, 1, 9],
        vec![1, 1, 1, 9, 1, 1],
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
