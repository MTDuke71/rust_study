//! # Mission 10 Tutorial - Step 7: Problem Solving Patterns
//! 
//! This final step teaches you how to recognize and solve Union-Find problems
//! in interviews and competitive programming. We'll work through classic problems
//! with step-by-step solutions and pattern recognition techniques.
//! 
//! ## Learning Objectives
//! 
//! After completing this step, you will:
//! - Recognize when Union-Find is the right approach
//! - Understand common problem patterns and keywords
//! - Know how to adapt Union-Find for different scenarios
//! - Be able to solve interview problems independently
//! - Understand time/space complexity trade-offs
//! 
//! ## Problems Covered
//! 
//! ### Classic Problems (3)
//! 1. **Number of Islands** - Grid connectivity
//! 2. **Friend Circles** - Social network components  
//! 3. **Redundant Connection** - Cycle detection
//! 
//! ### Intermediate Problems (4)
//! 4. **Accounts Merge** - String-based connectivity
//! 5. **Most Stones Removed** - Coordinate-based sets
//! 6. **Satisfiability of Equality Equations** - Constraint satisfaction
//! 7. **Smallest String With Swaps** - Lexicographical optimization
//! 
//! ### Advanced Problems (3)
//! 8. **Number of Islands II** - Dynamic connectivity
//! 9. **Evaluate Division** - Weighted Union-Find
//! 10. **Checking Edge Length Limited Paths** - Offline queries
//! 
//! ## Run This Example
//! 
//! ```bash
//! cd tutorials/Mission10_tut
//! cargo run --example step7_problem_solving
//! ```

use std::collections::{HashMap, HashSet};

/// Union-Find implementation for problem solving (optimized)
#[derive(Debug, Clone)]
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize, // Number of connected components
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: n,
        }
    }
    
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    
    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        
        if root_x == root_y {
            return false;
        }
        
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
    
    fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    
    fn count(&self) -> usize {
        self.count
    }
}

/// Demonstrates problem solving patterns and solutions
fn main() {
    println!("🧩 Mission 10 Tutorial - Step 7: Problem Solving Patterns");
    println!("{}", "=".repeat(70));
    
    show_pattern_recognition_guide();
    
    // Classic Problems
    solve_number_of_islands();
    solve_friend_circles();
    solve_redundant_connection();
    
    // Intermediate Problems  
    solve_accounts_merge();
    solve_most_stones_removed();
    solve_satisfiability_equations();
    solve_smallest_string_swaps();
    
    // Advanced Problems
    solve_number_of_islands_ii();
    solve_evaluate_division();
    solve_edge_length_limited_paths();
    
    show_interview_tips();
    show_practice_problems();
}

// ============================================================================
// PATTERN RECOGNITION GUIDE
// ============================================================================

/// Show comprehensive pattern recognition guide
fn show_pattern_recognition_guide() {
    println!("\n🎯 Pattern Recognition Guide");
    println!("{}", "=".repeat(50));
    
    println!("\n📋 When to Use Union-Find - Checklist:");
    println!("✓ Problem involves grouping/clustering elements");
    println!("✓ Need to track connected components dynamically");
    println!("✓ Queries about connectivity between elements");
    println!("✓ Cycle detection in undirected graphs");
    println!("✓ Merging sets based on relationships");
    println!("✓ \"Friends of friends\" type problems");
    println!("✓ Grid connectivity (islands, regions)");
    
    println!("\n🔍 Common Keywords in Problem Statements:");
    println!("• \"connected components\"");
    println!("• \"group\", \"cluster\", \"set\"");
    println!("• \"merge\", \"union\", \"combine\"");
    println!("• \"friends\", \"relationships\"");
    println!("• \"islands\", \"regions\"");
    println!("• \"cycle detection\"");
    println!("• \"dynamic connectivity\"");
    
    println!("\n⚖️  Union-Find vs Alternatives:");
    println!("Use Union-Find when:");
    println!("• Frequent union operations");
    println!("• Need to maintain components over time");
    println!("• Connectivity queries are common");
    
    println!("\nUse DFS/BFS when:");
    println!("• One-time traversal is sufficient");
    println!("• Need path information");
    println!("• Graph is static (no dynamic updates)");
    
    println!("\nUse Strongly Connected Components when:");
    println!("• Directed graphs");
    println!("• Need actual component structure");
    
    println!("\n🏗️ Common Implementation Patterns:");
    println!("1. **Direct mapping**: element index → Union-Find index");
    println!("2. **Coordinate mapping**: 2D grid (i,j) → 1D index");
    println!("3. **String/ID mapping**: Use HashMap to map strings to indices");
    println!("4. **Multiple Union-Find**: Separate structures for different constraints");
}

