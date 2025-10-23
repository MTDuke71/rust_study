use mission8::*;
use std::collections::HashMap;

/// Dependency resolver using DFS for cycle detection and topological analysis
///
/// This example demonstrates:
/// - Modeling software dependencies as directed graphs
/// - Detecting circular dependencies (DFS-based cycle detection)
/// - Finding dependency chains and critical packages
/// - Analyzing package vulnerability impact
/// - Topological insights for build order planning
fn main() {
    println!("=== Mission 8: Dependency Resolver Application ===\n");

    // Analyze different dependency scenarios
    analyze_dependencies("Simple Library Project", create_simple_dependencies());
    analyze_dependencies("Complex Web Application", create_complex_dependencies());
    analyze_dependencies("Circular Dependencies", create_circular_dependencies());
    analyze_dependencies("Enterprise Application", create_enterprise_dependencies());

    println!("🎉 All dependency analysis demonstrations complete!");
    println!("\n💡 Key learnings:");
    println!("  • Dependencies form directed acyclic graphs (DAGs) when healthy");
    println!("  • DFS detects circular dependency problems efficiently");
    println!("  • Critical dependencies impact many downstream packages");
    println!("  • Dependency depth affects build time and stability");
    println!("  • Graph algorithms provide insights for package management");
}

/// Analyze a dependency tree and display comprehensive results
fn analyze_dependencies(name: &str, deps: DependencyGraph) {
    println!("📦 Analyzing: {}", name);
    println!(
        "📊 Overview: {} packages, {} dependencies",
        deps.package_count(),
        deps.dependency_count()
    );

    // Display dependency structure
    deps.display_structure();

    // 1. Circular Dependency Detection
    println!("🔄 Circular Dependency Analysis:");
    if let Some(cycle) = detect_dependency_cycles(&deps) {
        println!("  ❌ CIRCULAR DEPENDENCY DETECTED!");
        println!(
            "  🔴 Problematic cycle: {}",
            cycle
                .iter()
                .map(|&pkg| deps.get_package_info(pkg).unwrap().name.as_str())
                .collect::<Vec<_>>()
                .join(" → ")
        );
        println!("  💡 Resolution: Remove one dependency from the cycle");
    } else {
        println!("  ✅ No circular dependencies found");
        println!("  📈 Dependency graph is acyclic (healthy)");
    }

    // 2. Critical Package Analysis
    println!("\n🎯 Critical Package Analysis:");
    analyze_critical_packages(&deps);

    // 3. Dependency Depth Analysis
    println!("\n📏 Dependency Depth Analysis:");
    analyze_dependency_depths(&deps);

    // 4. Security Impact Analysis
    println!("\n🛡️ Security Impact Analysis:");
    analyze_security_impact(&deps);

    // 5. Build Order Suggestions
    if detect_dependency_cycles(&deps).is_none() {
        println!("\n🔨 Build Order Analysis:");
        suggest_build_order(&deps);
    }

    println!("{}\n", "=".repeat(60));
}

/// Detect circular dependencies using DFS
fn detect_dependency_cycles(deps: &DependencyGraph) -> Option<Vec<PackageId>> {
    let packages = deps.get_all_packages();

    // Try to find any cycle starting from each package
    for &start_package in &packages {
        if let Some(cycle) = find_cycle_from_package(deps, start_package) {
            return Some(cycle);
        }
    }

    None
}

/// Find cycle starting from a specific package using DFS
fn find_cycle_from_package(deps: &DependencyGraph, start: PackageId) -> Option<Vec<PackageId>> {
    let mut visited = std::collections::HashSet::new();
    let mut path = Vec::new();

    fn dfs_cycle_detection(
        deps: &DependencyGraph,
        current: PackageId,
        visited: &mut std::collections::HashSet<PackageId>,
        path: &mut Vec<PackageId>,
    ) -> Option<Vec<PackageId>> {
        if path.contains(&current) {
            // Found a cycle! Extract the cycle from the path
            let cycle_start = path.iter().position(|&p| p == current).unwrap();
            let mut cycle = path[cycle_start..].to_vec();
            cycle.push(current); // Complete the cycle
            return Some(cycle);
        }

        if visited.contains(&current) {
            return None; // Already explored this path
        }

        visited.insert(current);
        path.push(current);

        // Explore all dependencies
        for dependency in deps.neighbors(current) {
            if let Some(cycle) = dfs_cycle_detection(deps, dependency, visited, path) {
                return Some(cycle);
            }
        }

        path.pop();
        None
    }

    dfs_cycle_detection(deps, start, &mut visited, &mut path)
}

