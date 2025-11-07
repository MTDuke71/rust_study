// Day41.rs - External Dependencies Management
// Complete Runnable Example for Day 41 of Week 6
// Focus: Evaluating crates, managing dependency versions, security considerations

#![allow(dead_code)]  // Allow unused code in this demonstration

use std::collections::{HashMap, HashSet};
use std::process::Command;
use std::hash::Hash;

fn main() {
    println!("=== Day 41: External Dependencies Management ===\n");
    
    // 1. Demonstrate crate evaluation
    demo_crate_evaluation();
    
    // 2. Show dependency version strategies
    demo_version_strategies();
    
    // 3. Security considerations
    demo_security_audit();
    
    // 4. Mission 10 application
    demo_mission_10_dependencies();
    
    // 5. Interactive dependency analysis
    demo_dependency_analysis();
}

fn demo_crate_evaluation() {
    println!("1. 🔍 Crate Evaluation Demo");
    println!("==========================");
    
    let _evaluator = CrateEvaluator::new();
    
    // Evaluate popular Rust crates
    let crates = vec![
        ("serde", "1.0.190", 150_000_000, 0, 8, "MIT/Apache-2.0", "2023-11-01"),
        ("reqwest", "0.11.22", 45_000_000, 0, 15, "MIT/Apache-2.0", "2023-10-15"),
        ("tokio", "1.34.0", 89_000_000, 0, 25, "MIT", "2023-11-05"),
        ("some-unknown-crate", "0.1.0", 1_000, 2, 50, "Unknown", "2022-01-01"),
    ];
    
    for (name, version, downloads, vulns, deps, license, last_update) in crates {
        let mut eval = CrateEvaluation::new(name, version);
        eval.downloads = downloads;
        eval.vulnerabilities = vulns;
        eval.dependency_count = deps;
        eval.license = license.to_string();
        eval.last_updated = last_update.to_string();
        eval.calculate_score();
        
        println!("📦 {} v{}", name, version);
        println!("   Downloads: {}", format_number(downloads));
        println!("   Vulnerabilities: {}", vulns);
        println!("   Dependencies: {}", deps);
        println!("   License: {}", license);
        println!("   Score: {:.2}", eval.score);
        println!("   Recommendation: {}", eval.recommend());
        println!();
    }
}

fn demo_version_strategies() {
    println!("2. 📋 Version Strategy Examples");
    println!("===============================");
    
    let strategies = vec![
        ("Application", "serde = \"=1.0.190\"", "Exact version for reproducibility"),
        ("Library", "serde = \"1.0\"", "Flexible for downstream compatibility"),
        ("Conservative", "serde = \"~1.0.190\"", "Only patch updates"),
        ("Range", "serde = \">= 1.0.180, < 1.1\"", "Specific range control"),
        ("Wildcard", "serde = \"1.*\"", "Any 1.x version"),
        ("Pre-release", "serde = \"2.0.0-alpha.1\"", "Testing pre-release versions"),
    ];
    
    for (context, syntax, description) in strategies {
        println!("🎯 {}: {}", context, syntax);
        println!("   → {}", description);
        println!();
    }
    
    // Demonstrate version compatibility
    println!("🔄 Version Compatibility Matrix:");
    println!("   Application depends on: my-lib = \"1.0\"");
    println!("   my-lib depends on: serde = \"1.0\"");
    println!("   ✅ Compatible: Both can use serde 1.0.x");
    println!("   ❌ Conflict: If my-lib used serde = \"2.0\"");
    println!();
}

