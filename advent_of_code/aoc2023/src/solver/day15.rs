use anyhow::Result;

/// Day 15: Lens Library - HASH algorithm
///
/// # Problem
/// Implement the Holiday ASCII String Helper (HASH) algorithm:
/// - Start with current_value = 0
/// - For each character:
///   1. Add ASCII code to current_value
///   2. Multiply by 17
///   3. Take remainder when divided by 256
///
/// Part 1: Sum HASH values of all comma-separated steps
pub fn solve_part1(input: &str) -> Result<String> {
    let steps = parse_input(input);
    let total: usize = steps.iter().map(|step| hash(step) as usize).sum();
    Ok(total.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let steps = parse_input(input);
    
    // Create 256 boxes, each containing a vector of (label, focal_length) pairs
    let mut boxes: Vec<Vec<(String, u32)>> = vec![vec![]; 256];
    
    // Process each step
    for step in steps {
        if step.contains('=') {
            process_add_operation(&mut boxes, &step)?;
        } else if step.contains('-') {
            process_remove_operation(&mut boxes, &step);
        }
    }
    
    let total_power = calculate_focusing_power(&boxes);
    Ok(total_power.to_string())
}

/// Parse input into individual steps (comma-separated, ignore newlines)
fn parse_input(input: &str) -> Vec<String> {
    input
        .chars()
        .filter(|c| *c != '\n')  // Ignore newlines
        .collect::<String>()
        .split(',')
        .map(|s| s.to_string())
        .collect()
}

/// Process add/replace operation: "label=focal_length"
///
/// If lens with label exists, replace its focal length.
/// Otherwise, add new lens to the back of the box.
fn process_add_operation(boxes: &mut [Vec<(String, u32)>], step: &str) -> Result<()> {
    let pos = step.find('=').expect("Add operation must contain '='");
    let label = &step[..pos];
    let focal_length: u32 = step[pos+1..].parse()?;
    let box_num = hash(label) as usize;
    
    // Check if lens with this label already exists in the box
    if let Some(existing_idx) = boxes[box_num].iter().position(|(l, _)| l == label) {
        // Replace existing lens
        boxes[box_num][existing_idx].1 = focal_length;
    } else {
        // Add new lens to the back
        boxes[box_num].push((label.to_string(), focal_length));
    }
    
    Ok(())
}

/// Process remove operation: "label-"
///
/// Remove lens with matching label from the appropriate box.
fn process_remove_operation(boxes: &mut [Vec<(String, u32)>], step: &str) {
    let pos = step.find('-').expect("Remove operation must contain '-'");
    let label = &step[..pos];
    let box_num = hash(label) as usize;
    
    // Remove lens with this label if present
    boxes[box_num].retain(|(l, _)| l != label);
}

/// Calculate total focusing power across all boxes
///
/// Focusing power = (box_number + 1) × (slot + 1) × focal_length
fn calculate_focusing_power(boxes: &[Vec<(String, u32)>]) -> usize {
    let mut total_power = 0;
    for (box_num, lenses) in boxes.iter().enumerate() {
        for (slot, (_, focal_length)) in lenses.iter().enumerate() {
            let power = (box_num + 1) * (slot + 1) * (*focal_length as usize);
            total_power += power;
        }
    }
    total_power
}

/// HASH algorithm: converts string to number 0-255
///
/// For each character:
/// 1. current_value += ASCII code
/// 2. current_value *= 17
/// 3. current_value %= 256
fn hash(s: &str) -> u8 {
    let mut value: u16 = 0;
    for ch in s.chars() {
        value += ch as u16;
        value *= 17;
        value %= 256;
    }
    value as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash_algorithm() {
        // HASH("HASH") = 52 (from problem description)
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_individual_hashes() {
        // From problem description examples
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
        assert_eq!(hash("qp=3"), 97);
        assert_eq!(hash("cm=2"), 47);
        assert_eq!(hash("qp-"), 14);
        assert_eq!(hash("pc=4"), 180);
        assert_eq!(hash("ot=9"), 9);
        assert_eq!(hash("ab=5"), 197);
        assert_eq!(hash("pc-"), 48);
        assert_eq!(hash("pc=6"), 214);
        assert_eq!(hash("ot=7"), 231);
    }

    #[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result, "1320");
    }

    #[test]
    fn test_parse_input() {
        let steps = parse_input("rn=1,cm-,qp=3");
        assert_eq!(steps, vec!["rn=1", "cm-", "qp=3"]);
    }

    #[test]
    fn test_parse_input_with_newlines() {
        let steps = parse_input("rn=1,cm-\n,qp=3");
        assert_eq!(steps, vec!["rn=1", "cm-", "qp=3"]);
    }

    #[test]
    fn test_part2_example() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result, "145");
    }

    #[test]
    fn test_hashmap_operations() {
        // Test individual operations on boxes
        let input = "rn=1,cm=2,rn=3"; // rn box 0, cm box 0, replace rn
        let result = solve_part2(input).unwrap();
        // Box 0: [rn 3] [cm 2]
        // rn: 1 * 1 * 3 = 3
        // cm: 1 * 2 * 2 = 4
        // Total: 7
        assert_eq!(result, "7");
    }
}