/// Analyze critical packages (those that many others depend on)
fn analyze_critical_packages(deps: &DependencyGraph) {
    let packages = deps.get_all_packages();
    let mut dependents_count: HashMap<PackageId, usize> = HashMap::new();

    // Count how many packages depend on each package
    for &package in &packages {
        dependents_count.insert(package, 0);
    }

    for &package in &packages {
        for dependency in deps.neighbors(package) {
            *dependents_count.entry(dependency).or_insert(0) += 1;
        }
    }

    // Find most critical packages
    let mut critical_packages: Vec<_> = dependents_count
        .iter()
        .filter(|(_, &count)| count > 0)
        .collect();
    critical_packages.sort_by_key(|(_, &count)| std::cmp::Reverse(count));

    if critical_packages.is_empty() {
        println!("  📦 No shared dependencies found (all packages are independent)");
    } else {
        println!("  🔥 Most critical packages (high dependent count):");
        for (package_id, &dependent_count) in critical_packages.iter().take(5) {
            let package_info = deps.get_package_info(**package_id).unwrap();
            println!(
                "    🎯 {}: {} packages depend on it",
                package_info.name, dependent_count
            );
        }

        // Risk analysis
        let highest_count = critical_packages[0].1;
        if *highest_count >= packages.len() / 2 {
            println!("  ⚠️  HIGH RISK: Some packages have many dependents");
            println!("  💡 Consider alternatives to reduce single points of failure");
        }
    }
}

/// Analyze dependency depths (how deep the dependency chains go)
fn analyze_dependency_depths(deps: &DependencyGraph) {
    let packages = deps.get_all_packages();
    let mut max_depth = 0;
    let mut depth_distribution: HashMap<usize, usize> = HashMap::new();

    for &package in &packages {
        let depth = calculate_dependency_depth(deps, package);
        max_depth = max_depth.max(depth);
        *depth_distribution.entry(depth).or_insert(0) += 1;
    }

    println!("  📐 Maximum dependency depth: {} levels", max_depth);
    println!("  📊 Depth distribution:");

    for depth in 0..=max_depth {
        let count = depth_distribution.get(&depth).unwrap_or(&0);
        println!("    Level {}: {} packages", depth, count);
    }

    // Analysis insights
    if max_depth > 5 {
        println!(
            "  ⚠️  Deep dependency chains detected ({}+ levels)",
            max_depth
        );
        println!("  💡 Consider flattening dependencies to reduce complexity");
    } else {
        println!("  ✅ Reasonable dependency depth (≤5 levels)");
    }
}

/// Calculate the maximum dependency depth for a package
fn calculate_dependency_depth(deps: &DependencyGraph, package: PackageId) -> usize {
    let mut visited = std::collections::HashSet::new();

    fn dfs_depth(
        deps: &DependencyGraph,
        current: PackageId,
        visited: &mut std::collections::HashSet<PackageId>,
    ) -> usize {
        if visited.contains(&current) {
            return 0; // Avoid infinite recursion
        }

        visited.insert(current);

        let max_child_depth = deps
            .neighbors(current)
            .iter()
            .map(|&dep| dfs_depth(deps, dep, visited))
            .max()
            .unwrap_or(0);

        visited.remove(&current);
        max_child_depth + 1
    }

    dfs_depth(deps, package, &mut visited).saturating_sub(1)
}

