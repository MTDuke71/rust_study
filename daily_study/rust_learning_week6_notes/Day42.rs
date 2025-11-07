// Day42.rs - Module Practice & Complex Project Organization
// Complete Runnable Example for Day 42 of Week 6
// Focus: Organizing complex projects, refactoring for modularity, best practices

#![allow(dead_code)]  // Allow unused code in this demonstration

// Imports handled within individual modules

fn main() {
    println!("=== Day 42: Module Practice & Complex Project Organization ===\n");
    
    // Demonstrate modular architecture
    demonstrate_modular_architecture();
    
    // Show refactoring benefits
    demonstrate_refactoring_benefits();
    
    // Project organization patterns
    demonstrate_organization_patterns();
    
    println!("\n=== Complete Week 6 Summary ===");
    print_week_summary();
}

// ============================================================================
// Core Domain Layer - Essential data structures and algorithms
// ============================================================================

mod core {
    use std::collections::HashMap;
    
    pub struct UnionFind {
        pub parent: Vec<usize>,  // Made public for demonstration
        rank: Vec<usize>,
        size: Vec<usize>,
        components: usize,
    }
    
    impl UnionFind {
        pub fn new(n: usize) -> Self {
            Self {
                parent: (0..n).collect(),
                rank: vec![0; n],
                size: vec![1; n],
                components: n,
            }
        }
        
        pub fn find(&mut self, x: usize) -> usize {
            if self.parent[x] != x {
                self.parent[x] = self.find(self.parent[x]); // Path compression
            }
            self.parent[x]
        }
        
        pub fn union(&mut self, x: usize, y: usize) -> bool {
            let root_x = self.find(x);
            let root_y = self.find(y);
            
            if root_x == root_y {
                return false;
            }
            
            // Union by rank optimization
            match self.rank[root_x].cmp(&self.rank[root_y]) {
                std::cmp::Ordering::Less => {
                    self.parent[root_x] = root_y;
                    self.size[root_y] += self.size[root_x];
                }
                std::cmp::Ordering::Greater => {
                    self.parent[root_y] = root_x;
                    self.size[root_x] += self.size[root_y];
                }
                std::cmp::Ordering::Equal => {
                    self.parent[root_y] = root_x;
                    self.size[root_x] += self.size[root_y];
                    self.rank[root_x] += 1;
                }
            }
            
            self.components -= 1;
            true
        }
        
        pub fn connected(&mut self, x: usize, y: usize) -> bool {
            self.find(x) == self.find(y)
        }
        
        pub fn component_size(&mut self, x: usize) -> usize {
            let root = self.find(x);
            self.size[root]
        }
        
        pub fn component_count(&self) -> usize {
            self.components
        }
        
        pub fn get_components(&mut self) -> HashMap<usize, Vec<usize>> {
            let mut components = HashMap::new();
            let size = self.parent.len();
            
            for i in 0..size {
                let root = self.find(i);
                components.entry(root).or_insert_with(Vec::new).push(i);
            }
            
            components
        }
    }
    
    pub struct GraphAlgorithms;
    
    impl GraphAlgorithms {
        pub fn connected_components(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
            let mut uf = UnionFind::new(n);
            
            // Process all edges
            for &(u, v) in edges {
                uf.union(u, v);
            }
            
            // Get components as vectors
            uf.get_components().into_values().collect()
        }
        
        pub fn has_cycle(n: usize, edges: &[(usize, usize)]) -> bool {
            let mut uf = UnionFind::new(n);
            
            for &(u, v) in edges {
                if uf.connected(u, v) {
                    return true; // Cycle detected
                }
                uf.union(u, v);
            }
            
            false
        }
        
        pub fn minimum_spanning_tree_edges(n: usize, edges: &[(usize, usize, u32)]) -> Vec<(usize, usize, u32)> {
            let mut sorted_edges = edges.to_vec();
            sorted_edges.sort_by_key(|&(_, _, weight)| weight);
            
            let mut uf = UnionFind::new(n);
            let mut mst_edges = Vec::new();
            
            for &(u, v, weight) in &sorted_edges {
                if !uf.connected(u, v) {
                    uf.union(u, v);
                    mst_edges.push((u, v, weight));
                }
            }
            
            mst_edges
        }
    }
}

// ============================================================================
// API Layer - High-level interfaces for end users
// ============================================================================

mod api {
    use super::core::UnionFind;
    use std::fmt;
    
    #[derive(Debug)]
    pub enum EdgeResult {
        Connected {
            components_merged: usize,
            new_component_size: usize,
        },
        AlreadyConnected {
            component_size: usize,
        },
    }
    
