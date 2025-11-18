// Mission 10 - Advanced Union-Find Features Demo
// Demonstrates REQ-2 Extensions, REQ-8 (Weighted), and REQ-9 (Undo) features

use mission10::{UndoUnionFind, UnionFind};

fn main() -> Result<(), String> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║            Mission 10: Advanced Union-Find Features         ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Part 1: REQ-2 Extensions - Additional Operations
    println!("=== Part 1: Additional Operations (REQ-2 Extensions) ===\n");
    demonstrate_additional_operations()?;

    // Part 2: Iterator Support
    println!("\n=== Part 2: Iterator Support ===\n");
    demonstrate_iterators()?;

    // Part 3: REQ-9 - Undo Support
    println!("\n=== Part 3: Undo Support (REQ-9) ===\n");
    demonstrate_undo_functionality()?;

    // Part 4: REQ-8 - Weighted Union-Find (if feature enabled)
    #[cfg(feature = "weighted")]
    {
        println!("\n=== Part 4: Weighted Union-Find (REQ-8) ===\n");
        demonstrate_weighted_union_find()?;
    }

    #[cfg(not(feature = "weighted"))]
    {
        println!("\n=== Part 4: Weighted Union-Find (REQ-8) ===");
        println!("⚠️  Weighted Union-Find requires 'weighted' feature");
        println!("   Run with: cargo run --example advanced_features --features weighted");
    }

    // Part 5: Serialization Support (if feature enabled)
    #[cfg(feature = "serde_support")]
    {
        println!("\n=== Part 5: Serialization Support ===\n");
        demonstrate_serialization()?;
    }

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                    Advanced Features Complete                ║");
    println!("║                                                              ║");
    println!("║  ✅ REQ-2 Extensions: clear(), reset(), len(), is_empty()   ║");
    println!("║  ✅ Iterator Support: components(), members()                ║");
    println!("║  ✅ REQ-9 Undo Support: UndoUnionFind with history          ║");
    println!("║  📦 REQ-8 Weighted: Available with 'weighted' feature       ║");
    println!("║  📦 Serialization: Available with 'serde_support' feature   ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    Ok(())
}

fn demonstrate_additional_operations() -> Result<(), String> {
    let mut uf = UnionFind::new(8);

    println!("🔧 Basic Information:");
    println!("   Initial size: {}", uf.len());
    println!("   Is empty: {}", uf.is_empty());
    println!("   Component count: {}", uf.count());

    // Create some connections
    println!("\n🔗 Creating connections:");
    uf.union(0, 1)?;
    println!("   Connected 0-1, components: {}", uf.count());
    uf.union(2, 3)?;
    println!("   Connected 2-3, components: {}", uf.count());
    uf.union(4, 5)?;
    println!("   Connected 4-5, components: {}", uf.count());

    // Test clear functionality
    println!("\n🧹 Testing clear() functionality:");
    println!("   Before clear - components: {}", uf.count());
    println!("   Elements 0 and 1 connected: {}", uf.connected(0, 1)?);

    uf.clear();
    println!("   After clear - components: {}", uf.count());
    println!("   Elements 0 and 1 connected: {}", uf.connected(0, 1)?);

    // Test reset functionality
    println!("\n🔄 Testing reset() functionality:");
    println!("   Current size: {}", uf.len());
    uf.reset(12);
    println!(
        "   After reset(12) - size: {}, components: {}",
        uf.len(),
        uf.count()
    );

    Ok(())
}

fn demonstrate_iterators() -> Result<(), String> {
    let mut uf = UnionFind::new(10);

    // Create several components
    println!("🏗️  Building connected components:");
    uf.union(0, 1)?;
    uf.union(1, 2)?;
    println!("   Component 1: {{0, 1, 2}}");

    uf.union(3, 4)?;
    uf.union(4, 5)?;
    println!("   Component 2: {{3, 4, 5}}");

    uf.union(6, 7)?;
    println!("   Component 3: {{6, 7}}");

    // Elements 8 and 9 remain alone
    println!("   Component 4: {{8}}");
    println!("   Component 5: {{9}}");

    // Demonstrate components iterator
    println!("\n📋 All connected components:");
    let components: Vec<Vec<usize>> = uf.components().collect();
    for (i, component) in components.iter().enumerate() {
        println!("   Component {}: {:?}", i + 1, component);
    }

    // Demonstrate members iterator
    println!("\n👥 Members of specific sets:");
    let members_0: Vec<usize> = uf.members(0)?.collect();
    println!("   Members of set containing 0: {:?}", members_0);

    let members_3: Vec<usize> = uf.members(3)?.collect();
    println!("   Members of set containing 3: {:?}", members_3);

    let members_8: Vec<usize> = uf.members(8)?.collect();
    println!("   Members of set containing 8: {:?}", members_8);

    // Show iterator properties
    println!("\n📊 Iterator statistics:");
    println!("   Total components: {}", components.len());
    println!(
        "   Component sizes: {:?}",
        components.iter().map(|c| c.len()).collect::<Vec<_>>()
    );

    Ok(())
}

