// Mission 10 Demo - Basic Union-Find Usage
use mission10::UnionFind;

fn main() -> Result<(), String> {
    println!("=== Mission 10: Union-Find Demo ===\n");

    let mut uf = UnionFind::new(10);

    println!("Initial state: {} disjoint sets", uf.count());

    // Union some elements
    println!("\nPerforming unions:");
    uf.union(0, 1)?;
    println!("  union(0, 1) - sets: {}", uf.count());

    uf.union(2, 3)?;
    println!("  union(2, 3) - sets: {}", uf.count());

    uf.union(0, 2)?;
    println!("  union(0, 2) - sets: {}", uf.count());

    // Check connectivity
    println!("\nConnectivity queries:");
    println!("  0 and 3 connected? {}", uf.connected(0, 3)?);
    println!("  0 and 4 connected? {}", uf.connected(0, 4)?);

    // Get set sizes
    println!("\nSet sizes:");
    println!("  Set containing 0: {} elements", uf.size(0)?);
    println!("  Set containing 4: {} elements", uf.size(4)?);

    Ok(())
}
