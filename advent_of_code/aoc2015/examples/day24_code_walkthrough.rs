use aoc2015::solver::day24::*;

// A step-by-step walkthrough of the Day 24 solution approach.
// Run with:
//   cargo run -p aoc2015 --example day24_code_walkthrough
// or
//   cargo run -p aoc2015 --example day24_code_walkthrough --release

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use the bundled example input
    let input = include_str!("../inputs/day24_example.txt");
    let weights = parse_weights(input)?;
    let mut sorted = weights.clone();
    sorted.sort_by(|a, b| b.cmp(a));
    let total: u64 = sorted.iter().sum();

    println!("=== AoC 2015 Day 24: Code Walkthrough ===\n");
    println!("Packages (sorted desc): {:?}", sorted);
    println!("Total weight: {}\n", total);

    // Part 1 (3 groups)
    walkthrough_for_groups(&sorted, 3)?;

    // Part 2 (4 groups)
    walkthrough_for_groups(&sorted, 4)?;

    Ok(())
}

fn walkthrough_for_groups(weights_desc: &[u64], num_groups: u64) -> Result<(), Box<dyn std::error::Error>> {
    let total: u64 = weights_desc.iter().sum();
    if !total.is_multiple_of(num_groups) {
        println!("Part ({} groups): total not divisible by {}\n", num_groups, num_groups);
        return Ok(());
    }
    let target = total / num_groups;
    println!("-- {} Groups Walkthrough --", num_groups);
    println!("Target per group: {}", target);

    // Enumerate subsets by increasing size until we find valid first groups
    let mut best_qe: Option<u64> = None;
    let mut minimal_size_found: Option<usize> = None;

    for size in 1..=weights_desc.len() {
        let combos = find_subsets(weights_desc, target, size);
        if combos.is_empty() {
            continue;
        }

        println!("\nTrying first-group size = {} ({} subsets sum to target)", size, combos.len());
        let mut used_any_for_size = false;

        for idxs in combos.into_iter().take(10) { // show only first few for brevity
            let mut chosen = Vec::with_capacity(size);
            let mut used = vec![false; weights_desc.len()];
            for i in idxs {
                chosen.push(weights_desc[i]);
                used[i] = true;
            }
            let remaining: Vec<u64> = weights_desc
                .iter()
                .enumerate()
                .filter_map(|(i, &v)| if !used[i] { Some(v) } else { None })
                .collect();

            let possible = can_partition_remaining(&remaining, target, num_groups - 1);
            println!(
                "  candidate {:?} -> QE={} | remaining partitionable: {}",
                chosen,
                quantum_entanglement(&chosen),
                possible
            );

            if possible {
                let qe = quantum_entanglement(&chosen);
                used_any_for_size = true;
                best_qe = match best_qe {
                    None => Some(qe),
                    Some(cur) => Some(cur.min(qe)),
                };
            }
        }

        if used_any_for_size {
            minimal_size_found = Some(size);
            break; // First size with feasible group is minimal by construction
        }
    }

    match (minimal_size_found, best_qe) {
        (Some(size), Some(qe)) => {
            println!("\nResult for {} groups:", num_groups);
            println!("- Minimal first-group size: {}", size);
            println!("- Minimal quantum entanglement: {}", qe);
        }
        _ => {
            println!("\nNo valid partition found for {} groups.", num_groups);
        }
    }

    println!();
    Ok(())
}

