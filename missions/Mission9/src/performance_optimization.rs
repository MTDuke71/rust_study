//! # Performance Optimizations for Pathfinding
//!
//! This module implements advanced performance optimizations for pathfinding algorithms:
//! - REQ-3: Bidirectional search optimization
//! - REQ-4: Early termination strategies  
//! - Memory pool management
//! - Cache-friendly data structures
//!
//! ## Performance Features
//! - Bidirectional A* and Dijkstra algorithms
//! - Memory-optimized node structures
//! - Early termination with bound checking
//! - Custom memory allocators for node management

use std::collections::{BinaryHeap, HashSet, HashMap};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// Memory-optimized node structure
// Keep the original OptimizedNode for compatibility
#[derive(Debug, Clone)]
pub struct OptimizedNode {
    pub position: (usize, usize),
    pub g_cost: u32,        // Actual cost from start
    pub h_cost: u32,        // Heuristic cost to goal
    pub parent: Option<(usize, usize)>,
}

impl OptimizedNode {
    pub fn new(position: (usize, usize), g_cost: u32, h_cost: u32, parent: Option<(usize, usize)>) -> Self {
        Self {
            position,
            g_cost,
            h_cost,
            parent,
        }
    }

    #[inline]
    pub fn f_cost(&self) -> u32 {
        self.g_cost + self.h_cost
    }
}

// Add a truly memory-optimized version with u32 coordinates
#[derive(Debug, Clone)]
pub struct TrulyOptimizedNode {
    pub position: (u32, u32),          // 8 bytes instead of 16
    pub g_cost: u32,                   // Actual cost from start
    pub h_cost: u32,                   // Heuristic cost to goal
    pub parent: Option<(u32, u32)>,    // 12 bytes instead of 24
}

impl TrulyOptimizedNode {
    pub fn new(position: (u32, u32), g_cost: u32, h_cost: u32, parent: Option<(u32, u32)>) -> Self {
        Self {
            position,
            g_cost,
            h_cost,
            parent,
        }
    }

    #[inline]
    pub fn f_cost(&self) -> u32 {
        self.g_cost + self.h_cost
    }

    /// Convert from usize coordinates (for compatibility with existing APIs)
    pub fn from_usize(position: (usize, usize), g_cost: u32, h_cost: u32, parent: Option<(usize, usize)>) -> Self {
        Self {
            position: (position.0 as u32, position.1 as u32),
            g_cost,
            h_cost,
            parent: parent.map(|(x, y)| (x as u32, y as u32)),
        }
    }

    /// Convert to usize coordinates (for compatibility with existing APIs)
    pub fn to_usize(&self) -> ((usize, usize), Option<(usize, usize)>) {
        (
            (self.position.0 as usize, self.position.1 as usize),
            self.parent.map(|(x, y)| (x as usize, y as usize))
        )
    }
}

impl PartialEq for TrulyOptimizedNode {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for TrulyOptimizedNode {}

impl Hash for TrulyOptimizedNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
    }
}

impl Ord for TrulyOptimizedNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min-heap: lower f_cost has higher priority
        other.f_cost().cmp(&self.f_cost())
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for TrulyOptimizedNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for OptimizedNode {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for OptimizedNode {}

impl Hash for OptimizedNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
    }
}

impl Ord for OptimizedNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min-heap: lower f_cost has higher priority
        other.f_cost().cmp(&self.f_cost())
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for OptimizedNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Memory pool for efficient node allocation
pub struct NodePool {
    nodes: Vec<OptimizedNode>,
    free_indices: Vec<usize>,
    capacity: usize,
}

impl NodePool {
    pub fn new(capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity),
            free_indices: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn allocate(&mut self, position: (usize, usize), g_cost: u32, h_cost: u32, parent: Option<(usize, usize)>) -> usize {
        let node = OptimizedNode::new(position, g_cost, h_cost, parent);
        
        if let Some(index) = self.free_indices.pop() {
            self.nodes[index] = node;
            index
        } else if self.nodes.len() < self.capacity {
            let index = self.nodes.len();
            self.nodes.push(node);
            index
        } else {
            // Pool is full, expand it
            self.capacity *= 2;
            self.nodes.reserve(self.capacity - self.nodes.len());
            let index = self.nodes.len();
            self.nodes.push(node);
            index
        }
    }