/// Analyze security impact of vulnerable packages
fn analyze_security_impact(deps: &DependencyGraph) {
    let packages = deps.get_all_packages();

    // Simulate vulnerable packages (in real scenario, this would come from security database)
    let vulnerable_packages: Vec<_> = packages
        .iter()
        .filter(|&&pkg| {
            let info = deps.get_package_info(pkg).unwrap();
            info.name.contains("old")
                || info.name.contains("legacy")
                || info.version.starts_with("1.") // Simulate old major versions
        })
        .copied()
        .collect();

    if vulnerable_packages.is_empty() {
        println!("  ✅ No obviously vulnerable packages detected");
        println!("  💡 This is a simplified analysis - use security scanners in production");
    } else {
        println!("  🔴 Potentially vulnerable packages found:");
        for &vulnerable_pkg in &vulnerable_packages {
            let info = deps.get_package_info(vulnerable_pkg).unwrap();

            // Find packages that transitively depend on this vulnerable package
            let affected_packages = find_transitive_dependents(deps, vulnerable_pkg);

            println!(
                "    ⚠️  {}: affects {} packages transitively",
                info.name,
                affected_packages.len()
            );

            if affected_packages.len() > packages.len() / 3 {
                println!("      🚨 HIGH IMPACT: Affects >1/3 of codebase");
            }
        }

        println!("  💡 Priority: Update vulnerable packages with high transitive impact");
    }
}

/// Find all packages that transitively depend on a given package
fn find_transitive_dependents(deps: &DependencyGraph, target: PackageId) -> Vec<PackageId> {
    let packages = deps.get_all_packages();
    let mut dependents = Vec::new();

    for &package in &packages {
        if package != target && has_transitive_dependency(deps, package, target) {
            dependents.push(package);
        }
    }

    dependents
}

/// Check if package A has a transitive dependency on package B
fn has_transitive_dependency(deps: &DependencyGraph, from: PackageId, to: PackageId) -> bool {
    let mut visited = std::collections::HashSet::new();

    fn dfs_search(
        deps: &DependencyGraph,
        current: PackageId,
        target: PackageId,
        visited: &mut std::collections::HashSet<PackageId>,
    ) -> bool {
        if current == target {
            return true;
        }

        if visited.contains(&current) {
            return false;
        }

        visited.insert(current);

        for dependency in deps.neighbors(current) {
            if dfs_search(deps, dependency, target, visited) {
                return true;
            }
        }

        false
    }

    dfs_search(deps, from, to, &mut visited)
}

/// Suggest build order for packages (topological sort concept)
fn suggest_build_order(deps: &DependencyGraph) {
    let packages = deps.get_all_packages();

    // Calculate dependency counts for simple ordering
    let mut dependency_counts: Vec<_> = packages
        .iter()
        .map(|&pkg| {
            let direct_deps = deps.neighbors(pkg).len();
            let total_depth = calculate_dependency_depth(deps, pkg);
            (pkg, direct_deps, total_depth)
        })
        .collect();

    // Sort by dependency depth (packages with fewer dependencies build first)
    dependency_counts.sort_by_key(|(_, direct, total)| (*total, *direct));

    println!("  🔨 Suggested build order (low to high complexity):");
    for (i, (pkg, direct_deps, total_depth)) in dependency_counts.iter().enumerate() {
        let info = deps.get_package_info(*pkg).unwrap();
        println!(
            "    {}. {} (deps: {}, depth: {})",
            i + 1,
            info.name,
            direct_deps,
            total_depth
        );
    }

    println!("  💡 Build order rationale: Dependencies → Dependents");
}

/// Package information
#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub package_type: PackageType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageType {
    Library,
    Framework,
    Tool,
    Application,
}

impl PackageInfo {
    pub fn new(name: &str, version: &str, package_type: PackageType) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            package_type,
        }
    }
}

/// Package ID type for efficient graph operations
pub type PackageId = u32;

