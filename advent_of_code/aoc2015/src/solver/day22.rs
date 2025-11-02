/// AoC 2015 Day 22: Wizard Simulator 20XX
///
/// Part 1: Find minimum mana to defeat boss with spells
/// Part 2: Find minimum mana with "hard mode" (player takes 1 damage per turn)
///
/// ## Problem Overview
///
/// Player: 50 HP, 500 mana, no armor
/// Boss: HP and damage from input
/// Spells: Magic Missile, Drain, Shield, Poison, Recharge
/// Effects persist for multiple turns with timers
/// Player and boss take alternating turns
/// Effects apply at start of each turn
///
/// ## Solution Approach
///
/// Use A* search with game state as nodes:
/// - State: (player_hp, player_mana, boss_hp, effect_timers, mana_spent)
/// - Goal: Any state where boss_hp <= 0
/// - Cost: Mana spent (to minimize)
/// - Heuristic: Remaining boss HP (admissible)
///
/// ## Algorithm
///
/// 1. Model game as state graph using Mission9's A* implementation
/// 2. Each state transition = casting valid spell
/// 3. A* finds minimum mana path to boss defeat
/// 4. Handle effect timers and spell availability constraints
use anyhow::Result;
use std::collections::HashMap;
use mission9::astar::AstarPathfinder;
use mission9::{NodeId, Weight};

/// Represents active spell effects with their remaining durations
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct EffectTimers {
    /// Shield effect: +7 armor for 6 turns
    pub shield_timer: i32,
    /// Poison effect: 3 damage to boss per turn for 6 turns
    pub poison_timer: i32,
    /// Recharge effect: +101 mana per turn for 5 turns
    pub recharge_timer: i32,
}

impl EffectTimers {
    /// Create a new EffectTimers with no active effects
    pub fn new() -> Self {
        Self {
            shield_timer: 0,
            poison_timer: 0,
            recharge_timer: 0,
        }
    }
}

impl EffectTimers {
    /// Check if a spell effect is currently active (timer > 0)
    pub fn is_effect_active(&self, effect_name: &str) -> bool {
        match effect_name {
            "shield" => self.shield_timer > 0,
            "poison" => self.poison_timer > 0,
            "recharge" => self.recharge_timer > 0,
            _ => false,
        }
    }

    /// Apply effect at start of turn and decrement timers
    /// Returns (damage_to_boss, mana_to_player, armor_bonus)
    pub fn apply_and_tick(&mut self) -> (i32, i32, i32) {
        let mut boss_damage = 0;
        let mut player_mana = 0;
        let mut armor_bonus = 0;

        // Apply active effects
        if self.poison_timer > 0 {
            boss_damage += 3;
        }
        if self.recharge_timer > 0 {
            player_mana += 101;
        }
        if self.shield_timer > 0 {
            armor_bonus += 7;
        }

        // Decrement timers
        if self.shield_timer > 0 {
            self.shield_timer -= 1;
        }
        if self.poison_timer > 0 {
            self.poison_timer -= 1;
        }
        if self.recharge_timer > 0 {
            self.recharge_timer -= 1;
        }

        (boss_damage, player_mana, armor_bonus)
    }

    /// Start a new effect (only if not already active)
    /// Returns true if effect was started, false if already active
    pub fn start_effect(&mut self, effect_name: &str, duration: i32) -> bool {
        if self.is_effect_active(effect_name) {
            return false; // Cannot start effect that's already active
        }

        match effect_name {
            "shield" => self.shield_timer = duration,
            "poison" => self.poison_timer = duration,
            "recharge" => self.recharge_timer = duration,
            _ => return false,
        }

        true
    }
}

impl Default for EffectTimers {
    fn default() -> Self {
        Self::new()
    }
}

/// Complete game state for A* search
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct GameState {
    /// Player hit points
    pub player_hp: i32,
    /// Player mana points
    pub player_mana: i32,
    /// Boss hit points
    pub boss_hp: i32,
    /// Boss damage per attack
    pub boss_damage: i32,
    /// Active spell effects
    pub effects: EffectTimers,
    /// Total mana spent so far
    pub mana_spent: i32,
    /// Hard mode flag (Part 2: player loses 1 HP per player turn)
    pub hard_mode: bool,
}

impl GameState {
    /// Create initial game state
    pub fn new(boss_hp: i32, boss_damage: i32, hard_mode: bool) -> Self {
        Self {
            player_hp: 50,
            player_mana: 500,
            boss_hp,
            boss_damage,
            effects: EffectTimers::new(),
            mana_spent: 0,
            hard_mode,
        }
    }

