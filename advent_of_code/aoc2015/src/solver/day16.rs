use anyhow::Result;
use std::collections::HashMap;

/// AoC 2015 Day 16: Aunt Sue
///
/// Part 1: Find which Aunt Sue gave you the gift based on MFCSAM analysis
/// Part 2: What is the number of the real Aunt Sue after adding the new comparison rules?

#[derive(Debug, Clone)]
pub struct AuntSue {
    number: usize,
    properties: HashMap<String, usize>,
}

impl AuntSue {
    pub fn new(number: usize) -> Self {
        Self {
            number,
            properties: HashMap::new(),
        }
    }

    pub fn add_property(&mut self, key: String, value: usize) {
        self.properties.insert(key, value);
    }

    pub fn matches(&self, target: &HashMap<String, usize>) -> bool {
        // Check if all known properties match the target
        for (key, value) in &self.properties {
            if let Some(&target_value) = target.get(key) {
                if *value != target_value {
                    return false;
                }
            }
        }
        true
    }

    pub fn matches_part2(&self, target: &HashMap<String, usize>) -> bool {
        // Part 2: Different comparison rules for different properties
        for (key, value) in &self.properties {
            if let Some(&target_value) = target.get(key) {
                match key.as_str() {
                    "cats" | "trees" => {
                        // Must be GREATER than target
                        if *value <= target_value {
                            return false;
                        }
                    }
                    "pomeranians" | "goldfish" => {
                        // Must be FEWER than target
                        if *value >= target_value {
                            return false;
                        }
                    }
                    _ => {
                        // Must be EQUAL to target
                        if *value != target_value {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

pub fn solve_part1(input: &str) -> Result<String> {
    let aunts = parse_input(input)?;

    // MFCSAM analysis results
    let mut target = HashMap::new();
    target.insert("children".to_string(), 3);
    target.insert("cats".to_string(), 7);
    target.insert("samoyeds".to_string(), 2);
    target.insert("pomeranians".to_string(), 3);
    target.insert("akitas".to_string(), 0);
    target.insert("vizslas".to_string(), 0);
    target.insert("goldfish".to_string(), 5);
    target.insert("trees".to_string(), 3);
    target.insert("cars".to_string(), 2);
    target.insert("perfumes".to_string(), 1);

    // Find the Aunt Sue that matches
    for aunt in &aunts {
        if aunt.matches(&target) {
            return Ok(aunt.number.to_string());
        }
    }

    Ok("0".to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let aunts = parse_input(input)?;

    // MFCSAM analysis results (same as Part 1)
    let mut target = HashMap::new();
    target.insert("children".to_string(), 3);
    target.insert("cats".to_string(), 7);
    target.insert("samoyeds".to_string(), 2);
    target.insert("pomeranians".to_string(), 3);
    target.insert("akitas".to_string(), 0);
    target.insert("vizslas".to_string(), 0);
    target.insert("goldfish".to_string(), 5);
    target.insert("trees".to_string(), 3);
    target.insert("cars".to_string(), 2);
    target.insert("perfumes".to_string(), 1);

    // Find the Aunt Sue that matches with Part 2 rules
    for aunt in &aunts {
        if aunt.matches_part2(&target) {
            return Ok(aunt.number.to_string());
        }
    }

    Ok("0".to_string())
}

fn parse_input(input: &str) -> Result<Vec<AuntSue>> {
    let mut aunts = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        // Parse: "Sue 1: goldfish: 9, cars: 0, samoyeds: 9"
        // Split at first colon to separate Sue number from properties
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 2 {
            continue;
        }

        // Extract Sue number from "Sue 1"
        let sue_part = parts[0].trim();
        let number = sue_part
            .strip_prefix("Sue ")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0);

        let mut aunt = AuntSue::new(number);

        // Parse properties: "goldfish: 9, cars: 0, samoyeds: 9"
        // Join remaining parts in case there are colons in values (shouldn't be, but safe)
        let properties_str = parts[1..].join(":");

        // Split by comma to get individual property: value pairs
        for property_pair in properties_str.split(',') {
            let prop_parts: Vec<&str> = property_pair.split(':').collect();
            if prop_parts.len() == 2 {
                let key = prop_parts[0].trim().to_string();
                if let Ok(value) = prop_parts[1].trim().parse::<usize>() {
                    aunt.add_property(key, value);
                }
            }
        }

        aunts.push(aunt);
    }

    Ok(aunts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input =
            "Sue 1: goldfish: 9, cars: 0, samoyeds: 9\nSue 2: perfumes: 5, trees: 8, goldfish: 8";
        let aunts = parse_input(input).unwrap();
        assert_eq!(aunts.len(), 2);
        assert_eq!(aunts[0].number, 1);
        assert_eq!(aunts[0].properties.get("goldfish"), Some(&9));
        assert_eq!(aunts[0].properties.get("cars"), Some(&0));
        assert_eq!(aunts[0].properties.get("samoyeds"), Some(&9));
        assert_eq!(aunts[1].number, 2);
        assert_eq!(aunts[1].properties.get("perfumes"), Some(&5));
    }

    #[test]
    fn test_aunt_matches_all_properties() {
        let mut aunt = AuntSue::new(1);
        aunt.add_property("cats".to_string(), 7);
        aunt.add_property("cars".to_string(), 2);

        let mut target = HashMap::new();
        target.insert("cats".to_string(), 7);
        target.insert("cars".to_string(), 2);
        target.insert("perfumes".to_string(), 1);

        assert!(aunt.matches(&target));
    }

    #[test]
    fn test_aunt_doesnt_match() {
        let mut aunt = AuntSue::new(1);
        aunt.add_property("cats".to_string(), 5); // Wrong value
        aunt.add_property("cars".to_string(), 2);

        let mut target = HashMap::new();
        target.insert("cats".to_string(), 7);
        target.insert("cars".to_string(), 2);

        assert!(!aunt.matches(&target));
    }

    #[test]
    fn test_aunt_matches_subset() {
        // Aunt only knows 2 properties, target has 3
        let mut aunt = AuntSue::new(1);
        aunt.add_property("cats".to_string(), 7);
        aunt.add_property("cars".to_string(), 2);

        let mut target = HashMap::new();
        target.insert("cats".to_string(), 7);
        target.insert("cars".to_string(), 2);
        target.insert("perfumes".to_string(), 1);

        // Should match because known properties match
        assert!(aunt.matches(&target));
    }

    #[test]
    fn test_part2_cats_greater_than() {
        let mut aunt = AuntSue::new(1);
        aunt.add_property("cats".to_string(), 8); // Greater than 7
        aunt.add_property("cars".to_string(), 2);

        let mut target = HashMap::new();
        target.insert("cats".to_string(), 7);
        target.insert("cars".to_string(), 2);

        // Should match in Part 2 (cats > 7)
        assert!(aunt.matches_part2(&target));
        // Should NOT match in Part 1 (cats != 7)
        assert!(!aunt.matches(&target));
    }

    #[test]
    fn test_part2_trees_greater_than() {
        let mut aunt = AuntSue::new(1);
        aunt.add_property("trees".to_string(), 5); // Greater than 3

        let mut target = HashMap::new();
        target.insert("trees".to_string(), 3);

        assert!(aunt.matches_part2(&target));
    }

    #[test]
    fn test_part2_goldfish_fewer_than() {
        let mut aunt = AuntSue::new(1);
        aunt.add_property("goldfish".to_string(), 3); // Fewer than 5

        let mut target = HashMap::new();
        target.insert("goldfish".to_string(), 5);

        assert!(aunt.matches_part2(&target));
    }

    #[test]
    fn test_part2_pomeranians_fewer_than() {
        let mut aunt = AuntSue::new(1);
        aunt.add_property("pomeranians".to_string(), 2); // Fewer than 3

        let mut target = HashMap::new();
        target.insert("pomeranians".to_string(), 3);

        assert!(aunt.matches_part2(&target));
    }

    #[test]
    fn test_part2_exact_match_for_normal_properties() {
        let mut aunt = AuntSue::new(1);
        aunt.add_property("children".to_string(), 3); // Must be exact
        aunt.add_property("cars".to_string(), 2); // Must be exact

        let mut target = HashMap::new();
        target.insert("children".to_string(), 3);
        target.insert("cars".to_string(), 2);

        assert!(aunt.matches_part2(&target));

        // Wrong value should fail
        aunt.add_property("children".to_string(), 4);
        assert!(!aunt.matches_part2(&target));
    }

    #[test]
    fn test_part2_cats_equal_or_less_fails() {
        let mut aunt = AuntSue::new(1);
        aunt.add_property("cats".to_string(), 7); // Equal to target

        let mut target = HashMap::new();
        target.insert("cats".to_string(), 7);

        // Should fail because cats must be > 7, not == 7
        assert!(!aunt.matches_part2(&target));
    }

    #[test]
    fn test_part2_goldfish_equal_or_more_fails() {
        let mut aunt = AuntSue::new(1);
        aunt.add_property("goldfish".to_string(), 5); // Equal to target

        let mut target = HashMap::new();
        target.insert("goldfish".to_string(), 5);

        // Should fail because goldfish must be < 5, not == 5
        assert!(!aunt.matches_part2(&target));
    }
}
