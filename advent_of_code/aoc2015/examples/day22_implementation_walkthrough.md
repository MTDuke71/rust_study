# Day 22: Wizard Simulator 20XX - Implementation Walkthrough

## Overview

Day 22 introduces spell-based combat using A* search to find optimal spell sequences. This represents a significant algorithmic shift from Day 21's brute force equipment optimization to informed graph search through a state space of millions of potential game states.

## Core Data Structures

### `EffectTimers` Struct (Lines 35-121)

**Purpose**: Manages the three spell effects (Shield, Poison, Recharge) with their timers.

**Key Methods**:
- `new()` - Clean initialization
- `is_effect_active()` - Checks if effect is running (timer > 0)
- `apply_and_tick()` - **Critical**: Applies effects AND decrements timers in one atomic operation. Returns `(boss_damage, player_mana, armor_bonus)` tuple
- `start_effect()` - Starts effect only if not already active (prevents overlapping effects)

**Design Decision**: Single `apply_and_tick()` method ensures effects and timer decrements happen together, matching the problem's turn sequence.

### `GameState` Struct (Lines 124-175)

**Purpose**: Complete game state node for A* search graph.

**Fields**:
- Player stats (HP, mana)
- Boss stats (HP, damage)
- `effects: EffectTimers` - Active spell effects
- `mana_spent` - Path cost for A* (what we're minimizing)
- `hard_mode` - Part 2 flag

**Key Methods**:
- `new()` - Initial state constructor
- `is_win()` / `is_loss()` - Terminal state checks
- `player_armor()` - Computed armor from effects
- `heuristic()` - **Boss HP remaining** (admissible: never overestimates remaining cost)

**Design Decision**: All fields are public for easy access, but state transitions happen through pure functions to maintain immutability.

### `Spell` Enum (Lines 177-236)

**Purpose**: Type-safe representation of all 5 spells.

**Variants**:
- `MagicMissile` (53 mana, 4 damage)
- `Drain` (73 mana, 2 damage + 2 heal)
- `Shield` (113 mana, 6 turns, +7 armor)
- `Poison` (173 mana, 6 turns, 3 damage/turn)
- `Recharge` (229 mana, 5 turns, +101 mana/turn)

**Key Methods**:
- `cost()` - Mana cost for A* edge weights
- `effect_name()` - Links spells to EffectTimers (None for instant spells)
- `can_cast()` - **Business logic**: Checks mana AND effect availability
- `all_spells()` - Iterator over all possible actions

**Design Decision**: Enum prevents invalid spell casting, `can_cast()` encapsulates all casting constraints.

## Core Simulation Functions

### `cast_spell()` (Lines 239-268)

**Purpose**: Pure function that applies spell effects to create new state.

**Flow**:
1. Validate casting with `spell.can_cast(state)`
2. Clone state and deduct mana
3. Apply spell-specific effects:
   - Instant spells: Direct damage/healing
   - Effect spells: Start timers via `effects.start_effect()`

**Design Decision**: Returns `Option<GameState>` - None if casting invalid. Pure function enables easy testing.

### `simulate_turn()` (Lines 271-315)

**Purpose**: **Complete turn simulation** - the heart of the game logic.

**Turn Sequence** (matches problem exactly):
1. **Hard mode HP loss** (Part 2 only)
2. **Effects apply** (player turn start)
3. **Spell cast**
4. **Effects apply** (boss turn start)
5. **Boss attacks** (with armor reduction)

**Key Details**:
- Effects applied at start of BOTH turns
- Win/loss checks after each phase
- Armor caps damage at minimum 1
- Returns `Option<GameState>` - None if player dies

**Design Decision**: Single function encapsulates entire turn, making A* state transitions clean.

## A* Search Implementation

### `BossHpHeuristic` (Lines 331-349)

**Purpose**: Admissible heuristic for A* search.

**Implementation**: Returns 0 in `estimate_distance()` because we compute heuristic from game state, not node IDs.

**Why Admissible**: Boss HP remaining never overestimates steps needed (could take 1 damage/turn but might need effects).

### `WizardPathfinder` (Lines 351-435)

**Purpose**: A* search specialized for wizard combat.

**Key Method: `find_minimum_mana_path_with_sequence()`**:

**State Management**:
- `state_to_id` / `id_to_state`: Maps between GameState and NodeId
- Avoids duplicate states (state space explosion prevention)

**A* Components**:
- `open_set`: Priority queue with `(f_score, node_id)`
- `g_scores`: Cost from start to node (mana spent)
- `came_from`: Path reconstruction (though we only need final cost)
- `spell_used`: Tracks which spell led to each state

**Search Loop**:
1. Pop lowest f-score state
2. If win: return mana spent and spell sequence
3. If loss: skip
4. Generate neighbors: try all spells via `simulate_turn()`
5. Update if better path found

**Design Decision**: Uses standard A* but specialized for game state graph. Mission9 provides efficient priority queue.

## Entry Points

### `solve_part1/2()` (Lines 438-461)

**Purpose**: AoC integration points.

**Flow**:
1. Parse boss stats
2. Create initial state (hard_mode flag)
3. Run A* search
4. Return mana cost as string

## Test Suite (Lines 464-589)

**8 comprehensive tests** covering:
- Input parsing (`test_parse_boss`)
- Effect management (`test_effect_timers`)
- Spell casting (`test_spell_casting`)
- State transitions (`test_game_state`)
- Full turn simulation (`test_turn_simulation`)
- A* search integration (`test_wizard_pathfinder`)
- Edge cases (`test_simple_battle`, `test_hard_mode`)

**Design Decision**: Tests validate each layer independently, ensuring robust implementation.

## Key Architectural Insights

1. **State Space Modeling**: Game state as graph nodes, spell casts as edges
2. **Pure Functions**: State transitions are side-effect free
3. **Effect System**: Clean separation of instant vs. timer-based effects
4. **A* Integration**: Mission9 provides battle-tested pathfinding
5. **Comprehensive Testing**: Each component validated independently

## Algorithmic Evolution from Day 21

| Aspect | Day 21 (RPG) | Day 22 (Wizard) |
|--------|--------------|-----------------|
| **Search Method** | Brute force enumeration | A* informed search |
| **State Space** | ~8,000 equipment combos | Millions of game states |
| **Cost Function** | Equipment cost | Mana spent |
| **State Representation** | Equipment vectors | Game state with effects |
| **Optimization Goal** | Min cost to win | Min mana to win |
| **Constraints** | Equipment limits | Spell availability rules |

## Performance Characteristics

- **State Space**: Millions of states from effect combinations and HP values
- **Search Algorithm**: A* with admissible heuristic (boss HP remaining)
- **Actual Runtime**: ~1-2 seconds for both parts
- **Mission9 Benefits**: Efficient priority queue, reusable pathfinding infrastructure

## Victory Path Analysis

### Part 1 (Normal Mode) - 900 Mana
**Spell Sequence**: Poison → Magic Missile → Recharge → Poison → Magic Missile → Shield → Magic Missile → Magic Missile

**Strategy Breakdown**:
1. **Early Pressure**: Poison for sustained damage
2. **Mana Management**: Recharge when mana gets low
3. **Final Protection**: Shield before last attacks
4. **Burst Damage**: Dual Magic Missiles to finish

### Part 2 (Hard Mode) - 1216 Mana
**Spell Sequence**: Poison → Drain → Recharge → Poison → Shield → Recharge → Poison → Magic Missile

**Key Adjustments**:
1. **Drain Usage**: Provides both damage and healing
2. **More Mana Management**: Additional Recharge due to HP pressure
3. **Conservative Healing**: Drain's healing helps offset HP loss
4. **Extended Timeline**: More turns needed due to HP drain

## Mission9 Integration

**Note**: Mission 9 was included as a requirement during problem definition but proved unnecessary for the final implementation. The solution uses only basic types (`NodeId`, `Weight`) and trait definitions from Mission 9, implementing its own complete A* search loop due to the problem's unique characteristics.

The solution leverages Mission9's foundational pathfinding infrastructure:

```rust
use mission9::astar::AstarPathfinder;
use mission9::{NodeId, Weight};
```

**Minimal Usage**:
- `NodeId` and `Weight` type aliases
- `Heuristic` trait for admissibility guarantees
- Core A* search algorithm implemented from scratch

**Why Custom Implementation**: Day 22's dynamic state generation, complex turn-based simulation, and spell sequence tracking required full control over the search process, making Mission 9's high-level interface unsuitable despite providing excellent low-level components.

## Conclusion

Day 22 demonstrates the power of informed search algorithms when brute force becomes impractical. The A* implementation elegantly solves a complex optimization problem by modeling game combat as a graph search, finding optimal spell sequences through the vast state space of possible wizard battles.

---

## Related Resources

- [[../../zettelkasten/A-Star-Algorithm-Deep-Dive]] - A* algorithm theory and implementation
- [[../../missions/Mission9/README]] - Advanced pathfinding algorithms (Dijkstra, A*)
- [[../Problem_Statements/day22]] - Original problem statement
- [[../Problem_Statements/HIGHLIGHTS_SUMMARY]] - AoC 2015 patterns overview
- [[../../zettelkasten/Missions Overview]] - Data structure implementations used in solutions

*Tags: #aoc #aoc2015 #day22 #astar #informed-search #state-space-search #game-ai #optimization*
