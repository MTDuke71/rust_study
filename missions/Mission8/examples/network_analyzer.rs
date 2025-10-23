use mission8::*;
use std::collections::HashMap;

/// Network analyzer using BFS and DFS algorithms
///
/// This example demonstrates:
/// - Modeling computer networks as graphs
/// - Finding shortest paths between nodes (BFS)  
/// - Detecting network partitions (connected_components)
/// - Finding critical connections (DFS-based)
/// - Network topology analysis and visualization
fn main() {
    println!("=== Mission 8: Network Analyzer Application ===\n");

    // Analyze different network topologies
    analyze_network("Small Office Network", create_small_office_network());
    analyze_network("Data Center Network", create_data_center_network());
    analyze_network("Campus Network", create_campus_network());
    analyze_network("Partitioned Network", create_partitioned_network());

    println!("🎉 All network analysis demonstrations complete!");
    println!("\n💡 Key learnings:");
    println!("  • Networks map naturally to graph structures");
    println!("  • BFS finds shortest communication paths");
    println!("  • Connected components detect network partitions");
    println!("  • DFS helps identify critical infrastructure");
    println!("  • Graph algorithms provide network health insights");
}

/// Analyze a single network and display comprehensive results
fn analyze_network(name: &str, network: Network) {
    println!("🌐 Analyzing: {}", name);
    println!(
        "📊 Topology: {} nodes, {} connections",
        network.node_count(),
        network.connection_count()
    );

    // Display network topology
    network.display_topology();

    // 1. Connectivity Analysis
    println!("🔗 Connectivity Analysis:");
    let components = connected_components(&network);
    if components.len() == 1 {
        println!("  ✅ Network is fully connected");
        println!("  📡 All nodes can communicate with each other");
    } else {
        println!(
            "  ⚠️  Network has {} isolated partitions:",
            components.len()
        );
        for (i, component) in components.iter().enumerate() {
            println!("    Partition {}: {} nodes", i + 1, component.len());
        }
    }

    // 2. Path Analysis (only if network is connected)
    if components.len() == 1 {
        println!("\n📍 Path Analysis:");
        analyze_communication_paths(&network);

        println!("\n🔍 Network Efficiency:");
        calculate_network_efficiency(&network);
    }

    // 3. Topology Robustness
    println!("\n🛡️ Robustness Analysis:");
    analyze_network_robustness(&network);

    // 4. Critical Node Analysis
    println!("\n⚡ Critical Infrastructure:");
    find_critical_nodes(&network);

    println!("{}\n", "=".repeat(60));
}

/// Analyze communication paths between key nodes
fn analyze_communication_paths(network: &Network) {
    let nodes = network.get_all_nodes();

    if nodes.len() >= 2 {
        // Sample a few important paths
        let test_pairs = vec![
            (&nodes[0], &nodes[nodes.len() - 1]), // First to last
            (&nodes[0], &nodes[nodes.len() / 2]), // First to middle
        ];

        for (&source, &target) in test_pairs {
            match shortest_path(network, source, target) {
                Ok(path) => {
                    let source_info = network.get_node_info(source).unwrap();
                    let target_info = network.get_node_info(target).unwrap();
                    println!(
                        "  📤 {} → {}: {} hops",
                        source_info,
                        target_info,
                        path.len() - 1
                    );
                    println!(
                        "    Route: {}",
                        path.iter()
                            .map(|&n| network.get_node_info(n).unwrap().to_string())
                            .collect::<Vec<_>>()
                            .join(" → ")
                    );
                }
                Err(_) => {
                    let source_info = network.get_node_info(source).unwrap();
                    let target_info = network.get_node_info(target).unwrap();
                    println!("  ❌ {} → {}: No route available", source_info, target_info);
                }
            }
        }
    }
}

