# AoC Pattern Library

*Navigation: [[zettel-index]] | [[AoC Patterns MOC]] | [[aoc-optimization-strategies]] | [[Collections MOC]]*

---

## Overview

This is a **practical quick-reference** for common AoC solution patterns. It provides copy-paste code templates organized by problem type. For detailed algorithm explanations, see [[AoC Patterns MOC]]. For optimization strategies, see [[aoc-optimization-strategies]].

**Philosophy**: Recognize the pattern → Apply the template → Customize for the problem.

---

## 🔲 Grid Patterns (Most Common ~30%)

### Pattern: 2D Grid Navigation

```rust
// Standard 4-directional movement
const DIRS_4: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

// 8-directional (including diagonals)
const DIRS_8: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0, 1),
    (1, -1),  (1, 0),  (1, 1),
];

fn neighbors_4(pos: (usize, usize), rows: usize, cols: usize) -> impl Iterator<Item = (usize, usize)> {
    DIRS_4.iter().filter_map(move |(dr, dc)| {
        let nr = pos.0 as i32 + dr;
        let nc = pos.1 as i32 + dc;
        if nr >= 0 && nc >= 0 && (nr as usize) < rows && (nc as usize) < cols {
            Some((nr as usize, nc as usize))
        } else {
            None
        }
    })
}
```

**When to use**: Maze navigation, flood fill, cellular automata, pathfinding.

**Related**: [[mission-6]] provides `Grid<T>` with `neighbors_4_bounded()` and `neighbors_8_bounded()`.

### Pattern: BFS Shortest Path

```rust
use std::collections::{VecDeque, HashSet};

fn bfs_shortest_path(grid: &[Vec<char>], start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let rows = grid.len();
    let cols = grid[0].len();
    
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    queue.push_back((start, 0));  // (position, distance)
    visited.insert(start);
    
    while let Some((pos, dist)) = queue.pop_front() {
        if pos == end {
            return Some(dist);
        }
        
        for next in neighbors_4(pos, rows, cols) {
            if grid[next.0][next.1] != '#' && !visited.contains(&next) {
                visited.insert(next);
                queue.push_back((next, dist + 1));
            }
        }
    }
    None
}
```

**When to use**: Unweighted shortest path, "minimum steps to reach goal".

**Related**: [[mission-8]] provides `bfs()` and `Graph` trait.

### Pattern: Flood Fill / Connected Components

```rust
fn flood_fill(grid: &mut [Vec<char>], start: (usize, usize), old: char, new: char) -> usize {
    if grid[start.0][start.1] != old { return 0; }
    
    let mut stack = vec![start];
    let mut count = 0;
    
    while let Some(pos) = stack.pop() {
        if grid[pos.0][pos.1] != old { continue; }
        
        grid[pos.0][pos.1] = new;
        count += 1;
        
        for next in neighbors_4(pos, grid.len(), grid[0].len()) {
            if grid[next.0][next.1] == old {
                stack.push(next);
            }
        }
    }
    count
}
```

**When to use**: Finding regions, counting islands, connected component problems.

**Related**: [[mission-10]] provides `UnionFind` for alternative approach.

---

## 📝 Parsing Patterns (~25%)

### Pattern: Line-by-Line with Split

```rust
// Simple whitespace-separated numbers
fn parse_numbers(input: &str) -> Vec<i64> {
    input.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

// Comma-separated values
fn parse_csv_numbers(line: &str) -> Vec<i64> {
    line.split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect()
}

// Grid of digits
fn parse_digit_grid(input: &str) -> Vec<Vec<u32>> {
    input.lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}
```

### Pattern: Regex Extraction

```rust
use regex::Regex;

// Extract structured data with named captures
fn parse_instruction(line: &str) -> Option<(i32, i32, i32)> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let caps = re.captures(line)?;
    Some((
        caps[1].parse().ok()?,
        caps[2].parse().ok()?,
        caps[3].parse().ok()?,
    ))
}

// Find all numbers (including negative)
fn extract_all_numbers(input: &str) -> Vec<i64> {
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(input)
        .filter_map(|m| m.as_str().parse().ok())
        .collect()
}
```