// ============================================================================
// CLASSIC PROBLEMS
// ============================================================================

/// LeetCode 200: Number of Islands
/// Pattern: Grid connectivity, 2D to 1D mapping
fn solve_number_of_islands() {
    println!("\n🏝️  Problem 1: Number of Islands (LeetCode 200)");
    println!("{}", "-".repeat(50));
    
    println!("📝 Problem Statement:");
    println!("Given a 2D grid of '1's (land) and '0's (water), count the number");
    println!("of islands. An island is surrounded by water and formed by connecting");
    println!("adjacent lands horizontally or vertically.");
    
    println!("\n💡 Approach:");
    println!("1. Convert 2D grid to 1D Union-Find indices");
    println!("2. Union adjacent land cells ('1')");
    println!("3. Count distinct components of land cells");
    
    // Example grid
    let grid = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    
    println!("\n🗺️  Example Grid:");
    for row in &grid {
        println!("    {:?}", row);
    }
    
    let result = count_islands(&grid);
    println!("\n✅ Result: {} islands", result);
    
    println!("\n⏱️  Time Complexity: O(m*n*α(m*n)) where m,n are grid dimensions");
    println!("💾 Space Complexity: O(m*n) for Union-Find structure");
    
    println!("\n🔧 Implementation Details:");
    println!("• Map 2D coordinate (i,j) to 1D index: i*cols + j");
    println!("• Only process land cells ('1')");
    println!("• Union with adjacent land cells (up, down, left, right)");
    println!("• Count components by counting distinct roots of land cells");
}

fn count_islands(grid: &[Vec<char>]) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }
    
    let rows = grid.len();
    let cols = grid[0].len();
    let mut uf = UnionFind::new(rows * cols);
    
    // Union adjacent land cells
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '1' {
                let current = i * cols + j;
                
                // Check right neighbor
                if j + 1 < cols && grid[i][j + 1] == '1' {
                    uf.union(current, current + 1);
                }
                
                // Check bottom neighbor
                if i + 1 < rows && grid[i + 1][j] == '1' {
                    uf.union(current, current + cols);
                }
            }
        }
    }
    
    // Count unique components of land cells
    let mut roots = HashSet::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '1' {
                roots.insert(uf.find(i * cols + j));
            }
        }
    }
    
    roots.len() as i32
}

/// LeetCode 547: Friend Circles (Number of Provinces)  
/// Pattern: Adjacency matrix, social network
fn solve_friend_circles() {
    println!("\n👥 Problem 2: Friend Circles (LeetCode 547)");
    println!("{}", "-".repeat(50));
    
    println!("📝 Problem Statement:");
    println!("Given an n x n matrix M where M[i][j] = 1 if persons i and j are");
    println!("direct friends, find the total number of friend circles (groups).");
    
    println!("\n💡 Approach:");
    println!("1. Each person is an element in Union-Find");
    println!("2. Union people who are direct friends (M[i][j] = 1)");
    println!("3. Count total connected components");
    
    // Example friendship matrix
    #[rustfmt::skip]
    let friendships = vec![
        vec![1, 1, 0],
        vec![1, 1, 0], 
        vec![0, 0, 1],
    ];
    
    println!("\n🤝 Example Friendship Matrix:");
    println!("   Person: 0 1 2");
    for (i, row) in friendships.iter().enumerate() {
        println!("Person {}: {:?}", i, row);
    }
    
    let result = count_friend_circles(&friendships);
    println!("\n✅ Result: {} friend circles", result);
    println!("   Groups: {{0, 1}}, {{2}}");
    
    println!("\n⏱️  Time Complexity: O(n²*α(n)) where n is number of people");
    println!("💾 Space Complexity: O(n) for Union-Find structure");
    
    println!("\n🔧 Implementation Details:");
    println!("• Each person maps directly to Union-Find index");
    println!("• Process upper triangle of matrix (avoid duplicate unions)");
    println!("• M[i][i] = 1 always (person is friend with themselves)");
    println!("• Final component count gives number of friend circles");
}

