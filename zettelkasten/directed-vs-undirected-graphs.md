# Directed vs Undirected Graphs: Edge Counting Demystified

*Created: 2025-10-10*  
*Context: Common confusion about edge counting in undirected graphs*  
*Status: Fundamental graph theory concept*

---

## 🎯 The Core Question

**"Why does adding X <-> Y (undirected) show as 1 edge, but X→Y + Y→X (directed) shows as 2 edges?"**

This is one of the most common points of confusion when learning graph theory. Let's break it down completely.

---

## 📊 Quick Visual Comparison

```
UNDIRECTED GRAPH:
X ━━━ Y
  ONE bidirectional edge
  Edge count: 1

DIRECTED GRAPH (bidirectional):
    ↗
X     Y
    ↖
  TWO unidirectional edges
  Edge count: 2
```

---

## 🔄 Undirected Graphs: ONE Edge, TWO Directions

### **Definition**

An **undirected edge** is a **single connection** that can be traversed in **both directions**.

```
X <-> Y  =  ONE undirected edge
```

### **Properties**

- **Symmetric relationship**: If X connects to Y, then Y connects to X (automatically)
- **No direction**: The edge has no "from" or "to" - it's just "connected"
- **Single entity**: Conceptually one object in graph theory
- **Edge count = 1**: Counted once, regardless of adjacency list storage

### **Real-World Examples**

```
Friendship Graph:
- Alice ━━━ Bob  (1 edge: they're friends, no direction needed)

Road Network (bidirectional roads):
- CityA ━━━ CityB  (1 road connecting both cities)

Computer Network (Ethernet):
- Computer1 ━━━ Computer2  (1 cable, data flows both ways)

Social Media (mutual follow):
- UserX ━━━ UserY  (1 mutual connection)
```

### **Adjacency List Representation**

**Key insight**: Storage ≠ Counting

```rust
// Adding ONE undirected edge X <-> Y
graph.add_undirected_edge(x, y);

// Internal storage (adjacency list):
X's neighbors: [Y]  // Store Y in X's list
Y's neighbors: [X]  // Store X in Y's list

// Edge count: 1 (conceptual)
// Why stored twice? For O(1) lookup efficiency!
```

**Why store in both lists?**
- To check "Can I go from X to Y?": Look in X's neighbor list → O(1)
- To check "Can I go from Y to X?": Look in Y's neighbor list → O(1)
- It's **ONE edge** stored in **TWO places** for performance

### **Edge Counting Implementation**

```rust
impl<T> Graph<T> {
    /// Counts edges in the graph.
    /// For undirected graphs, each undirected edge counts as 1.
    pub fn edge_count(&self) -> usize {
        // Sum all adjacency list lengths
        let total_entries = self.adjacency
            .iter()
            .map(|neighbors| neighbors.len())
            .sum::<usize>();
        
        if self.is_undirected {
            // Each undirected edge appears twice in adjacency lists
            // Divide by 2 to get the true edge count
            total_entries / 2
        } else {
            // Directed: each entry is a separate edge
            total_entries
        }
    }
}
```

---

## ➡️ Directed Graphs: TWO Edges for Bidirectional

### **Definition**

A **directed edge** has a **specific direction**: from one node to another.

```
X -> Y  =  ONE directed edge (from X to Y)
Y -> X  =  ANOTHER directed edge (from Y to X)
```

To achieve **bidirectional connectivity**, you need **TWO separate directed edges**.

### **Properties**

- **Asymmetric relationship**: X→Y does NOT imply Y→X
- **Has direction**: Clear "from" (source) and "to" (destination)
- **Independent edges**: X→Y and Y→X are completely separate entities
- **Edge count = 2**: Each direction is counted separately

### **Real-World Examples**

```
Twitter Follow Graph:
- Alice → Bob  (Alice follows Bob)
- Bob → Alice  (Bob follows Alice)
  These are TWO separate edges (2 follow actions)

One-Way Streets:
- StreetA → StreetB  (can drive A to B)
- StreetB → StreetA  (can drive B to A, different street)

Hyperlinks:
- PageX → PageY  (PageX links to PageY)
- PageY → PageX  (PageY links back to PageX)
  These are TWO separate hyperlinks

Debt:
- Person1 → Person2  (Person1 owes Person2 money)
- Person2 → Person1  (Person2 owes Person1 money)
  These are TWO separate debts
```

