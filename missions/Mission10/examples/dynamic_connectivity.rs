//! # Dynamic Connectivity with Union-Find
//!
//! This example demonstrates how Union-Find excels at handling dynamic connectivity
//! queries in networks. It's perfect for scenarios where edges are added over time
//! and you need to answer connectivity queries efficiently.
//!
//! ## Use Cases
//!
//! - **Network monitoring**: Track connectivity as links come online
//! - **Social networks**: Monitor friend connections and community formation  
//! - **Transportation**: Track route connectivity as roads/flights are added
//! - **Communication networks**: Monitor network partitions and healing
//!
//! ## Time Complexity
//!
//! - **Add connection**: O(α(n)) amortized
//! - **Check connectivity**: O(α(n)) amortized  
//! - **Get component count**: O(1)
//! - **Get component size**: O(α(n)) amortized

use mission10::UnionFind;
use std::collections::HashMap;
use std::fmt;

/// Represents a dynamic network that supports online connectivity queries
pub struct DynamicNetwork {
    uf: UnionFind,
    node_names: HashMap<usize, String>,
    connection_history: Vec<Connection>,
    next_node_id: usize,
}

/// Represents a connection between two nodes
#[derive(Debug, Clone)]
pub struct Connection {
    pub from: usize,
    pub to: usize,
    pub from_name: String,
    pub to_name: String,
    pub timestamp: usize,
    pub connected_components_after: usize,
}

impl Connection {
    fn new(from: usize, to: usize, from_name: String, to_name: String, 
           timestamp: usize, components_after: usize) -> Self {
        Self {
            from,
            to,
            from_name,
            to_name,
            timestamp,
            connected_components_after: components_after,
        }
    }
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "T{}: {} <-> {} (components: {})", 
               self.timestamp, self.from_name, self.to_name, 
               self.connected_components_after)
    }
}

impl DynamicNetwork {
    /// Create a new dynamic network with specified capacity
    pub fn new(max_nodes: usize) -> Self {
        Self {
            uf: UnionFind::new(max_nodes),
            node_names: HashMap::new(),
            connection_history: Vec::new(),
            next_node_id: 0,
        }
    }

    /// Add a named node to the network and return its ID
    pub fn add_node(&mut self, name: String) -> Result<usize, String> {
        if self.next_node_id >= self.uf.len() {
            return Err("Network at maximum capacity".to_string());
        }
        
        let node_id = self.next_node_id;
        self.node_names.insert(node_id, name);
        self.next_node_id += 1;
        
        Ok(node_id)
    }

    /// Add a connection between two nodes
    pub fn add_connection(&mut self, from: usize, to: usize) -> Result<bool, String> {
        let from_name = self.get_node_name(from)?;
        let to_name = self.get_node_name(to)?;
        
        let was_connected = self.uf.connected(from, to)?;
        let connection_added = self.uf.union(from, to)?;
        
        // Record this connection in history
        let connection = Connection::new(
            from, 
            to, 
            from_name, 
            to_name, 
            self.connection_history.len() + 1,
            self.uf.count()
        );
        
        self.connection_history.push(connection);
        
        if was_connected {
            println!("  ℹ️  {} and {} were already connected", 
                     self.get_node_name(from)?, self.get_node_name(to)?);
        }
        
        Ok(connection_added)
    }

    /// Check if two nodes are connected
    pub fn are_connected(&mut self, from: usize, to: usize) -> Result<bool, String> {
        self.uf.connected(from, to)
    }

    /// Get the size of the component containing the given node
    pub fn component_size(&mut self, node: usize) -> Result<usize, String> {
        self.uf.size(node)
    }

    /// Get the total number of connected components
    pub fn component_count(&self) -> usize {
        self.uf.count()
    }

    /// Get node name by ID
    pub fn get_node_name(&self, node_id: usize) -> Result<String, String> {
        self.node_names.get(&node_id)
            .cloned()
            .ok_or_else(|| format!("Node {} not found", node_id))
    }

