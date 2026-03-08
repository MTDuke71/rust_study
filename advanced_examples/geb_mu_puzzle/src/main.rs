//! # The MU Puzzle — GEB Chapter 1
//!
//! Hofstadter's MIU formal system from "Gödel, Escher, Bach".
//!
//! Starting from axiom **MI**, apply four rules to derive new strings:
//!   1. xI  → xIU     (if it ends in I, append U)
//!   2. Mx  → Mxx     (double everything after M)
//!   3. xIIIy → xUy   (replace any III with U)
//!   4. xUUy  → xy    (drop any UU)
//!
//! **The puzzle**: Can you derive **MU** from **MI**?

use std::collections::{BTreeSet, HashSet, VecDeque};
use std::fmt;

// ── The MIU System ──────────────────────────────────────────────────

/// Which rule was applied to produce a string.
#[derive(Debug, Clone, Copy)]
enum Rule {
    Axiom,
    /// Rule 1: xI → xIU
    AppendU,
    /// Rule 2: Mx → Mxx
    DoubleAfterM,
    /// Rule 3: replace III at position `usize` with U
    ReplaceIII(usize),
    /// Rule 4: drop UU at position `usize`
    DropUU(usize),
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rule::Axiom => write!(f, "axiom"),
            Rule::AppendU => write!(f, "Rule 1 (xI -> xIU)"),
            Rule::DoubleAfterM => write!(f, "Rule 2 (Mx -> Mxx)"),
            Rule::ReplaceIII(pos) => write!(f, "Rule 3 (III->U at pos {pos})"),
            Rule::DropUU(pos) => write!(f, "Rule 4 (UU dropped at pos {pos})"),
        }
    }
}

/// A derivation step: the string produced and which rule created it.
#[derive(Debug, Clone)]
struct Step {
    string: String,
    rule: Rule,
    parent: Option<usize>, // index into the BFS history
}

/// Apply all four MIU rules to `s`, returning every possible result.
fn apply_rules(s: &str) -> Vec<(String, Rule)> {
    let mut results = Vec::new();

    // Rule 1: if it ends with I, append U
    if s.ends_with('I') {
        results.push((format!("{s}U"), Rule::AppendU));
    }

    // Rule 2: Mx → Mxx (double everything after M)
    if s.starts_with('M') && s.len() > 1 {
        let after_m = &s[1..];
        // Guard against strings that would blow up memory
        if after_m.len() <= 40 {
            results.push((format!("M{after_m}{after_m}"), Rule::DoubleAfterM));
        }
    }

    // Rule 3: replace any occurrence of III with U
    let bytes = s.as_bytes();
    for i in 0..s.len().saturating_sub(2) {
        if bytes[i] == b'I' && bytes[i + 1] == b'I' && bytes[i + 2] == b'I' {
            let mut new = String::with_capacity(s.len() - 2);
            new.push_str(&s[..i]);
            new.push('U');
            new.push_str(&s[i + 3..]);
            results.push((new, Rule::ReplaceIII(i)));
        }
    }

    // Rule 4: drop any occurrence of UU
    for i in 0..s.len().saturating_sub(1) {
        if bytes[i] == b'U' && bytes[i + 1] == b'U' {
            let mut new = String::with_capacity(s.len() - 2);
            new.push_str(&s[..i]);
            new.push_str(&s[i + 2..]);
            results.push((new, Rule::DropUU(i)));
        }
    }

    results
}

/// Count the number of I's in a MIU string.
fn count_i(s: &str) -> usize {
    s.chars().filter(|&c| c == 'I').count()
}

// ── BFS Exploration ─────────────────────────────────────────────────

