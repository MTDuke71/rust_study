use anyhow::{Context, Result};
use std::collections::HashSet;

/// Represents a replacement rule: from => to
#[derive(Debug, Clone)]
struct Replacement {
    from: String,
    to: String,
}

/// Parse the input to extract replacement rules and the starting molecule
fn parse_input(input: &str) -> Result<(Vec<Replacement>, String)> {
    let lines: Vec<&str> = input.lines().collect();

    // Find the blank line that separates rules from the molecule
    let blank_line_idx = lines
        .iter()
        .position(|line| line.trim().is_empty())
        .context("No blank line found to separate rules from molecule")?;

    // Parse replacement rules
    let mut replacements = Vec::new();
    for (i, line) in lines.iter().take(blank_line_idx).enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(" => ").collect();
        if parts.len() != 2 {
            anyhow::bail!(
                "Invalid replacement rule format at line {}: '{}'",
                i + 1,
                line
            );
        }

        replacements.push(Replacement {
            from: parts[0].to_string(),
            to: parts[1].to_string(),
        });
    }

    // Get the starting molecule (should be the last non-empty line)
    let molecule = lines
        .iter()
        .skip(blank_line_idx + 1)
        .find(|line| !line.trim().is_empty())
        .context("No starting molecule found after blank line")?
        .trim()
        .to_string();

    Ok((replacements, molecule))
}

/// Generate all possible molecules after one replacement step
fn generate_molecules(replacements: &[Replacement], molecule: &str) -> HashSet<String> {
    let mut new_molecules = HashSet::new();

    for replacement in replacements {
        // Find all occurrences of the 'from' pattern in the molecule
        let mut start = 0;
        while let Some(pos) = molecule[start..].find(&replacement.from) {
            let actual_pos = start + pos;

            // Create new molecule by replacing this occurrence
            let mut new_molecule = String::new();
            new_molecule.push_str(&molecule[..actual_pos]);
            new_molecule.push_str(&replacement.to);
            new_molecule.push_str(&molecule[actual_pos + replacement.from.len()..]);

            new_molecules.insert(new_molecule);

            // Move start position to look for next occurrence
            start = actual_pos + 1;
        }
    }

    new_molecules
}

/// Find the minimum number of steps to create the target molecule from 'e'
/// This uses a reverse approach: start from target and work backwards to 'e'
fn find_min_steps_to_create(replacements: &[Replacement], target: &str) -> Result<usize> {
    // Create reverse replacement map: to => from
    let mut reverse_replacements = Vec::new();
    for replacement in replacements {
        reverse_replacements.push(Replacement {
            from: replacement.to.clone(),
            to: replacement.from.clone(),
        });
    }

    // Sort by length of 'from' (now the original 'to') in descending order
    // This ensures we make the most progress with each replacement
    reverse_replacements.sort_by(|a, b| b.from.len().cmp(&a.from.len()));

    let mut current = target.to_string();
    let mut steps = 0;

    while current != "e" {
        let original_current = current.clone();

        // Try to apply any reverse replacement
        for replacement in &reverse_replacements {
            if let Some(_pos) = current.find(&replacement.from) {
                // Apply the first possible replacement
                current = current.replacen(&replacement.from, &replacement.to, 1);
                steps += 1;
                break;
            }
        }

        // If no replacement was made, we're stuck
        if current == original_current {
            anyhow::bail!("Cannot reduce molecule '{}' further", current);
        }

        // Safety check to prevent infinite loops
        if steps > 1000 {
            anyhow::bail!("Too many steps (>1000), possible infinite loop");
        }
    }

    Ok(steps)
}

pub fn solve_part1(input: &str) -> Result<String> {
    let (replacements, molecule) = parse_input(input)?;

    let new_molecules = generate_molecules(&replacements, &molecule);

    Ok(new_molecules.len().to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let (replacements, molecule) = parse_input(input)?;

    let steps = find_min_steps_to_create(&replacements, &molecule)?;

    Ok(steps.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_parsing() {
        let input = "H => HO\nH => OH\nO => HH\n\nHOH";
        let (replacements, molecule) = parse_input(input).unwrap();

        assert_eq!(replacements.len(), 3);
        assert_eq!(replacements[0].from, "H");
        assert_eq!(replacements[0].to, "HO");
        assert_eq!(replacements[1].from, "H");
        assert_eq!(replacements[1].to, "OH");
        assert_eq!(replacements[2].from, "O");
        assert_eq!(replacements[2].to, "HH");
        assert_eq!(molecule, "HOH");
    }

    #[test]
    fn test_simple_generation() {
        let input = "H => HO\nH => OH\nO => HH\n\nHOH";
        let (replacements, molecule) = parse_input(input).unwrap();

        let new_molecules = generate_molecules(&replacements, &molecule);

        // Expected: HOOH (H=>HO on first H), HOHO (H=>HO on second H),
        //          OHOH (H=>OH on first H), HOOH (H=>OH on second H), HHHH (O=>HH)
        // Distinct: HOOH, HOHO, OHOH, HHHH = 4 molecules
        assert_eq!(new_molecules.len(), 4);

        assert!(new_molecules.contains("HOOH")); // H=>HO on first H or H=>OH on second H
        assert!(new_molecules.contains("HOHO")); // H=>HO on second H
        assert!(new_molecules.contains("OHOH")); // H=>OH on first H
        assert!(new_molecules.contains("HHHH")); // O=>HH
    }

    #[test]
    fn test_hohoho_example() {
        let input = "H => HO\nH => OH\nO => HH\n\nHOHOHO";
        let (replacements, molecule) = parse_input(input).unwrap();

        let new_molecules = generate_molecules(&replacements, &molecule);

        // The problem states HOHOHO can become 7 distinct molecules
        assert_eq!(new_molecules.len(), 7);
    }

    #[test]
    fn test_replacement_without_regard_for_surrounding() {
        let input = "H => OO\n\nH2O";
        let (replacements, molecule) = parse_input(input).unwrap();

        let new_molecules = generate_molecules(&replacements, &molecule);

        // Should result in OO2O
        assert_eq!(new_molecules.len(), 1);
        assert!(new_molecules.contains("OO2O"));
    }
}