/// Dependency graph structure
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    dependencies: HashMap<PackageId, Vec<PackageId>>, // package -> its dependencies
    package_info: HashMap<PackageId, PackageInfo>,
    next_id: PackageId,
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            package_info: HashMap::new(),
            next_id: 0,
        }
    }

    /// Add a new package and return its ID
    pub fn add_package(
        &mut self,
        name: &str,
        version: &str,
        package_type: PackageType,
    ) -> PackageId {
        let package_id = self.next_id;
        self.next_id += 1;

        self.package_info
            .insert(package_id, PackageInfo::new(name, version, package_type));
        self.dependencies.insert(package_id, Vec::new());

        package_id
    }

    /// Add a dependency relationship (from depends on to)
    pub fn add_dependency(&mut self, from: PackageId, to: PackageId) {
        self.dependencies.entry(from).or_default().push(to);
    }

    /// Get package information by ID
    pub fn get_package_info(&self, package_id: PackageId) -> Option<&PackageInfo> {
        self.package_info.get(&package_id)
    }

    /// Get all packages
    pub fn get_all_packages(&self) -> Vec<PackageId> {
        self.dependencies.keys().copied().collect()
    }

    /// Count total packages
    pub fn package_count(&self) -> usize {
        self.dependencies.len()
    }

    /// Count total dependencies
    pub fn dependency_count(&self) -> usize {
        self.dependencies.values().map(|deps| deps.len()).sum()
    }

    /// Display dependency structure
    pub fn display_structure(&self) {
        println!("🗺️  Dependency Structure:");
        let mut packages: Vec<_> = self.dependencies.keys().copied().collect();
        packages.sort();

        for package in packages {
            let dependencies = &self.dependencies[&package];
            let package_info = &self.package_info[&package];

            if dependencies.is_empty() {
                println!(
                    "  📦 {} v{} (no dependencies)",
                    package_info.name, package_info.version
                );
            } else {
                let dep_names: Vec<String> = dependencies
                    .iter()
                    .map(|&dep| {
                        let dep_info = &self.package_info[&dep];
                        format!("{} v{}", dep_info.name, dep_info.version)
                    })
                    .collect();
                println!(
                    "  📦 {} v{} → [{}]",
                    package_info.name,
                    package_info.version,
                    dep_names.join(", ")
                );
            }
        }
    }
}

/// Implement Graph trait for DependencyGraph
impl Graph for DependencyGraph {
    type Node = PackageId;

    fn neighbors(&self, node: Self::Node) -> Vec<Self::Node> {
        self.dependencies.get(&node).cloned().unwrap_or_default()
    }

    fn contains(&self, node: Self::Node) -> bool {
        self.dependencies.contains_key(&node)
    }

    fn nodes(&self) -> Vec<Self::Node> {
        self.dependencies.keys().copied().collect()
    }
}

// Dependency creation functions for different scenarios

fn create_simple_dependencies() -> DependencyGraph {
    let mut deps = DependencyGraph::new();

    // Create packages
    let utils = deps.add_package("utils", "2.1.0", PackageType::Library);
    let json = deps.add_package("json-parser", "1.5.0", PackageType::Library);
    let http = deps.add_package("http-client", "3.0.0", PackageType::Library);
    let app = deps.add_package("my-app", "1.0.0", PackageType::Application);

    // Add dependencies
    deps.add_dependency(app, http);
    deps.add_dependency(app, json);
    deps.add_dependency(http, utils);
    deps.add_dependency(json, utils);

    deps
}

fn create_complex_dependencies() -> DependencyGraph {
    let mut deps = DependencyGraph::new();

    // Core libraries
    let core = deps.add_package("core-utils", "4.2.0", PackageType::Library);
    let crypto = deps.add_package("crypto", "2.8.0", PackageType::Library);
    let logging = deps.add_package("logging", "1.3.0", PackageType::Library);

    // Middleware
    let auth = deps.add_package("auth-middleware", "2.1.0", PackageType::Framework);
    let validation = deps.add_package("validation", "1.7.0", PackageType::Library);
    let database = deps.add_package("database-orm", "3.5.0", PackageType::Framework);

    // Web framework
    let web_framework = deps.add_package("web-framework", "5.0.0", PackageType::Framework);
    let router = deps.add_package("router", "2.3.0", PackageType::Library);

    // Application
    let api = deps.add_package("api-server", "1.2.0", PackageType::Application);
    let frontend = deps.add_package("frontend-app", "2.0.0", PackageType::Application);

    // Build core dependencies
    deps.add_dependency(auth, crypto);
    deps.add_dependency(auth, core);
    deps.add_dependency(validation, core);
    deps.add_dependency(database, core);
    deps.add_dependency(database, logging);

    // Web framework dependencies
    deps.add_dependency(web_framework, router);
    deps.add_dependency(web_framework, auth);
    deps.add_dependency(web_framework, validation);
    deps.add_dependency(router, core);

    // Application dependencies
    deps.add_dependency(api, web_framework);
    deps.add_dependency(api, database);
    deps.add_dependency(api, logging);
    deps.add_dependency(frontend, api); // Frontend calls API

    deps
}

