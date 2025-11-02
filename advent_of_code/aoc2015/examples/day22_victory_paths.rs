/// Demonstration of optimal spell sequences for Day 22 Wizard Simulator
/// Shows the complete victory paths for both Part 1 (Normal) and Part 2 (Hard Mode)
use aoc2015::solver::day22::{GameState, Spell, WizardPathfinder, simulate_turn, parse_boss};

/// Pretty-print a spell for display
fn format_spell(spell: &Spell) -> &'static str {
    match spell {
        Spell::MagicMissile => "Magic Missile",
        Spell::Drain => "Drain",
        Spell::Shield => "Shield",
        Spell::Poison => "Poison",
        Spell::Recharge => "Recharge",
    }
}

/// Display current game state in the format from the problem statement
fn display_state(state: &GameState, turn_number: usize, is_player_turn: bool) {
    if is_player_turn {
        println!("-- Player turn {} --", turn_number);
    } else {
        println!("-- Boss turn {} --", turn_number);
    }

    println!("- Player has {} hit points, {} armor, {} mana", state.player_hp, state.player_armor(), state.player_mana);
    println!("- Boss has {} hit points", state.boss_hp);
}

/// Simulate and display a complete victory sequence
fn demonstrate_victory_path(boss_hp: i32, boss_damage: i32, hard_mode: bool, title: &str) {
    println!("{}\n", title);
    println!("{}", "=".repeat(60));

    let initial_state = GameState::new(boss_hp, boss_damage, hard_mode);
    let pathfinder = WizardPathfinder::new();

    match pathfinder.find_minimum_mana_path_with_sequence(&initial_state) {
        Some((total_mana, spell_sequence)) => {
            println!("Optimal spell sequence found! Total mana: {}\n", total_mana);

            let mut current_state = initial_state.clone();
            let mut turn_number = 1;

            for (_i, spell) in spell_sequence.iter().enumerate() {
                // Player turn
                display_state(&current_state, turn_number, true);
                println!("Player casts {}.", format_spell(spell));
                println!();

                // Simulate the full turn (includes boss turn)
                match simulate_turn(current_state.clone(), spell) {
                    Some(new_state) => {
                        current_state = new_state;

                        // Boss turn details are shown in simulate_turn, but we need to show the state after effects
                        if current_state.is_win() {
                            println!("-- Boss turn {} --", turn_number);
                            println!("- Player has {} hit points, {} armor, {} mana", current_state.player_hp, current_state.player_armor(), current_state.player_mana);
                            println!("- Boss has {} hit points", current_state.boss_hp);
                            println!("Boss attacks but player wins!");
                            break;
                        }

                        // Continue to next turn
                        turn_number += 1;
                    }
                    None => {
                        println!("Player died during turn!");
                        break;
                    }
                }
            }

            println!("\nVICTORY! Total mana spent: {}\n", total_mana);
        }
        None => {
            println!("No winning sequence found!\n");
        }
    }
}

fn main() {
    // Use the actual boss from the problem
    let boss_input = "Hit Points: 51\nDamage: 9";
    let (boss_hp, boss_damage) = parse_boss(boss_input);

    demonstrate_victory_path(
        boss_hp,
        boss_damage,
        false,
        "🎉 DAY 22: WIZARD SIMULATOR - PART 1 VICTORY PATH (Normal Mode)"
    );

    demonstrate_victory_path(
        boss_hp,
        boss_damage,
        true,
        "🎯 DAY 22: WIZARD SIMULATOR - PART 2 VICTORY PATH (Hard Mode)"
    );
}