**When to use**: Complex structured input, extracting specific patterns.

### Pattern: State Machine Parsing

```rust
// For multi-section inputs with blank line separators
fn parse_sections(input: &str) -> Vec<Vec<&str>> {
    input.split("\n\n")
        .map(|section| section.lines().collect())
        .collect()
}

// Key-value parsing
fn parse_key_values(input: &str) -> HashMap<String, String> {
    input.lines()
        .filter_map(|line| {
            let (k, v) = line.split_once(':')?;
            Some((k.trim().to_string(), v.trim().to_string()))
        })
        .collect()
}
```

---

## 🧮 Dynamic Programming Patterns (~20%)

### Pattern: Top-Down Memoization

```rust
use std::collections::HashMap;

fn solve_memoized(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if let Some(&cached) = cache.get(&n) {
        return cached;
    }
    
    let result = if n <= 1 {
        n
    } else {
        // Recursive case - customize for problem
        solve_memoized(n - 1, cache) + solve_memoized(n - 2, cache)
    };
    
    cache.insert(n, result);
    result
}
```

**When to use**: Recursive problems with overlapping subproblems, counting paths.

### Pattern: Bottom-Up Tabulation

```rust
fn solve_tabulated(n: usize) -> usize {
    if n <= 1 { return n; }
    
    let mut dp = vec![0; n + 1];
    dp[1] = 1;
    
    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    
    dp[n]
}

// Space-optimized (when only need last few values)
fn solve_optimized(n: usize) -> usize {
    if n <= 1 { return n; }
    
    let (mut prev, mut curr) = (0, 1);
    for _ in 2..=n {
        (prev, curr) = (curr, prev + curr);
    }
    curr
}
```

**When to use**: Path counting, optimization problems, when full DP table not needed for backtracking.

---

## 🔍 Search Patterns

### Pattern: Binary Search on Answer

```rust
// When searching for minimum/maximum value that satisfies condition
fn binary_search_answer<F>(mut lo: i64, mut hi: i64, is_valid: F) -> i64 
where F: Fn(i64) -> bool 
{
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if is_valid(mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

// Usage: find minimum X where condition is true
let answer = binary_search_answer(1, 1_000_000, |x| can_complete_in_time(x));
```

**When to use**: "What is the minimum X such that...", optimization problems with monotonic condition.

**Related**: [[mission-3]] provides binary search utilities.

### Pattern: Dijkstra's Algorithm

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn dijkstra(graph: &[Vec<(usize, u64)>], start: usize) -> Vec<u64> {
    let n = graph.len();
    let mut dist = vec![u64::MAX; n];
    let mut heap = BinaryHeap::new();
    
    dist[start] = 0;
    heap.push(Reverse((0, start)));  // (distance, node)
    
    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] { continue; }  // Skip outdated entries
        
        for &(v, weight) in &graph[u] {
            let new_dist = d + weight;
            if new_dist < dist[v] {
                dist[v] = new_dist;
                heap.push(Reverse((new_dist, v)));
            }
        }
    }
    dist
}
```

**When to use**: Weighted shortest path, minimum cost problems.

---

## 🧵 Concurrency Patterns (Optimization)

### Pattern: Parallel Search with Rayon

```rust
use rayon::prelude::*;

// Parallel brute force search
fn find_answer_parallel(search_space: &[Input]) -> Option<u64> {
    search_space.par_iter()
        .find_any(|input| is_solution(input))
        .map(|input| compute_answer(input))
}

// Parallel grid processing
fn count_matches(grid: &[Vec<char>]) -> usize {
    grid.par_iter()
        .map(|row| row.iter().filter(|&&c| c == '#').count())
        .sum()
}
```

**When to use**: Large search spaces, independent computations.

**Related**: [[rayon-parallel-iterators]] | [[async-vs-threads-decision]]

### Pattern: Channel-Based Pipeline

```rust
use std::sync::mpsc;
use std::thread;