fn count_friend_circles(friendships: &[Vec<i32>]) -> i32 {
    let n = friendships.len();
    let mut uf = UnionFind::new(n);
    
    // Union direct friends
    for (i, friendship_row) in friendships.iter().enumerate() {
        for (j, &is_friend) in friendship_row.iter().enumerate().skip(i + 1) {
            if is_friend == 1 {
                uf.union(i, j);
            }
        }
    }
    
    uf.count() as i32
}

/// LeetCode 684: Redundant Connection
/// Pattern: Cycle detection in undirected graph
fn solve_redundant_connection() {
    println!("\n🔄 Problem 3: Redundant Connection (LeetCode 684)");
    println!("{}", "-".repeat(50));
    
    println!("📝 Problem Statement:");
    println!("Given a tree with one additional edge (forming a cycle), find the edge");
    println!("that can be removed to make it a tree again. Return the last edge in");
    println!("the input that creates the cycle.");
    
    println!("\n💡 Approach:");
    println!("1. Process edges one by one");
    println!("2. Before adding edge, check if endpoints are already connected");
    println!("3. If connected, this edge creates a cycle - return it");
    println!("4. Otherwise, union the endpoints");
    
    let edges = vec![
        vec![1, 2],
        vec![1, 3], 
        vec![2, 3],
    ];
    
    println!("\n🔗 Example Edges: {:?}", edges);
    println!("   Forms cycle: 1-2-3-1");
    
    let result = find_redundant_connection(&edges);
    println!("\n✅ Result: Redundant edge {:?}", result);
    
    println!("\n⏱️  Time Complexity: O(n*α(n)) where n is number of edges");
    println!("💾 Space Complexity: O(n) for Union-Find structure");
    
    println!("\n🔧 Implementation Details:");
    println!("• Process edges in order given");
    println!("• Check connectivity before union operation");
    println!("• First edge that connects already-connected nodes is answer");
    println!("• Node indices may start from 1 (adjust accordingly)");
}

fn find_redundant_connection(edges: &[Vec<i32>]) -> Vec<i32> {
    let n = edges.len();
    let mut uf = UnionFind::new(n + 1); // +1 since nodes are 1-indexed
    
    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        
        if uf.connected(u, v) {
            return edge.clone(); // This edge creates a cycle
        }
        
        uf.union(u, v);
    }
    
    vec![] // Should not reach here given valid input
}

// ============================================================================
// INTERMEDIATE PROBLEMS
// ============================================================================

/// LeetCode 721: Accounts Merge
/// Pattern: String-based connectivity, multiple keys per element
fn solve_accounts_merge() {
    println!("\n📧 Problem 4: Accounts Merge (LeetCode 721)");
    println!("{}", "-".repeat(50));
    
    println!("📝 Problem Statement:");
    println!("Given accounts with names and emails, merge accounts that belong");
    println!("to the same person (same name + overlapping emails).");
    
    println!("\n💡 Approach:");
    println!("1. Create Union-Find for account indices");
    println!("2. Use HashMap to track which account owns each email");
    println!("3. Union accounts that share any email");
    println!("4. Group emails by account root");
    
    let accounts = vec![
        vec!["John".to_string(), "johnsmith@mail.com".to_string(), "john_newyork@mail.com".to_string()],
        vec!["John".to_string(), "johnsmith@mail.com".to_string(), "john00@mail.com".to_string()],
        vec!["Mary".to_string(), "mary@mail.com".to_string()],
        vec!["John".to_string(), "johnnybravo@mail.com".to_string()],
    ];
    
    println!("\n👤 Example Accounts:");
    for (i, account) in accounts.iter().enumerate() {
        println!("  {}: {:?}", i, account);
    }
    
    let result = accounts_merge(&accounts);
    println!("\n✅ Result:");
    for merged in result {
        println!("  {:?}", merged);
    }
    
    println!("\n⏱️  Time Complexity: O(n*k*α(n)) where n=accounts, k=avg emails per account");
    println!("💾 Space Complexity: O(n*k) for HashMap and Union-Find");
}

