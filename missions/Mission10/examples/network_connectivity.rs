// Mission 10 Example - Network Connectivity
use mission10::UnionFind;

fn main() -> Result<(), String> {
    println!("=== Network Connectivity Example ===\n");

    let num_computers = 6;
    let mut network = UnionFind::new(num_computers);

    println!("Network with {} computers\n", num_computers);

    // Add connections (cables)
    let connections = vec![(0, 1, "Cable A"), (1, 2, "Cable B"), (3, 4, "Cable C")];

    println!("Adding connections:");
    for (x, y, name) in &connections {
        network.union(*x, *y)?;
        println!("  {} connects computers {} and {}", name, x, y);
    }

    println!("\nNetwork status:");
    println!("  Separate networks: {}", network.count());

    // Test connectivity
    println!("\nConnectivity tests:");
    println!(
        "  Computer 0 can reach Computer 2? {}",
        network.connected(0, 2)?
    );
    println!(
        "  Computer 0 can reach Computer 5? {}",
        network.connected(0, 5)?
    );

    // Add bridge connection
    println!("\nAdding Cable D to connect networks...");
    network.union(2, 3)?;

    println!("\nAfter bridge:");
    println!("  Separate networks: {}", network.count());
    println!(
        "  Computer 0 can reach Computer 4? {}",
        network.connected(0, 4)?
    );

    Ok(())
}
