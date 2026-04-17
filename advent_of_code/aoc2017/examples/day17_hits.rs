//! Analysis tool: dump every iteration `i` where `(pos + steps) % i == 0`
//! (i.e., the writes that actually land at index 1 in Day 17 Part 2).
//!
//! Usage: `cargo run --release -p aoc2017 --example day17_hits`
//! Output: `aoc2017/analysis/day17_hits.csv` with columns (hit_num, i).

use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    let steps: usize = std::fs::read_to_string("inputs/day17.txt")
        .expect("need inputs/day17.txt")
        .trim()
        .parse()
        .expect("valid step count");

    const N: usize = 50_000_000;
    std::fs::create_dir_all("analysis")?;
    let mut out = BufWriter::new(File::create("analysis/day17_hits.csv")?);
    writeln!(out, "hit_num,i")?;

    let mut pos = 0usize;
    let mut hits = 0u64;
    for i in 1..=N {
        pos = (pos + steps) % i;
        if pos == 0 {
            hits += 1;
            writeln!(out, "{hits},{i}")?;
        }
        pos += 1;
    }

    println!("Wrote {hits} hits to analysis/day17_hits.csv");
    Ok(())
}