### **Adjacency List Representation**

```rust
// Adding TWO directed edges for bidirectional connection
graph.add_edge(x, y);  // First edge: X → Y
graph.add_edge(y, x);  // Second edge: Y → X

// Internal storage (adjacency list):
X's neighbors: [Y]  // X can reach Y
Y's neighbors: [X]  // Y can reach X

// Edge count: 2 (both directions)
```

### **Edge Counting Implementation**

```rust
impl<T> Graph<T> {
    pub fn edge_count(&self) -> usize {
        // For directed graphs: just count all entries
        self.adjacency
            .iter()
            .map(|neighbors| neighbors.len())
            .sum()
    }
}
```

---

## 🆚 Side-by-Side Comparison

### **Conceptual Level (Graph Theory)**

| Aspect | Undirected Graph | Directed Graph |
|--------|------------------|----------------|
| **Edge Representation** | X ━━━ Y | X → Y (separate from Y → X) |
| **Relationship Type** | Symmetric | Asymmetric |
| **Bidirectional Connection** | ONE edge (inherently bidirectional) | TWO edges (one per direction) |
| **Edge Count (X ↔ Y)** | **1** | **2** |
| **Notation** | {X, Y} or X—Y | (X, Y) for X→Y, (Y, X) for Y→X |

### **Implementation Level (Adjacency List)**

| Aspect | Undirected Graph | Directed Graph |
|--------|------------------|----------------|
| **Storage for X ↔ Y** | X: [Y], Y: [X] | X: [Y], Y: [X] |
| **Storage Entries** | 2 entries | 2 entries |
| **Edge Count Calculation** | `total_entries / 2` | `total_entries` |
| **Result** | **1 edge** | **2 edges** |

### **Performance Characteristics**

| Operation | Undirected | Directed | Complexity |
|-----------|------------|----------|------------|
| **Add Edge** | Update both adjacency lists | Update source list | O(1) amortized |
| **Remove Edge** | Update both adjacency lists | Update source list | O(degree) |
| **Check Edge Exists** | Check one adjacency list | Check source list | O(degree) |
| **Get Neighbors** | Return adjacency list | Return adjacency list | O(1) |
| **Count Edges** | `sum(degrees) / 2` | `sum(degrees)` | O(V) |

---

## 🧮 Edge Counting Examples

### **Example 1: Simple Undirected Graph**

```rust
let mut g = Graph::new_undirected();
let a = g.add_node("A");
let b = g.add_node("B");
let c = g.add_node("C");

g.add_undirected_edge(a, b);  // Edge 1: A <-> B
g.add_undirected_edge(b, c);  // Edge 2: B <-> C
g.add_undirected_edge(c, a);  // Edge 3: C <-> A

// Adjacency lists:
// A: [B, C]  (2 entries)
// B: [A, C]  (2 entries)
// C: [B, A]  (2 entries)
// Total entries: 6

// Edge count = 6 / 2 = 3 ✅
println!("Edges: {}", g.edge_count());  // Output: 3
```

**Visual:**
```
    A
   / \
  /   \
 B ━━━ C

3 undirected edges
```

### **Example 2: Directed Graph (Cycle)**

```rust
let mut g = Graph::new();
let a = g.add_node("A");
let b = g.add_node("B");
let c = g.add_node("C");

g.add_edge(a, b);  // Edge 1: A → B
g.add_edge(b, c);  // Edge 2: B → C
g.add_edge(c, a);  // Edge 3: C → A

// Adjacency lists:
// A: [B]  (1 entry)
// B: [C]  (1 entry)
// C: [A]  (1 entry)
// Total entries: 3

// Edge count = 3 ✅
println!("Edges: {}", g.edge_count());  // Output: 3
```

**Visual:**
```
    A
    ↓
    B
    ↓
    C
    ↓
   (A)

3 directed edges (cycle)
```

### **Example 3: Directed Graph (Bidirectional)**

