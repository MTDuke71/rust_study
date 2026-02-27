//! Debug demonstration of Day 11 Part 2 modular arithmetic
//!
//! Shows how worry levels are managed with and without modulo to prevent overflow

use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    fn apply(&self, old: u64) -> u64 {
        match self {
            Operation::Add(n) => old + n,
            Operation::Multiply(n) => old * n,
            Operation::Square => old * old,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: VecDeque<u64>,
    operation: Operation,
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
    inspection_count: u64,
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║   AoC 2022 Day 11 Part 2: Modular Arithmetic Demonstration      ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    // Simplified example: 2 monkeys
    let mut monkeys = vec![
        Monkey {
            id: 0,
            items: VecDeque::from(vec![79, 98]),
            operation: Operation::Multiply(19),
            divisible_by: 23,
            if_true: 1,
            if_false: 1,
            inspection_count: 0,
        },
        Monkey {
            id: 1,
            items: VecDeque::from(vec![54, 65]),
            operation: Operation::Add(6),
            divisible_by: 19,
            if_true: 0,
            if_false: 0,
            inspection_count: 0,
        },
    ];

    // Calculate modulo (product of all divisors)
    let modulo: u64 = monkeys.iter().map(|m| m.divisible_by).product();
    println!("🔢 Modulo calculation:");
    println!("   Divisors: 23, 19");
    println!("   Product: 23 × 19 = {}\n", modulo);

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Simulate 3 rounds with detailed output
    for round in 1..=3 {
        println!("╭─────────────────────────────────────────────────────────────────╮");
        println!(
            "│ ROUND {}                                                         │",
            round
        );
        println!("╰─────────────────────────────────────────────────────────────────╯\n");

        for monkey_idx in 0..monkeys.len() {
            if monkeys[monkey_idx].items.is_empty() {
                println!("  Monkey {}: No items to inspect\n", monkey_idx);
                continue;
            }

            println!("  Monkey {} turn:", monkey_idx);
            println!("  Starting items: {:?}", monkeys[monkey_idx].items);
            println!();

            while let Some(worry) = monkeys[monkey_idx].items.pop_front() {
                monkeys[monkey_idx].inspection_count += 1;

                println!("    Item with worry level: {}", worry);

                // Apply operation
                let new_worry_unbounded = monkeys[monkey_idx].operation.apply(worry);
                println!(
                    "      Operation {:?}: {} → {}",
                    monkeys[monkey_idx].operation, worry, new_worry_unbounded
                );

                // Apply modulo (Part 2 - NO division by 3!)
                let new_worry = new_worry_unbounded % modulo;

                if new_worry != new_worry_unbounded {
                    println!(
                        "      Apply modulo {}: {} → {} (reduced)",
                        modulo, new_worry_unbounded, new_worry
                    );
                } else {
                    println!("      No reduction needed (already < {})", modulo);
                }

                // Test divisibility
                let divisor = monkeys[monkey_idx].divisible_by;
                let divisible = new_worry % divisor == 0;

                println!(
                    "      Test: {} % {} = {} (divisible: {})",
                    new_worry,
                    divisor,
                    new_worry % divisor,
                    divisible
                );

                // Demonstrate that modulo preserves divisibility
                let unbounded_divisible = new_worry_unbounded % divisor == 0;
                if unbounded_divisible == divisible {
                    println!(
                        "      ✓ Divisibility preserved! ({} % {} = {})",
                        new_worry_unbounded,
                        divisor,
                        new_worry_unbounded % divisor
                    );
                }

                let target = if divisible {
                    monkeys[monkey_idx].if_true
                } else {
                    monkeys[monkey_idx].if_false
                };

                println!("      → Throw to monkey {}", target);
                monkeys[target].items.push_back(new_worry);
                println!();
            }

            println!();
        }

        // Show state after round
        println!("  After round {}:", round);
        for monkey in &monkeys {
            println!(
                "    Monkey {}: items = {:?}, inspections = {}",
                monkey.id, monkey.items, monkey.inspection_count
            );
        }
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }

    // Final summary
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║                         FINAL SUMMARY                            ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    for monkey in &monkeys {
        println!(
            "  Monkey {}: {} inspections",
            monkey.id, monkey.inspection_count
        );
    }

    println!("\n📊 Key Observations:");
    println!(
        "   1. Worry levels stay bounded (< {}) due to modulo",
        modulo
    );
    println!("   2. Divisibility tests are preserved after applying modulo");
    println!("   3. Without modulo, values would grow exponentially and overflow");
    println!("\n💡 Mathematical Principle:");
    println!("   For any divisor d in the product M = d₁ × d₂ × ... × dₙ:");
    println!("   (a mod M) mod d = a mod d");
    println!("   Therefore, reducing modulo M preserves all divisibility tests!\n");

    // Demonstrate exponential growth without modulo
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    println!("⚠️  WITHOUT MODULO (exponential growth):\n");

    let mut demo_worry = 79u64;
    for round in 1..=10 {
        let old_worry = demo_worry;
        demo_worry = demo_worry.saturating_mul(19); // Multiply by 19 (Monkey 0's operation)

        if demo_worry == u64::MAX {
            println!(
                "   Round {}: {} → OVERFLOW! (would exceed u64::MAX)",
                round, old_worry
            );
            break;
        } else {
            println!(
                "   Round {}: {} → {} (growth: {:.1}×)",
                round,
                old_worry,
                demo_worry,
                demo_worry as f64 / old_worry as f64
            );
        }
    }

    println!("\n✅ WITH MODULO (bounded growth):\n");

    let mut demo_worry = 79u64;
    for round in 1..=10 {
        let old_worry = demo_worry;
        demo_worry = (demo_worry * 19) % modulo;
        println!(
            "   Round {}: {} → {} (stays < {})",
            round, old_worry, demo_worry, modulo
        );
    }

    println!("\n🎯 Result: Modulo keeps values manageable for 10,000 rounds!\n");
}