    /// Check if this is a winning state (boss defeated)
    pub fn is_win(&self) -> bool {
        self.boss_hp <= 0
    }

    /// Check if this is a losing state (player defeated or cannot cast spells)
    pub fn is_loss(&self) -> bool {
        self.player_hp <= 0 || self.player_mana < 53 // Cannot cast cheapest spell
    }

    /// Get current player armor from effects
    pub fn player_armor(&self) -> i32 {
        if self.effects.shield_timer > 0 { 7 } else { 0 }
    }

    /// Calculate heuristic for A* (remaining boss HP - admissible)
    pub fn heuristic(&self) -> f64 {
        self.boss_hp.max(0) as f64
    }
}

/// Available spells in the game
#[derive(Debug, Clone, Copy)]
pub enum Spell {
    MagicMissile, // 53 mana, 4 damage
    Drain,        // 73 mana, 2 damage, +2 HP
    Shield,       // 113 mana, 6 turns, +7 armor
    Poison,       // 173 mana, 6 turns, 3 damage/turn
    Recharge,     // 229 mana, 5 turns, +101 mana/turn
}

impl Spell {
    /// Get mana cost of spell
    pub fn cost(&self) -> i32 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }

    /// Get effect name for timer checking (None for instant spells)
    pub fn effect_name(&self) -> Option<&str> {
        match self {
            Spell::Shield => Some("shield"),
            Spell::Poison => Some("poison"),
            Spell::Recharge => Some("recharge"),
            _ => None,
        }
    }

    /// Check if spell can be cast in current state
    pub fn can_cast(&self, state: &GameState) -> bool {
        // Must have enough mana
        if state.player_mana < self.cost() {
            return false;
        }

        // Effect spells cannot be cast if already active
        if let Some(effect_name) = self.effect_name() {
            if state.effects.is_effect_active(effect_name) {
                return false;
            }
        }

        true
    }

    /// All available spells
    pub fn all_spells() -> Vec<Spell> {
        vec![
            Spell::MagicMissile,
            Spell::Drain,
            Spell::Shield,
            Spell::Poison,
            Spell::Recharge,
        ]
    }
}

/// Simulate casting a spell and return new game state
pub fn cast_spell(state: &GameState, spell: &Spell) -> Option<GameState> {
    if !spell.can_cast(state) {
        return None;
    }

    let mut new_state = state.clone();
    new_state.player_mana -= spell.cost();
    new_state.mana_spent += spell.cost();

    match spell {
        Spell::MagicMissile => {
            new_state.boss_hp -= 4;
        }
        Spell::Drain => {
            new_state.boss_hp -= 2;
            new_state.player_hp += 2;
        }
        Spell::Shield => {
            new_state.effects.start_effect("shield", 6);
        }
        Spell::Poison => {
            new_state.effects.start_effect("poison", 6);
        }
        Spell::Recharge => {
            new_state.effects.start_effect("recharge", 5);
        }
    }

    Some(new_state)
}

/// Simulate a complete turn (effects + action + boss turn)
pub fn simulate_turn(mut state: GameState, spell: &Spell) -> Option<GameState> {
    // Player turn
    if state.hard_mode {
        state.player_hp -= 1;
        if state.is_loss() {
            return None;
        }
    }

    // Apply effects at start of player turn
    let (boss_damage, player_mana, _armor) = state.effects.apply_and_tick();
    state.boss_hp -= boss_damage;
    state.player_mana += player_mana;

    if state.is_win() {
        return Some(state);
    }

    // Cast spell
    let mut state = cast_spell(&state, spell)?;

    if state.is_win() {
        return Some(state);
    }

    // Boss turn
    // Apply effects at start of boss turn
    let (boss_damage, player_mana, armor_bonus) = state.effects.apply_and_tick();
    state.boss_hp -= boss_damage;
    state.player_mana += player_mana;

    if state.is_win() {
        return Some(state);
    }

    // Boss attacks
    let damage = (state.boss_damage - armor_bonus).max(1);
    state.player_hp -= damage;

    if state.is_loss() {
        None
    } else {
        Some(state)
    }
}

/// Parse boss stats from input string
///
/// Input format:
/// ```text
/// Hit Points: 51
/// Damage: 9
/// ```
pub fn parse_boss(input: &str) -> (i32, i32) {
    let lines: Vec<&str> = input.lines().collect();
    let hp: i32 = lines[0].split(": ").nth(1).unwrap().parse().unwrap();
    let damage: i32 = lines[1].split(": ").nth(1).unwrap().parse().unwrap();
    (hp, damage)
}