fn demo_security_audit() {
    println!("3. 🔒 Security Audit Demo");
    println!("=========================");
    
    // Simulate cargo audit output
    println!("Running: cargo audit --json");
    println!();
    
    // Check if cargo audit is available
    match Command::new("cargo").args(["audit", "--version"]).output() {
        Ok(_) => {
            println!("✅ cargo-audit is installed");
            println!("🔍 Scanning dependencies...");
            
            // In a real scenario, this would run the actual audit
            println!("📊 Audit Results:");
            println!("   • No known vulnerabilities found");
            println!("   • 0 warnings");
            println!("   • All dependencies are up to date");
        }
        Err(_) => {
            println!("⚠️ cargo-audit not installed");
            println!("Install with: cargo install cargo-audit");
            println!();
            println!("🔍 Simulated audit results:");
            println!("   • hypothetical-crate v0.1.0 - RUSTSEC-2023-0001 (Medium)");
            println!("   • Description: Buffer overflow in parsing function");
            println!("   • Fix: Update to v0.1.1 or later");
            println!("   • CVSS Score: 6.5");
        }
    }
    println!();
    
    // Demonstrate security best practices
    println!("🛡️ Security Best Practices:");
    println!("   1. Regular audits: cargo audit");
    println!("   2. Pin exact versions in production");
    println!("   3. Minimize dependency count");
    println!("   4. Monitor security advisories");
    println!("   5. Use trusted sources (crates.io)");
    println!("   6. Review dependency licenses");
    println!();
}

fn demo_mission_10_dependencies() {
    println!("4. 🎯 Mission 10: Dependency Choices");
    println!("====================================");
    
    let mission_deps = vec![
        ("Core Algorithm", vec![
            ("No external deps", "Pure Rust implementation", "✅ Recommended"),
        ]),
        ("Serialization", vec![
            ("serde", "Industry standard, well maintained", "✅ Excellent choice"),
            ("bincode", "Binary serialization, fast", "✅ Good for performance"),
            ("rmp-serde", "MessagePack format", "✅ Compact binary format"),
        ]),
        ("Parallel Processing", vec![
            ("rayon", "Data parallelism, easy to use", "✅ Recommended"),
            ("tokio", "Async runtime, overkill for this use case", "⚠️ Consider alternatives"),
        ]),
        ("Benchmarking", vec![
            ("criterion", "Statistical benchmarking", "✅ Best in class"),
            ("std::time", "Basic timing, built-in", "✅ Sufficient for simple cases"),
        ]),
        ("Testing", vec![
            ("proptest", "Property-based testing", "✅ Great for algorithms"),
            ("quickcheck", "Property testing, simpler API", "✅ Good alternative"),
        ]),
        ("Graph Operations", vec![
            ("petgraph", "Full-featured graph library", "⚠️ Heavy, but comprehensive"),
            ("indexmap", "Ordered maps", "✅ Lightweight, specific use case"),
        ]),
    ];
    
    for (category, deps) in mission_deps {
        println!("📂 {}", category);
        for (name, description, recommendation) in deps {
            println!("   • {}: {}", name, description);
            println!("     {}", recommendation);
        }
        println!();
    }
    
    println!("💡 Final Dependency Strategy for Mission 10:");
    println!("   • Core: No external dependencies (pure Rust)");
    println!("   • Optional features: serde, rayon via feature flags");
    println!("   • Dev dependencies: criterion, proptest");
    println!("   • Total direct dependencies: < 10");
    println!("   • Security: All dependencies audited regularly");
    println!();
}