    /// Print current network status
    pub fn print_status(&mut self) -> Result<(), String> {
        println!("📊 Network Status:");
        println!("   Total nodes: {}", self.next_node_id);
        println!("   Connected components: {}", self.component_count());
        println!("   Total connections made: {}", self.connection_history.len());
        
        // Group nodes by component
        let mut components: HashMap<usize, Vec<(usize, String)>> = HashMap::new();
        
        for node_id in 0..self.next_node_id {
            let root = self.uf.find(node_id)?;
            let name = self.get_node_name(node_id)?;
            components.entry(root).or_insert_with(Vec::new).push((node_id, name));
        }
        
        println!("   Components:");
        for (i, (root, nodes)) in components.iter().enumerate() {
            let names: Vec<String> = nodes.iter().map(|(_, name)| name.clone()).collect();
            println!("     {}: {} (size: {}, root: {})", 
                     i + 1, names.join(", "), nodes.len(), root);
        }
        
        Ok(())
    }

    /// Print connection history
    pub fn print_history(&self) {
        println!("📜 Connection History:");
        for connection in &self.connection_history {
            println!("   {}", connection);
        }
    }

    /// Simulate a network partition healing scenario
    pub fn simulate_partition_healing(&mut self) -> Result<(), String> {
        println!("🔄 Simulating network partition healing...");
        
        // Initially, we should have multiple components
        let initial_components = self.component_count();
        println!("   Starting with {} components", initial_components);
        
        if initial_components == 1 {
            println!("   Network is already fully connected!");
            return Ok(());
        }
        
        // Find nodes in different components to connect
        let mut components_map: HashMap<usize, Vec<usize>> = HashMap::new();
        
        for node_id in 0..self.next_node_id {
            let root = self.uf.find(node_id)?;
            components_map.entry(root).or_insert_with(Vec::new).push(node_id);
        }
        
        let component_roots: Vec<usize> = components_map.keys().cloned().collect();
        
        // Connect components pairwise
        for i in 1..component_roots.len() {
            let comp1_nodes = &components_map[&component_roots[0]];
            let comp2_nodes = &components_map[&component_roots[i]];
            
            let node1 = comp1_nodes[0];
            let node2 = comp2_nodes[0];
            
            println!("   🔗 Connecting {} to {} (healing partition)", 
                     self.get_node_name(node1)?, self.get_node_name(node2)?);
            
            self.add_connection(node1, node2)?;
        }
        
        println!("   ✅ Network healing complete! Components: {}", self.component_count());
        
        Ok(())
    }
}

/// Example: Basic dynamic connectivity
fn example_basic_connectivity() -> Result<(), String> {
    println!("=== Basic Dynamic Connectivity Example ===");
    
    let mut network = DynamicNetwork::new(10);
    
    // Add nodes to represent a computer network
    let server = network.add_node("Server".to_string())?;
    let laptop1 = network.add_node("Laptop-1".to_string())?;
    let laptop2 = network.add_node("Laptop-2".to_string())?;
    let printer = network.add_node("Printer".to_string())?;
    let router = network.add_node("Router".to_string())?;
    
    println!("🖥️  Initial network setup:");
    network.print_status()?;
    
    println!("\n📡 Adding connections...");
    
    // Connect devices step by step
    println!("1. Connecting Server to Router...");
    network.add_connection(server, router)?;
    
    println!("2. Connecting Laptop-1 to Router...");
    network.add_connection(laptop1, router)?;
    
    println!("3. Can Laptop-1 reach Server? {}", 
             network.are_connected(laptop1, server)?);
    
    println!("4. Connecting Printer to Router...");
    network.add_connection(printer, router)?;
    
    println!("5. Can Laptop-1 reach Printer? {}", 
             network.are_connected(laptop1, printer)?);
    
    println!("6. Laptop-2 is still isolated. Can it reach Server? {}",
             network.are_connected(laptop2, server)?);
    
    println!("7. Connecting Laptop-2 to Router...");
    network.add_connection(laptop2, router)?;
    
    println!("8. Now can Laptop-2 reach Server? {}",
             network.are_connected(laptop2, server)?);
    
    println!("\n📊 Final network state:");
    network.print_status()?;
    
    println!();
    Ok(())
}