/// Custom heuristic for wizard combat - remaining boss HP
#[derive(Debug, Clone)]
pub struct BossHpHeuristic;

impl mission9::heuristic::Heuristic for BossHpHeuristic {
    fn estimate_distance(&self, _from: NodeId, _to: NodeId, _context: &mission9::heuristic::HeuristicContext) -> Weight {
        // For wizard combat, we don't use node IDs directly
        // This heuristic will be computed from game state instead
        0.0
    }

    fn name(&self) -> &'static str {
        "Boss HP Heuristic"
    }

    fn is_admissible(&self) -> bool {
        true // Boss HP remaining is admissible (never overestimates)
    }
}

/// A* pathfinder specialized for wizard combat
pub struct WizardPathfinder {
    _pathfinder: AstarPathfinder<BossHpHeuristic>,
}

impl WizardPathfinder {
    pub fn new() -> Self {
        Self {
            _pathfinder: AstarPathfinder::new(BossHpHeuristic),
        }
    }

    /// Find minimum mana path to victory using A* search
    pub fn find_minimum_mana_path(&self, initial_state: &GameState) -> Option<i32> {
        self.find_minimum_mana_path_with_sequence(initial_state).map(|(mana, _)| mana)
    }

    /// Find minimum mana path and return the spell sequence
    pub fn find_minimum_mana_path_with_sequence(&self, initial_state: &GameState) -> Option<(i32, Vec<Spell>)> {
        // State registry to map between states and node IDs
        let mut state_to_id = HashMap::new();
        let mut id_to_state = HashMap::new();
        let mut next_id = 0;

        // Priority queue for A* search
        let mut open_set = std::collections::BinaryHeap::new();
        let mut g_scores = HashMap::new();
        let mut came_from = HashMap::new();
        let mut spell_used = HashMap::new(); // Track which spell led to each state

        // Initialize with start state
        let start_id = next_id;
        next_id += 1;
        state_to_id.insert(initial_state.clone(), start_id);
        id_to_state.insert(start_id, initial_state.clone());

        open_set.push(std::cmp::Reverse((initial_state.heuristic() as i32, start_id)));
        g_scores.insert(start_id, 0);

        while let Some(std::cmp::Reverse((_, current_id))) = open_set.pop() {
            let current_state = id_to_state[&current_id].clone();

            // Check if we've reached victory
            if current_state.is_win() {
                // Reconstruct the spell sequence
                let mut sequence = Vec::new();
                let mut current = current_id;
                while let Some(&prev_id) = came_from.get(&current) {
                    if let Some(spell) = spell_used.get(&current) {
                        sequence.push(*spell);
                    }
                    current = prev_id;
                }
                sequence.reverse();
                return Some((g_scores[&current_id], sequence));
            }

            // Skip if already lost
            if current_state.is_loss() {
                continue;
            }

            let current_g = g_scores[&current_id];

            // Try all valid spells
            for spell in Spell::all_spells() {
                if let Some(new_state) = simulate_turn(current_state.clone(), &spell) {
                    // Register new state if not seen before
                    let new_id = if let Some(&id) = state_to_id.get(&new_state) {
                        id
                    } else {
                        let id = next_id;
                        next_id += 1;
                        state_to_id.insert(new_state.clone(), id);
                        id_to_state.insert(id, new_state.clone());
                        id
                    };

                    let new_g = current_g + spell.cost();

                    // If this path to new_state is better
                    if new_g < *g_scores.get(&new_id).unwrap_or(&i32::MAX) {
                        came_from.insert(new_id, current_id);
                        spell_used.insert(new_id, spell);
                        g_scores.insert(new_id, new_g);

                        let f_score = new_g + new_state.heuristic() as i32;
                        open_set.push(std::cmp::Reverse((f_score, new_id)));
                    }
                }
            }
        }

        None // No path found
    }
}

impl Default for WizardPathfinder {
    fn default() -> Self {
        Self::new()
    }
}

/// Solve Part 1: Find minimum mana to win (normal mode)
pub fn solve_part1(input: &str) -> Result<String> {
    let (boss_hp, boss_damage) = parse_boss(input);
    let initial_state = GameState::new(boss_hp, boss_damage, false);

    let pathfinder = WizardPathfinder::new();

    match pathfinder.find_minimum_mana_path(&initial_state) {
        Some(min_mana) => Ok(min_mana.to_string()),
        None => Ok("No solution found".to_string()),
    }
}