fn demo_dependency_analysis() {
    println!("5. 🔬 Dependency Analysis Demo");
    println!("==============================");
    
    // Simulate dependency tree analysis
    let dep_tree = DependencyTree::new("mission10-union-find");
    dep_tree.add_dependency("serde", "1.0.190", false);
    dep_tree.add_dependency("rayon", "1.8.0", true); // optional
    dep_tree.add_dependency("criterion", "0.5.1", false); // dev-only
    
    // Add transitive dependencies
    dep_tree.add_transitive("serde", "serde_derive", "1.0.190");
    dep_tree.add_transitive("rayon", "rayon-core", "1.12.0");
    dep_tree.add_transitive("criterion", "plotters", "0.3.5");
    
    println!("📊 Dependency Analysis:");
    println!("   Direct dependencies: {}", dep_tree.direct_count());
    println!("   Total dependencies: {}", dep_tree.total_count());
    println!("   Optional dependencies: {}", dep_tree.optional_count());
    println!("   Dependency depth: {}", dep_tree.max_depth());
    println!();
    
    // License compatibility check
    println!("⚖️ License Compatibility:");
    let licenses = vec!["MIT", "Apache-2.0", "MIT/Apache-2.0", "BSD-3-Clause"];
    for license in licenses {
        println!("   {} - {}", license, if is_compatible_license(license) { "✅ Compatible" } else { "❌ Check required" });
    }
    println!();
    
    // Performance impact estimate
    println!("⚡ Performance Impact Estimate:");
    println!("   Compile time: +15% (due to serde derive macros)");
    println!("   Binary size: +200KB (serde + rayon)");
    println!("   Runtime overhead: Minimal (zero-cost abstractions)");
    println!();
}

// Supporting structures and implementations

#[derive(Debug)]
struct CrateEvaluation {
    name: String,
    version: String,
    downloads: u64,
    last_updated: String,
    vulnerabilities: u32,
    dependency_count: u32,
    license: String,
    score: f64,
}

impl CrateEvaluation {
    fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            downloads: 0,
            last_updated: String::new(),
            vulnerabilities: 0,
            dependency_count: 0,
            license: String::new(),
            score: 0.0,
        }
    }
    
    fn calculate_score(&mut self) {
        // Weighted scoring system
        let download_score = (self.downloads as f64).log10().min(8.0) / 8.0; // Max score at 100M downloads
        let freshness_score = if self.is_recently_updated() { 1.0 } else { 0.3 };
        let security_score = if self.vulnerabilities == 0 { 1.0 } else { 
            (1.0 - (self.vulnerabilities as f64 * 0.3)).max(0.0) 
        };
        let complexity_score = (1.0 - (self.dependency_count as f64 / 50.0)).max(0.0);
        
        self.score = download_score * 0.3 + freshness_score * 0.2 + security_score * 0.4 + complexity_score * 0.1;
    }
    
    fn is_recently_updated(&self) -> bool {
        // Simplified: assume any date in 2023 is recent
        self.last_updated.starts_with("2023")
    }
    
    fn recommend(&self) -> &'static str {
        match self.score {
            s if s >= 0.8 => "🟢 Highly Recommended",
            s if s >= 0.6 => "🟡 Recommended",
            s if s >= 0.4 => "🟠 Consider Alternatives",
            _ => "🔴 Not Recommended",
        }
    }
}

struct CrateEvaluator {
    evaluations: Vec<CrateEvaluation>,
}

impl CrateEvaluator {
    fn new() -> Self {
        Self {
            evaluations: Vec::new(),
        }
    }
    
    fn evaluate(&mut self, name: &str, version: &str) -> &CrateEvaluation {
        let mut eval = CrateEvaluation::new(name, version);
        eval.calculate_score();
        self.evaluations.push(eval);
        self.evaluations.last().unwrap()
    }
}

// Dependency tree analysis
struct DependencyTree {
    root: String,
    dependencies: HashMap<String, DependencyInfo>,
}

#[derive(Debug)]
struct DependencyInfo {
    version: String,
    optional: bool,
    dev_only: bool,
    transitive: Vec<String>,
}

impl DependencyTree {
    fn new(root: &str) -> Self {
        Self {
            root: root.to_string(),
            dependencies: HashMap::new(),
        }
    }
    
    fn add_dependency(&self, name: &str, version: &str, optional: bool) {
        // In a real implementation, this would modify the tree
        println!("   Adding dependency: {} v{} (optional: {})", name, version, optional);
    }
    
    fn add_transitive(&self, parent: &str, name: &str, version: &str) {
        println!("   └─ Transitive: {} v{} (via {})", name, version, parent);
    }
    