fn parallel_pipeline<T: Send + 'static>(
    inputs: Vec<T>,
    process: fn(T) -> u64,
) -> u64 {
    let (tx, rx) = mpsc::channel();
    let handles: Vec<_> = inputs.into_iter()
        .map(|input| {
            let tx = tx.clone();
            thread::spawn(move || {
                tx.send(process(input)).unwrap();
            })
        })
        .collect();
    
    drop(tx);  // Close sender so receiver knows when done
    
    for h in handles { h.join().unwrap(); }
    rx.iter().sum()
}
```

**Related**: [[message-passing-channels]] | [[shared-state-concurrency]]

---

## 🗃️ Data Structure Selection

| **Problem Type** | **Best Data Structure** | **Mission** |
|------------------|-------------------------|-------------|
| LIFO operations, DFS | `Stack` / `Vec` | [[mission-1]] |
| FIFO operations, BFS | `VecDeque` | [[mission-2]] |
| Fast lookup by key | `HashMap` | [[mission-5]] |
| Unique elements | `HashSet` | [[mission-5]] |
| 2D grid operations | `Grid<T>` | [[mission-6]] |
| Graph traversal | `Graph` trait | [[mission-8]] |
| Minimum/maximum tracking | `BinaryHeap` | [[mission-9]] |
| Connected components | `UnionFind` | [[mission-10]] |

---

## 🎯 Pattern Recognition Quick Guide

```
┌─────────────────────────────────────────────────────────────────┐
│                    READ PROBLEM DESCRIPTION                      │
└───────────────────────────┬─────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        ▼                   ▼                   ▼
   ┌─────────┐        ┌──────────┐        ┌──────────┐
   │ Grid /  │        │ Shortest │        │ Counting │
   │ 2D Map? │        │  Path?   │        │  Ways?   │
   └────┬────┘        └────┬─────┘        └────┬─────┘
        │                  │                   │
        ▼                  ▼                   ▼
   Use Grid           Weighted?            Overlapping
   Patterns           ├─ Yes: Dijkstra     Subproblems?
   + BFS/DFS          └─ No: BFS           ├─ Yes: DP/Memo
                                           └─ No: Combinatorics
```

### Quick Pattern Signals

| **In Problem Description** | **Pattern to Try** |
|---------------------------|-------------------|
| "minimum steps", "shortest path" | BFS |
| "with weights", "minimum cost" | Dijkstra |
| "all possible", "count ways" | DP or backtracking |
| "grid", "map", "coordinates" | Grid patterns |
| "simulate", "apply rules" | State machine |
| "parse", "extract", "instructions" | Regex + parsing |
| "connected", "groups", "regions" | Union-Find or flood fill |

---

## Related Concepts

### Detailed Guides
- [[AoC Patterns MOC]] - Comprehensive pattern navigation
- [[aoc-optimization-strategies]] - Performance optimization levels
- [[rayon-parallel-iterators]] - Parallel processing with Rayon

### Concurrency
- [[message-passing-channels]] - Producer/consumer pipelines
- [[shared-state-concurrency]] - Shared memo caches, accumulators
- [[sync-send-traits]] - Thread-safety checklist
- [[async-vs-threads-decision]] - Choosing concurrency model

### Mission Libraries
- [[mission-5]] - HashMap for caching and lookups
- [[mission-6]] - Grid infrastructure
- [[mission-8]] - Graph algorithms
- [[mission-10]] - Union-Find for components

---

*Tags: #aoc #patterns #algorithms #templates #quick-reference #competitive-programming*

*Links: [[zettel-index]] | [[AoC Patterns MOC]] | [[aoc-optimization-strategies]] | [[Collections MOC]] | [[rust-concurrency-moc]]*
