# TSP Algorithms - Traveling Salesman Problem Implementation Patterns

## Overview

The Traveling Salesman Problem (TSP) is a classic optimization problem: given a set of cities and distances between them, find the shortest/longest route that visits each city exactly once and returns to the starting city.

## Problem Variants

### Classic TSP (Day 9)

- **Goal**: Find shortest route through all cities
- **Constraints**: Visit each city exactly once, return to start
- **Graph**: Weighted, undirected, complete
- **Real-world**: Package delivery, circuit board drilling

### Circular Seating TSP (Day 13)

- **Goal**: Find arrangement maximizing happiness in circular seating
- **Constraints**: Each person has exactly two neighbors (circular)
- **Graph**: Weighted, directed, complete (adjacency graph)
- **Key Difference**: No need to return to start (already circular)

### Linear Arrangement

- **Goal**: Arrange items in a line (not circular)
- **Constraints**: First/last items have only one neighbor
- **Applications**: Assembly line optimization, DNA sequencing

## Algorithm Approaches

### 1. Brute Force (Exact Solution)

```rust
pub fn solve_tsp_brute_force(graph: &Graph) -> (i32, Vec<City>) {
    let cities = graph.get_cities();
    let mut best_cost = if minimize { i32::MAX } else { i32::MIN };
    let mut best_route = Vec::new();
    
    // Generate all permutations
    generate_permutations(&mut cities, |route| {
        let cost = calculate_route_cost(graph, route);
        if is_better(cost, best_cost, minimize) {
            best_cost = cost;
            best_route = route.to_vec();
        }
    });
    
    (best_cost, best_route)
}
```

**Time Complexity**: O(n! × n)
**Space Complexity**: O(n)
**Practical Limit**: ~12-15 cities

### 2. Dynamic Programming (Held-Karp Algorithm)

```rust
// State: (current_city, visited_set) -> min_cost
fn tsp_dp(graph: &Graph) -> i32 {
    let n = graph.size();
    let mut memo = HashMap::new();
    
    // memo[(current, visited_mask)] = min_cost_to_complete
    fn solve(current: usize, visited: u32, graph: &Graph, memo: &mut HashMap<(usize, u32), i32>) -> i32 {
        if visited == (1 << n) - 1 {
            // All cities visited, return to start
            return graph.distance(current, 0);
        }
        
        if let Some(&cached) = memo.get(&(current, visited)) {
            return cached;
        }
        
        let mut min_cost = i32::MAX;
        for next in 0..n {
            if visited & (1 << next) == 0 {  // City not visited
                let new_visited = visited | (1 << next);
                let cost = graph.distance(current, next) + 
                          solve(next, new_visited, graph, memo);
                min_cost = min_cost.min(cost);
            }
        }
        
        memo.insert((current, visited), min_cost);
        min_cost
    }
    
    solve(0, 1, graph, &mut memo)  // Start at city 0
}
```

**Time Complexity**: O(n² × 2ⁿ)
**Space Complexity**: O(n × 2ⁿ)
**Practical Limit**: ~20-25 cities

### 3. Approximation Algorithms

#### Nearest Neighbor Heuristic

```rust
fn tsp_nearest_neighbor(graph: &Graph, start: usize) -> (i32, Vec<usize>) {
    let mut route = vec![start];
    let mut visited = vec![false; graph.size()];
    visited[start] = true;
    let mut total_cost = 0;
    let mut current = start;
    
    for _ in 1..graph.size() {
        let mut nearest = None;
        let mut min_distance = i32::MAX;
        
        for next in 0..graph.size() {
            if !visited[next] {
                let dist = graph.distance(current, next);
                if dist < min_distance {
                    min_distance = dist;
                    nearest = Some(next);
                }
            }
        }
        
        if let Some(next) = nearest {
            route.push(next);
            visited[next] = true;
            total_cost += min_distance;
            current = next;
        }
    }
    
    // Return to start
    total_cost += graph.distance(current, start);
    route.push(start);
    
    (total_cost, route)
}
```

**Time Complexity**: O(n²)
**Approximation Ratio**: Can be arbitrarily bad, but often good in practice

## Symmetry Optimizations

### Rotational Symmetry

For circular arrangements, all rotations are equivalent:

```rust
// These represent the same circular arrangement:
[A, B, C, D] ≡ [B, C, D, A] ≡ [C, D, A, B] ≡ [D, A, B, C]

// Fix first element to eliminate rotational duplicates
fn tsp_with_rotation_optimization(graph: &Graph) -> (i32, Vec<City>) {
    let cities = graph.get_cities();
    let fixed_city = cities[0].clone();
    let mut remaining = cities[1..].to_vec();
    
    // Generate permutations of remaining cities only
    let mut best_cost = i32::MIN;  // or i32::MAX for minimization
    let mut best_route = Vec::new();
    
    generate_permutations(&mut remaining, |perm| {
        let mut full_route = vec![fixed_city.clone()];
        full_route.extend_from_slice(perm);
        
        let cost = calculate_circular_cost(graph, &full_route);
        if cost > best_cost {  // or < for minimization
            best_cost = cost;
            best_route = full_route.clone();
        }
    });
    
    (best_cost, best_route)
}
```