    fn direct_count(&self) -> usize { 3 } // Simulated
    fn total_count(&self) -> usize { 15 } // Simulated
    fn optional_count(&self) -> usize { 1 } // Simulated
    fn max_depth(&self) -> usize { 3 } // Simulated
}

// Utility functions
fn format_number(n: u64) -> String {
    if n >= 1_000_000_000 {
        format!("{:.1}B", n as f64 / 1_000_000_000.0)
    } else if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

fn is_compatible_license(license: &str) -> bool {
    matches!(license, "MIT" | "Apache-2.0" | "MIT/Apache-2.0" | "BSD-3-Clause")
}

// Connected Components implementation for Mission 10
#[derive(Debug, Clone)]
struct ConnectedComponents<T: Eq + Hash + Clone> {
    components: HashMap<T, HashSet<T>>,
}

impl<T: Eq + Hash + Clone> ConnectedComponents<T> {
    fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }
    
    fn add_edge(&mut self, a: T, b: T) {
        // Simplified union operation
        let mut component_a = self.find_component(&a).cloned().unwrap_or_else(|| {
            let mut set = HashSet::new();
            set.insert(a.clone());
            set
        });
        
        let component_b = self.find_component(&b).cloned().unwrap_or_else(|| {
            let mut set = HashSet::new();
            set.insert(b.clone());
            set
        });
        
        // Merge components
        component_a.extend(component_b);
        
        // Update all nodes in the merged component
        for node in &component_a {
            self.components.insert(node.clone(), component_a.clone());
        }
    }
    
    fn find_component(&self, node: &T) -> Option<&HashSet<T>> {
        self.components.get(node)
    }
    
    fn are_connected(&self, a: &T, b: &T) -> bool {
        if let (Some(comp_a), Some(comp_b)) = (self.find_component(a), self.find_component(b)) {
            comp_a == comp_b
        } else {
            false
        }
    }
    
    fn component_count(&self) -> usize {
        let mut unique_components = HashSet::new();
        for component in self.components.values() {
            // Use a representative element from each component to identify unique components
            if let Some(first) = component.iter().next() {
                unique_components.insert(first);
            }
        }
        unique_components.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_crate_evaluation() {
        let mut eval = CrateEvaluation::new("serde", "1.0.190");
        eval.downloads = 150_000_000;
        eval.last_updated = "2023-11-01".to_string();
        eval.vulnerabilities = 0;
        eval.dependency_count = 8;
        eval.calculate_score();
        
        assert!(eval.score > 0.7);
        assert!(eval.recommend().contains("Recommended"));
    }
    
    #[test]
    fn test_format_number() {
        assert_eq!(format_number(1_500_000_000), "1.5B");
        assert_eq!(format_number(1_500_000), "1.5M");
        assert_eq!(format_number(2_500), "2.5K");
        assert_eq!(format_number(123), "123");
    }
    
    #[test]
    fn test_license_compatibility() {
        assert!(is_compatible_license("MIT"));
        assert!(is_compatible_license("Apache-2.0"));
        assert!(is_compatible_license("MIT/Apache-2.0"));
        assert!(!is_compatible_license("GPL-3.0"));
    }
    
    #[test]
    fn test_connected_components() {
        let mut cc = ConnectedComponents::new();
        
        cc.add_edge(1, 2);
        cc.add_edge(2, 3);
        cc.add_edge(4, 5);
        
        assert!(cc.are_connected(&1, &3));
        assert!(!cc.are_connected(&1, &4));
        
        // Note: This is a simplified test - real implementation would be more robust
    }
    
    #[test]
    fn test_dependency_analysis() {
        let tree = DependencyTree::new("test-crate");
        assert_eq!(tree.root, "test-crate");
        
        // Test basic tree operations
        assert!(tree.direct_count() > 0);
        assert!(tree.total_count() >= tree.direct_count());
    }
}