    pub fn deallocate(&mut self, index: usize) {
        if index < self.nodes.len() {
            self.free_indices.push(index);
        }
    }

    pub fn get(&self, index: usize) -> &OptimizedNode {
        &self.nodes[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut OptimizedNode {
        &mut self.nodes[index]
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.free_indices.clear();
    }

    pub fn stats(&self) -> (usize, usize, usize) {
        (self.nodes.len(), self.free_indices.len(), self.capacity)
    }
}

/// Performance metrics for algorithm comparison
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub nodes_explored: usize,
    pub nodes_generated: usize,
    pub peak_open_set_size: usize,
    pub peak_closed_set_size: usize,
    pub memory_allocations: usize,
    pub early_terminations: usize,
    pub path_length: usize,
    pub path_cost: u32,
    pub search_time_ns: u128,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            nodes_explored: 0,
            nodes_generated: 0,
            peak_open_set_size: 0,
            peak_closed_set_size: 0,
            memory_allocations: 0,
            early_terminations: 0,
            path_length: 0,
            path_cost: 0,
            search_time_ns: 0,
        }
    }

    pub fn combine(&mut self, other: &PerformanceMetrics) {
        self.nodes_explored += other.nodes_explored;
        self.nodes_generated += other.nodes_generated;
        self.peak_open_set_size = self.peak_open_set_size.max(other.peak_open_set_size);
        self.peak_closed_set_size = self.peak_closed_set_size.max(other.peak_closed_set_size);
        self.memory_allocations += other.memory_allocations;
        self.early_terminations += other.early_terminations;
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// REQ-3: Bidirectional A* Search Implementation
pub struct BidirectionalAstar {
    node_pool: NodePool,
    metrics: PerformanceMetrics,
}

impl BidirectionalAstar {
    pub fn new(pool_capacity: usize) -> Self {
        Self {
            node_pool: NodePool::new(pool_capacity),
            metrics: PerformanceMetrics::new(),
        }
    }

    pub fn find_path<F, N>(
        &mut self,
        start: (usize, usize),
        goal: (usize, usize),
        heuristic: F,
        neighbors: N,
    ) -> Option<Vec<(usize, usize)>>
    where
        F: Fn((usize, usize), (usize, usize)) -> u32,
        N: Fn((usize, usize)) -> Vec<((usize, usize), u32)>,
    {
        let start_time = std::time::Instant::now();
        self.metrics = PerformanceMetrics::new();
        self.node_pool.clear();

        // Forward search state
        let mut forward_open = BinaryHeap::new();
        let mut forward_closed = HashSet::new();
        let mut forward_g_scores = HashMap::new();
        let mut forward_came_from = HashMap::new();

        // Backward search state  
        let mut backward_open = BinaryHeap::new();
        let mut backward_closed = HashSet::new();
        let mut backward_g_scores = HashMap::new();
        let mut backward_came_from = HashMap::new();

        // Initialize forward search
        let forward_start = OptimizedNode::new(start, 0, heuristic(start, goal), None);
        forward_open.push(forward_start);
        forward_g_scores.insert(start, 0);
        self.metrics.nodes_generated += 1;

        // Initialize backward search
        let backward_start = OptimizedNode::new(goal, 0, heuristic(goal, start), None);
        backward_open.push(backward_start);
        backward_g_scores.insert(goal, 0);
        self.metrics.nodes_generated += 1;

        let mut best_path_cost = u32::MAX;
        let mut meeting_point: Option<(usize, usize)> = None;

        // Main bidirectional search loop
        while !forward_open.is_empty() && !backward_open.is_empty() {
            // Update peak sizes
            self.metrics.peak_open_set_size = self.metrics.peak_open_set_size
                .max(forward_open.len() + backward_open.len());
            self.metrics.peak_closed_set_size = self.metrics.peak_closed_set_size
                .max(forward_closed.len() + backward_closed.len());

            // Expand forward search
            if let Some(forward_current) = forward_open.pop() {
                self.metrics.nodes_explored += 1;

                // REQ-4: Early termination check
                if forward_current.f_cost() >= best_path_cost {
                    self.metrics.early_terminations += 1;
                    break;
                }

                // Check for meeting with backward search
                if let Some(&backward_g) = backward_g_scores.get(&forward_current.position) {
                    let total_cost = forward_current.g_cost + backward_g;
                    if total_cost < best_path_cost {
                        best_path_cost = total_cost;
                        meeting_point = Some(forward_current.position);
                    }
                }

                forward_closed.insert(forward_current.position);

                // Expand neighbors
                for (neighbor_pos, edge_cost) in neighbors(forward_current.position) {
                    if forward_closed.contains(&neighbor_pos) {
                        continue;
                    }

                    let tentative_g = forward_current.g_cost + edge_cost;

                    if let Some(&existing_g) = forward_g_scores.get(&neighbor_pos) {
                        if tentative_g >= existing_g {
                            continue;
                        }
                    }

                    forward_g_scores.insert(neighbor_pos, tentative_g);
                    forward_came_from.insert(neighbor_pos, forward_current.position);

                    let neighbor_h = heuristic(neighbor_pos, goal);
                    let neighbor_node = OptimizedNode::new(neighbor_pos, tentative_g, neighbor_h, Some(forward_current.position));

                    forward_open.push(neighbor_node);
                    self.metrics.nodes_generated += 1;
                }
            }

            // Expand backward search
            if let Some(backward_current) = backward_open.pop() {
                self.metrics.nodes_explored += 1;

                // REQ-4: Early termination check
                if backward_current.f_cost() >= best_path_cost {
                    self.metrics.early_terminations += 1;
                    break;
                }

                // Check for meeting with forward search
                if let Some(&forward_g) = forward_g_scores.get(&backward_current.position) {
                    let total_cost = backward_current.g_cost + forward_g;
                    if total_cost < best_path_cost {
                        best_path_cost = total_cost;
                        meeting_point = Some(backward_current.position);
                    }
                }

                backward_closed.insert(backward_current.position);

                // Expand neighbors
                for (neighbor_pos, edge_cost) in neighbors(backward_current.position) {
                    if backward_closed.contains(&neighbor_pos) {
                        continue;
                    }

                    let tentative_g = backward_current.g_cost + edge_cost;

                    if let Some(&existing_g) = backward_g_scores.get(&neighbor_pos) {
                        if tentative_g >= existing_g {
                            continue;
                        }
                    }

                    backward_g_scores.insert(neighbor_pos, tentative_g);
                    backward_came_from.insert(neighbor_pos, backward_current.position);

                    let neighbor_h = heuristic(neighbor_pos, start);
                    let neighbor_node = OptimizedNode::new(neighbor_pos, tentative_g, neighbor_h, Some(backward_current.position));

                    backward_open.push(neighbor_node);
                    self.metrics.nodes_generated += 1;
                }
            }

            // REQ-4: Additional early termination condition
            // If both searches have reached their optimal bounds
            if !forward_open.is_empty() && !backward_open.is_empty() {
                let forward_min_f = forward_open.peek().map(|n| n.f_cost()).unwrap_or(u32::MAX);
                let backward_min_f = backward_open.peek().map(|n| n.f_cost()).unwrap_or(u32::MAX);
                
                if forward_min_f + backward_min_f >= best_path_cost {
                    self.metrics.early_terminations += 1;
                    break;
                }
            }
        }

        self.metrics.search_time_ns = start_time.elapsed().as_nanos();

        // Reconstruct path if meeting point found
        if let Some(meeting) = meeting_point {
            let path = self.reconstruct_bidirectional_path(
                &forward_came_from,
                &backward_came_from,
                start,
                goal,
                meeting,
            );

            self.metrics.path_length = path.len();
            self.metrics.path_cost = best_path_cost;
            Some(path)
        } else {
            None
        }
    }

    fn reconstruct_bidirectional_path(
        &self,
        forward_came_from: &HashMap<(usize, usize), (usize, usize)>,
        backward_came_from: &HashMap<(usize, usize), (usize, usize)>,
        start: (usize, usize),
        goal: (usize, usize),
        meeting: (usize, usize),
    ) -> Vec<(usize, usize)> {
        let mut path = Vec::new();

        // Build forward path (start to meeting point)
        let mut forward_path = Vec::new();
        let mut current = meeting;
        forward_path.push(current);

        while let Some(&parent) = forward_came_from.get(&current) {
            forward_path.push(parent);
            current = parent;
            if current == start {
                break;
            }
        }
        forward_path.reverse();

        // Build backward path (meeting point to goal)
        let mut backward_path = Vec::new();
        current = meeting;

        while let Some(&parent) = backward_came_from.get(&current) {
            backward_path.push(parent);
            current = parent;
            if current == goal {
                break;
            }
        }

        // Combine paths (avoid duplicating meeting point)
        path.extend(forward_path);
        path.extend(backward_path);

        path
    }

    pub fn get_metrics(&self) -> &PerformanceMetrics {
        &self.metrics
    }

    pub fn reset_metrics(&mut self) {
        self.metrics = PerformanceMetrics::new();
    }
}

/// REQ-3: Bidirectional Dijkstra Implementation
pub struct BidirectionalDijkstra {
    node_pool: NodePool,
    metrics: PerformanceMetrics,
}

impl BidirectionalDijkstra {
    pub fn new(pool_capacity: usize) -> Self {
        Self {
            node_pool: NodePool::new(pool_capacity),
            metrics: PerformanceMetrics::new(),
        }
    }

    pub fn find_path<N>(
        &mut self,
        start: (usize, usize),
        goal: (usize, usize),
        neighbors: N,
    ) -> Option<Vec<(usize, usize)>>
    where
        N: Fn((usize, usize)) -> Vec<((usize, usize), u32)>,
    {
        let start_time = std::time::Instant::now();
        self.metrics = PerformanceMetrics::new();
        self.node_pool.clear();

        // Forward search state
        let mut forward_open = BinaryHeap::new();
        let mut forward_distances = HashMap::new();
        let mut forward_came_from = HashMap::new();

        // Backward search state
        let mut backward_open = BinaryHeap::new();
        let mut backward_distances = HashMap::new();
        let mut backward_came_from = HashMap::new();

        // Initialize searches
        let forward_start = OptimizedNode::new(start, 0, 0, None);
        forward_open.push(forward_start);
        forward_distances.insert(start, 0);

        let backward_start = OptimizedNode::new(goal, 0, 0, None);
        backward_open.push(backward_start);
        backward_distances.insert(goal, 0);

        let mut best_distance = u32::MAX;
        let mut meeting_point: Option<(usize, usize)> = None;

        // Main search loop
        while !forward_open.is_empty() && !backward_open.is_empty() {
            self.metrics.peak_open_set_size = self.metrics.peak_open_set_size
                .max(forward_open.len() + backward_open.len());

            // Expand forward search
            if let Some(forward_current) = forward_open.pop() {
                self.metrics.nodes_explored += 1;

                // REQ-4: Early termination
                if forward_current.g_cost >= best_distance {
                    self.metrics.early_terminations += 1;
                    break;
                }

                // Check for meeting
                if let Some(&backward_dist) = backward_distances.get(&forward_current.position) {
                    let total_dist = forward_current.g_cost + backward_dist;
                    if total_dist < best_distance {
                        best_distance = total_dist;
                        meeting_point = Some(forward_current.position);
                    }
                }

                // Expand neighbors
                for (neighbor_pos, edge_cost) in neighbors(forward_current.position) {
                    let new_distance = forward_current.g_cost + edge_cost;

                    if let Some(&existing_dist) = forward_distances.get(&neighbor_pos) {
                        if new_distance >= existing_dist {
                            continue;
                        }
                    }

                    forward_distances.insert(neighbor_pos, new_distance);
                    forward_came_from.insert(neighbor_pos, forward_current.position);

                    let neighbor_node = OptimizedNode::new(neighbor_pos, new_distance, 0, Some(forward_current.position));
                    forward_open.push(neighbor_node);
                    self.metrics.nodes_generated += 1;
                }
            }

            // Expand backward search (similar logic)
            if let Some(backward_current) = backward_open.pop() {
                self.metrics.nodes_explored += 1;

                if backward_current.g_cost >= best_distance {
                    self.metrics.early_terminations += 1;
                    break;
                }

                if let Some(&forward_dist) = forward_distances.get(&backward_current.position) {
                    let total_dist = backward_current.g_cost + forward_dist;
                    if total_dist < best_distance {
                        best_distance = total_dist;
                        meeting_point = Some(backward_current.position);
                    }
                }

                for (neighbor_pos, edge_cost) in neighbors(backward_current.position) {
                    let new_distance = backward_current.g_cost + edge_cost;

                    if let Some(&existing_dist) = backward_distances.get(&neighbor_pos) {
                        if new_distance >= existing_dist {
                            continue;
                        }
                    }

                    backward_distances.insert(neighbor_pos, new_distance);
                    backward_came_from.insert(neighbor_pos, backward_current.position);

                    let neighbor_node = OptimizedNode::new(neighbor_pos, new_distance, 0, Some(backward_current.position));
                    backward_open.push(neighbor_node);
                    self.metrics.nodes_generated += 1;
                }
            }
        }

        self.metrics.search_time_ns = start_time.elapsed().as_nanos();

        // Reconstruct path
        if let Some(meeting) = meeting_point {
            let path = self.reconstruct_bidirectional_path(
                &forward_came_from,
                &backward_came_from,
                start,
                goal,
                meeting,
            );

            self.metrics.path_length = path.len();
            self.metrics.path_cost = best_distance;
            Some(path)
        } else {
            None
        }
    }

    fn reconstruct_bidirectional_path(
        &self,
        forward_came_from: &HashMap<(usize, usize), (usize, usize)>,
        backward_came_from: &HashMap<(usize, usize), (usize, usize)>,
        start: (usize, usize),
        goal: (usize, usize),
        meeting: (usize, usize),
    ) -> Vec<(usize, usize)> {
        let mut path = Vec::new();

        // Build forward path
        let mut forward_path = Vec::new();
        let mut current = meeting;
        forward_path.push(current);

        while let Some(&parent) = forward_came_from.get(&current) {
            forward_path.push(parent);
            current = parent;
            if current == start {
                break;
            }
        }
        forward_path.reverse();

        // Build backward path
        let mut backward_path = Vec::new();
        current = meeting;

        while let Some(&parent) = backward_came_from.get(&current) {
            backward_path.push(parent);
            current = parent;
            if current == goal {
                break;
            }
        }

        // Combine paths
        path.extend(forward_path);
        path.extend(backward_path);

        path
    }

    pub fn get_metrics(&self) -> &PerformanceMetrics {
        &self.metrics
    }
}

/// Memory-optimized single-direction A* for comparison
pub struct MemoryOptimizedAstar {
    node_pool: NodePool,
    metrics: PerformanceMetrics,
}

impl MemoryOptimizedAstar {
    pub fn new(pool_capacity: usize) -> Self {
        Self {
            node_pool: NodePool::new(pool_capacity),
            metrics: PerformanceMetrics::new(),
        }
    }

    pub fn find_path<F, N>(
        &mut self,
        start: (usize, usize),
        goal: (usize, usize),
        heuristic: F,
        neighbors: N,
    ) -> Option<Vec<(usize, usize)>>
    where
        F: Fn((usize, usize), (usize, usize)) -> u32,
        N: Fn((usize, usize)) -> Vec<((usize, usize), u32)>,
    {
        let start_time = std::time::Instant::now();
        self.metrics = PerformanceMetrics::new();
        self.node_pool.clear();

        let mut open_set = BinaryHeap::new();
        let mut closed_set = HashSet::new();
        let mut g_scores = HashMap::new();
        let mut came_from = HashMap::new();

        // Initialize start node
        let start_node = OptimizedNode::new(start, 0, heuristic(start, goal), None);
        open_set.push(start_node);
        g_scores.insert(start, 0);
        self.metrics.nodes_generated += 1;

        while let Some(current) = open_set.pop() {
            self.metrics.nodes_explored += 1;
            self.metrics.peak_open_set_size = self.metrics.peak_open_set_size.max(open_set.len());
            self.metrics.peak_closed_set_size = self.metrics.peak_closed_set_size.max(closed_set.len());

            if current.position == goal {
                let path = self.reconstruct_path(&came_from, start, goal);
                self.metrics.search_time_ns = start_time.elapsed().as_nanos();
                self.metrics.path_length = path.len();
                self.metrics.path_cost = current.g_cost;
                return Some(path);
            }

            closed_set.insert(current.position);

            for (neighbor_pos, edge_cost) in neighbors(current.position) {
                if closed_set.contains(&neighbor_pos) {
                    continue;
                }

                let tentative_g = current.g_cost + edge_cost;

                if let Some(&existing_g) = g_scores.get(&neighbor_pos) {
                    if tentative_g >= existing_g {
                        continue;
                    }
                }

                g_scores.insert(neighbor_pos, tentative_g);
                came_from.insert(neighbor_pos, current.position);

                let neighbor_h = heuristic(neighbor_pos, goal);
                let neighbor_node = OptimizedNode::new(neighbor_pos, tentative_g, neighbor_h, Some(current.position));

                open_set.push(neighbor_node);
                self.metrics.nodes_generated += 1;
            }
        }

        self.metrics.search_time_ns = start_time.elapsed().as_nanos();
        None
    }

    fn reconstruct_path(
        &self,
        came_from: &HashMap<(usize, usize), (usize, usize)>,
        start: (usize, usize),
        goal: (usize, usize),
    ) -> Vec<(usize, usize)> {
        let mut path = Vec::new();
        let mut current = goal;
        path.push(current);

        while let Some(&parent) = came_from.get(&current) {
            path.push(parent);
            current = parent;
            if current == start {
                break;
            }
        }

        path.reverse();
        path
    }

    pub fn get_metrics(&self) -> &PerformanceMetrics {
        &self.metrics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> u32 {
        let dx = a.0.abs_diff(b.0);
        let dy = a.1.abs_diff(b.1);
        (dx + dy) as u32
    }

    fn grid_neighbors(pos: (usize, usize), width: usize, height: usize) -> Vec<((usize, usize), u32)> {
        let mut neighbors = Vec::new();
        let (x, y) = pos;

        if x > 0 { neighbors.push(((x-1, y), 1)); }
        if x + 1 < width { neighbors.push(((x+1, y), 1)); }
        if y > 0 { neighbors.push(((x, y-1), 1)); }
        if y + 1 < height { neighbors.push(((x, y+1), 1)); }

        neighbors
    }

    #[test]
    fn test_bidirectional_astar() {
        let mut astar = BidirectionalAstar::new(1000);
        let start = (0, 0);
        let goal = (9, 9);

        let path = astar.find_path(
            start,
            goal,
            manhattan_distance,
            |pos| grid_neighbors(pos, 10, 10),
        );

        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path.first(), Some(&start));
        assert_eq!(path.last(), Some(&goal));
        
        let metrics = astar.get_metrics();
        assert!(metrics.nodes_explored > 0);
        assert!(metrics.path_cost > 0);
    }

    #[test]
    fn test_bidirectional_dijkstra() {
        let mut dijkstra = BidirectionalDijkstra::new(1000);
        let start = (0, 0);
        let goal = (5, 5);

        let path = dijkstra.find_path(
            start,
            goal,
            |pos| grid_neighbors(pos, 10, 10),
        );

        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path.first(), Some(&start));
        assert_eq!(path.last(), Some(&goal));

        let metrics = dijkstra.get_metrics();
        assert!(metrics.nodes_explored > 0);
    }

    #[test]
    fn test_memory_optimized_astar() {
        let mut astar = MemoryOptimizedAstar::new(1000);
        let start = (0, 0);
        let goal = (7, 7);

        let path = astar.find_path(
            start,
            goal,
            manhattan_distance,
            |pos| grid_neighbors(pos, 10, 10),
        );

        assert!(path.is_some());
        let metrics = astar.get_metrics();
        assert!(metrics.nodes_explored > 0);
    }

    #[test]
    fn test_node_pool() {
        let mut pool = NodePool::new(10);
        
        // Test allocation
        let idx1 = pool.allocate((0, 0), 0, 5, None);
        let idx2 = pool.allocate((1, 1), 1, 4, Some((0, 0)));
        
        assert_eq!(pool.get(idx1).position, (0, 0));
        assert_eq!(pool.get(idx2).position, (1, 1));
        
        // Test deallocation and reuse
        pool.deallocate(idx1);
        let idx3 = pool.allocate((2, 2), 2, 3, None);
        
        // Should reuse the deallocated slot
        assert_eq!(idx3, idx1);
        assert_eq!(pool.get(idx3).position, (2, 2));
    }

    #[test]
    fn test_early_termination() {
        let mut astar = BidirectionalAstar::new(1000);
        let start = (0, 0);
        let goal = (2, 2); // Close goal for early termination

        astar.find_path(
            start,
            goal,
            manhattan_distance,
            |pos| grid_neighbors(pos, 10, 10),
        );

        let metrics = astar.get_metrics();
        // Should find path quickly with minimal exploration
        assert!(metrics.nodes_explored < 20);
    }
}