fn accounts_merge(accounts: &[Vec<String>]) -> Vec<Vec<String>> {
    let n = accounts.len();
    let mut uf = UnionFind::new(n);
    let mut email_to_account: HashMap<String, usize> = HashMap::new();
    
    // Build email ownership map and union accounts with shared emails
    for (i, account) in accounts.iter().enumerate() {
        for email in account.iter().skip(1) {
            if let Some(&owner) = email_to_account.get(email) {
                uf.union(i, owner);
            } else {
                email_to_account.insert(email.clone(), i);
            }
        }
    }
    
    // Group emails by account root
    let mut root_to_emails: HashMap<usize, HashSet<String>> = HashMap::new();
    for (email, &account_idx) in &email_to_account {
        let root = uf.find(account_idx);
        root_to_emails.entry(root).or_default().insert(email.clone());
    }
    
    // Build result with sorted emails
    let mut result = Vec::new();
    for (&root, emails) in &root_to_emails {
        let mut account_emails: Vec<String> = emails.iter().cloned().collect();
        account_emails.sort();
        
        let mut merged_account = vec![accounts[root][0].clone()]; // Name
        merged_account.extend(account_emails);
        result.push(merged_account);
    }
    
    result
}

/// LeetCode 947: Most Stones Removed  
/// Pattern: Coordinate-based sets, shared row/column
fn solve_most_stones_removed() {
    println!("\n🪨 Problem 5: Most Stones Removed (LeetCode 947)");
    println!("{}", "-".repeat(50));
    
    println!("📝 Problem Statement:");
    println!("Stones on 2D plane. Remove a stone if it shares row OR column with");
    println!("another stone. Find maximum number of stones that can be removed.");
    
    println!("\n💡 Approach:");
    println!("1. Stones sharing row or column belong to same component");
    println!("2. From each component of size k, we can remove k-1 stones");
    println!("3. Answer = total_stones - number_of_components");
    
    let stones = vec![
        vec![0, 0],
        vec![0, 1], 
        vec![1, 0],
        vec![1, 2],
        vec![2, 1],
        vec![2, 2],
    ];
    
    println!("\n🎯 Example Stones: {:?}", stones);
    
    let result = remove_stones(&stones);
    println!("\n✅ Result: {} stones can be removed", result);
    
    println!("\n⏱️  Time Complexity: O(n*α(n)) where n is number of stones");
    println!("💾 Space Complexity: O(n) for Union-Find structure");
    
    println!("\n🔧 Implementation Details:");
    println!("• Union stones that share row OR column");
    println!("• Each connected component can be reduced to 1 stone");
    println!("• Maximum removals = total stones - components");
}

fn remove_stones(stones: &[Vec<i32>]) -> i32 {
    let n = stones.len();
    let mut uf = UnionFind::new(n);
    
    // Union stones that share row or column
    for i in 0..n {
        for j in i + 1..n {
            if stones[i][0] == stones[j][0] || stones[i][1] == stones[j][1] {
                uf.union(i, j);
            }
        }
    }
    
    n as i32 - uf.count() as i32
}

/// LeetCode 990: Satisfiability of Equality Equations
/// Pattern: Multiple Union-Find for constraints
fn solve_satisfiability_equations() {
    println!("\n⚖️  Problem 6: Satisfiability of Equality Equations (LeetCode 990)");
    println!("{}", "-".repeat(50));
    
    println!("📝 Problem Statement:");
    println!("Given equations like \"a==b\", \"b!=c\", determine if all equations");
    println!("can be satisfied simultaneously.");
    
    println!("\n💡 Approach:");
    println!("1. Process all equality constraints first (a==b)");
    println!("2. Then check inequality constraints (a!=b)");
    println!("3. If any inequality violates equality, return false");
    
    let equations = vec![
        "a==b".to_string(),
        "b!=a".to_string(),
    ];
    
    println!("\n📋 Example Equations: {:?}", equations);
    
    let result = equations_possible(&equations);
    println!("\n✅ Result: {} (equations are satisfiable)", result);
    
    println!("\n⏱️  Time Complexity: O(n*α(26)) ≈ O(n) where n is number of equations");
    println!("💾 Space Complexity: O(1) since only 26 variables maximum");
}