```rust
let mut g = Graph::new();
let x = g.add_node("X");
let y = g.add_node("Y");

g.add_edge(x, y);  // Edge 1: X → Y
g.add_edge(y, x);  // Edge 2: Y → X

// Adjacency lists:
// X: [Y]  (1 entry)
// Y: [X]  (1 entry)
// Total entries: 2

// Edge count = 2 ✅
println!("Edges: {}", g.edge_count());  // Output: 2
```

**Visual:**
```
    ↗
X     Y
    ↖

2 directed edges (bidirectional)
```

### **Example 4: Mixed (Some Bidirectional)**

```rust
let mut g = Graph::new();
let a = g.add_node("A");
let b = g.add_node("B");
let c = g.add_node("C");

g.add_edge(a, b);  // Edge 1: A → B
g.add_edge(b, a);  // Edge 2: B → A (bidirectional with edge 1)
g.add_edge(b, c);  // Edge 3: B → C (unidirectional)

// Adjacency lists:
// A: [B]     (1 entry)
// B: [A, C]  (2 entries)
// C: []      (0 entries)
// Total entries: 3

// Edge count = 3 ✅
println!("Edges: {}", g.edge_count());  // Output: 3
```

**Visual:**
```
    ↗
A     B → C
    ↖

3 directed edges
(A and B are bidirectionally connected via 2 edges)
```

---

## 🤔 Common Confusions Explained

### **Confusion 1: "Why only 1 edge when stored in 2 places?"**

**Question:**
```
Undirected graph: X <-> Y
X's neighbors: [Y]  ← One entry
Y's neighbors: [X]  ← Another entry
Why edge count = 1 and not 2?
```

**Answer:**
- **Storage** (implementation detail) ≠ **Count** (conceptual meaning)
- We store in both places for **lookup efficiency** (O(1) instead of O(E))
- Conceptually, there's **ONE undirected edge** between X and Y
- The graph structure itself defines counting rules, not the storage method

**Analogy:**
```
Phone contact list:
- Alice has Bob's number
- Bob has Alice's number
How many friendships? ONE (stored in 2 phones for convenience)
```

### **Confusion 2: "Website says bidirectional needs 2 edges in directed graph"**

**Website quote:**
> "In a directed graph, this would be represented by two directed edges: one from A to B and another from B to A."

**This is 100% correct!** Key word: **"directed graph"**

```
Directed graph for X ↔ Y:
- Need X → Y (edge 1)
- Need Y → X (edge 2)
- Total: 2 edges

Undirected graph for X ↔ Y:
- ONE edge X—Y (inherently bidirectional)
- Total: 1 edge
```

### **Confusion 3: "Same adjacency list, different counts?"**

**Observation:**
```
Both graphs show:
X's neighbors: [Y]
Y's neighbors: [X]

But undirected counts as 1 edge, directed counts as 2!
```

**Explanation:**
- The **graph type** (directed vs undirected) determines **interpretation**
- Same data structure, different **semantics**
- Adjacency list is just **storage**, not the definition of edges

**Code Example:**
```rust
// Same adjacency structure, different edge counts
let adj_x = vec![y];  // X's neighbors
let adj_y = vec![x];  // Y's neighbors

// Undirected interpretation:
let undirected_edges = (adj_x.len() + adj_y.len()) / 2;  // = 1

// Directed interpretation:
let directed_edges = adj_x.len() + adj_y.len();  // = 2
```

### **Confusion 4: "How to convert between directed and undirected?"**

```rust
// Undirected → Directed (replace 1 edge with 2 edges)
fn undirected_to_directed(undirected: &Graph) -> Graph {
    let mut directed = Graph::new();
    
    for (u, v) in undirected.edges() {
        directed.add_edge(u, v);  // Add forward edge
        directed.add_edge(v, u);  // Add backward edge
    }
    // Edge count doubles!
    directed
}

// Directed → Undirected (merge bidirectional pairs into 1 edge)
fn directed_to_undirected(directed: &Graph) -> Graph {
    let mut undirected = Graph::new_undirected();
    
    for (u, v) in directed.edges() {
        undirected.add_undirected_edge(u, v);
    }
    // Edge count may decrease (if bidirectional pairs exist)
    undirected
}
```

