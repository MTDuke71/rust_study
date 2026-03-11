//! Day 10: Balance Bots
//!
//! Simulate a network of bots passing microchips by value (low/high).
//! Part 1: Which bot compares chips 61 and 17?
//! Part 2: Product of values in outputs 0, 1, and 2.

use std::collections::HashMap;

/// Where a bot sends its low or high chip
#[derive(Debug, Clone, Copy)]
enum Destination {
    Bot(usize),
    Output(usize),
}

/// A bot's distribution rule
#[derive(Debug, Clone, Copy)]
struct Rule {
    low: Destination,
    high: Destination,
}

/// Parsed input: initial value assignments + bot rules
#[derive(Debug)]
struct ParsedData {
    /// (value, bot_id) — "value X goes to bot Y"
    assignments: Vec<(usize, usize)>,
    /// bot_id → Rule
    rules: HashMap<usize, Rule>,
}

fn parse_destination(kind: &str, id: &str) -> Destination {
    let id: usize = id.parse().unwrap();
    match kind {
        "bot" => Destination::Bot(id),
        "output" => Destination::Output(id),
        _ => panic!("unknown destination: {kind}"),
    }
}

fn parse_input(input: &str) -> ParsedData {
    let mut assignments = Vec::new();
    let mut rules = HashMap::new();

    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words[0] == "value" {
            // "value X goes to bot Y"
            let value: usize = words[1].parse().unwrap();
            let bot: usize = words[5].parse().unwrap();
            assignments.push((value, bot));
        } else {
            // "bot X gives low to [bot|output] Y and high to [bot|output] Z"
            let bot_id: usize = words[1].parse().unwrap();
            let low = parse_destination(words[5], words[6]);
            let high = parse_destination(words[10], words[11]);
            rules.insert(bot_id, Rule { low, high });
        }
    }

    ParsedData { assignments, rules }
}

/// Simulate the bot network. Returns (part1_bot, part2_product).
fn simulate(data: &ParsedData, target_low: usize, target_high: usize) -> (usize, usize) {
    let mut chips: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut outputs: HashMap<usize, usize> = HashMap::new();
    let mut part1 = 0;

    // Initial assignments
    for &(value, bot) in &data.assignments {
        chips.entry(bot).or_default().push(value);
    }

    // Process until no bot has 2 chips
    loop {
        // Find a bot with 2 chips
        let ready = chips.iter().find(|(_, v)| v.len() == 2).map(|(&id, _)| id);

        let Some(bot_id) = ready else { break };

        let mut held = chips.remove(&bot_id).unwrap();
        held.sort();
        let (lo, hi) = (held[0], held[1]);

        // Part 1 check
        if lo == target_low && hi == target_high {
            part1 = bot_id;
        }

        let rule = data.rules[&bot_id];

        // Distribute low
        match rule.low {
            Destination::Bot(id) => chips.entry(id).or_default().push(lo),
            Destination::Output(id) => { outputs.insert(id, lo); }
        }

        // Distribute high
        match rule.high {
            Destination::Bot(id) => chips.entry(id).or_default().push(hi),
            Destination::Output(id) => { outputs.insert(id, hi); }
        }
    }

    let part2 = outputs.get(&0).unwrap_or(&0)
        * outputs.get(&1).unwrap_or(&0)
        * outputs.get(&2).unwrap_or(&0);

    (part1, part2)
}

fn solve_part1_with_data(data: &ParsedData) -> usize {
    simulate(data, 17, 61).0
}

fn solve_part2_with_data(data: &ParsedData) -> usize {
    simulate(data, 17, 61).1
}

pub fn solve(input: &str) -> (String, String) {
    let data = parse_input(input);
    let (p1, p2) = simulate(&data, 17, 61);
    (p1.to_string(), p2.to_string())
}

pub fn solve_part1(input: &str) -> String {
    let data = parse_input(input);
    solve_part1_with_data(&data).to_string()
}

pub fn solve_part2(input: &str) -> String {
    let data = parse_input(input);
    solve_part2_with_data(&data).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";

    #[test]
    fn test_parse() {
        let data = parse_input(EXAMPLE);
        assert_eq!(data.assignments.len(), 3);
        assert_eq!(data.rules.len(), 3);
    }

    #[test]
    fn test_part1_example() {
        // In the example, bot 2 compares values 5 and 2
        let data = parse_input(EXAMPLE);
        assert_eq!(simulate(&data, 2, 5).0, 2);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day10.txt");
        let data = parse_input(input);
        assert_eq!(solve_part1_with_data(&data), 157);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day10.txt");
        let data = parse_input(input);
        assert_eq!(solve_part2_with_data(&data), 1085);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day10.txt");
        assert_eq!(solve(input), ("157".to_string(), "1085".to_string()));
    }
}