/// Calculate network efficiency metrics
fn calculate_network_efficiency(network: &Network) {
    let nodes = network.get_all_nodes();
    let mut total_hops = 0;
    let mut paths_calculated = 0;
    let mut max_hops = 0;

    // Calculate average path length between all node pairs
    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            if let Ok(path) = shortest_path(network, nodes[i], nodes[j]) {
                let hops = path.len() - 1;
                total_hops += hops;
                paths_calculated += 1;
                max_hops = max_hops.max(hops);
            }
        }
    }

    if paths_calculated > 0 {
        let avg_hops = total_hops as f64 / paths_calculated as f64;
        println!("  📏 Average path length: {:.2} hops", avg_hops);
        println!("  📐 Maximum path length: {} hops", max_hops);
        println!(
            "  🎯 Network diameter: {} (worst-case communication delay)",
            max_hops
        );

        // Network efficiency score (lower is better)
        let efficiency_score = avg_hops / nodes.len() as f64;
        println!(
            "  ⚡ Efficiency score: {:.3} (lower = more efficient)",
            efficiency_score
        );
    }
}

/// Analyze network robustness by simulating node failures
fn analyze_network_robustness(network: &Network) {
    let nodes = network.get_all_nodes();
    let original_components = connected_components(network).len();

    println!(
        "  🔧 Original network: {} partition(s)",
        original_components
    );

    // Test removing each node to see impact on connectivity
    let mut critical_failures = 0;
    let mut worst_case_partitions = original_components;

    for &node in &nodes {
        let reduced_network = network.without_node(&node);
        let new_components = connected_components(&reduced_network).len();

        if new_components > original_components {
            critical_failures += 1;
            worst_case_partitions = worst_case_partitions.max(new_components);
        }
    }

    println!(
        "  ⚠️  Critical single points of failure: {}/{}",
        critical_failures,
        nodes.len()
    );
    if critical_failures > 0 {
        println!(
            "  💥 Worst case: {} partitions if critical node fails",
            worst_case_partitions
        );
    } else {
        println!("  ✅ Network survives any single node failure");
    }
}

/// Find nodes that are critical for network connectivity
fn find_critical_nodes(network: &Network) {
    let nodes = network.get_all_nodes();
    let original_components = connected_components(network).len();

    let mut critical_nodes = Vec::new();

    for &node in &nodes {
        let reduced_network = network.without_node(&node);
        let new_components = connected_components(&reduced_network).len();

        if new_components > original_components {
            let connection_count = network.neighbors(node).len();
            critical_nodes.push((node, connection_count, new_components - original_components));
        }
    }

    if critical_nodes.is_empty() {
        println!("  ✅ No critical single points of failure detected");
    } else {
        println!("  ⚡ Critical nodes (removal causes network partition):");
        critical_nodes.sort_by_key(|(_, connections, _)| std::cmp::Reverse(*connections));

        for (node, connections, partitions_added) in critical_nodes {
            let node_info = network.get_node_info(node).unwrap();
            println!(
                "    🔴 {}: {} connections, +{} partition(s) if removed",
                node_info, connections, partitions_added
            );
        }
    }
}

/// Represents a network node (computer, server, router, etc.)
/// Using u32 ID with lookup table for efficiency with Graph trait
pub type NetworkNode = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeType {
    Computer,
    Server,
    Router,
    Switch,
    Gateway,
}

/// Node metadata for display and analysis
#[derive(Debug, Clone)]
pub struct NodeInfo {
    pub name: String,
    pub node_type: NodeType,
}

impl NodeInfo {
    pub fn new(name: &str, node_type: NodeType) -> Self {
        Self {
            name: name.to_string(),
            node_type,
        }
    }
}

impl std::fmt::Display for NodeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_symbol = match self.node_type {
            NodeType::Computer => "💻",
            NodeType::Server => "🖥️",
            NodeType::Router => "📡",
            NodeType::Switch => "🔀",
            NodeType::Gateway => "🌐",
        };
        write!(f, "{}{}", type_symbol, self.name)
    }
}

/// Network structure with nodes and connections
#[derive(Debug, Clone)]
pub struct Network {
    connections: HashMap<NetworkNode, Vec<NetworkNode>>,
    node_info: HashMap<NetworkNode, NodeInfo>,
    next_id: NetworkNode,
}

impl Default for Network {
    fn default() -> Self {
        Self::new()
    }
}

