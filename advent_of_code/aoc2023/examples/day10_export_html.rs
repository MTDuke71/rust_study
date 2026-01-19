use aoc2023::solver::day10;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (input, output_file) = if args.len() > 1 && args[1] == "--real" {
        let input = include_str!("../inputs/day10.txt");
        let output = "day10_visualization_real.html";
        println!("🎨 Generating HTML visualization for REAL puzzle input...");
        println!("   Grid size: 140 × 140 cells");
        (input, output)
    } else {
        let input = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let output = "day10_visualization_example.html";
        println!("🎨 Generating HTML visualization for EXAMPLE input...");
        println!("   Grid size: 9 × 11 cells");
        (input, output)
    };

    match day10::export_visualization_html(input, output_file) {
        Ok(()) => {
            println!("✅ Successfully exported to: {}", output_file);
            println!();
            println!("📂 Open the HTML file in your browser to view the visualization!");
            println!("   Features:");
            println!("   • Proper Unicode box-drawing characters (│─└┘┐┌)");
            println!("   • Color-coded loop (blue) and inside tiles (red)");
            println!("   • Statistics: grid size, loop length, farthest point, inside count");
            println!("   • Three views: Loop Structure, Distance Map, Inside/Outside");
        }
        Err(e) => {
            eprintln!("❌ Error exporting HTML: {}", e);
            std::process::exit(1);
        }
    }
}