/// Explore the MIU derivation space via BFS up to `max_steps` derivations.
/// Returns the history of all discovered strings.
fn bfs_explore(axiom: &str, max_steps: usize) -> Vec<Step> {
    let mut history: Vec<Step> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<usize> = VecDeque::new(); // indices into history

    // Seed with axiom
    history.push(Step {
        string: axiom.to_string(),
        rule: Rule::Axiom,
        parent: None,
    });
    seen.insert(axiom.to_string());
    queue.push_back(0);

    while let Some(idx) = queue.pop_front() {
        if history.len() >= max_steps {
            break;
        }
        let current = history[idx].string.clone();
        for (new_string, rule) in apply_rules(&current) {
            if seen.contains(&new_string) {
                continue;
            }
            seen.insert(new_string.clone());
            let new_idx = history.len();
            history.push(Step {
                string: new_string,
                rule,
                parent: Some(idx),
            });
            queue.push_back(new_idx);
        }
    }

    history
}

/// Reconstruct the derivation path from axiom to the step at `target_idx`.
fn derivation_path(history: &[Step], target_idx: usize) -> Vec<(String, Rule)> {
    let mut path = Vec::new();
    let mut idx = target_idx;
    loop {
        let step = &history[idx];
        path.push((step.string.clone(), step.rule));
        match step.parent {
            Some(parent) => idx = parent,
            None => break,
        }
    }
    path.reverse();
    path
}

// ── The Impossibility Proof ─────────────────────────────────────────

fn impossibility_proof() {
    println!("=== WHY MU IS IMPOSSIBLE ===\n");
    println!("The key insight: track the number of I's modulo 3.\n");
    println!("  MI starts with 1 I.  (1 mod 3 = 1)\n");
    println!("  MU needs 0 I's.     (0 mod 3 = 0)\n");
    println!("How each rule affects the I-count:\n");
    println!("  Rule 1 (xI -> xIU):  I-count unchanged");
    println!("  Rule 2 (Mx -> Mxx):  I-count DOUBLES");
    println!("  Rule 3 (III -> U):   I-count decreases by 3");
    println!("  Rule 4 (UU -> gone): I-count unchanged\n");
    println!("Starting from 1 (mod 3), let's trace all possibilities:\n");

    // Show the mod-3 cycle
    let mut seen_states = BTreeSet::new();
    let mut value = 1_u64;
    println!("  Start:  I-count = 1  (mod 3 = {})", value % 3);
    seen_states.insert(value % 3);

    for step in 1..=8 {
        let doubled = value * 2;
        let d_mod = doubled % 3;
        println!(
            "  Step {step}: double -> {doubled}  (mod 3 = {d_mod})"
        );
        seen_states.insert(d_mod);
        value = doubled;
    }

    println!("\n  Subtracting 3 never changes the mod-3 value.");
    println!("  (If n mod 3 = k, then (n-3) mod 3 = k)\n");
    println!(
        "  Reachable mod-3 values: {:?}",
        seen_states.iter().collect::<Vec<_>>()
    );
    println!("  We need mod 3 = 0, but we can only reach {{1, 2}}.");
    println!("\n  THEREFORE: MU is UNREACHABLE from MI.  QED\n");
}

// ── Visualization ───────────────────────────────────────────────────

fn print_derivation_tree(history: &[Step], limit: usize) {
    println!("=== DERIVATION TREE (first {limit} strings) ===\n");
    println!("  {:>4}  {:<30} {:<30}  I-count (mod 3)", "#", "String", "Rule");
    println!("  {}", "-".repeat(85));

    for (i, step) in history.iter().take(limit).enumerate() {
        let ic = count_i(&step.string);
        let mod3 = ic % 3;
        println!(
            "  {:>4}  {:<30} {:<30}  {} (mod 3 = {})",
            i, step.string, format!("{}", step.rule), ic, mod3
        );
    }
    println!();
}

// ── Main ────────────────────────────────────────────────────────────