    #[derive(Debug)]
    pub struct AnalysisReport {
        pub total_components: usize,
        pub component_sizes: Vec<usize>,
        pub operations_performed: usize,
        pub largest_component: usize,
        pub average_component_size: f64,
    }
    
    impl fmt::Display for AnalysisReport {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "📊 Graph Analysis Report")?;
            writeln!(f, "   • Total components: {}", self.total_components)?;
            writeln!(f, "   • Largest component: {} nodes", self.largest_component)?;
            writeln!(f, "   • Average component size: {:.1}", self.average_component_size)?;
            writeln!(f, "   • Operations performed: {}", self.operations_performed)?;
            write!(f, "   • Component distribution: {:?}", self.component_sizes)
        }
    }
    
    pub struct GraphAnalyzer {
        union_find: UnionFind,
        operation_count: usize,
        edge_history: Vec<(usize, usize)>,
    }
    
    impl GraphAnalyzer {
        pub fn new(size: usize) -> Self {
            Self {
                union_find: UnionFind::new(size),
                operation_count: 0,
                edge_history: Vec::new(),
            }
        }
        
        pub fn add_edge(&mut self, u: usize, v: usize) -> EdgeResult {
            self.operation_count += 1;
            self.edge_history.push((u, v));
            
            if self.union_find.connected(u, v) {
                EdgeResult::AlreadyConnected {
                    component_size: self.union_find.component_size(u),
                }
            } else {
                let old_components = self.union_find.component_count();
                self.union_find.union(u, v);
                let new_components = self.union_find.component_count();
                
                EdgeResult::Connected {
                    components_merged: old_components - new_components,
                    new_component_size: self.union_find.component_size(u),
                }
            }
        }
        
        pub fn analyze(&mut self) -> AnalysisReport {
            let components = self.union_find.get_components();
            let component_sizes: Vec<usize> = components.values().map(|c| c.len()).collect();
            
            let total_components = component_sizes.len();
            let largest_component = component_sizes.iter().max().copied().unwrap_or(0);
            let average_component_size = if total_components > 0 {
                component_sizes.iter().sum::<usize>() as f64 / total_components as f64
            } else {
                0.0
            };
            
            AnalysisReport {
                total_components,
                component_sizes,
                operations_performed: self.operation_count,
                largest_component,
                average_component_size,
            }
        }
        
        pub fn get_connectivity_timeline(&mut self) -> Vec<(usize, usize, usize)> {
            // Replay history to show component count evolution
            let mut temp_uf = super::core::UnionFind::new(self.union_find.parent.len());
            let mut timeline = Vec::new();
            
            for (i, &(u, v)) in self.edge_history.iter().enumerate() {
                temp_uf.union(u, v);
                timeline.push((i + 1, temp_uf.component_count(), temp_uf.component_size(u)));
            }
            
            timeline
        }
    }
    
    pub struct GraphBuilder {
        size: usize,
        edges: Vec<(usize, usize)>,
        weighted_edges: Vec<(usize, usize, u32)>,
    }
    
    impl GraphBuilder {
        pub fn new(size: usize) -> Self {
            Self {
                size,
                edges: Vec::new(),
                weighted_edges: Vec::new(),
            }
        }
        
        pub fn add_edge(mut self, u: usize, v: usize) -> Self {
            self.edges.push((u, v));
            self
        }
        
        pub fn add_weighted_edge(mut self, u: usize, v: usize, weight: u32) -> Self {
            self.weighted_edges.push((u, v, weight));
            self
        }
        
        pub fn build_analyzer(self) -> GraphAnalyzer {
            let mut analyzer = GraphAnalyzer::new(self.size);
            for (u, v) in self.edges {
                analyzer.add_edge(u, v);
            }
            analyzer
        }
        
        pub fn find_mst(self) -> Vec<(usize, usize, u32)> {
            super::core::GraphAlgorithms::minimum_spanning_tree_edges(self.size, &self.weighted_edges)
        }
    }
}

// ============================================================================
// Utils Layer - Helper functions and utilities
// ============================================================================

mod utils {
    use std::time::{Duration, Instant};
    use std::collections::HashSet;
    
    pub struct Validator;
    
    impl Validator {
        pub fn validate_node_range(node: usize, max_size: usize) -> Result<(), String> {
            if node >= max_size {
                Err(format!("Node {} is out of range [0, {})", node, max_size))
            } else {
                Ok(())
            }
        }
        
