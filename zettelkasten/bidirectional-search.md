# Bidirectional Search

**Definition**: A graph traversal optimization that runs two simultaneous searches - one forward from the start node and one backward from the goal node - meeting in the middle to find the shortest path more efficiently than a single-directional search.

## Core Concept

Instead of exploring the entire search space from start to goal, bidirectional search **reduces the branching factor** by searching from both ends simultaneously. When the two search frontiers meet, you've found a path.

```rust
// Conceptual bidirectional BFS
fn bidirectional_bfs<G: Graph>(
    graph: &G,
    start: NodeId,
    goal: NodeId,
) -> Option<Vec<NodeId>> {
    let mut forward_frontier = VecDeque::from([start]);
    let mut backward_frontier = VecDeque::from([goal]);
    
    let mut forward_visited = HashMap::new();  // Node -> parent
    let mut backward_visited = HashMap::new(); // Node -> parent
    
    forward_visited.insert(start, None);
    backward_visited.insert(goal, None);
    
    while !forward_frontier.is_empty() && !backward_frontier.is_empty() {
        // Search from start
        if let Some(meeting_point) = expand_frontier(
            &mut forward_frontier,
            &mut forward_visited,
            &backward_visited,
            graph,
            true,
        ) {
            return Some(reconstruct_path(
                meeting_point,
                &forward_visited,
                &backward_visited,
            ));
        }
        
        // Search from goal
        if let Some(meeting_point) = expand_frontier(
            &mut backward_frontier,
            &mut backward_visited,
            &forward_visited,
            graph,
            false,
        ) {
            return Some(reconstruct_path(
                meeting_point,
                &forward_visited,
                &backward_visited,
            ));
        }
    }
    
    None // No path found
}

fn expand_frontier<G: Graph>(
    frontier: &mut VecDeque<NodeId>,
    visited: &mut HashMap<NodeId, Option<NodeId>>,
    other_visited: &HashMap<NodeId, Option<NodeId>>,
    graph: &G,
    forward: bool,
) -> Option<NodeId> {
    if let Some(current) = frontier.pop_front() {
        for neighbor in graph.neighbors(current) {
            // Check if we've met the other search
            if other_visited.contains_key(&neighbor) {
                visited.insert(neighbor, Some(current));
                return Some(neighbor);
            }
            
            if !visited.contains_key(&neighbor) {
                visited.insert(neighbor, Some(current));
                frontier.push_back(neighbor);
            }
        }
    }
    None
}

fn reconstruct_path(
    meeting_point: NodeId,
    forward_visited: &HashMap<NodeId, Option<NodeId>>,
    backward_visited: &HashMap<NodeId, Option<NodeId>>,
) -> Vec<NodeId> {
    let mut path = Vec::new();
    
    // Trace forward path (start -> meeting)
    let mut current = Some(meeting_point);
    let mut forward_path = Vec::new();
    while let Some(node) = current {
        forward_path.push(node);
        current = forward_visited.get(&node).and_then(|&parent| parent);
    }
    forward_path.reverse();
    
    // Trace backward path (meeting -> goal)
    current = backward_visited.get(&meeting_point).and_then(|&parent| parent);
    while let Some(node) = current {
        path.push(node);
        current = backward_visited.get(&node).and_then(|&parent| parent);
    }
    
    // Combine paths
    forward_path.extend(path);
    forward_path
}
```

## Time Complexity Advantage

**Single-directional BFS**: $O(b^d)$ where $b$ is branching factor, $d$ is depth

**Bidirectional BFS**: $O(b^{d/2} + b^{d/2}) = O(2 \cdot b^{d/2})$

**Example**: For $b=10$, $d=6$:
- Single: $10^6 = 1,000,000$ nodes explored
- Bidirectional: $2 \cdot 10^3 = 2,000$ nodes explored (500x improvement!)

## Key Insights

1. **Exponential Savings**: The branching factor compounds less when exploring half the depth from both ends
2. **Meeting Point Detection**: Critical to efficiently check when frontiers intersect
3. **Path Reconstruction**: Must stitch together forward and backward paths correctly
4. **Directed Graphs**: Backward search requires reverse edges (graph transposition)
5. **Weighted Graphs**: Bidirectional Dijkstra is more complex (must find *optimal* meeting point)

## When to Use Bidirectional Search

✅ **Good Cases**:
- Known start and goal nodes (not exploring all reachable nodes)
- Large branching factor (exponential savings matter)
- Uniform cost edges (BFS-based algorithms)
- Undirected graphs or graphs with reverse edges available