fn equations_possible(equations: &[String]) -> bool {
    let mut uf = UnionFind::new(26); // 26 letters
    
    // Process equality constraints first
    for eq in equations {
        if eq.contains("==") {
            let chars: Vec<char> = eq.chars().collect();
            let var1 = (chars[0] as u8 - b'a') as usize;
            let var2 = (chars[3] as u8 - b'a') as usize;
            uf.union(var1, var2);
        }
    }
    
    // Check inequality constraints
    for eq in equations {
        if eq.contains("!=") {
            let chars: Vec<char> = eq.chars().collect();
            let var1 = (chars[0] as u8 - b'a') as usize;
            let var2 = (chars[3] as u8 - b'a') as usize;
            if uf.connected(var1, var2) {
                return false; // Contradiction
            }
        }
    }
    
    true
}

/// LeetCode 1202: Smallest String With Swaps
/// Pattern: Lexicographical optimization within components
fn solve_smallest_string_swaps() {
    println!("\n🔤 Problem 7: Smallest String With Swaps (LeetCode 1202)");
    println!("{}", "-".repeat(50));
    
    println!("📝 Problem Statement:");
    println!("Given string and list of swappable index pairs, find lexicographically");
    println!("smallest string possible after any number of swaps.");
    
    println!("\n💡 Approach:");
    println!("1. Union indices that can be swapped (transitively)");
    println!("2. For each component, sort characters within that component");
    println!("3. Reconstruct string with sorted characters in each component");
    
    let s = "dcab".to_string();
    let pairs = vec![vec![0, 3], vec![1, 2]];
    
    println!("\n📝 Example: s = \"{}\", pairs = {:?}", s, pairs);
    
    let result = smallest_string_with_swaps(&s, &pairs);
    println!("\n✅ Result: \"{}\"", result);
    
    println!("\n⏱️  Time Complexity: O(n*log(n)*α(n)) where n is string length");
    println!("💾 Space Complexity: O(n) for Union-Find and sorting");
}

fn smallest_string_with_swaps(s: &str, pairs: &[Vec<i32>]) -> String {
    let n = s.len();
    let mut uf = UnionFind::new(n);
    let chars: Vec<char> = s.chars().collect();
    
    // Union swappable indices
    for pair in pairs {
        uf.union(pair[0] as usize, pair[1] as usize);
    }
    
    // Group indices by component root
    let mut components: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..n {
        let root = uf.find(i);
        components.entry(root).or_default().push(i);
    }
    
    let mut result = vec![' '; n];
    
    // Sort characters within each component
    for indices in components.values() {
        let mut component_chars: Vec<char> = indices.iter().map(|&i| chars[i]).collect();
        component_chars.sort();
        
        let mut sorted_indices = indices.clone();
        sorted_indices.sort();
        
        for (i, &idx) in sorted_indices.iter().enumerate() {
            result[idx] = component_chars[i];
        }
    }
    
    result.into_iter().collect()
}

// ============================================================================
// ADVANCED PROBLEMS
// ============================================================================

/// LeetCode 305: Number of Islands II (Premium)
/// Pattern: Dynamic connectivity, online queries
fn solve_number_of_islands_ii() {
    println!("\n🏝️  Problem 8: Number of Islands II (LeetCode 305)");
    println!("{}", "-".repeat(50));
    
    println!("📝 Problem Statement:");
    println!("Start with empty 2D grid. Add islands one by one at given positions.");
    println!("After each addition, return current number of islands.");
    
    println!("\n💡 Approach:");
    println!("1. Start with Union-Find for all grid positions");
    println!("2. For each new land position:");
    println!("   a. Increment island count");
    println!("   b. Union with adjacent existing lands");
    println!("   c. Each union decreases island count by 1");
    
    let positions = vec![
        vec![0, 0],
        vec![0, 1],
        vec![1, 2],
        vec![2, 1],
    ];
    
    println!("\n📍 Example Positions: {:?}", positions);
    
    let result = num_islands_ii(3, 3, &positions);
    println!("\n✅ Result: {:?}", result);
    println!("   After each addition: [1, 1, 2, 3]");
    
    println!("\n⏱️  Time Complexity: O(k*α(m*n)) where k=operations, m,n=grid size");
    println!("💾 Space Complexity: O(m*n) for Union-Find structure");
}