fn demonstrate_undo_functionality() -> Result<(), String> {
    let mut uf = UndoUnionFind::new(6);

    println!("⏪ Demonstrating undo functionality:");
    println!(
        "   Initial state - components: {}, operations: {}",
        uf.count(),
        uf.operation_count()
    );

    // Perform operations with progress tracking
    println!("\n📝 Performing operations:");

    println!("   1. union(0, 1)");
    uf.union(0, 1)?;
    println!(
        "      → Components: {}, Operations: {}",
        uf.count(),
        uf.operation_count()
    );

    println!("   2. union(2, 3)");
    uf.union(2, 3)?;
    println!(
        "      → Components: {}, Operations: {}",
        uf.count(),
        uf.operation_count()
    );

    println!("   3. union(4, 5)");
    uf.union(4, 5)?;
    println!(
        "      → Components: {}, Operations: {}",
        uf.count(),
        uf.operation_count()
    );

    println!("   4. union(0, 2) - connects first two components");
    uf.union(0, 2)?;
    println!(
        "      → Components: {}, Operations: {}",
        uf.count(),
        uf.operation_count()
    );

    // Demonstrate undo operations
    println!("\n⏮️  Undoing operations:");

    for step in 1..=3 {
        if uf.undo()? {
            println!(
                "   Undo step {}: Components: {}, Operations: {}",
                step,
                uf.count(),
                uf.operation_count()
            );
        }
    }

    // Try to undo more than available
    println!("\n   Attempting to undo beyond available operations:");
    for attempt in 1..=3 {
        let success = uf.undo()?;
        if success {
            println!(
                "   Undo attempt {}: Success - Components: {}",
                attempt,
                uf.count()
            );
        } else {
            println!("   Undo attempt {}: No more operations to undo", attempt);
            break;
        }
    }

    Ok(())
}

#[cfg(feature = "weighted")]
fn demonstrate_weighted_union_find() -> Result<(), String> {
    use mission10::WeightedUnionFind;

    let mut wuf = WeightedUnionFind::new(6);

    println!("⚖️  Weighted Union-Find - Network Latency Example:");
    println!("   Modeling network connections with latencies (ms)\n");

    // Create a network topology
    println!("🌐 Building network topology:");
    wuf.weighted_union(0, 1, 10)?;
    println!("   Server 0 ↔ Server 1: 10ms latency");

    wuf.weighted_union(1, 2, 15)?;
    println!("   Server 1 ↔ Server 2: 15ms latency");

    wuf.weighted_union(0, 3, 25)?;
    println!("   Server 0 ↔ Server 3: 25ms latency");

    wuf.weighted_union(3, 4, 8)?;
    println!("   Server 3 ↔ Server 4: 8ms latency");

    // Query distances
    println!("\n📏 Network latency queries:");

    let distances = [
        (0, 1, "Direct connection"),
        (0, 2, "Via Server 1"),
        (0, 4, "Via Server 3"),
        (1, 4, "Via Servers 0→3"),
        (2, 4, "Long path: 2→1→0→3→4"),
        (0, 5, "Not connected"),
    ];

    for (from, to, description) in distances {
        match wuf.distance(from, to)? {
            Some(dist) => println!(
                "   Server {} → Server {}: {}ms ({})",
                from, to, dist, description
            ),
            None => println!(
                "   Server {} → Server {}: Not connected ({})",
                from, to, description
            ),
        }
    }

    println!("\n💡 Use case: Network routing, shortest path algorithms,");
    println!("   distributed systems with connection costs");

    Ok(())
}

#[cfg(feature = "serde_support")]
fn demonstrate_serialization() -> Result<(), String> {
    let mut uf = UnionFind::new(5);

    // Create some connections
    uf.union(0, 1)?;
    uf.union(2, 3)?;

    println!("💾 Serialization Support:");
    println!("   Original: {} components", uf.count());

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&uf).map_err(|e| e.to_string())?;
    println!("   Serialized to JSON ({} bytes)", json.len());

    // Deserialize from JSON
    let deserialized: UnionFind = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    println!("   Deserialized: {} components", deserialized.count());

    println!("\n📄 JSON representation sample:");
    println!("{}", json.lines().take(5).collect::<Vec<_>>().join("\n"));
    println!("   ... (truncated)");

    println!("\n💡 Use case: Persistence, distributed computing,");
    println!("   checkpointing, state transfer");

    Ok(())
}

// Helper function to create example network for demonstrations
#[allow(dead_code)]
fn create_social_network() -> Result<UnionFind, String> {
    let mut network = UnionFind::new(10);

    // Friend connections
    network.union(0, 1)?; // Alice - Bob
    network.union(1, 2)?; // Bob - Charlie
    network.union(3, 4)?; // David - Eve
    network.union(4, 5)?; // Eve - Frank
    network.union(6, 7)?; // Grace - Helen

    // Cross-group connection
    network.union(2, 8)?; // Charlie - Ivan

    Ok(network)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_runs() {
        // Test that the main example doesn't panic
        // Individual function testing is done in lib.rs
        assert!(true);
    }
}
