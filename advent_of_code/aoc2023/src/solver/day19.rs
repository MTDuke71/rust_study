//! Day 19: Aplenty - Workflow Processing with Rule Evaluation
//!
//! **Problem**: Process machine parts through a series of workflows with conditional rules.
//! Each workflow has rules that check part attributes (x, m, a, s) and route to other workflows
//! or accept/reject the part.
//!
//! **Approach**:
//! - Part 1: Simulate workflow execution for each part, sum ratings of accepted parts
//! - Part 2: (TBD after Part 1 completion)
//!
//! **Key Patterns**:
//! - Workflow state machine with rule-based transitions
//! - Pattern matching on conditions and destinations
//! - HashMap for workflow lookup
//!
//! **Zettelkasten Links**: [[workflow-pattern-matching]], [[Error Handling Patterns]], [[state-machine-rust]]

use std::collections::HashMap;

/// Part rating with four attributes
#[derive(Debug, Clone, Copy)]
pub struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    /// Get total rating (sum of all attributes)
    fn rating(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }

    /// Get attribute value by name
    fn get(&self, attr: char) -> u32 {
        match attr {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("Invalid attribute: {}", attr),
        }
    }
}

/// Comparison operator for rule conditions
#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    LessThan,
    GreaterThan,
}

/// Destination for a rule (workflow name, Accept, or Reject)
#[derive(Debug, Clone, PartialEq)]
enum Destination {
    Workflow(String),
    Accept,
    Reject,
}

impl Destination {
    fn from_str(s: &str) -> Self {
        match s {
            "A" => Destination::Accept,
            "R" => Destination::Reject,
            name => Destination::Workflow(name.to_string()),
        }
    }
}

/// A single rule in a workflow
#[derive(Debug, Clone)]
enum Rule {
    /// Conditional rule: if attr op value then goto dest
    Conditional {
        attr: char,
        op: Op,
        value: u32,
        dest: Destination,
    },
    /// Unconditional rule: always goto dest (fallback)
    Unconditional { dest: Destination },
}

impl Rule {
    /// Check if this rule matches the given part
    fn matches(&self, part: &Part) -> bool {
        match self {
            Rule::Conditional {
                attr, op, value, ..
            } => {
                let part_value = part.get(*attr);
                match op {
                    Op::LessThan => part_value < *value,
                    Op::GreaterThan => part_value > *value,
                }
            }
            Rule::Unconditional { .. } => true, // Always matches
        }
    }

    /// Get destination if this rule matches
    fn destination(&self) -> &Destination {
        match self {
            Rule::Conditional { dest, .. } => dest,
            Rule::Unconditional { dest } => dest,
        }
    }
}

/// A workflow with a name and list of rules
#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    /// Process a part through this workflow, returning the destination
    fn process(&self, part: &Part) -> &Destination {
        for rule in &self.rules {
            if rule.matches(part) {
                return rule.destination();
            }
        }
        panic!("Workflow {} has no matching rule", self.name);
    }
}

/// Parse a part from input like "{x=787,m=2655,a=1222,s=2876}"
fn parse_part(line: &str) -> Part {
    let line = line.trim_matches(|c| c == '{' || c == '}');
    let mut x = 0;
    let mut m = 0;
    let mut a = 0;
    let mut s = 0;

    for pair in line.split(',') {
        let (attr, value) = pair.split_once('=').unwrap();
        let val: u32 = value.parse().unwrap();
        match attr {
            "x" => x = val,
            "m" => m = val,
            "a" => a = val,
            "s" => s = val,
            _ => panic!("Unknown attribute: {}", attr),
        }
    }

    Part { x, m, a, s }
}

/// Parse a rule from text like "x>10:one" or "A"
fn parse_rule(text: &str) -> Rule {
    if let Some((condition, dest_str)) = text.split_once(':') {
        // Conditional rule
        let attr = condition.chars().next().unwrap();
        let op = if condition.contains('<') {
            Op::LessThan
        } else {
            Op::GreaterThan
        };
        let value: u32 = condition[2..].parse().unwrap();
        let dest = Destination::from_str(dest_str);

        Rule::Conditional {
            attr,
            op,
            value,
            dest,
        }
    } else {
        // Unconditional rule (fallback)
        Rule::Unconditional {
            dest: Destination::from_str(text),
        }
    }
}

/// Parse a workflow from input like "px{a<2006:qkq,m>2090:A,rfg}"
fn parse_workflow(line: &str) -> Workflow {
    let (name, rules_text) = line.split_once('{').unwrap();
    let rules_text = rules_text.trim_end_matches('}');

    let rules: Vec<Rule> = rules_text.split(',').map(parse_rule).collect();

    Workflow {
        name: name.to_string(),
        rules,
    }
}

/// Process a part through all workflows, starting at "in"
fn process_part(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
    let mut current = "in";

    loop {
        let workflow = workflows.get(current).expect("Workflow not found");
        let dest = workflow.process(part);

        match dest {
            Destination::Accept => return true,
            Destination::Reject => return false,
            Destination::Workflow(name) => current = name,
        }
    }
}

pub fn solve_part1(input: &str) -> u32 {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let workflows_text = sections[0];
    let parts_text = sections[1];

    // Parse workflows into HashMap
    let mut workflows = HashMap::new();
    for line in workflows_text.lines() {
        let workflow = parse_workflow(line);
        workflows.insert(workflow.name.clone(), workflow);
    }

    // Parse and process parts
    let parts: Vec<Part> = parts_text.lines().map(parse_part).collect();

    let mut total = 0;
    for part in parts {
        if process_part(&part, &workflows) {
            total += part.rating();
        }
    }

    total
}