        pub fn validate_edges(edges: &[(usize, usize)], max_size: usize) -> Result<(), String> {
            for &(u, v) in edges {
                Self::validate_node_range(u, max_size)?;
                Self::validate_node_range(v, max_size)?;
                if u == v {
                    return Err(format!("Self-loop detected: ({}, {})", u, v));
                }
            }
            Ok(())
        }
        
        pub fn remove_duplicate_edges(edges: &[(usize, usize)]) -> Vec<(usize, usize)> {
            let mut unique_edges = HashSet::new();
            let mut result = Vec::new();
            
            for &(u, v) in edges {
                let edge = if u <= v { (u, v) } else { (v, u) };
                if unique_edges.insert(edge) {
                    result.push(edge);
                }
            }
            
            result
        }
    }
    
    pub struct PerformanceProfiler {
        start_time: Instant,
        operation_times: Vec<Duration>,
        operation_names: Vec<String>,
    }
    
    impl PerformanceProfiler {
        pub fn new() -> Self {
            Self {
                start_time: Instant::now(),
                operation_times: Vec::new(),
                operation_names: Vec::new(),
            }
        }
        
        pub fn time_operation<F, R>(&mut self, name: &str, operation: F) -> R 
        where 
            F: FnOnce() -> R 
        {
            let start = Instant::now();
            let result = operation();
            let duration = start.elapsed();
            
            self.operation_times.push(duration);
            self.operation_names.push(name.to_string());
            
            result
        }
        
        pub fn generate_report(&self) -> String {
            let mut report = String::from("⏱️  Performance Profile:\n");
            
            for (i, (name, time)) in self.operation_names.iter().zip(&self.operation_times).enumerate() {
                report.push_str(&format!("   {}. {}: {:?}\n", i + 1, name, time));
            }
            
            if !self.operation_times.is_empty() {
                let total: Duration = self.operation_times.iter().sum();
                let average = total / self.operation_times.len() as u32;
                report.push_str(&format!("   📊 Average operation time: {:?}\n", average));
            }
            
            report.push_str(&format!("   🕐 Total elapsed time: {:?}", self.start_time.elapsed()));
            
            report
        }
    }
    
    pub struct GraphVisualizer;
    
    impl GraphVisualizer {
        pub fn generate_dot_format(components: &[Vec<usize>]) -> String {
            let mut dot = String::from("graph G {\n");
            
            for (comp_idx, component) in components.iter().enumerate() {
                dot.push_str(&format!("  subgraph cluster_{} {{\n", comp_idx));
                dot.push_str(&format!("    color={};\n", Self::get_color(comp_idx)));
                dot.push_str(&format!("    label=\"Component {}\";\n", comp_idx + 1));
                
                // Add nodes
                for &node in component {
                    dot.push_str(&format!("    {};\n", node));
                }
                
                // Add edges within component (simplified - just connect sequentially)
                for window in component.windows(2) {
                    dot.push_str(&format!("    {} -- {};\n", window[0], window[1]));
                }
                
                dot.push_str("  }\n");
            }
            
            dot.push_str("}\n");
            dot
        }
        
        fn get_color(index: usize) -> &'static str {
            const COLORS: &[&str] = &["red", "blue", "green", "purple", "orange", "brown"];
            COLORS[index % COLORS.len()]
        }
    }
}

// ============================================================================
// Demonstration Functions
// ============================================================================

fn demonstrate_modular_architecture() {
    println!("1. 🏗️  Modular Architecture Demonstration");
    println!("==========================================\n");
    
    // High-level API usage
    println!("📋 High-Level API Usage:");
    let mut analyzer = api::GraphAnalyzer::new(8);
    
    let edges = vec![
        (0, 1), (1, 2), (3, 4),  // Two separate components
        (5, 6), (6, 7),          // Third component
        (2, 3),                  // Merge first two components
    ];
    
    for (u, v) in &edges {
        match analyzer.add_edge(*u, *v) {
            api::EdgeResult::Connected { components_merged, new_component_size } => {
                println!("   ✅ Connected {} - {} (merged {} components → size {})", 
                         u, v, components_merged, new_component_size);
            }
            api::EdgeResult::AlreadyConnected { component_size } => {
                println!("   ℹ️  {} - {} already connected (component size: {})", 
                         u, v, component_size);
            }
        }
    }
    
    let report = analyzer.analyze();
    println!("\n   {}\n", report);
    
    // Show connectivity timeline
    println!("📈 Connectivity Timeline:");
    let timeline = analyzer.get_connectivity_timeline();
    for (step, components, largest_size) in timeline {
        println!("   Step {}: {} components (largest: {} nodes)", 
                 step, components, largest_size);
    }
}

