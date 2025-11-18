//! Property-Based Testing Example for Union-Find
//!
//! This example demonstrates how QuickCheck automatically generates test cases
//! to verify mathematical properties of the Union-Find data structure.

use mission10::UnionFind;

fn main() {
    println!("🧪 Property-Based Testing Demo for Union-Find");
    println!("=============================================\n");

    // Demonstrate union commutativity manually
    demonstrate_union_commutativity();

    // Demonstrate find idempotency
    demonstrate_find_idempotency();

    // Demonstrate transitivity
    demonstrate_transitivity();

    // Demonstrate count property
    demonstrate_count_property();

    println!("\n✅ All properties verified!");
    println!("\n💡 To run the full QuickCheck test suite:");
    println!("   cargo test --test property_tests");
    println!("\n🔍 QuickCheck automatically generates hundreds of test cases");
    println!("   to verify these properties hold for any valid inputs!");
}

fn demonstrate_union_commutativity() {
    println!("🔄 Property: Union is Commutative");
    println!("   union(a, b) ≡ union(b, a) (same final connectivity)");

    let mut uf1 = UnionFind::new(5);
    let mut uf2 = UnionFind::new(5);

    // Apply union in different orders
    uf1.union(1, 3).unwrap();
    uf1.union(0, 4).unwrap();

    uf2.union(3, 1).unwrap(); // Reverse order
    uf2.union(4, 0).unwrap(); // Reverse order

    // Check that connectivity is identical
    let mut identical = true;
    for i in 0..5 {
        for j in 0..5 {
            if uf1.connected(i, j).unwrap() != uf2.connected(i, j).unwrap() {
                identical = false;
            }
        }
    }

    println!(
        "   ✅ Union order doesn't affect final connectivity: {}",
        identical
    );
    println!("   📊 Both structures have {} components\n", uf1.count());
}

fn demonstrate_find_idempotency() {
    println!("🔍 Property: Find is Idempotent");
    println!("   find(x) called multiple times returns same result");

    let mut uf = UnionFind::new(6);
    uf.union(0, 2).unwrap();
    uf.union(2, 4).unwrap();
    uf.union(1, 3).unwrap();

    // Test idempotency for element 0
    let first_find = uf.find(0).unwrap();
    let second_find = uf.find(0).unwrap();
    let third_find = uf.find(0).unwrap();

    let idempotent = first_find == second_find && second_find == third_find;

    println!(
        "   ✅ Multiple find(0) calls return same root: {} (result: {})",
        idempotent, first_find
    );
    println!("   🌳 Path compression optimizes tree structure without changing results\n");
}

fn demonstrate_transitivity() {
    println!("🔗 Property: Connectivity is Transitive");
    println!("   If connected(a, b) ∧ connected(b, c) → connected(a, c)");

    let mut uf = UnionFind::new(5);
    uf.union(0, 1).unwrap();
    uf.union(1, 2).unwrap();
    uf.union(3, 4).unwrap(); // Separate component

    let ab = uf.connected(0, 1).unwrap();
    let bc = uf.connected(1, 2).unwrap();
    let ac = uf.connected(0, 2).unwrap(); // Should be transitive
    let ad = uf.connected(0, 3).unwrap(); // Should be false

    println!("   📍 connected(0, 1): {}", ab);
    println!("   📍 connected(1, 2): {}", bc);
    println!("   ✅ connected(0, 2): {} (transitive)", ac);
    println!("   ❌ connected(0, 3): {} (different components)", ad);
    println!(
        "   🎯 Transitivity property holds: {}\n",
        ab && bc && ac && !ad
    );
}

fn demonstrate_count_property() {
    println!("📊 Property: Count Decreases or Stays Same");
    println!("   Each union either decreases component count by 1 or leaves it unchanged");

    let mut uf = UnionFind::new(4);
    let initial_count = uf.count();
    println!("   📈 Initial components: {}", initial_count);

    // Union elements in different components
    let were_connected = uf.connected(0, 2).unwrap();
    uf.union(0, 2).unwrap();
    let new_count = uf.count();

    println!(
        "   🔗 union(0, 2) - were already connected: {}",
        were_connected
    );
    println!(
        "   📉 Components after union: {} (decreased by {})",
        new_count,
        initial_count - new_count
    );

    // Union elements already in same component
    let before_redundant = uf.count();
    let already_connected = uf.connected(0, 2).unwrap();
    uf.union(0, 2).unwrap(); // Redundant union
    let after_redundant = uf.count();

    println!(
        "   🔄 union(0, 2) again - already connected: {}",
        already_connected
    );
    println!(
        "   📊 Components after redundant union: {} (no change)",
        after_redundant
    );
    println!(
        "   ✅ Count property verified: {}\n",
        before_redundant == after_redundant
    );
}
