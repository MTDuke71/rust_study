
const EXAMPLE3: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

fn main() {
    println!("Testing Example 3 (should be 4):");
    println!("{}", EXAMPLE3);
    
    // I'll manually trace the algorithm
    let lines: Vec<&str> = EXAMPLE3.lines().collect();
    println!("\nRow-by-row analysis:");
    
    for (i, line) in lines.iter().enumerate() {
        println!("Row {}: {}", i, line);
    }
}