impl Network {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
            node_info: HashMap::new(),
            next_id: 0,
        }
    }

    /// Add a new node to the network and return its ID
    pub fn add_node(&mut self, name: &str, node_type: NodeType) -> NetworkNode {
        let node_id = self.next_id;
        self.next_id += 1;

        self.node_info
            .insert(node_id, NodeInfo::new(name, node_type));
        self.connections.insert(node_id, Vec::new());

        node_id
    }

    /// Add a bidirectional connection between two nodes
    pub fn add_connection(&mut self, node1: NetworkNode, node2: NetworkNode) {
        self.connections.entry(node1).or_default().push(node2);
        self.connections.entry(node2).or_default().push(node1);
    }

    /// Get node information by ID
    pub fn get_node_info(&self, node_id: NetworkNode) -> Option<&NodeInfo> {
        self.node_info.get(&node_id)
    }

    /// Get all nodes in the network
    pub fn get_all_nodes(&self) -> Vec<NetworkNode> {
        self.connections.keys().copied().collect()
    }

    /// Count total nodes
    pub fn node_count(&self) -> usize {
        self.connections.len()
    }

    /// Count total connections (bidirectional connections counted once)
    pub fn connection_count(&self) -> usize {
        self.connections
            .values()
            .map(|neighbors| neighbors.len())
            .sum::<usize>()
            / 2 // Divide by 2 because each connection is stored twice
    }

    /// Display network topology
    pub fn display_topology(&self) {
        println!("🗺️  Network Topology:");
        let mut nodes: Vec<_> = self.connections.keys().copied().collect();
        nodes.sort();

        for node in nodes {
            let neighbors = &self.connections[&node];
            let node_info = &self.node_info[&node];

            if neighbors.is_empty() {
                println!("  {} (isolated)", node_info);
            } else {
                let connections: Vec<String> = neighbors
                    .iter()
                    .map(|&n| self.node_info[&n].to_string())
                    .collect();
                println!("  {} ↔ [{}]", node_info, connections.join(", "));
            }
        }
    }

    /// Create network with one node removed (for robustness testing)
    pub fn without_node(&self, removed_node: &NetworkNode) -> Self {
        let mut new_network = Network::new();

        // Copy node info (except for removed node)
        for (&node_id, node_info) in &self.node_info {
            if node_id != *removed_node {
                new_network.node_info.insert(node_id, node_info.clone());
            }
        }

        // Copy connections (excluding removed node)
        for (&node, neighbors) in &self.connections {
            if node != *removed_node {
                let filtered_neighbors: Vec<_> = neighbors
                    .iter()
                    .copied()
                    .filter(|&n| n != *removed_node)
                    .collect();
                new_network.connections.insert(node, filtered_neighbors);
            }
        }

        new_network.next_id = self.next_id; // Maintain ID sequence
        new_network
    }
}

/// Implement Graph trait for Network to enable BFS/DFS algorithms
impl Graph for Network {
    type Node = NetworkNode;

    fn neighbors(&self, node: Self::Node) -> Vec<Self::Node> {
        self.connections.get(&node).cloned().unwrap_or_default()
    }

    fn contains(&self, node: Self::Node) -> bool {
        self.connections.contains_key(&node)
    }

    fn nodes(&self) -> Vec<Self::Node> {
        self.connections.keys().cloned().collect()
    }
}

// Network creation functions for different topologies

fn create_small_office_network() -> Network {
    let mut network = Network::new();

    // Create nodes
    let router = network.add_node("Router1", NodeType::Router);
    let switch = network.add_node("Switch1", NodeType::Switch);
    let server = network.add_node("FileServer", NodeType::Server);
    let pc1 = network.add_node("PC1", NodeType::Computer);
    let pc2 = network.add_node("PC2", NodeType::Computer);
    let pc3 = network.add_node("PC3", NodeType::Computer);
    let gateway = network.add_node("Internet", NodeType::Gateway);

    // Create connections (star topology with central switch)
    network.add_connection(router, gateway);
    network.add_connection(router, switch);
    network.add_connection(switch, server);
    network.add_connection(switch, pc1);
    network.add_connection(switch, pc2);
    network.add_connection(switch, pc3);

    network
}