fn demonstrate_refactoring_benefits() {
    println!("\n2. 🔄 Refactoring Benefits Demonstration");
    println!("========================================\n");
    
    // Builder pattern usage
    println!("🏗️  Builder Pattern Usage:");
    let graph = api::GraphBuilder::new(6)
        .add_weighted_edge(0, 1, 4)
        .add_weighted_edge(0, 2, 3)
        .add_weighted_edge(1, 2, 1)
        .add_weighted_edge(1, 3, 2)
        .add_weighted_edge(2, 3, 5)
        .add_weighted_edge(3, 4, 6);
    
    let mst = graph.find_mst();
    println!("   Minimum Spanning Tree edges:");
    for (u, v, weight) in mst {
        println!("   {} -- {} (weight: {})", u, v, weight);
    }
    
    // Performance profiling
    println!("\n⚡ Performance Profiling:");
    let mut profiler = utils::PerformanceProfiler::new();
    
    let _components = profiler.time_operation("Find Connected Components", || {
        core::GraphAlgorithms::connected_components(100, &[
            (0, 10), (10, 20), (20, 30), (40, 50), (50, 60)
        ])
    });
    
    let _has_cycle = profiler.time_operation("Cycle Detection", || {
        core::GraphAlgorithms::has_cycle(50, &[
            (0, 1), (1, 2), (2, 3), (10, 11), (11, 12)
        ])
    });
    
    println!("{}", profiler.generate_report());
    
    // Validation
    println!("\n🔍 Input Validation:");
    let edges = [(0, 1), (5, 2), (10, 3)]; // One invalid edge
    match utils::Validator::validate_edges(&edges, 5) {
        Ok(_) => println!("   ✅ All edges valid"),
        Err(e) => println!("   ❌ Validation error: {}", e),
    }
    
    let cleaned = utils::Validator::remove_duplicate_edges(&[(0, 1), (1, 0), (0, 1), (2, 3)]);
    println!("   🧹 Cleaned edges: {:?}", cleaned);
}

fn demonstrate_organization_patterns() {
    println!("\n3. 📁 Project Organization Patterns");
    println!("===================================\n");
    
    println!("🎯 Layer-Based Architecture:");
    println!("   📦 Core Layer:");
    println!("      ├── UnionFind (data structure)");
    println!("      └── GraphAlgorithms (algorithms)");
    println!("   🌐 API Layer:");
    println!("      ├── GraphAnalyzer (high-level interface)");
    println!("      └── GraphBuilder (construction pattern)");
    println!("   🛠️  Utils Layer:");
    println!("      ├── Validator (input validation)");
    println!("      ├── PerformanceProfiler (metrics)");
    println!("      └── GraphVisualizer (output formatting)");
    
    // Generate example DOT output
    let components = vec![
        vec![0, 1, 2],
        vec![3, 4],
        vec![5, 6, 7, 8],
    ];
    
    println!("\n🎨 Visualization Example (DOT format):");
    let dot_output = utils::GraphVisualizer::generate_dot_format(&components);
    for (i, line) in dot_output.lines().take(8).enumerate() {
        println!("   {}", line);
        if i == 7 {
            println!("   ... (truncated)");
            break;
        }
    }
    
    println!("\n📋 Design Patterns Applied:");
    println!("   🏗️  Builder: GraphBuilder for fluent construction");
    println!("   🎭 Facade: GraphAnalyzer simplifies complex operations");
    println!("   📊 Strategy: Different algorithm implementations");
    println!("   🔧 Template Method: Consistent validation patterns");
    println!("   🏭 Factory: Component creation from algorithms");
}

fn print_week_summary() {
    println!("🎓 Week 6 Learning Journey Completed!");
    println!("   Day 36: ✅ Module basics and visibility rules");
    println!("   Day 37: ✅ Crate organization and structure");
    println!("   Day 38: ✅ Cargo features and conditional compilation");
    println!("   Day 39: ✅ Workspace management and multi-crate projects");
    println!("   Day 40: ✅ Publishing crates and documentation");
    println!("   Day 41: ✅ External dependencies and evaluation");
    println!("   Day 42: ✅ Module practice and complex organization");
    
    println!("\n🏆 Key Achievements:");
    println!("   ✨ Mastered Rust module system and visibility rules");
    println!("   🏗️  Learned complex project organization patterns");
    println!("   📦 Understanding of crate publishing and workspace management");
    println!("   🔧 Applied design patterns for maintainable code");
    println!("   🎯 Prepared Mission 10 with professional-grade organization");
    
    println!("\n🚀 Ready for Advanced Rust Development!");
    println!("   Next: Apply these patterns to real-world projects");
    println!("   Mission 10: Implement Union-Find with modular architecture");
    println!("   Future: Build production-quality Rust applications");
}