---

## 📐 Mathematical Definitions

### **Undirected Graph**

```
G = (V, E)

where:
  V = set of vertices (nodes)
  E ⊆ {{u, v} | u, v ∈ V}  (unordered pairs)

Example:
  V = {A, B, C}
  E = {{A, B}, {B, C}}  ← 2 edges (unordered sets)

Edge {A, B} = {B, A}  (same edge, no direction)
```

### **Directed Graph (Digraph)**

```
G = (V, E)

where:
  V = set of vertices (nodes)
  E ⊆ {(u, v) | u, v ∈ V}  (ordered pairs)

Example:
  V = {A, B, C}
  E = {(A, B), (B, C)}  ← 2 edges (ordered pairs)

Edge (A, B) ≠ (B, A)  (different edges, has direction)
```

### **Maximum Edge Counts**

```
Complete Undirected Graph:
  Max edges = |V| × (|V| - 1) / 2
  Example: 4 nodes → 4 × 3 / 2 = 6 edges

Complete Directed Graph:
  Max edges = |V| × (|V| - 1)
  Example: 4 nodes → 4 × 3 = 12 edges

Note: Directed graph can have twice as many edges!
```

---

## 🎯 When to Use Which?

### **Use Undirected Graphs When:**

✅ **Relationships are symmetric**
- Social networks (mutual friendships)
- Computer networks (Ethernet cables)
- Road networks (two-way streets)
- Molecule bonds (atoms connected both ways)

✅ **No direction matters**
- Geographic distance (A to B = B to A)
- Similarity measures (item1 similar to item2)
- Collaboration networks (co-authors)

✅ **Modeling physical connections**
- Electrical circuits
- Neural networks (some models)
- Infrastructure networks

### **Use Directed Graphs When:**

✅ **Relationships are asymmetric**
- Twitter follows (Alice follows Bob ≠ Bob follows Alice)
- Web hyperlinks (PageA links to PageB)
- Dependency graphs (Package A depends on B)
- Debt networks (Who owes whom)

✅ **Direction matters**
- Traffic flow (one-way streets)
- Data flow (pipelines, streams)
- Causal relationships (A causes B)
- Workflow sequences (task dependencies)

✅ **Modeling directed processes**
- State machines (transitions)
- Food chains (predator-prey)
- Influence propagation (information spread)

---

## 💻 Implementation Patterns

### **Pattern 1: Add Undirected Edge (Helper Method)**

```rust
impl<T> Graph<T> {
    /// Adds an undirected edge between two nodes.
    /// Internally adds TWO directed edges for bidirectional traversal.
    /// Edge count increases by 1 (conceptually one undirected edge).
    pub fn add_undirected_edge(&mut self, from: NodeId, to: NodeId) -> bool {
        if !self.is_valid_node(from) || !self.is_valid_node(to) {
            return false;
        }
        
        // Add both directions
        let forward = self.add_directed_edge_internal(from, to);
        let backward = self.add_directed_edge_internal(to, from);
        
        forward && backward
    }
    
    fn add_directed_edge_internal(&mut self, from: NodeId, to: NodeId) -> bool {
        let neighbors = &mut self.adjacency[from];
        if !neighbors.contains(&to) {
            neighbors.push(to);
            true
        } else {
            false
        }
    }
}
```

### **Pattern 2: Remove Undirected Edge**

```rust
impl<T> Graph<T> {
    /// Removes an undirected edge.
    /// Must remove from BOTH adjacency lists.
    pub fn remove_undirected_edge(&mut self, from: NodeId, to: NodeId) -> bool {
        let forward = self.remove_directed_edge_internal(from, to);
        let backward = self.remove_directed_edge_internal(to, from);
        forward && backward
    }
    
    fn remove_directed_edge_internal(&mut self, from: NodeId, to: NodeId) -> bool {
        let neighbors = &mut self.adjacency[from];
        if let Some(pos) = neighbors.iter().position(|&n| n == to) {
            neighbors.remove(pos);
            true
        } else {
            false
        }
    }
}
```