/// Example: Social network formation
fn example_social_network() -> Result<(), String> {
    println!("=== Social Network Formation Example ===");
    
    let mut network = DynamicNetwork::new(20);
    
    // Add people to the social network
    let alice = network.add_node("Alice".to_string())?;
    let bob = network.add_node("Bob".to_string())?;
    let charlie = network.add_node("Charlie".to_string())?;
    let diana = network.add_node("Diana".to_string())?;
    let eve = network.add_node("Eve".to_string())?;
    let frank = network.add_node("Frank".to_string())?;
    
    println!("👥 Social network with 6 people initially disconnected");
    network.print_status()?;
    
    println!("\n🤝 Friend connections forming...");
    
    // Simulate friend connections over time
    println!("Day 1: Alice and Bob become friends");
    network.add_connection(alice, bob)?;
    
    println!("Day 2: Bob and Charlie become friends");
    network.add_connection(bob, charlie)?;
    
    println!("Day 3: Diana and Eve become friends");
    network.add_connection(diana, eve)?;
    
    println!("\n📊 After 3 days:");
    network.print_status()?;
    
    println!("🔍 Connectivity queries:");
    println!("• Can Alice reach Charlie through mutual friends? {}", 
             network.are_connected(alice, charlie)?);
    println!("• Can Alice reach Diana? {}", 
             network.are_connected(alice, diana)?);
    println!("• Size of Alice's friend network: {}", 
             network.component_size(alice)?);
    
    println!("\nDay 4: Charlie and Diana become friends (connecting the groups!)");
    network.add_connection(charlie, diana)?;
    
    println!("Day 5: Frank finally joins and befriends Alice");
    network.add_connection(frank, alice)?;
    
    println!("\n📊 Final social network:");
    network.print_status()?;
    
    println!("🎉 Everyone is now connected in one big friend network!");
    println!("   Network size: {}", network.component_size(alice)?);
    
    println!();
    Ok(())
}

/// Example: Network partition and healing
fn example_network_partition() -> Result<(), String> {
    println!("=== Network Partition and Healing Example ===");
    
    let mut network = DynamicNetwork::new(15);
    
    // Create a larger network topology
    let mut nodes = Vec::new();
    let node_names = vec![
        "DataCenter-A", "DataCenter-B", "DataCenter-C",
        "Office-NYC", "Office-LA", "Office-Chicago",
        "Edge-1", "Edge-2", "Edge-3", "Edge-4"
    ];
    
    for name in node_names {
        nodes.push(network.add_node(name.to_string())?);
    }
    
    println!("🌐 Created distributed network with {} nodes", nodes.len());
    
    // Create initial connectivity (multiple components)
    println!("\n🔗 Building initial network topology...");
    
    // East coast cluster
    network.add_connection(nodes[0], nodes[3])?; // DataCenter-A <-> Office-NYC
    network.add_connection(nodes[3], nodes[6])?; // Office-NYC <-> Edge-1
    
    // West coast cluster  
    network.add_connection(nodes[1], nodes[4])?; // DataCenter-B <-> Office-LA
    network.add_connection(nodes[4], nodes[7])?; // Office-LA <-> Edge-2
    
    // Central cluster
    network.add_connection(nodes[2], nodes[5])?; // DataCenter-C <-> Office-Chicago
    network.add_connection(nodes[5], nodes[8])?; // Office-Chicago <-> Edge-3
    
    // Isolated nodes
    // nodes[9] (Edge-4) remains isolated
    
    println!("📊 Network after initial setup:");
    network.print_status()?;
    
    // Test connectivity across partitions
    println!("\n🔍 Testing cross-partition connectivity:");
    println!("• Can DataCenter-A reach DataCenter-B? {}", 
             network.are_connected(nodes[0], nodes[1])?);
    println!("• Can Office-NYC reach Office-LA? {}", 
             network.are_connected(nodes[3], nodes[4])?);
    
    // Simulate partition healing
    println!("\n🩹 Healing network partitions...");
    network.simulate_partition_healing()?;
    
    println!("\n🔍 Testing connectivity after healing:");
    println!("• Can DataCenter-A reach DataCenter-B? {}", 
             network.are_connected(nodes[0], nodes[1])?);
    println!("• Can Office-NYC reach Office-LA? {}", 
             network.are_connected(nodes[3], nodes[4])?);
    println!("• Can Edge-4 (previously isolated) reach DataCenter-A? {}", 
             network.are_connected(nodes[9], nodes[0])?);
    
    println!("\n📜 Connection history:");
    network.print_history();
    
    println!();
    Ok(())
}