**Reduction**: n! → (n-1)! permutations (n× speedup)

### Reflectional Symmetry

Clockwise and counter-clockwise arrangements are equivalent:

```rust
// These represent the same circular arrangement:
[A, B, C, D] ≡ [A, D, C, B]  (clockwise ≡ counter-clockwise)
// Person A has same neighbors (D and B) in both arrangements

// Fix first TWO ADJACENT elements to eliminate both rotational and reflectional duplicates
fn tsp_full_optimization(graph: &Graph) -> (i32, Vec<City>) {
    let cities = graph.get_cities();
    
    // CRITICAL: Fix positions 0 and 1 (adjacent in circle)
    let fixed_position_0 = cities[0].clone();  // Rotational anchor
    let fixed_position_1 = cities[1].clone();  // Directional anchor (clockwise from 0)
    let mut remaining = cities[2..].to_vec();   // Only permute positions 2,3,4...
    
    let mut best_cost = i32::MIN;
    let mut best_route = Vec::new();
    
    // Generate permutations of remaining positions only
    generate_permutations(&mut remaining, |perm| {
        let mut full_route = vec![fixed_position_0.clone(), fixed_position_1.clone()];
        full_route.extend_from_slice(perm);
        
        let cost = calculate_circular_cost(graph, &full_route);
        if cost > best_cost {
            best_cost = cost;
            best_route = full_route.clone();
        }
    });
    
    (best_cost, best_route)
}
```

**How It Works**:

1. **Position 0**: Always contains `cities[0]` (eliminates rotational symmetry)
2. **Position 1**: Always contains `cities[1]` (eliminates reflectional symmetry)  
3. **Positions 2-N**: Permute `cities[2..]` only (reduces from n! to (n-2)!)

**Combined Reduction**: n! → (n-2)! permutations  

- **8 people**: 40,320 → 720 (56× speedup)
- **10 people**: 3,628,800 → 40,320 (90× speedup)

## Implementation Patterns in Rust

### Permutation Generation

```rust
// Heap's Algorithm - most efficient for TSP
pub fn generate_permutations<T: Clone>(items: &mut [T], callback: &mut impl FnMut(&[T])) {
    fn heap_permute<T: Clone>(items: &mut [T], k: usize, callback: &mut impl FnMut(&[T])) {
        if k == 1 {
            callback(items);
            return;
        }
        
        for i in 0..k {
            heap_permute(items, k - 1, callback);
            
            if k % 2 == 0 {
                items.swap(i, k - 1);
            } else {
                items.swap(0, k - 1);
            }
        }
    }
    
    heap_permute(items, items.len(), callback);
}
```

### Distance/Cost Calculation

```rust
// Linear route (start → cities → end)
fn calculate_linear_cost(graph: &Graph, route: &[City]) -> i32 {
    let mut total = 0;
    for i in 0..route.len() - 1 {
        total += graph.distance(&route[i], &route[i + 1]);
    }
    total
}

// Circular route (each city connects to neighbors)
fn calculate_circular_cost(graph: &Graph, route: &[City]) -> i32 {
    let n = route.len();
    if n <= 1 { return 0; }
    
    let mut total = 0;
    for i in 0..n {
        let current = &route[i];
        let left = &route[(i + n - 1) % n];
        let right = &route[(i + 1) % n];
        
        // Each person contributes happiness toward both neighbors
        total += graph.happiness(current, left);
        total += graph.happiness(current, right);
    }
    total
}
```

### Generic TSP Solver

```rust
pub fn solve_tsp<T: Clone + Eq + Hash>(
    items: Vec<T>,
    cost_fn: impl Fn(&[T]) -> i32,
    minimize: bool
) -> (i32, Vec<T>) {
    let mut best_cost = if minimize { i32::MAX } else { i32::MIN };
    let mut best_arrangement = Vec::new();
    
    let mut items_mut = items;
    generate_permutations(&mut items_mut, &mut |arrangement| {
        let cost = cost_fn(arrangement);
        
        let is_better = if minimize { 
            cost < best_cost 
        } else { 
            cost > best_cost 
        };
        
        if is_better {
            best_cost = cost;
            best_arrangement = arrangement.to_vec();
        }
    });
    
    (best_cost, best_arrangement)
}
```

## AoC Implementations

### Day 9: All in a Single Night

```rust
// Classic TSP with distance matrix
let (shortest, _) = solve_tsp(cities.clone(), |route| {
    calculate_route_distance(&distance_matrix, route)
}, true);  // minimize = true

let (longest, _) = solve_tsp(cities.clone(), |route| {
    calculate_route_distance(&distance_matrix, route)
}, false);  // minimize = false
```

### Day 13: Knights of the Dinner Table

```rust
// Circular seating TSP with happiness optimization
let (max_happiness, arrangement) = solve_tsp(people.clone(), |seating| {
    calculate_circular_happiness(&happiness_graph, seating)
}, false);  // maximize happiness

// Part 2: Add neutral person and re-optimize
happiness_graph.add_neutral_person("You");
let (part2_happiness, part2_arrangement) = solve_tsp(
    happiness_graph.get_people(), 
    |seating| calculate_circular_happiness(&happiness_graph, seating),
    false
);
```

