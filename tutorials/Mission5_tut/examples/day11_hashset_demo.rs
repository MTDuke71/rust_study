// Example from Day 11 HashSet tutorial
use std::collections::HashSet;

fn main() {
    println!("=== Day 11: HashSet Examples ===\n");

    // Breaking down: ["React", "Vue", "Angular"].into_iter().collect()
    let frameworks: HashSet<&str> = ["React", "Vue", "Angular"].iter().cloned().collect();
    println!("1. Array -> Iterator -> HashSet");
    println!("   Frameworks: {:?}\n", frameworks);

    // Alternative creation methods
    let frameworks2 = HashSet::from(["React", "Vue", "Angular"]);
    println!("2. Using HashSet::from()");
    println!("   Frameworks2: {:?}\n", frameworks2);

    // Set operations
    let set1: HashSet<i32> = vec![1, 2, 3, 4].into_iter().collect();
    let set2: HashSet<i32> = vec![3, 4, 5, 6].into_iter().collect();

    println!("3. Set Theory Operations:");
    println!("   Set 1: {:?}", set1);
    println!("   Set 2: {:?}", set2);

    let union: HashSet<_> = set1.union(&set2).cloned().collect();
    let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
    let difference: HashSet<_> = set1.difference(&set2).cloned().collect();

    println!("   Union: {:?}", union);
    println!("   Intersection: {:?}", intersection);
    println!("   Difference (set1 - set2): {:?}", difference);
}