### **Pattern 3: Check if Graph is Undirected (Validation)**

```rust
impl<T> Graph<T> {
    /// Checks if the graph is truly undirected.
    /// Returns true if every edge u→v has a corresponding v→u.
    pub fn is_undirected(&self) -> bool {
        for (u, neighbors) in self.adjacency.iter().enumerate() {
            for &v in neighbors {
                // Check if reverse edge exists
                if !self.adjacency[v].contains(&u) {
                    return false;  // Found asymmetric edge
                }
            }
        }
        true
    }
}
```

---

## 🧪 Testing Edge Counting

### **Test 1: Undirected Edge Count**

```rust
#[test]
fn undirected_edge_count() {
    let mut g = Graph::new_undirected();
    let a = g.add_node("A");
    let b = g.add_node("B");
    let c = g.add_node("C");
    
    assert_eq!(g.edge_count(), 0);
    
    g.add_undirected_edge(a, b);
    assert_eq!(g.edge_count(), 1);  // ONE undirected edge
    
    g.add_undirected_edge(b, c);
    assert_eq!(g.edge_count(), 2);  // TWO undirected edges
    
    g.add_undirected_edge(c, a);
    assert_eq!(g.edge_count(), 3);  // THREE undirected edges
    
    // Verify adjacency lists have double the entries
    let total_entries: usize = g.adjacency.iter()
        .map(|n| n.len())
        .sum();
    assert_eq!(total_entries, 6);  // 3 edges × 2 entries each
}
```

### **Test 2: Directed Edge Count**

```rust
#[test]
fn directed_edge_count() {
    let mut g = Graph::new();
    let a = g.add_node("A");
    let b = g.add_node("B");
    let c = g.add_node("C");
    
    assert_eq!(g.edge_count(), 0);
    
    g.add_edge(a, b);
    assert_eq!(g.edge_count(), 1);  // ONE directed edge
    
    g.add_edge(b, a);
    assert_eq!(g.edge_count(), 2);  // TWO directed edges (bidirectional)
    
    g.add_edge(b, c);
    assert_eq!(g.edge_count(), 3);  // THREE directed edges
}
```

### **Test 3: Comparison Test**

```rust
#[test]
fn directed_vs_undirected_edge_count() {
    // Same nodes, same connections, different interpretation
    
    // Undirected version
    let mut undirected = Graph::new_undirected();
    let a1 = undirected.add_node("A");
    let b1 = undirected.add_node("B");
    undirected.add_undirected_edge(a1, b1);
    
    // Directed version (bidirectional)
    let mut directed = Graph::new();
    let a2 = directed.add_node("A");
    let b2 = directed.add_node("B");
    directed.add_edge(a2, b2);
    directed.add_edge(b2, a2);
    
    // Undirected: 1 edge
    assert_eq!(undirected.edge_count(), 1);
    
    // Directed: 2 edges
    assert_eq!(directed.edge_count(), 2);
    
    // Both have same adjacency list structure
    assert_eq!(undirected.adjacency[a1], directed.adjacency[a2]);
    assert_eq!(undirected.adjacency[b1], directed.adjacency[b2]);
}
```

---

## 📚 Real-World Applications

### **Undirected Graph Examples**

**Social Network (Facebook Friends):**
```rust
// Friendship is mutual (undirected)
let mut friends = Graph::new_undirected();
let alice = friends.add_node("Alice");
let bob = friends.add_node("Bob");
let charlie = friends.add_node("Charlie");

friends.add_undirected_edge(alice, bob);     // 1 friendship
friends.add_undirected_edge(bob, charlie);   // 1 friendship
friends.add_undirected_edge(charlie, alice); // 1 friendship

println!("Total friendships: {}", friends.edge_count());  // 3
```

**Computer Network:**
```rust
// Ethernet cables are bidirectional (undirected)
let mut network = Graph::new_undirected();
let server = network.add_node("Server");
let router = network.add_node("Router");
let pc1 = network.add_node("PC1");
let pc2 = network.add_node("PC2");

network.add_undirected_edge(server, router);  // 1 cable
network.add_undirected_edge(router, pc1);     // 1 cable
network.add_undirected_edge(router, pc2);     // 1 cable

println!("Total cables: {}", network.edge_count());  // 3
```