fn create_circular_dependencies() -> DependencyGraph {
    let mut deps = DependencyGraph::new();

    // Create packages with circular dependency
    let package_a = deps.add_package("package-a", "1.0.0", PackageType::Library);
    let package_b = deps.add_package("package-b", "1.0.0", PackageType::Library);
    let package_c = deps.add_package("package-c", "1.0.0", PackageType::Library);
    let utils = deps.add_package("utils", "2.0.0", PackageType::Library);

    // Create circular dependency: A → B → C → A
    deps.add_dependency(package_a, package_b);
    deps.add_dependency(package_b, package_c);
    deps.add_dependency(package_c, package_a); // This creates the cycle!

    // Add some normal dependencies
    deps.add_dependency(package_a, utils);
    deps.add_dependency(package_b, utils);

    deps
}

fn create_enterprise_dependencies() -> DependencyGraph {
    let mut deps = DependencyGraph::new();

    // Foundation layer
    let core = deps.add_package("enterprise-core", "6.1.0", PackageType::Framework);
    let config = deps.add_package("config-manager", "2.4.0", PackageType::Library);
    let monitoring = deps.add_package("monitoring", "3.2.0", PackageType::Tool);
    let security = deps.add_package("security-framework", "4.1.0", PackageType::Framework);

    // Data layer
    let cache = deps.add_package("distributed-cache", "5.0.0", PackageType::Library);
    let database = deps.add_package("enterprise-db", "8.2.0", PackageType::Framework);
    let search = deps.add_package("search-engine", "7.1.0", PackageType::Library);

    // Service layer
    let messaging = deps.add_package("message-queue", "4.3.0", PackageType::Framework);
    let workflows = deps.add_package("workflow-engine", "3.8.0", PackageType::Framework);
    let scheduler = deps.add_package("job-scheduler", "2.6.0", PackageType::Tool);

    // Legacy systems (potentially vulnerable)
    let legacy_connector = deps.add_package("legacy-connector", "1.9.0", PackageType::Library);
    let old_parser = deps.add_package("old-xml-parser", "1.2.0", PackageType::Library);

    // Applications
    let user_service = deps.add_package("user-service", "3.1.0", PackageType::Application);
    let order_service = deps.add_package("order-service", "2.7.0", PackageType::Application);
    let admin_dashboard = deps.add_package("admin-dashboard", "4.0.0", PackageType::Application);

    // Foundation dependencies
    deps.add_dependency(cache, core);
    deps.add_dependency(database, core);
    deps.add_dependency(database, config);
    deps.add_dependency(search, core);
    deps.add_dependency(messaging, core);
    deps.add_dependency(workflows, core);
    deps.add_dependency(workflows, messaging);
    deps.add_dependency(scheduler, core);
    deps.add_dependency(security, core);

    // Legacy system dependencies
    deps.add_dependency(legacy_connector, old_parser);
    deps.add_dependency(legacy_connector, core);

    // Service dependencies
    deps.add_dependency(user_service, database);
    deps.add_dependency(user_service, cache);
    deps.add_dependency(user_service, security);
    deps.add_dependency(user_service, monitoring);

    deps.add_dependency(order_service, database);
    deps.add_dependency(order_service, messaging);
    deps.add_dependency(order_service, workflows);
    deps.add_dependency(order_service, legacy_connector); // Uses legacy system
    deps.add_dependency(order_service, search);

    deps.add_dependency(admin_dashboard, user_service);
    deps.add_dependency(admin_dashboard, order_service);
    deps.add_dependency(admin_dashboard, monitoring);
    deps.add_dependency(admin_dashboard, scheduler);

    deps
}