fn create_data_center_network() -> Network {
    let mut network = Network::new();

    // Create core infrastructure
    let core_router = network.add_node("CoreRouter", NodeType::Router);
    let switch1 = network.add_node("Switch1", NodeType::Switch);
    let switch2 = network.add_node("Switch2", NodeType::Switch);
    let gateway = network.add_node("Internet", NodeType::Gateway);

    // Create servers
    let web_server1 = network.add_node("WebServer1", NodeType::Server);
    let web_server2 = network.add_node("WebServer2", NodeType::Server);
    let db_server = network.add_node("DBServer", NodeType::Server);
    let backup_server = network.add_node("BackupServer", NodeType::Server);

    // Create redundant connections for high availability
    network.add_connection(core_router, gateway);
    network.add_connection(core_router, switch1);
    network.add_connection(core_router, switch2);
    network.add_connection(switch1, switch2); // Redundant link

    // Connect servers to both switches for redundancy
    network.add_connection(switch1, web_server1);
    network.add_connection(switch2, web_server1);
    network.add_connection(switch1, web_server2);
    network.add_connection(switch2, web_server2);
    network.add_connection(switch1, db_server);
    network.add_connection(switch2, db_server);
    network.add_connection(switch1, backup_server);
    network.add_connection(switch2, backup_server);

    network
}

fn create_campus_network() -> Network {
    let mut network = Network::new();

    // Create building networks
    let main_router = network.add_node("MainRouter", NodeType::Router);
    let building_a_router = network.add_node("BuildingA", NodeType::Router);
    let building_b_router = network.add_node("BuildingB", NodeType::Router);
    let library_router = network.add_node("Library", NodeType::Router);
    let gateway = network.add_node("Internet", NodeType::Gateway);

    // Create department networks
    let cs_server = network.add_node("CSServer", NodeType::Server);
    let cs_lab1 = network.add_node("CSLab1", NodeType::Computer);
    let cs_lab2 = network.add_node("CSLab2", NodeType::Computer);

    let admin_server = network.add_node("AdminServer", NodeType::Server);
    let admin_pc1 = network.add_node("AdminPC1", NodeType::Computer);
    let admin_pc2 = network.add_node("AdminPC2", NodeType::Computer);

    let lib_server = network.add_node("LibServer", NodeType::Server);
    let lib_pc1 = network.add_node("LibPC1", NodeType::Computer);
    let lib_pc2 = network.add_node("LibPC2", NodeType::Computer);

    // Create backbone connections
    network.add_connection(main_router, gateway);
    network.add_connection(main_router, building_a_router);
    network.add_connection(main_router, building_b_router);
    network.add_connection(main_router, library_router);

    // Building A (Computer Science)
    network.add_connection(building_a_router, cs_server);
    network.add_connection(building_a_router, cs_lab1);
    network.add_connection(cs_server, cs_lab2);

    // Building B (Administration)
    network.add_connection(building_b_router, admin_server);
    network.add_connection(building_b_router, admin_pc1);
    network.add_connection(admin_server, admin_pc2);

    // Library
    network.add_connection(library_router, lib_server);
    network.add_connection(library_router, lib_pc1);
    network.add_connection(lib_server, lib_pc2);

    network
}

fn create_partitioned_network() -> Network {
    let mut network = Network::new();

    // Create two separate network partitions

    // Partition 1: Office network
    let office_router = network.add_node("OfficeRouter", NodeType::Router);
    let office_pc1 = network.add_node("OfficePC1", NodeType::Computer);
    let office_pc2 = network.add_node("OfficePC2", NodeType::Computer);

    network.add_connection(office_router, office_pc1);
    network.add_connection(office_router, office_pc2);

    // Partition 2: Remote lab (completely isolated)
    let lab_server = network.add_node("RemoteLabServer", NodeType::Server);
    let lab_pc1 = network.add_node("LabPC1", NodeType::Computer);
    let lab_pc2 = network.add_node("LabPC2", NodeType::Computer);

    network.add_connection(lab_server, lab_pc1);
    network.add_connection(lab_server, lab_pc2);

    // Isolated node (no connections)
    network.add_node("IsolatedPC", NodeType::Computer);

    network
}
