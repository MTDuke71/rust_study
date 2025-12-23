use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> usize {
    let graph = parse_input(input);
    find_triangles_with_t(&graph)
}

pub fn part2(input: &str) -> String {
    let graph = parse_input(input);
    find_largest_clique(&graph)
}

type Graph = HashMap<String, HashSet<String>>;

fn parse_input(input: &str) -> Graph {
    let mut graph: Graph = HashMap::new();
    
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = line.split('-').collect();
        if parts.len() == 2 {
            let a = parts[0].to_string();
            let b = parts[1].to_string();
            
            // Undirected graph - add both directions
            graph.entry(a.clone()).or_default().insert(b.clone());
            graph.entry(b).or_default().insert(a);
        }
    }
    
    graph
}

fn find_triangles_with_t(graph: &Graph) -> usize {
    let mut triangles = HashSet::new();
    
    // For each node
    for (node_a, neighbors_a) in graph {
        // For each neighbor of node_a
        for node_b in neighbors_a {
            // For each neighbor of node_b
            if let Some(neighbors_b) = graph.get(node_b) {
                for node_c in neighbors_b {
                    // Check if node_c is also connected to node_a (completing the triangle)
                    if neighbors_a.contains(node_c) {
                        // Found a triangle: a-b-c
                        // Check if at least one starts with 't'
                        if node_a.starts_with('t') || node_b.starts_with('t') || node_c.starts_with('t') {
                            // Create a sorted tuple to avoid counting the same triangle multiple times
                            let mut triangle = [node_a.as_str(), node_b.as_str(), node_c.as_str()];
                            triangle.sort();
                            triangles.insert((triangle[0], triangle[1], triangle[2]));
                        }
                    }
                }
            }
        }
    }
    
    triangles.len()
}

fn find_largest_clique(graph: &Graph) -> String {
    let nodes: Vec<String> = graph.keys().cloned().collect();
    let mut largest_clique: Vec<String> = Vec::new();
    
    // Bron-Kerbosch algorithm for finding maximum clique
    // Initialize the three sets:
    // R = {} (current clique being built)
    // P = all nodes (candidates to add to clique)
    // X = {} (nodes already processed)
    let r = HashSet::new();
    let p: HashSet<String> = nodes.into_iter().collect();
    let x = HashSet::new();
    
    bron_kerbosch(graph, r, p, x, &mut largest_clique);
    
    // Sort and join with commas
    largest_clique.sort();
    largest_clique.join(",")
}

/// Bron-Kerbosch algorithm with pivoting for finding maximum clique
/// 
/// A clique is a set of nodes where every node is connected to every other node.
/// This algorithm systematically explores all possible cliques using three sets:
/// 
/// - **R**: Current clique being constructed (nodes definitely in this clique)
/// - **P**: Candidate nodes that could extend R (not yet explored)
/// - **X**: Nodes already processed (to avoid duplicate work)
/// 
/// **Base case**: When P and X are empty, R is a maximal clique
/// 
/// **Recursive case**: For each candidate node v in P:
///   1. Add v to R (try including it in the clique)
///   2. Filter P and X to only keep v's neighbors (a clique requires all nodes connected)
///   3. Recurse with updated sets
///   4. Move v from P to X (mark as processed)
/// 
/// **Pivoting optimization**: Choose a "pivot" node from P ∪ X. Only iterate over
/// nodes NOT connected to the pivot. This dramatically reduces the search space
/// because if a node is connected to the pivot, it can't form a larger clique
/// than one including the pivot itself.
fn bron_kerbosch(
    graph: &Graph,
    r: HashSet<String>,      // Current clique (nodes definitely included)
    mut p: HashSet<String>,  // Candidate nodes (could extend clique)
    mut x: HashSet<String>,  // Already processed nodes (avoid duplicates)
    max_clique: &mut Vec<String>,
) {
    // INSTRUMENTATION: Show current state
    println!("\n─────────────────────────────────────");
    println!("ENTER: R={:?} (size {})", 
             r.iter().cloned().collect::<Vec<_>>(), r.len());
    println!("       P size={}, X size={}", p.len(), x.len());
    
    // BASE CASE: If no more candidates (P empty) and no exclusions (X empty),
    // then R is a maximal clique (can't be extended further)
    if p.is_empty() && x.is_empty() {
        println!("🎯 BASE CASE! Found maximal clique of size {}: {:?}", 
                 r.len(), r.iter().cloned().collect::<Vec<_>>());
        
        // Check if this clique is larger than the current maximum
        if r.len() > max_clique.len() {
            println!("   ✨ NEW MAXIMUM! (previous max was {})", max_clique.len());
            *max_clique = r.iter().cloned().collect();
        }
        return;
    }
    
    // PIVOTING: Choose a pivot node from P ∪ X
    // The pivot helps prune the search tree. We only need to explore nodes
    // that are NOT neighbors of the pivot, because:
    // - If a node IS a neighbor of pivot, any clique containing it could
    //   also contain the pivot (making it no larger)
    // - If a node is NOT a neighbor of pivot, it might form a larger clique
    let pivot = p.union(&x).next().cloned();
    
    // Calculate candidates: P \ N(pivot)
    // (All candidates EXCEPT those connected to the pivot)
    let candidates: Vec<String> = if let Some(ref pivot_node) = pivot {
        let pivot_neighbors = graph.get(pivot_node).cloned().unwrap_or_default();
        println!("🔄 PIVOT: {:?} (has {} neighbors)", pivot_node, pivot_neighbors.len());
        let cands = p.difference(&pivot_neighbors).cloned().collect();
        println!("   Candidates after pivot: {} nodes (was {} before pivot)", 
                 p.difference(&pivot_neighbors).count(), p.len());
        cands
    } else {
        println!("🔄 NO PIVOT (P∪X empty)");
        p.iter().cloned().collect()
    };
    
    println!("📋 Will try {} candidates", candidates.len());
    
    // RECURSIVE CASE: Try adding each candidate node to the clique
    for (i, v) in candidates.iter().enumerate() {
        let neighbors = graph.get(v).cloned().unwrap_or_default();
        
        println!("\n  ➜ Trying candidate {}/{}: {:?} (has {} neighbors)", 
                 i + 1, candidates.len(), v, neighbors.len());
        
        // Build new R: Current clique + node v
        let mut new_r = r.clone();
        new_r.insert(v.clone());
        
        // Build new P: Candidates that are ALSO neighbors of v
        // (For a clique, all nodes must be connected to each other)
        let new_p: HashSet<String> = p.intersection(&neighbors).cloned().collect();
        
        // Build new X: Processed nodes that are ALSO neighbors of v
        let new_x: HashSet<String> = x.intersection(&neighbors).cloned().collect();
        
        println!("    New sets: R size={}, P size={}, X size={}", 
                 new_r.len(), new_p.len(), new_x.len());
        
        // Recurse: Try to extend the clique containing v
        bron_kerbosch(graph, new_r, new_p, new_x, max_clique);
        
        // Move v from P to X (mark as processed)
        println!("  ← Back from recursion. Moving {:?} from P to X", v);
        p.remove(v);
        x.insert(v.clone());
    }
    
    println!("✓ Done with this level. P now has {} nodes, X has {} nodes", p.len(), x.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE), 7);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE), "co,de,ka,ta");
    }
}
