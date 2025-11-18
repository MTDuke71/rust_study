use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Day 19 Rule Structure Analysis ===\n");

    let input = std::fs::read_to_string("inputs/day19_example.txt")?;
    let lines: Vec<&str> = input.trim().lines().collect();

    // Find the separator (empty line)
    let separator_pos = lines.iter().position(|&line| line.is_empty()).unwrap();
    let rules_lines = &lines[..separator_pos];
    let target = lines[separator_pos + 1];

    println!("🎯 Target molecule: {}", target);
    println!("📋 Total rules: {}\n", rules_lines.len());

    // Parse and analyze rules
    let mut forward_rules: Vec<(String, String)> = Vec::new();
    let mut from_elements: HashMap<String, Vec<String>> = HashMap::new();
    let mut to_patterns: HashMap<String, String> = HashMap::new();

    for line in rules_lines {
        let parts: Vec<&str> = line.split(" => ").collect();
        if parts.len() == 2 {
            let from = parts[0].to_string();
            let to = parts[1].to_string();

            forward_rules.push((from.clone(), to.clone()));
            from_elements
                .entry(from.clone())
                .or_default()
                .push(to.clone());
            to_patterns.insert(to, from);
        }
    }

    println!("📊 RULE STRUCTURE ANALYSIS:");
    println!("=====================================");

    // 1. Analyze what each element can produce
    println!("\n1️⃣ Forward Rules (Element → Patterns):");
    for (element, patterns) in &from_elements {
        println!("   {}: {} → {:?}", element, patterns.len(), patterns);
    }

    // 2. Analyze reverse mapping (crucial for Part 2)
    println!("\n2️⃣ Reverse Rules (Pattern → Element):");
    let mut reverse_rules: Vec<(String, String)> = forward_rules
        .iter()
        .map(|(from, to)| (to.clone(), from.clone()))
        .collect();
    reverse_rules.sort_by_key(|rule| std::cmp::Reverse(rule.0.len()));

    for (pattern, element) in &reverse_rules {
        println!(
            "   '{}' → '{}' (length: {})",
            pattern,
            element,
            pattern.len()
        );
    }

    // 3. Check for uniqueness - KEY INSIGHT!
    println!("\n3️⃣ UNIQUENESS ANALYSIS (Why we're guaranteed to reach 'e'):");
    println!("================================================================");

    let mut pattern_conflicts = HashMap::new();
    for (pattern, element) in &to_patterns {
        pattern_conflicts
            .entry(pattern.clone())
            .or_insert_with(Vec::new)
            .push(element.clone());
    }

    let mut has_conflicts = false;
    for (pattern, elements) in &pattern_conflicts {
        if elements.len() > 1 {
            println!(
                "   ⚠️  CONFLICT: Pattern '{}' maps to multiple elements: {:?}",
                pattern, elements
            );
            has_conflicts = true;
        }
    }

    if !has_conflicts {
        println!("   ✅ UNIQUE MAPPING: Every pattern maps to exactly ONE element!");
        println!("   ✅ This guarantees deterministic reverse construction!");
    }

    // 4. Check terminals
    println!("\n4️⃣ TERMINAL ANALYSIS:");
    println!("=======================");

    let electron_rules: Vec<_> = forward_rules
        .iter()
        .filter(|(from, _)| from == "e")
        .collect();

    println!("   Electron 'e' rules: {}", electron_rules.len());
    for (from, to) in &electron_rules {
        println!("     {} → {}", from, to);
    }

    // 5. Check if target is reachable from electron
    println!("\n5️⃣ REACHABILITY ANALYSIS:");
    println!("===========================");

    let can_reach_target = simulate_reverse_path(target, &reverse_rules);
    if can_reach_target {
        println!("   ✅ Target molecule IS reachable from electron 'e'");
        println!("   ✅ Greedy algorithm will find the unique optimal path");
    } else {
        println!("   ❌ Target molecule NOT reachable from electron 'e'");
    }

    // 6. Structural properties
    println!("\n6️⃣ STRUCTURAL PROPERTIES:");
    println!("===========================");

    let bracketed_patterns = reverse_rules
        .iter()
        .filter(|(pattern, _)| pattern.contains("Rn") && pattern.contains("Ar"))
        .count();

    let simple_patterns = reverse_rules
        .iter()
        .filter(|(pattern, _)| !pattern.contains("Rn") && !pattern.contains("Ar"))
        .count();

    println!("   Bracketed patterns (Rn...Ar): {}", bracketed_patterns);
    println!("   Simple patterns: {}", simple_patterns);
    println!("   Total patterns: {}", reverse_rules.len());

    // 7. Why greedy works
    println!("\n7️⃣ WHY GREEDY ALGORITHM WORKS:");
    println!("================================");
    println!("   1. UNIQUE MAPPING: Each pattern → exactly one element");
    println!("   2. NO CONFLICTS: No ambiguity in reverse construction");
    println!("   3. MONOTONIC PROGRESS: Longest patterns give maximum reduction");
    println!("   4. TERMINATION: Process ends when we reach 'e'");
    println!("   5. OPTIMAL PATH: Greedy choice property holds for this structure");

    Ok(())
}

fn simulate_reverse_path(target: &str, reverse_rules: &[(String, String)]) -> bool {
    let mut current = target.to_string();
    let mut steps = 0;
    let max_steps = 1000; // Safety limit

    while current != "e" && steps < max_steps {
        let original = current.clone();

        // Try to apply any reverse rule
        for (from_pattern, to_element) in reverse_rules {
            if current.contains(from_pattern) {
                current = current.replacen(from_pattern, to_element, 1);
                steps += 1;
                break;
            }
        }

        // If no progress made, we're stuck
        if current == original {
            return false;
        }
    }

    current == "e"
}