// ============================================================================
// Tests - Comprehensive modular testing demonstration
// ============================================================================

#[cfg(test)]
mod tests {
    
    mod core_tests {
        use super::super::core::{UnionFind, GraphAlgorithms};
        
        #[test]
        fn test_union_find_operations() {
            let mut uf = UnionFind::new(5);
            
            // Test initial state
            assert_eq!(uf.component_count(), 5);
            assert!(!uf.connected(0, 1));
            
            // Test union operation
            assert!(uf.union(0, 1));
            assert!(uf.connected(0, 1));
            assert_eq!(uf.component_count(), 4);
            assert_eq!(uf.component_size(0), 2);
            
            // Test duplicate union
            assert!(!uf.union(0, 1));
            assert_eq!(uf.component_count(), 4);
        }
        
        #[test]
        fn test_path_compression() {
            let mut uf = UnionFind::new(4);
            uf.union(0, 1);
            uf.union(1, 2);
            uf.union(2, 3);
            
            // Find should apply path compression
            let root = uf.find(0);
            assert_eq!(uf.find(3), root);
        }
        
        #[test]
        fn test_connected_components() {
            let components = GraphAlgorithms::connected_components(6, &[
                (0, 1), (1, 2), (3, 4)
            ]);
            
            assert_eq!(components.len(), 3); // Three components: {0,1,2}, {3,4}, {5}
        }
        
        #[test]
        fn test_cycle_detection() {
            // No cycle
            assert!(!GraphAlgorithms::has_cycle(4, &[(0, 1), (1, 2), (2, 3)]));
            
            // Has cycle
            assert!(GraphAlgorithms::has_cycle(3, &[(0, 1), (1, 2), (2, 0)]));
        }
    }
    
    mod api_tests {
        use super::super::api::{GraphAnalyzer, GraphBuilder};
        
        #[test]
        fn test_graph_analyzer() {
            let mut analyzer = GraphAnalyzer::new(4);
            
            analyzer.add_edge(0, 1);
            analyzer.add_edge(2, 3);
            
            let report = analyzer.analyze();
            assert_eq!(report.total_components, 2);
            assert_eq!(report.operations_performed, 2);
            assert_eq!(report.largest_component, 2);
        }
        
        #[test]
        fn test_builder_pattern() {
            let mst = GraphBuilder::new(4)
                .add_weighted_edge(0, 1, 1)
                .add_weighted_edge(1, 2, 2)
                .add_weighted_edge(2, 3, 3)
                .add_weighted_edge(0, 3, 4)
                .find_mst();
            
            assert_eq!(mst.len(), 3); // MST has n-1 edges
        }
    }
    
    mod utils_tests {
        use super::super::utils::{Validator, PerformanceProfiler};
        
        #[test]
        fn test_validation() {
            // Valid edges
            assert!(Validator::validate_edges(&[(0, 1), (1, 2)], 3).is_ok());
            
            // Self-loop error
            assert!(Validator::validate_edges(&[(0, 0)], 3).is_err());
            
            // Out of range error
            assert!(Validator::validate_edges(&[(0, 5)], 3).is_err());
        }
        
        #[test]
        fn test_duplicate_removal() {
            let edges = [(0, 1), (1, 0), (0, 1), (2, 3)];
            let unique = Validator::remove_duplicate_edges(&edges);
            assert_eq!(unique.len(), 2); // Should have (0,1) and (2,3)
        }
        
        #[test]
        fn test_performance_profiler() {
            let mut profiler = PerformanceProfiler::new();
            
            let _result = profiler.time_operation("test_operation", || {
                std::thread::sleep(std::time::Duration::from_millis(1));
                42
            });
            
            let report = profiler.generate_report();
            assert!(report.contains("test_operation"));
            assert!(report.contains("Performance Profile"));
        }
    }
    
    #[test]
    fn test_integration() {
        // End-to-end test combining multiple modules
        let mut analyzer = crate::api::GraphAnalyzer::new(6);
        
        // Valid edges
        let edges = [(0, 1), (1, 2), (3, 4)];
        assert!(crate::utils::Validator::validate_edges(&edges, 6).is_ok());
        
        // Add edges to analyzer
        for (u, v) in edges {
            analyzer.add_edge(u, v);
        }
        
        // Analyze results
        let report = analyzer.analyze();
        assert_eq!(report.total_components, 3); // {0,1,2}, {3,4}, {5}
        assert_eq!(report.largest_component, 3);
    }
}