## Performance Comparison

| Algorithm | Time | Space | Accuracy | Practical Limit |
|-----------|------|-------|----------|-----------------|
| **Brute Force** | O(n! × n) | O(n) | Optimal | 12-15 cities |
| **Brute + Symmetry** | O((n-1)!/2 × n) | O(n) | Optimal | 15-18 cities |
| **Dynamic Programming** | O(n² × 2ⁿ) | O(n × 2ⁿ) | Optimal | 20-25 cities |
| **Nearest Neighbor** | O(n²) | O(n) | Heuristic | Any size |
| **2-opt Local Search** | O(n²) | O(n) | Heuristic | Any size |

## When to Use Each Approach

### Brute Force

- ✅ **Small instances** (≤ 15 items)
- ✅ **Guaranteed optimal** solution needed
- ✅ **Simple implementation** required
- ❌ **Larger instances** (exponential explosion)

### Dynamic Programming

- ✅ **Medium instances** (15-25 items)
- ✅ **Guaranteed optimal** solution needed
- ✅ **More memory available** (for memoization)
- ❌ **Very large instances** (still exponential)

### Heuristics

- ✅ **Large instances** (25+ items)
- ✅ **Quick approximation** acceptable
- ✅ **Real-time constraints** (immediate response needed)
- ❌ **Optimal solution** required

## Advanced Optimizations

### Branch and Bound

```rust
fn tsp_branch_and_bound(graph: &Graph) -> i32 {
    let mut best_cost = i32::MAX;
    
    fn branch(current_path: &mut Vec<usize>, 
              current_cost: i32, 
              visited: &mut [bool],
              graph: &Graph,
              best_cost: &mut i32) {
        
        // Bound: estimate remaining cost
        let lower_bound = current_cost + estimate_remaining_cost(visited, graph);
        if lower_bound >= *best_cost {
            return;  // Prune this branch
        }
        
        if current_path.len() == graph.size() {
            // Complete tour, update best
            let total = current_cost + graph.distance(current_path.last().unwrap(), &0);
            *best_cost = (*best_cost).min(total);
            return;
        }
        
        // Branch: try all unvisited cities
        for next in 0..graph.size() {
            if !visited[next] {
                visited[next] = true;
                current_path.push(next);
                let edge_cost = graph.distance(current_path[current_path.len()-2], &next);
                
                branch(current_path, current_cost + edge_cost, visited, graph, best_cost);
                
                // Backtrack
                current_path.pop();
                visited[next] = false;
            }
        }
    }
    
    let mut path = vec![0];
    let mut visited = vec![false; graph.size()];
    visited[0] = true;
    
    branch(&mut path, 0, &mut visited, graph, &mut best_cost);
    best_cost
}
```

### Parallel Processing

```rust
use rayon::prelude::*;

fn tsp_parallel(graph: &Graph) -> (i32, Vec<usize>) {
    let cities = (1..graph.size()).collect::<Vec<_>>();
    
    // Generate chunks of permutations for parallel processing
    let results: Vec<_> = cities.into_par_iter()
        .map(|first_city| {
            // Each thread processes permutations starting with first_city
            let mut remaining: Vec<_> = (1..graph.size())
                .filter(|&c| c != first_city)
                .collect();
            
            solve_tsp_chunk(graph, first_city, remaining)
        })
        .collect();
    
    // Find best result across all threads
    results.into_iter()
        .min_by_key(|(cost, _)| *cost)
        .unwrap()
}
```

## Educational Value

### Algorithm Design Principles

1. **Problem Recognition** - Identifying TSP variants in different contexts
2. **Optimization Strategy** - When to use exact vs. approximate solutions
3. **Symmetry Exploitation** - Mathematical insights for computational speedup
4. **Trade-off Analysis** - Time vs. space vs. accuracy decisions

### Implementation Skills

1. **Permutation Algorithms** - Heap's algorithm, lexicographic generation
2. **Memoization Patterns** - Dynamic programming with complex state
3. **Generic Programming** - Reusable TSP solver for different cost functions
4. **Performance Engineering** - Measuring and optimizing algorithm performance

### Mathematical Foundations

1. **Combinatorics** - Understanding factorial growth and permutation spaces
2. **Graph Theory** - Adjacency representations and path calculations
3. **Optimization Theory** - Local vs. global optimization, greedy algorithm failures
4. **Complexity Analysis** - Big-O notation and practical scalability limits

---

*Created: 2025-10-19*
*Last Updated: 2025-10-19*

*Tags: #traveling-salesman #algorithms #optimization #permutations #dynamic-programming #branch-and-bound #heuristics #graph-theory #combinatorial-optimization*

*Links: [[Graph Theory MOC]] | [[day13_analysis]] | [[../advent_of_code/aoc2015/Problem_Statements/day09]] | [[Heap's Algorithm Deep Dive]] | [[Symmetry in Algorithms]] | [[Performance Engineering]] | [[Memory Optimization]] | [[Combinatorial Optimization]]*