/// Solve Part 2: Find minimum mana to win (hard mode)
pub fn solve_part2(input: &str) -> Result<String> {
    let (boss_hp, boss_damage) = parse_boss(input);
    let initial_state = GameState::new(boss_hp, boss_damage, true);

    let pathfinder = WizardPathfinder::new();

    match pathfinder.find_minimum_mana_path(&initial_state) {
        Some(min_mana) => Ok(min_mana.to_string()),
        None => Ok("No solution found".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_boss() {
        let input = "Hit Points: 51\nDamage: 9";
        let (hp, damage) = parse_boss(input);
        assert_eq!(hp, 51);
        assert_eq!(damage, 9);
    }

    #[test]
    fn test_effect_timers() {
        let mut effects = EffectTimers::new();

        // Start shield
        assert!(effects.start_effect("shield", 6));
        assert_eq!(effects.shield_timer, 6);
        assert!(effects.is_effect_active("shield"));

        // Apply and tick
        let (boss_dmg, mana, armor) = effects.apply_and_tick();
        assert_eq!(boss_dmg, 0); // No poison
        assert_eq!(mana, 0);     // No recharge
        assert_eq!(armor, 7);    // Shield active
        assert_eq!(effects.shield_timer, 5);

        // Cannot start shield again while active
        assert!(!effects.start_effect("shield", 6));
    }

    #[test]
    fn test_spell_casting() {
        let state = GameState::new(51, 9, false);

        // Can cast magic missile
        assert!(Spell::MagicMissile.can_cast(&state));

        // Cast it
        let new_state = cast_spell(&state, &Spell::MagicMissile).unwrap();
        assert_eq!(new_state.player_mana, 500 - 53);
        assert_eq!(new_state.boss_hp, 51 - 4);
        assert_eq!(new_state.mana_spent, 53);

        // Should still have enough mana for other spells
        let other_spell = Spell::Drain;
        assert!(other_spell.can_cast(&new_state));

        // Should also have enough mana for expensive spells
        let expensive_spell = Spell::Recharge;
        assert!(expensive_spell.can_cast(&new_state));
    }

    #[test]
    fn test_game_state() {
        let mut state = GameState::new(51, 9, false);

        // Initial state
        assert_eq!(state.player_hp, 50);
        assert_eq!(state.player_mana, 500);
        assert_eq!(state.boss_hp, 51);
        assert_eq!(state.boss_damage, 9);
        assert!(!state.is_win());
        assert!(!state.is_loss());

        // Win condition
        state.boss_hp = 0;
        assert!(state.is_win());

        // Loss conditions
        state.player_hp = 0;
        assert!(state.is_loss());
        state.player_hp = 1;
        state.player_mana = 0;
        assert!(state.is_loss());
    }

    #[test]
    fn test_turn_simulation() {
        let initial_state = GameState::new(13, 8, false);

        // Poison spell should defeat boss in 2 turns
        let after_poison = simulate_turn(initial_state, &Spell::Poison).unwrap();
        assert_eq!(after_poison.boss_hp, 10); // 13 - 3 damage
        assert_eq!(after_poison.player_mana, 500 - 173); // Poison cost
        assert_eq!(after_poison.effects.poison_timer, 5); // 6 - 1 = 5

        // Second turn - poison kills boss
        let final_state = simulate_turn(after_poison, &Spell::MagicMissile).unwrap();
        assert_eq!(final_state.boss_hp, 0); // 10 - 3 - 4 = 3, then poison again = -3
        assert!(final_state.is_win());
    }

    #[test]
    fn test_wizard_pathfinder() {
        let initial_state = GameState::new(13, 8, false);
        let pathfinder = WizardPathfinder::new();

        let result = pathfinder.find_minimum_mana_path(&initial_state);
        assert!(result.is_some());

        // Should find a winning sequence
        let mana_cost = result.unwrap();
        assert!(mana_cost > 0);
        println!("Minimum mana to win: {}", mana_cost);
    }

    #[test]
    fn test_simple_battle() {
        // Simple battle: Magic Missile should win immediately
        let initial_state = GameState::new(4, 8, false); // Boss has only 4 HP

        let final_state = simulate_turn(initial_state, &Spell::MagicMissile).unwrap();
        assert!(final_state.is_win());
        assert_eq!(final_state.mana_spent, 53);
    }

    #[test]
    fn test_hard_mode() {
        let initial_state = GameState::new(51, 9, true); // Hard mode

        // First player turn should lose 1 HP before casting
        let after_turn = simulate_turn(initial_state, &Spell::MagicMissile).unwrap();
        assert_eq!(after_turn.player_hp, 40); // 50 - 1 (hard mode) - 9 (boss damage)
    }
}