#[allow(dead_code)]
fn num_islands_ii(m: i32, n: i32, positions: &[Vec<i32>]) -> Vec<i32> {
    let rows = m as usize;
    let cols = n as usize;
    let mut uf = UnionFind::new(rows * cols);
    let mut is_land = vec![vec![false; cols]; rows];
    let mut result = Vec::new();
    let mut island_count = 0;
    
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    
    for pos in positions {
        let r = pos[0] as usize;
        let c = pos[1] as usize;
        
        if is_land[r][c] {
            result.push(island_count);
            continue;
        }
        
        is_land[r][c] = true;
        island_count += 1;
        let current_id = r * cols + c;
        
        // Check all 4 directions
        for (dr, dc) in directions {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            
            if nr >= 0 && nr < m && nc >= 0 && nc < n {
                let nr = nr as usize;
                let nc = nc as usize;
                
                if is_land[nr][nc] {
                    let neighbor_id = nr * cols + nc;
                    if uf.union(current_id, neighbor_id) {
                        island_count -= 1; // Merged two islands
                    }
                }
            }
        }
        
        result.push(island_count);
    }
    
    result
}

/// LeetCode 399: Evaluate Division  
/// Pattern: Weighted Union-Find for ratios
fn solve_evaluate_division() {
    println!("\n➗ Problem 9: Evaluate Division (LeetCode 399)");
    println!("{}", "-".repeat(50));
    
    println!("📝 Problem Statement:");
    println!("Given equations like a/b=2.0, b/c=3.0, evaluate queries like a/c=?");
    
    println!("\n💡 Approach:");
    println!("1. Use weighted Union-Find to track ratios");
    println!("2. For equation a/b=k, store weight from a to b as k");
    println!("3. For query a/c, find ratio using stored weights");
    
    let equations = vec![
        vec!["a".to_string(), "b".to_string()],
        vec!["b".to_string(), "c".to_string()],
    ];
    let values = vec![2.0, 3.0];
    let queries = vec![
        vec!["a".to_string(), "c".to_string()],
        vec!["b".to_string(), "a".to_string()],
        vec!["a".to_string(), "e".to_string()],
    ];
    
    println!("\n📊 Example:");
    println!("  Equations: {:?}", equations);
    println!("  Values: {:?}", values);
    println!("  Queries: {:?}", queries);
    
    let result = calc_equation(&equations, &values, &queries);
    println!("\n✅ Result: {:?}", result);
    println!("  a/c = 6.0, b/a = 0.5, a/e = -1.0 (undefined)");
    
    println!("\n⏱️  Time Complexity: O((E+Q)*α(V)) where E=equations, Q=queries, V=variables");
    println!("💾 Space Complexity: O(V) for Union-Find and variable mapping");
}

fn calc_equation(equations: &[Vec<String>], values: &[f64], queries: &[Vec<String>]) -> Vec<f64> {
    let mut var_to_id = HashMap::new();
    let mut id_counter = 0;
    
    // Map variables to indices
    for eq in equations {
        for var in eq {
            if !var_to_id.contains_key(var) {
                var_to_id.insert(var.clone(), id_counter);
                id_counter += 1;
            }
        }
    }
    
    // Simplified weighted Union-Find (conceptual - actual implementation would be complex)
    let mut graph: HashMap<String, Vec<(String, f64)>> = HashMap::new();
    
    // Build graph from equations
    for (i, eq) in equations.iter().enumerate() {
        let a = &eq[0];
        let b = &eq[1];
        let val = values[i];
        
        graph.entry(a.clone()).or_default().push((b.clone(), val));
        graph.entry(b.clone()).or_default().push((a.clone(), 1.0 / val));
    }
    
    let mut results = Vec::new();
    
    // Answer queries using DFS
    for query in queries {
        let start = &query[0];
        let end = &query[1];
        
        if !graph.contains_key(start) || !graph.contains_key(end) {
            results.push(-1.0);
        } else if start == end {
            results.push(1.0);
        } else {
            let mut visited = HashSet::new();
            let result = dfs(&graph, start, end, &mut visited);
            results.push(result.unwrap_or(-1.0));
        }
    }
    
    results
}

fn dfs(graph: &HashMap<String, Vec<(String, f64)>>, start: &str, end: &str, visited: &mut HashSet<String>) -> Option<f64> {
    if start == end {
        return Some(1.0);
    }
    
    visited.insert(start.to_string());
    
    if let Some(neighbors) = graph.get(start) {
        for (neighbor, weight) in neighbors {
            if !visited.contains(neighbor) {
                if let Some(path_weight) = dfs(graph, neighbor, end, visited) {
                    return Some(weight * path_weight);
                }
            }
        }
    }
    
    visited.remove(start);
    None
}