❌ **Problematic Cases**:
- Unknown goal (can't search backward)
- Small graphs (overhead not worth it)
- Highly non-uniform edge costs (meeting point may not be optimal)
- One direction much more efficient than the other

## Bidirectional A* (More Complex)

```rust
// Bidirectional A* requires careful handling of meeting conditions
fn bidirectional_astar<G, H>(
    graph: &G,
    start: NodeId,
    goal: NodeId,
    heuristic_forward: H,
    heuristic_backward: H,
) -> Option<(Vec<NodeId>, f64)>
where
    G: Graph,
    H: Fn(NodeId) -> f64,
{
    // Two priority queues (one for each direction)
    let mut forward_pq = BinaryHeap::new();
    let mut backward_pq = BinaryHeap::new();
    
    let mut forward_g_scores = HashMap::new();
    let mut backward_g_scores = HashMap::new();
    
    let mut forward_came_from = HashMap::new();
    let mut backward_came_from = HashMap::new();
    
    forward_g_scores.insert(start, 0.0);
    backward_g_scores.insert(goal, 0.0);
    
    forward_pq.push(SearchNode::new(start, heuristic_forward(start)));
    backward_pq.push(SearchNode::new(goal, heuristic_backward(goal)));
    
    let mut best_path_cost = f64::INFINITY;
    let mut best_meeting_point = None;
    
    // Continue until both queues empty or proven optimal
    while !forward_pq.is_empty() || !backward_pq.is_empty() {
        // Expand forward
        if let Some(current) = forward_pq.pop() {
            // Early termination: lower bound exceeds best found
            if current.f_score >= best_path_cost {
                continue;
            }
            
            for (neighbor, cost) in graph.neighbors_with_cost(current.node) {
                let tentative_g = forward_g_scores[&current.node] + cost;
                
                if tentative_g < *forward_g_scores.get(&neighbor).unwrap_or(&f64::INFINITY) {
                    forward_came_from.insert(neighbor, current.node);
                    forward_g_scores.insert(neighbor, tentative_g);
                    forward_pq.push(SearchNode::new(
                        neighbor,
                        tentative_g + heuristic_forward(neighbor),
                    ));
                    
                    // Check if backward search has visited this node
                    if let Some(&backward_g) = backward_g_scores.get(&neighbor) {
                        let total_cost = tentative_g + backward_g;
                        if total_cost < best_path_cost {
                            best_path_cost = total_cost;
                            best_meeting_point = Some(neighbor);
                        }
                    }
                }
            }
        }
        
        // Expand backward (similar logic with reversed graph)
        // ... (omitted for brevity)
    }
    
    best_meeting_point.map(|meeting| {
        let path = reconstruct_bidirectional_path(
            meeting,
            &forward_came_from,
            &backward_came_from,
        );
        (path, best_path_cost)
    })
}
```

## Mission 9 Implementation

[[mission-9]] includes production-quality bidirectional search:
- Bidirectional Dijkstra for unweighted shortest paths
- Bidirectional A* with proper meeting point detection
- Early termination when lower bounds exceed best path found
- Reverse graph construction for directed graphs

## Real-World Applications

1. **GPS Navigation**: Finding routes between two known locations
2. **Social Networks**: Finding shortest connection between two users
3. **Game AI**: Pathfinding when start and goal are known
4. **Network Routing**: Finding paths between two network nodes

## Relationship to Other Algorithms

- **[[bfs-breadth-first-search]]** - Bidirectional BFS uses two simultaneous BFS searches
- **[[dijkstra-algorithm]]** - Bidirectional Dijkstra extends this optimization to weighted graphs
- **[[a-star-algorithm]]** - Bidirectional A* requires careful heuristic handling
- **[[mission-8]]** - Basic graph traversal implementation foundation
- **[[mission-9]]** - Advanced pathfinding with bidirectional optimizations

## Common Pitfalls

1. **Meeting Point Not Optimal**: In weighted graphs, first meeting may not be optimal path
2. **Incorrect Path Reconstruction**: Must properly reverse one half of the path
3. **Directed Graph Issues**: Backward search needs reverse edges
4. **Heuristic Consistency**: In A*, both heuristics must be admissible and consistent
5. **Premature Termination**: Must ensure optimality before stopping search

## Performance Characteristics

- **Best Case**: $O(b^{d/2})$ - balanced exploration from both ends
- **Worst Case**: $O(b^d)$ - if one direction explores entire graph before meeting
- **Space Complexity**: $O(b^{d/2})$ - must store both frontiers and visited sets
- **Practical Speedup**: Often 10-1000x faster than single-directional search

---

*Tags: #algorithms #graph-traversal #optimization #bidirectional #pathfinding #search-algorithms #mission-9*

*Links: [[bfs-breadth-first-search]] • [[dijkstra-algorithm]] • [[a-star-algorithm]] • [[mission-8]] • [[mission-9]] • [[graph-search-strategies]]*
