// Mission 10 Example - Connected Components
use mission10::UnionFind;

fn main() -> Result<(), String> {
    println!("=== Connected Components Example ===\n");

    let num_nodes = 10;
    let mut uf = UnionFind::new(num_nodes);

    println!("Graph with {} nodes\n", num_nodes);

    // Add edges to form components
    let edges = vec![
        (0, 1),
        (1, 2),
        (2, 3),
        (5, 6),
        (6, 7),
        (8, 9),
    ];

    println!("Adding edges:");
    for (x, y) in &edges {
        uf.union(*x, *y)?;
        println!("  Edge: {} -- {}", x, y);
    }

    // Find components
    println!("\nConnected Components:");
    for i in 0..num_nodes {
        let root = uf.find(i)?;
        let size = uf.size(root)?;
        println!("  Node {} -> Component {} (size: {})", i, root, size);
    }

    println!("\nTotal number of components: {}", uf.count());

    // Test connectivity
    println!("\nConnectivity tests:");
    println!("  Nodes 0 and 3 connected? {}", uf.connected(0, 3)?);
    println!("  Nodes 0 and 5 connected? {}", uf.connected(0, 5)?);
    println!("  Nodes 5 and 7 connected? {}", uf.connected(5, 7)?);

    Ok(())
}
