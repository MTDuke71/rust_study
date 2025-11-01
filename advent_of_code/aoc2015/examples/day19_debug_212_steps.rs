use aoc2015::solver::day19;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Day 19 Part 2: Debug Trace Analysis ===\n");
    
    // Try to read the actual input file, fall back to example
    let input = if std::path::Path::new("inputs/day19.txt").exists() {
        println!("📁 Using actual AoC input file (inputs/day19.txt)");
        std::fs::read_to_string("inputs/day19.txt")?
    } else {
        println!("📁 Using example input file (inputs/day19_example.txt)");
        println!("ℹ️  To see the full 212-step trace, add your AoC input as inputs/day19.txt\n");
        std::fs::read_to_string("inputs/day19_example.txt")?
    };
    
    let lines: Vec<&str> = input.trim().lines().collect();
    
    // Find the separator (empty line)
    let separator_pos = lines.iter().position(|&line| line.is_empty()).unwrap();
    let rules_lines = &lines[..separator_pos];
    let target = lines[separator_pos + 1];
    
    println!("🎯 Target molecule: {} (length: {})", target, target.len());
    println!("📋 Total replacement rules: {}", rules_lines.len());
    
    // Parse the rules
    let mut forward_rules: Vec<(String, String)> = Vec::new();
    for line in rules_lines {
        let parts: Vec<&str> = line.split(" => ").collect();
        if parts.len() == 2 {
            forward_rules.push((parts[0].to_string(), parts[1].to_string()));
        }
    }
    
    // Create reverse rules (for Part 2)
    let mut reverse_rules: Vec<(String, String)> = forward_rules
        .iter()
        .map(|(from, to)| (to.clone(), from.clone()))
        .collect();
    
    // Sort by pattern length (descending) for greedy algorithm
    reverse_rules.sort_by_key(|rule| std::cmp::Reverse(rule.0.len()));
    
    println!("\n🔄 Reverse rules (longest first):");
    for (i, (from, to)) in reverse_rules.iter().enumerate() {
        println!("  {}: '{}' → '{}' (length: {})", i+1, from, to, from.len());
        if i >= 9 { // Show first 10 rules
            println!("  ... and {} more rules", reverse_rules.len() - 10);
            break;
        }
    }
    
    // Apply debug version of reverse construction
    let steps = debug_reverse_construction(target, &reverse_rules);
    
    println!("\n🎯 Final Result: {} steps to construct molecule from 'e'", steps);
    
    // Verify with the actual solver
    let official_result = day19::solve_part2(&input)?;
    println!("✅ Official solver result: {} steps", official_result);
    println!("✅ Results match: {}", steps == official_result.parse::<usize>().unwrap());
    
    Ok(())
}

fn debug_reverse_construction(target: &str, reverse_rules: &[(String, String)]) -> usize {
    let mut current = target.to_string();
    let mut steps = 0;
    let mut step_details: Vec<String> = Vec::new();
    
    println!("\n🚀 Starting reverse construction:");
    println!("Step {}: '{}' (length: {})", steps, current, current.len());
    
    while current != "e" {
        let mut found_replacement = false;
        
        // Try each rule (longest patterns first)
        for (from_pattern, to_pattern) in reverse_rules {
            if let Some(pos) = current.find(from_pattern) {
                // Found a pattern we can replace
                let before = &current[..pos];
                let after = &current[pos + from_pattern.len()..];
                let new_molecule = format!("{}{}{}", before, to_pattern, after);
                
                steps += 1;
                
                let detail = format!(
                    "Step {}: Replace '{}' at position {} → '{}' → '{}' (length: {} → {})",
                    steps, from_pattern, pos, to_pattern, new_molecule, current.len(), new_molecule.len()
                );
                
                println!("{}", detail);
                step_details.push(detail);
                
                current = new_molecule;
                found_replacement = true;
                
                // Show progress every 50 steps
                if steps % 50 == 0 {
                    println!("  📊 Progress: {} steps, current length: {}", steps, current.len());
                    println!("  🧬 Current molecule: {}...", 
                        if current.len() > 50 { &current[..50] } else { &current });
                }
                
                break; // Use first (longest) matching pattern
            }
        }
        
        if !found_replacement {
            println!("❌ ERROR: No replacement found for: '{}'", current);
            println!("Available patterns:");
            for (from, to) in reverse_rules.iter().take(10) {
                println!("  '{}' → '{}'", from, to);
            }
            break;
        }
        
        // Safety check to prevent infinite loops
        if steps > 1000 {
            println!("⚠️  WARNING: Stopped at {} steps to prevent infinite loop", steps);
            break;
        }
    }
    
    if current == "e" {
        println!("\n🎉 SUCCESS: Reached electron 'e' in {} steps!", steps);
    }
    
    // Show summary of step ranges
    println!("\n📊 Step Analysis:");
    let large_reductions = step_details.len(); // All steps make reductions
    
    println!("  • Total steps: {}", steps);
    println!("  • Steps with length reduction: {}", large_reductions);
    if steps > 0 {
        println!("  • Average molecule length reduced per step: ~{:.1}", 
            (target.len() as f32 - 1.0) / steps as f32);
    }
    
    // Show first few and last few steps
    println!("\n📝 First 5 steps:");
    for detail in step_details.iter().take(5) {
        println!("  {}", detail);
    }
    
    if step_details.len() > 10 {
        println!("  ... ({} steps omitted) ...", step_details.len() - 10);
        
        println!("\n📝 Last 5 steps:");
        for detail in step_details.iter().skip(step_details.len() - 5) {
            println!("  {}", detail);
        }
    }
    
    steps
}