/// Range of valid values for a single attribute
#[derive(Debug, Clone, Copy)]
struct Range {
    min: u64,
    max: u64,
}

impl Range {
    fn new() -> Self {
        Range { min: 1, max: 4000 }
    }

    fn size(&self) -> u64 {
        if self.max >= self.min {
            self.max - self.min + 1
        } else {
            0
        }
    }

    fn is_empty(&self) -> bool {
        self.max < self.min
    }
}

/// Ranges for all four attributes (x, m, a, s)
#[derive(Debug, Clone, Copy)]
struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRange {
    fn new() -> Self {
        PartRange {
            x: Range::new(),
            m: Range::new(),
            a: Range::new(),
            s: Range::new(),
        }
    }

    /// Get range for a specific attribute
    fn get(&self, attr: char) -> Range {
        match attr {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("Invalid attribute: {}", attr),
        }
    }

    /// Set range for a specific attribute
    fn set(&mut self, attr: char, range: Range) {
        match attr {
            'x' => self.x = range,
            'm' => self.m = range,
            'a' => self.a = range,
            's' => self.s = range,
            _ => panic!("Invalid attribute: {}", attr),
        }
    }

    /// Calculate total combinations (product of all range sizes)
    fn combinations(&self) -> u64 {
        self.x.size() * self.m.size() * self.a.size() * self.s.size()
    }

    /// Check if any range is empty
    fn is_empty(&self) -> bool {
        self.x.is_empty() || self.m.is_empty() || self.a.is_empty() || self.s.is_empty()
    }
}

/// Split a range based on a conditional rule
/// Returns (matching_range, non_matching_range)
fn split_range(range: Range, op: Op, value: u64) -> (Range, Range) {
    match op {
        Op::LessThan => {
            // Matching: [min, value-1]
            let matching = Range {
                min: range.min,
                max: range.max.min(value - 1),
            };
            // Non-matching: [value, max]
            let non_matching = Range {
                min: range.min.max(value),
                max: range.max,
            };
            (matching, non_matching)
        }
        Op::GreaterThan => {
            // Matching: [value+1, max]
            let matching = Range {
                min: range.min.max(value + 1),
                max: range.max,
            };
            // Non-matching: [min, value]
            let non_matching = Range {
                min: range.min,
                max: range.max.min(value),
            };
            (matching, non_matching)
        }
    }
}

/// Count accepted combinations starting from a workflow with given ranges
fn count_accepted(
    workflow_name: &str,
    mut ranges: PartRange,
    workflows: &HashMap<String, Workflow>,
) -> u64 {
    if ranges.is_empty() {
        return 0;
    }

    // Terminal cases
    if workflow_name == "A" {
        return ranges.combinations();
    }
    if workflow_name == "R" {
        return 0;
    }

    let workflow = workflows.get(workflow_name).expect("Workflow not found");
    let mut total = 0;

    for rule in &workflow.rules {
        match rule {
            Rule::Conditional {
                attr,
                op,
                value,
                dest,
            } => {
                let current_range = ranges.get(*attr);
                let (matching, non_matching) = split_range(current_range, *op, *value as u64);

                // Process matching range with destination
                if !matching.is_empty() {
                    let mut matching_ranges = ranges;
                    matching_ranges.set(*attr, matching);

                    let dest_name = match dest {
                        Destination::Accept => "A",
                        Destination::Reject => "R",
                        Destination::Workflow(name) => name.as_str(),
                    };

                    total += count_accepted(dest_name, matching_ranges, workflows);
                }

                // Continue with non-matching range to next rule
                if non_matching.is_empty() {
                    break; // No more possibilities
                }
                ranges.set(*attr, non_matching);
            }
            Rule::Unconditional { dest } => {
                // All remaining ranges go to this destination
                let dest_name = match dest {
                    Destination::Accept => "A",
                    Destination::Reject => "R",
                    Destination::Workflow(name) => name.as_str(),
                };

                total += count_accepted(dest_name, ranges, workflows);
                break;
            }
        }
    }

    total
}

pub fn solve_part2(input: &str) -> String {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let workflows_text = sections[0];

    // Parse workflows into HashMap
    let mut workflows = HashMap::new();
    for line in workflows_text.lines() {
        let workflow = parse_workflow(line);
        workflows.insert(workflow.name.clone(), workflow);
    }

    // Start with full ranges [1, 4000] for all attributes
    let initial_ranges = PartRange::new();
    let count = count_accepted("in", initial_ranges, &workflows);

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
hdj{m>838:A,pv}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_part_parsing() {
        let part = parse_part("{x=787,m=2655,a=1222,s=2876}");
        assert_eq!(part.x, 787);
        assert_eq!(part.m, 2655);
        assert_eq!(part.a, 1222);
        assert_eq!(part.s, 2876);
        assert_eq!(part.rating(), 787 + 2655 + 1222 + 2876);
    }

    #[test]
    fn test_workflow_parsing() {
        let workflow = parse_workflow("px{a<2006:qkq,m>2090:A,rfg}");
        assert_eq!(workflow.name, "px");
        assert_eq!(workflow.rules.len(), 3);
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(solve_part1(EXAMPLE), 19114);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(solve_part2(EXAMPLE), "167409079868000");
    }
}