fn main() {
    println!();
    println!("  ╔══════════════════════════════════════════════════╗");
    println!("  ║     THE MU PUZZLE — GEB Chapter 1               ║");
    println!("  ║     Hofstadter's MIU Formal System               ║");
    println!("  ╚══════════════════════════════════════════════════╝\n");

    println!("Axiom: MI");
    println!("Goal:  Derive MU (spoiler: it's impossible!)\n");
    println!("Rules:");
    println!("  1. xI  -> xIU     (ends in I? append U)");
    println!("  2. Mx  -> Mxx     (double after M)");
    println!("  3. xIIIy -> xUy   (replace III with U)");
    println!("  4. xUUy  -> xy    (drop UU)\n");

    // Phase 1: BFS exploration
    let max_explore = 200;
    let history = bfs_explore("MI", max_explore);
    println!("Explored {} unique strings via BFS.\n", history.len());

    // Show the derivation tree
    print_derivation_tree(&history, 40);

    // Phase 2: Check for MU
    let mu_found = history.iter().enumerate().find(|(_, s)| s.string == "MU");
    match mu_found {
        Some((idx, _)) => {
            println!("FOUND MU at step {idx}! (this should never happen)\n");
            let path = derivation_path(&history, idx);
            for (s, r) in &path {
                println!("  {s}  <- {r}");
            }
        }
        None => {
            println!("MU was NOT found among {} explored strings.\n", history.len());
            println!("Is it just because we didn't search far enough? No!\n");
        }
    }

    // Phase 3: Show a sample derivation
    println!("=== SAMPLE DERIVATION: MI -> MUIU ===\n");
    // Find MUIU in history if it exists
    if let Some((idx, _)) = history.iter().enumerate().find(|(_, s)| s.string == "MUIU") {
        let path = derivation_path(&history, idx);
        for (i, (s, r)) in path.iter().enumerate() {
            let ic = count_i(s);
            if i == 0 {
                println!("  {s:<20} ({r})  I-count: {ic}");
            } else {
                println!("  {s:<20} <- {r:<35}  I-count: {ic}");
            }
        }
        println!();
    }

    // Phase 4: Mod-3 invariant analysis
    println!("=== MOD-3 INVARIANT ACROSS ALL EXPLORED STRINGS ===\n");
    let mut mod3_counts = [0_usize; 3];
    for step in &history {
        let ic = count_i(&step.string);
        mod3_counts[ic % 3] += 1;
    }
    println!("  I-count mod 3 = 0:  {} strings", mod3_counts[0]);
    println!("  I-count mod 3 = 1:  {} strings", mod3_counts[1]);
    println!("  I-count mod 3 = 2:  {} strings\n", mod3_counts[2]);
    println!(
        "  MU has 0 I's (mod 3 = 0).  Strings with 0 I-count mod 3: {}",
        mod3_counts[0]
    );
    if mod3_counts[0] == 0 {
        println!("  ZERO strings with I-count divisible by 3!\n");
    }

    // Phase 5: The proof
    impossibility_proof();

    // Phase 6: Rule 4 in action (it IS reachable, just creates cycles)
    println!("=== RULE 4 IN ACTION ===\n");
    println!("  Rule 4 (drop UU) IS reachable — it just creates cycles back");
    println!("  to strings we've already seen, so BFS doesn't show them.\n");
    println!("  Example cycle:  MI -> MII -> MIIII -> MIIIIU -> MIUU -> MI");
    println!("                  (R2)  (R2)    (R1)     (R3)    (R4)");
    println!();
    let chain = ["MI", "MII", "MIIII", "MIIIIU", "MIUU", "MI"];
    for pair in chain.windows(2) {
        let results = apply_rules(pair[0]);
        let found = results.iter().any(|(s, _)| s == pair[1]);
        println!("  {:<10} -> {:<10}  valid: {found}", pair[0], pair[1]);
    }
    println!();

    // Phase 7: String length growth
    println!("=== STRING LENGTH GROWTH ===\n");
    println!("  Rule 2 doubles the string. Without Rule 3/4 to shrink it,");
    println!("  the search space grows exponentially.\n");
    let mut s = "MI".to_string();
    for step in 0..8 {
        let ic = count_i(&s);
        println!("  Step {step}: {s:<40} len={:<4} I-count={ic}", s.len());
        // Apply Rule 2 each time
        if s.starts_with('M') {
            let after = &s[1..];
            s = format!("M{after}{after}");
        }
    }
    println!("\n  After 7 doublings: length {} (exponential growth!)\n", s.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule1_append_u() {
        let results = apply_rules("MI");
        let r1: Vec<_> = results.iter().filter(|(_, r)| matches!(r, Rule::AppendU)).collect();
        assert_eq!(r1.len(), 1);
        assert_eq!(r1[0].0, "MIU");
    }

    #[test]
    fn rule1_not_applicable_if_no_trailing_i() {
        let results = apply_rules("MU");
        assert!(results.iter().all(|(_, r)| !matches!(r, Rule::AppendU)));
    }

    #[test]
    fn rule2_double() {
        let results = apply_rules("MIU");
        let r2: Vec<_> = results
            .iter()
            .filter(|(_, r)| matches!(r, Rule::DoubleAfterM))
            .collect();
        assert_eq!(r2.len(), 1);
        assert_eq!(r2[0].0, "MIUIU");
    }

    #[test]
    fn rule3_replace_iii() {
        let results = apply_rules("MIII");
        let r3: Vec<_> = results
            .iter()
            .filter(|(_, r)| matches!(r, Rule::ReplaceIII(_)))
            .collect();
        assert_eq!(r3.len(), 1);
        assert_eq!(r3[0].0, "MU");
    }

    #[test]
    fn rule3_multiple_positions() {
        // MIIII has III at positions 1 and 2
        let results = apply_rules("MIIII");
        let r3: Vec<_> = results
            .iter()
            .filter(|(_, r)| matches!(r, Rule::ReplaceIII(_)))
            .collect();
        assert_eq!(r3.len(), 2);
        assert_eq!(r3[0].0, "MUI");  // replace at pos 1
        assert_eq!(r3[1].0, "MIU");  // replace at pos 2
    }

    #[test]
    fn rule4_drop_uu() {
        let results = apply_rules("MUUI");
        let r4: Vec<_> = results
            .iter()
            .filter(|(_, r)| matches!(r, Rule::DropUU(_)))
            .collect();
        assert_eq!(r4.len(), 1);
        assert_eq!(r4[0].0, "MI");
    }

    #[test]
    fn mod3_invariant_holds() {
        // BFS from MI: every reachable string should have I-count mod 3 != 0
        let history = bfs_explore("MI", 500);
        for step in &history {
            let ic = count_i(&step.string);
            assert_ne!(
                ic % 3,
                0,
                "Found string '{}' with I-count {} divisible by 3!",
                step.string,
                ic
            );
        }
    }

    #[test]
    fn mu_is_unreachable() {
        let history = bfs_explore("MI", 1000);
        assert!(
            history.iter().all(|s| s.string != "MU"),
            "MU should never be reachable from MI"
        );
    }

    #[test]
    fn rule4_creates_cycle_back_to_mi() {
        // MI -> MII -> MIIII -> MIIIIU -> MIUU -> MI (full cycle via Rule 4)
        let step1 = apply_rules("MI");
        assert!(step1.iter().any(|(s, _)| s == "MII")); // Rule 2

        let step2 = apply_rules("MII");
        assert!(step2.iter().any(|(s, _)| s == "MIIII")); // Rule 2

        let step3 = apply_rules("MIIII");
        assert!(step3.iter().any(|(s, _)| s == "MIIIIU")); // Rule 1

        let step4 = apply_rules("MIIIIU");
        assert!(step4.iter().any(|(s, _)| s == "MIUU")); // Rule 3

        let step5 = apply_rules("MIUU");
        assert!(step5.iter().any(|(s, _)| s == "MI")); // Rule 4 — back to start!
    }

    #[test]
    fn derivation_path_reconstructs() {
        let history = bfs_explore("MI", 50);
        // Find MIU (should be step 1, derived from MI via Rule 1)
        let miu_idx = history.iter().position(|s| s.string == "MIU").unwrap();
        let path = derivation_path(&history, miu_idx);
        assert_eq!(path.len(), 2);
        assert_eq!(path[0].0, "MI");
        assert_eq!(path[1].0, "MIU");
    }
}