/// LeetCode 1697: Checking Existence of Edge Length Limited Paths
/// Pattern: Offline queries, sort by constraints
fn solve_edge_length_limited_paths() {
    println!("\n🛤️  Problem 10: Edge Length Limited Paths (LeetCode 1697)");
    println!("{}", "-".repeat(50));
    
    println!("📝 Problem Statement:");
    println!("Given weighted graph and queries [u,v,limit], check if path exists");
    println!("from u to v using only edges with weight < limit.");
    
    println!("\n💡 Approach:");
    println!("1. Sort queries by limit (ascending)");
    println!("2. Sort edges by weight (ascending)");
    println!("3. For each query, add edges with weight < limit to Union-Find");
    println!("4. Check if u and v are connected");
    
    println!("\n🔧 This is an offline algorithm - process queries in sorted order");
    println!("rather than in original order, then restore original order in result.");
    
    println!("\n⏱️  Time Complexity: O((E+Q)*log(E+Q)*α(V))");
    println!("💾 Space Complexity: O(V) for Union-Find structure");
    
    println!("\n📚 Key Learning: Offline query processing");
    println!("• Sort queries/operations by constraint values");
    println!("• Process in optimal order for algorithm");
    println!("• Restore original order in final result");
}

// ============================================================================
// INTERVIEW TIPS AND PRACTICE
// ============================================================================

/// Show comprehensive interview tips
fn show_interview_tips() {
    println!("\n💼 Interview Tips");
    println!("{}", "=".repeat(50));
    
    println!("\n🗣️  How to Explain Union-Find in Interviews:");
    println!("1. \"Union-Find tracks connected components in a dynamic graph\"");
    println!("2. \"Two main operations: find root, union two sets\"");
    println!("3. \"Optimization 1: Path compression flattens trees during find\"");
    println!("4. \"Optimization 2: Union by rank keeps trees balanced\"");
    println!("5. \"Combined: O(α(n)) amortized per operation (nearly constant)\"");
    
    println!("\n❓ Common Follow-up Questions:");
    println!("• \"What if we need to support deletions?\" → Very hard, mention rebuilding");
    println!("• \"Can this be done with DFS/BFS?\" → Discuss trade-offs");
    println!("• \"What's the worst case without optimizations?\" → O(n) per operation");
    println!("• \"How would you implement this in a different language?\" → Arrays vs objects");
    
    println!("\n🐛 Common Mistakes to Avoid:");
    println!("• Forgetting to handle 0-indexed vs 1-indexed problems");
    println!("• Not mapping strings/coordinates to Union-Find indices");
    println!("• Implementing union without rank (inefficient)");
    println!("• Forgetting path compression in find");
    println!("• Not considering edge cases (empty input, single element)");
    
    println!("\n🧪 Testing Strategy:");
    println!("• Test with small examples (2-3 elements)");
    println!("• Test edge cases: empty input, single element, all connected");
    println!("• Test the \"just before cycle\" case for cycle detection");
    println!("• Verify component counts manually for small examples");
    
    println!("\n⚡ Quick Implementation Template:");
    println!("```rust");
    println!("struct UnionFind {{");
    println!("    parent: Vec<usize>, rank: Vec<usize>, count: usize");
    println!("}}");
    println!("impl UnionFind {{");
    println!("    fn find(&mut self, x: usize) -> usize {{ /* path compression */ }}");
    println!("    fn union(&mut self, x: usize, y: usize) -> bool {{ /* union by rank */ }}");
    println!("}}");
    println!("```");
}