### **Directed Graph Examples**

**Twitter Follow Graph:**
```rust
// Following is directional (directed)
let mut twitter = Graph::new();
let alice = twitter.add_node("@alice");
let bob = twitter.add_node("@bob");
let charlie = twitter.add_node("@charlie");

twitter.add_edge(alice, bob);     // Alice follows Bob (1 edge)
twitter.add_edge(bob, alice);     // Bob follows Alice (1 edge)
twitter.add_edge(charlie, alice); // Charlie follows Alice (1 edge)
// Note: Alice and Bob mutual follow = 2 separate edges

println!("Total follows: {}", twitter.edge_count());  // 3
```

**Web Link Graph:**
```rust
// Hyperlinks are directional (directed)
let mut web = Graph::new();
let home = web.add_node("Home");
let about = web.add_node("About");
let contact = web.add_node("Contact");

web.add_edge(home, about);      // Home → About (1 link)
web.add_edge(home, contact);    // Home → Contact (1 link)
web.add_edge(about, home);      // About → Home (1 link)
// Note: Home and About link to each other = 2 separate links

println!("Total links: {}", web.edge_count());  // 3
```

---

## 🎓 Key Takeaways

### **For Learners**

1. **Graph type determines edge counting**
   - Undirected: X <-> Y = **1 edge**
   - Directed: X → Y + Y → X = **2 edges**

2. **Storage ≠ Counting**
   - Adjacency list entries ≠ edge count
   - Undirected edges stored twice for efficiency
   - Always divide by 2 for undirected graphs

3. **Both directions, different meanings**
   - Undirected: ONE edge that goes both ways
   - Directed: TWO edges that happen to point at each other

4. **Real-world analogy**
   - Undirected: Friendship (1 mutual relationship)
   - Directed: Twitter follow (2 separate follow actions)

### **For Implementers**

1. **Separate internal storage from conceptual model**
   ```rust
   // Internal: adjacency list (storage)
   // External: edge count (concept)
   ```

2. **Use helper methods for undirected graphs**
   ```rust
   add_undirected_edge()  // Maintains symmetry
   remove_undirected_edge()  // Removes from both lists
   ```

3. **Document your counting method**
   ```rust
   /// Returns the number of edges.
   /// For undirected graphs, counts each edge once (not twice).
   pub fn edge_count(&self) -> usize { ... }
   ```

4. **Validate undirected invariants**
   ```rust
   // Ensure every u→v has corresponding v→u
   debug_assert!(self.is_undirected());
   ```

---

## 🔗 Related Concepts

- [[graph-representations]] - Adjacency list vs adjacency matrix
- [[graph-traversal]] - BFS/DFS work same for both types
- [[graph-algorithms]] - Some algorithms specific to directed (topological sort)
- [[mission7-graph-implementation]] - Our Graph<T> implementation
- [[adjacency-list-performance]] - Why store edges twice in undirected graphs

---

## 📖 Further Reading

**Books:**
- "Introduction to Algorithms" (CLRS) - Chapter 22: Elementary Graph Algorithms
- "Algorithm Design Manual" (Skiena) - Section 5.1: Flavors of Graphs

**Online Resources:**
- [Graph Theory - Directed vs Undirected](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics))
- [Adjacency List Representation](https://www.geeksforgeeks.org/graph-and-its-representations/)

**AoC Problems:**
- 2015 Day 9: Traveling Salesman (undirected, complete graph)
- 2018 Day 7: Dependencies (directed acyclic graph)
- 2019 Day 6: Orbits (directed tree)

---

*Tags: #graph-theory #directed-graph #undirected-graph #edge-counting #data-structures #mission7 #concept*

*Links: [[zettel-index]] | [[mission7-graph-implementation]] | [[graph-traversal]] | [[adjacency-list]] | [[Missions Overview]] | [[AoC Patterns MOC]] | [[mission-7]]*

---

**Remember:** The graph type determines how we **interpret** the same adjacency list structure. Same storage, different semantics! 🎯