/// Example: Performance with large networks
fn example_large_network_performance() -> Result<(), String> {
    println!("=== Large Network Performance Example ===");
    
    let network_size = 10000;
    let mut network = DynamicNetwork::new(network_size);
    
    println!("🚀 Creating large network with {} nodes", network_size);
    
    // Add nodes
    let start_time = std::time::Instant::now();
    let mut nodes = Vec::new();
    for i in 0..network_size {
        nodes.push(network.add_node(format!("Node-{}", i))?);
    }
    let node_creation_time = start_time.elapsed();
    
    println!("⏱️  Node creation took: {:?}", node_creation_time);
    
    // Add random connections
    let start_time = std::time::Instant::now();
    let num_connections = network_size / 2; // Sparse network
    
    for i in 0..num_connections {
        let from = i % network_size;
        let to = (i * 7 + 13) % network_size; // Pseudo-random but deterministic
        
        if from != to {
            network.add_connection(from, to)?;
        }
    }
    
    let connection_time = start_time.elapsed();
    println!("⏱️  Adding {} connections took: {:?}", num_connections, connection_time);
    
    // Test connectivity queries
    let start_time = std::time::Instant::now();
    let num_queries = 1000;
    let mut connected_count = 0;
    
    for i in 0..num_queries {
        let from = (i * 17) % network_size;
        let to = (i * 23 + 7) % network_size;
        
        if network.are_connected(from, to)? {
            connected_count += 1;
        }
    }
    
    let query_time = start_time.elapsed();
    
    println!("⏱️  {} connectivity queries took: {:?}", num_queries, query_time);
    println!("📊 Query results: {}/{} pairs were connected", connected_count, num_queries);
    println!("📈 Average time per query: {:?}", query_time / (num_queries as u32));
    
    println!("🎯 Final network statistics:");
    println!("   Components: {}", network.component_count());
    println!("   Largest component size: ~{}", 
             network.component_size(0).unwrap_or(0));
    
    println!();
    Ok(())
}

fn main() -> Result<(), String> {
    println!("🌐 Dynamic Connectivity with Union-Find\n");
    
    example_basic_connectivity()?;
    example_social_network()?;
    example_network_partition()?;
    example_large_network_performance()?;
    
    println!("✅ All dynamic connectivity examples completed!");
    
    println!("\n💡 Key Benefits of Union-Find for Dynamic Connectivity:");
    println!("• O(α(n)) connectivity queries - nearly constant time");
    println!("• Efficient incremental updates as networks grow");
    println!("• Memory-efficient representation");
    println!("• Perfect for online algorithms and real-time systems");
    println!("• Handles network partitions and healing gracefully");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_network_operations() {
        let mut network = DynamicNetwork::new(5);
        
        let node1 = network.add_node("A".to_string()).unwrap();
        let node2 = network.add_node("B".to_string()).unwrap();
        let node3 = network.add_node("C".to_string()).unwrap();
        
        // Initially disconnected
        assert!(!network.are_connected(node1, node2).unwrap());
        assert_eq!(network.component_count(), 5); // 3 added + 2 unused
        
        // Connect A and B
        assert!(network.add_connection(node1, node2).unwrap());
        assert!(network.are_connected(node1, node2).unwrap());
        assert!(!network.are_connected(node1, node3).unwrap());
        assert_eq!(network.component_count(), 4);
        
        // Connect B and C
        network.add_connection(node2, node3).unwrap();
        assert!(network.are_connected(node1, node3).unwrap());
        assert_eq!(network.component_count(), 3);
        
        // Component sizes
        assert_eq!(network.component_size(node1).unwrap(), 3);
        assert_eq!(network.component_size(node2).unwrap(), 3);
        assert_eq!(network.component_size(node3).unwrap(), 3);
    }

    #[test]
    fn test_duplicate_connection() {
        let mut network = DynamicNetwork::new(3);
        
        let node1 = network.add_node("A".to_string()).unwrap();
        let node2 = network.add_node("B".to_string()).unwrap();
        
        // First connection should succeed
        assert!(network.add_connection(node1, node2).unwrap());
        
        // Second connection should return false (already connected)
        assert!(!network.add_connection(node1, node2).unwrap());
        assert!(!network.add_connection(node2, node1).unwrap()); // Symmetric
        
        // Component count should remain the same
        assert_eq!(network.component_count(), 2); // 1 connected component + 1 unused node
    }

    #[test]
    fn test_network_capacity() {
        let mut network = DynamicNetwork::new(2);
        
        network.add_node("A".to_string()).unwrap();
        network.add_node("B".to_string()).unwrap();
        
        // Should fail to add third node
        assert!(network.add_node("C".to_string()).is_err());
    }
}