/// Show practice problems for further study
fn show_practice_problems() {
    println!("\n🎯 Practice Problems");
    println!("{}", "=".repeat(50));
    
    println!("\n🌟 Easy Problems (Start Here):");
    println!("• LeetCode 200: Number of Islands");
    println!("• LeetCode 547: Number of Provinces (Friend Circles)");
    println!("• LeetCode 684: Redundant Connection");
    println!("• LeetCode 1319: Number of Operations to Make Network Connected");
    
    println!("\n⭐ Medium Problems:");
    println!("• LeetCode 721: Accounts Merge");
    println!("• LeetCode 947: Most Stones Removed");
    println!("• LeetCode 990: Satisfiability of Equality Equations");
    println!("• LeetCode 1202: Smallest String With Swaps");
    println!("• LeetCode 1135: Connecting Cities With Minimum Cost");
    println!("• LeetCode 1584: Min Cost to Connect All Points");
    
    println!("\n🌟 Hard Problems (Advanced):");
    println!("• LeetCode 305: Number of Islands II (Premium)");
    println!("• LeetCode 399: Evaluate Division");
    println!("• LeetCode 1697: Checking Existence of Edge Length Limited Paths");
    println!("• LeetCode 1489: Find Critical and Pseudo-Critical Edges in MST");
    
    println!("\n🏆 Competition Problems:");
    println!("• Codeforces: DSU on Tree problems");
    println!("• AtCoder: Many Union-Find problems in beginner contests");
    println!("• USACO: Connected Components, Minimum Spanning Trees");
    
    println!("\n📈 Difficulty Progression:");
    println!("Week 1: Solve 3-4 Easy problems");
    println!("Week 2: Solve 4-5 Medium problems"); 
    println!("Week 3: Attempt 2-3 Hard problems");
    println!("Week 4: Try competition problems");
    
    println!("\n🎓 Mastery Checklist:");
    println!("□ Can implement Union-Find from memory in 5 minutes");
    println!("□ Can recognize Union-Find problems in problem statements");
    println!("□ Can adapt Union-Find for different data types (strings, coordinates)");
    println!("□ Can explain time complexity and optimizations clearly");
    println!("□ Can choose between Union-Find and DFS/BFS appropriately");
    println!("□ Can handle edge cases and write robust test cases");
    
    println!("\n🚀 Next Steps:");
    println!("→ Complete at least 10 problems from the list above");
    println!("→ Practice explaining solutions out loud");
    println!("→ Review Mission 10 implementation for production code examples");
    println!("→ Study advanced variants from Step 6 for specialized use cases");
    println!("→ Consider taking online competitive programming contests");
    
    println!("\n🎯 Tutorial Complete!");
    println!("You now have the knowledge and patterns to tackle any Union-Find problem.");
    println!("Remember: Practice makes perfect. Start coding! 💻");
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_count_islands() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(count_islands(&grid), 1);
        
        let grid2 = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(count_islands(&grid2), 3);
    }
    
    #[test]
    fn test_friend_circles() {
        #[rustfmt::skip]
        let friendships = vec![
            vec![1, 1, 0],
            vec![1, 1, 0],
            vec![0, 0, 1],
        ];
        assert_eq!(count_friend_circles(&friendships), 2);
        
        #[rustfmt::skip]
        let friendships2 = vec![
            vec![1, 1, 0],
            vec![1, 1, 1],
            vec![0, 1, 1],
        ];
        assert_eq!(count_friend_circles(&friendships2), 1);
    }
    
    #[test]
    fn test_redundant_connection() {
        let edges = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
        assert_eq!(find_redundant_connection(&edges), vec![2, 3]);
        
        let edges2 = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]];
        assert_eq!(find_redundant_connection(&edges2), vec![1, 4]);
    }
    
    #[test]
    fn test_remove_stones() {
        let stones = vec![
            vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 2], vec![2, 1], vec![2, 2]
        ];
        assert_eq!(remove_stones(&stones), 5);
        
        let stones2 = vec![vec![0, 0], vec![0, 2], vec![1, 1], vec![2, 0], vec![2, 2]];
        assert_eq!(remove_stones(&stones2), 3);
    }
    
    #[test]
    fn test_equations_possible() {
        let equations = vec!["a==b".to_string(), "b!=a".to_string()];
        assert_eq!(equations_possible(&equations), false);
        
        let equations2 = vec!["b==a".to_string(), "a==b".to_string()];
        assert_eq!(equations_possible(&equations2), true);
    }
    
    #[test]
    fn test_smallest_string_with_swaps() {
        let s = "dcab".to_string();
        let pairs = vec![vec![0, 3], vec![1, 2]];
        assert_eq!(smallest_string_with_swaps(&s, &pairs), "bacd");
        
        let s2 = "dcab".to_string();
        let pairs2 = vec![vec![0, 3], vec![1, 2], vec![0, 2]];
        assert_eq!(smallest_string_with_swaps(&s2, &pairs2), "abcd");
    }
}