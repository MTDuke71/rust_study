# Physics Simulation Pattern - Separating Rules from Termination

**Created**: 2026-02-14
**Tags**: #pattern #simulation #design #separation-of-concerns #aoc
**Links**: [[aoc2022-day14]], [[mission6-integration-guide]], [[single-responsibility-principle]]

---

## Overview

The **Physics Simulation Pattern** separates the rules of how a system evolves from the conditions that stop the simulation. This creates modular, testable, and reusable simulation code.

**Core Principle**: One function defines "what happens next" (physics rules), another function defines "when to stop" (termination conditions).

---

## The Problem

Simulations often conflate two distinct concerns:

1. **Physics/Rules**: How does the system change at each step?
2. **Termination**: When should the simulation stop?

**Bad Example** (tightly coupled):

```rust
fn simulate_sand() -> usize {
    let mut count = 0;
    let mut pos = source;

    loop {
        // Physics mixed with termination
        let down = (pos.x, pos.y + 1);
        if down.1 > MAX_Y {
            return count; // Termination logic here
        }
        if grid[down] == Air {
            pos = down;
            continue;
        }

        let down_left = (pos.x - 1, pos.y + 1);
        if grid[down_left] == Air {
            pos = down_left;
            continue;
        }

        // ... more mixed logic
        if some_other_condition {
            return count; // Different termination here
        }

        grid[pos] = Sand;
        count += 1;
    }
}
```

**Problems**:
- Can't reuse physics with different termination conditions
- Hard to test physics independently
- Part 2 requires rewriting entire function
- Difficult to understand what changes between variants

---

## The Solution

**Separate into two functions**:

1. **Rules function**: Returns next state or signals "can't move"
2. **Driver function**: Applies rules repeatedly until termination condition

### Pattern Structure

```rust
// Physics: What happens in one step?
fn try_move(state: State) -> Option<State> {
    // Apply physics rules
    // Return Some(new_state) if moved
    // Return None if can't move (at rest)
}

// Simulation: When to stop?
fn simulate_until_condition(initial: State, stop_condition: impl Fn(State) -> bool) -> Result {
    let mut state = initial;

    loop {
        // Check termination FIRST
        if stop_condition(state) {
            return result;
        }

        // Apply physics
        match try_move(state) {
            Some(next) => state = next,
            None => {
                // Come to rest
                record(state);
                state = reset(); // Or break
            }
        }
    }
}
```

**Key aspects**:
- `try_move` has NO termination logic (pure physics)
- Termination happens in driver function
- Same physics, different drivers → different behaviors

---

## Real Example: AoC 2022 Day 14 - Falling Sand

### Problem Description

Sand falls from source (500, 0) following physics rules:
1. Try to move down
2. If blocked, try down-left
3. If blocked, try down-right
4. If all blocked, come to rest

**Part 1**: Stop when sand falls into abyss (below max_y)
**Part 2**: Stop when source is blocked (can't add more sand)

### Implementation

#### Step 1: Pure Physics Function

```rust
/// Try to move sand one step. Returns new position if moved, None if at rest.
/// Order: down, down-left, down-right
fn try_move_sand(grid: &Grid<Tile>, pos: Coord) -> Option<Coord> {
    // Try down
    let down = Coord::new(pos.x, pos.y + 1);
    if down.y < grid.height() && grid[down] == Tile::Air {
        return Some(down);
    }
    if down.y >= grid.height() {
        return Some(down); // Falls into abyss (out of bounds)
    }

    // Try down-left (with underflow protection)
    if pos.x > 0 {
        let down_left = Coord::new(pos.x - 1, pos.y + 1);
        if down_left.y < grid.height() && grid[down_left] == Tile::Air {
            return Some(down_left);
        }
    }

    // Try down-right
    let down_right = Coord::new(pos.x + 1, pos.y + 1);
    if down_right.x < grid.width() && down_right.y < grid.height()
       && grid[down_right] == Tile::Air {
        return Some(down_right);
    }
    if down_right.x >= grid.width() {
        return Some(down_right); // Falls off edge
    }

    None // Sand comes to rest (all moves blocked)
}
```

**Notice**:
- No termination logic (no `return false` or `break`)
- Only knows: "here's what happens next"
- Returns `Some(next_pos)` or `None` (at rest)
- Out-of-bounds positions are valid return values (caller decides meaning)

#### Step 2: Part 1 Driver (Abyss Termination)

```rust
/// Simulate one unit of sand falling. Returns true if it came to rest, false if into abyss.
fn drop_sand(grid: &mut Grid<Tile>, source: Coord, max_y: usize) -> bool {
    let mut pos = source;

    loop {
        match try_move_sand(grid, pos) {
            Some(next) => {
                // Check if fallen into abyss (termination condition)
                if next.y > max_y {
                    return false; // Stop simulation
                }
                pos = next;
            }
            None => {
                // Sand comes to rest
                grid[pos] = Tile::Sand;
                return true; // Continue simulation with next grain
            }
        }
    }
}

pub fn part1(data: &ParsedData) -> usize {
    let mut grid = data.grid.clone();
    let mut count = 0;

    while drop_sand(&mut grid, data.source, data.max_y) {
        count += 1;
    }

    count
}
```

**Termination logic**:
- If `next.y > max_y` → stop simulation (sand into abyss)
- Otherwise continue until first grain falls

#### Step 3: Part 2 Driver (Source Blocked Termination)

```rust
/// Simulate sand with infinite floor at y = floor_y
/// Returns true if sand came to rest, false if source is blocked
fn drop_sand_with_floor(grid: &mut Grid<Tile>, source: Coord, floor_y: usize) -> bool {
    // Check if source is already blocked (termination condition)
    if grid[source] == Tile::Sand {
        return false; // Stop simulation
    }

    let mut pos = source;

    loop {
        // Check if we hit the floor (different termination)
        if pos.y + 1 == floor_y {
            // Come to rest on floor
            grid[pos] = Tile::Sand;
            return true;
        }

        match try_move_sand(grid, pos) {
            Some(next) => {
                // Additional check for grid width (could expand, but we assume sufficient)
                if next.x >= grid.width() {
                    grid[pos] = Tile::Sand;
                    return true;
                }
                pos = next;
            }
            None => {
                // Sand comes to rest
                grid[pos] = Tile::Sand;
                return true;
            }
        }
    }
}

pub fn part2(data: &ParsedData) -> usize {
    let floor_y = data.max_y + 2;
    // ... expand grid for pyramid spread ...

    let mut count = 0;
    while drop_sand_with_floor(&mut grid, new_source, floor_y) {
        count += 1;
    }

    count
}
```

**Different termination logic**:
- Check source blocked before dropping
- Floor at `floor_y` instead of abyss check
- Same physics (`try_move_sand`), different driver

---

## Benefits of Separation

### 1. Code Reuse

**Same physics function** used by both Part 1 and Part 2.

```rust
// Part 1 uses it
match try_move_sand(grid, pos) { /* ... */ }

// Part 2 uses it
match try_move_sand(grid, pos) { /* ... */ }
```

No duplication of physics logic.

### 2. Independent Testing

**Test physics without termination**:

```rust
#[test]
fn test_sand_falls_straight_down() {
    let mut grid = Grid::new(3, 5, Tile::Air);
    let pos = Coord::new(1, 0);

    // Test one step
    let next = try_move_sand(&grid, pos);
    assert_eq!(next, Some(Coord::new(1, 1)));

    // Test blocked below
    grid[Coord::new(1, 2)] = Tile::Rock;
    let pos = Coord::new(1, 1);
    let next = try_move_sand(&grid, pos);
    // Should try diagonal
    assert_eq!(next, Some(Coord::new(0, 2)));
}
```

**Test termination without physics**:

```rust
#[test]
fn test_abyss_termination() {
    let mut grid = create_empty_grid();
    let result = drop_sand(&mut grid, source, max_y);
    assert_eq!(result, false); // Falls into abyss
}

#[test]
fn test_source_blocked_termination() {
    let mut grid = create_filled_grid();
    grid[source] = Tile::Sand;
    let result = drop_sand_with_floor(&mut grid, source, floor_y);
    assert_eq!(result, false); // Source blocked
}
```

### 3. Easier Reasoning

**Reading `try_move_sand`**:
- Focus on: "What are the movement rules?"
- Don't worry about: "When does simulation end?"

**Reading `drop_sand`**:
- Focus on: "What terminates Part 1?"
- Don't worry about: "How does sand move?"

Each function has single responsibility.

### 4. Flexible Variants

Easy to add new termination conditions without touching physics:

```rust
// Part 3 hypothetical: Stop after N grains
fn drop_sand_limited(
    grid: &mut Grid<Tile>,
    source: Coord,
    max_y: usize,
    limit: usize
) -> usize {
    let mut count = 0;

    while count < limit {  // New termination condition
        if !drop_sand(grid, source, max_y) {
            break;
        }
        count += 1;
    }

    count
}
```

Same physics, new termination. No changes to `try_move_sand`.

### 5. Performance Optimization

Can optimize physics separately from termination:

```rust
// Optimized physics (memoization, caching, etc.)
fn try_move_sand_optimized(
    grid: &Grid<Tile>,
    pos: Coord,
    cache: &mut HashSet<Coord>
) -> Option<Coord> {
    // Same logic, with caching
}
```

Drivers don't need to change.

---

## General Pattern Template

### Template Code

```rust
// ============================================================================
// Physics/Rules Layer
// ============================================================================

/// Represents the state of the simulation at one point in time
#[derive(Debug, Clone, Copy)]
struct State {
    position: Coord,
    // ... other state fields
}

/// Applies one step of physics rules.
/// Returns Some(next_state) if system evolved, None if at equilibrium.
fn step(current: State, environment: &Environment) -> Option<State> {
    // Apply physics rules
    // Return next state or None if can't evolve
}

// ============================================================================
// Simulation Driver Layer
// ============================================================================

/// Runs simulation until termination condition is met.
/// Returns final result.
fn simulate<F>(
    initial: State,
    environment: &mut Environment,
    should_stop: F,
) -> SimulationResult
where
    F: Fn(&State, &Environment) -> bool,
{
    let mut state = initial;

    loop {
        // Check termination first
        if should_stop(&state, environment) {
            return build_result(&state, environment);
        }

        // Apply physics step
        match step(state, environment) {
            Some(next) => state = next,
            None => {
                // Reached equilibrium
                handle_equilibrium(&state, environment);
                state = next_initial(); // Or break
            }
        }
    }
}

// ============================================================================
// Specific Simulations
// ============================================================================

fn part1(initial: State, env: Environment) -> Result {
    simulate(initial, env, |state, env| {
        // Part 1 termination condition
        state.position.y > env.max_y
    })
}

fn part2(initial: State, env: Environment) -> Result {
    simulate(initial, env, |state, env| {
        // Part 2 termination condition
        env.is_blocked(state.position)
    })
}
```

### Key Elements

1. **State struct**: Captures current simulation state
2. **step function**: Pure physics, no termination
3. **simulate function**: Generic driver with termination callback
4. **Specific drivers**: part1, part2 with different termination

---

## When to Use This Pattern

### ✅ Use When:

1. **Multiple Variants with Same Physics**
   - Part 1 vs Part 2 in AoC
   - Different game modes
   - Varying boundary conditions

2. **Complex Physics Rules**
   - Many steps in evolution
   - Non-trivial state transitions
   - Want to test physics in isolation

3. **Simulation-Based Problems**
   - Particle systems
   - Cellular automata
   - Physics engines
   - Game of Life variants

4. **Reusable Rules**
   - Same physics applies to different scenarios
   - Want to compose different termination conditions
   - Building simulation library

### ❌ Don't Use When:

1. **Single Use Case**
   - Only one termination condition ever needed
   - No plans for variants or reuse

2. **Simple Logic**
   - Physics and termination are both trivial (< 5 lines total)
   - Separation adds more overhead than clarity

3. **Tightly Coupled**
   - Physics and termination are fundamentally intertwined
   - Can't meaningfully separate concerns

---

## Variations

### Variation 1: Return Result Instead of Option

```rust
enum StepResult {
    Moved(State),
    AtRest,
    OutOfBounds,
}

fn step(state: State) -> StepResult {
    // Return explicit result categories
}
```

**When**: Need to distinguish multiple termination reasons.

### Variation 2: Mutable State

```rust
fn step(state: &mut State, environment: &Environment) {
    // Mutate state in place
}
```

**When**: State is large and cloning is expensive.

### Variation 3: Event Callback

```rust
fn simulate<F, G>(
    initial: State,
    should_stop: F,
    on_rest: G,
) -> Result
where
    F: Fn(&State) -> bool,
    G: Fn(&State) -> (),
{
    loop {
        if should_stop(&state) { return result; }

        match step(state) {
            Some(next) => state = next,
            None => {
                on_rest(&state); // Callback for side effects
                state = reset();
            }
        }
    }
}
```

**When**: Need to handle events during simulation (logging, visualization, state changes).

### Variation 4: Iterator-Based

```rust
struct SimulationIterator {
    state: State,
    environment: Environment,
}

impl Iterator for SimulationIterator {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        step(self.state, &self.environment)
            .map(|next| {
                self.state = next;
                self.state
            })
    }
}

// Usage
let simulation = SimulationIterator { state: initial, environment };
let final_count = simulation
    .take_while(|state| state.y <= max_y)
    .count();
```

**When**: Want to use iterator combinators for termination and aggregation.

---

## Related Patterns

### Single Responsibility Principle (SRP)

Physics simulation pattern is application of SRP:
- Physics function: responsible for "how system evolves"
- Driver function: responsible for "when to stop"

### Strategy Pattern

Driver functions are strategies for termination:

```rust
trait TerminationStrategy {
    fn should_stop(&self, state: &State) -> bool;
}

struct AbyssTermination { max_y: usize }
struct SourceBlockedTermination { source: Coord }

impl TerminationStrategy for AbyssTermination {
    fn should_stop(&self, state: &State) -> bool {
        state.position.y > self.max_y
    }
}
```

### Command Pattern

Physics steps can be commands:

```rust
trait PhysicsCommand {
    fn apply(&self, state: State) -> Option<State>;
}
```

---

## Common Mistakes

### Mistake 1: Termination Inside Physics

```rust
// ❌ BAD - termination mixed with physics
fn try_move_sand(grid: &Grid<Tile>, pos: Coord, max_y: usize) -> Option<Coord> {
    let down = Coord::new(pos.x, pos.y + 1);
    if down.y > max_y {
        return None; // WRONG - this is termination logic!
    }
    // ... physics ...
}
```

**Fix**: Remove termination from physics, handle in driver.

### Mistake 2: Duplicating Physics

```rust
// ❌ BAD - duplicated physics logic
fn drop_sand_part1() {
    // ... sand movement logic ...
}

fn drop_sand_part2() {
    // ... SAME sand movement logic ...
}
```

**Fix**: Extract physics to shared `try_move_sand`.

### Mistake 3: Side Effects in Physics

```rust
// ❌ BAD - physics modifies grid
fn try_move_sand(grid: &mut Grid<Tile>, pos: Coord) -> Option<Coord> {
    let next = /* calculate next position */;
    grid[pos] = Tile::Air;    // Side effect!
    grid[next] = Tile::Sand;  // Side effect!
    Some(next)
}
```

**Fix**: Keep physics pure (read-only). Driver handles state changes.

### Mistake 4: Over-Engineering Simple Cases

```rust
// ❌ OVERKILL for simple one-time simulation
fn step(state: State) -> Option<State> { /* 2 lines */ }
fn simulate<F>(initial: State, stop: F) -> Result { /* 10 lines */ }
fn part1() -> Result { /* 2 lines */ }

// ✅ BETTER for simple case - just write it directly
fn part1() -> Result {
    let mut state = initial;
    while !should_stop(&state) {
        state = evolve(state);
    }
    result
}
```

**Only separate when**: Multiple variants OR complex physics OR need testing isolation.

---

## Testing Strategy

### Test Physics in Isolation

```rust
#[test]
fn test_move_down_when_clear() {
    let grid = Grid::new(3, 3, Tile::Air);
    let pos = Coord::new(1, 0);
    let next = try_move_sand(&grid, pos);
    assert_eq!(next, Some(Coord::new(1, 1)));
}

#[test]
fn test_move_diagonal_when_blocked() {
    let mut grid = Grid::new(3, 3, Tile::Air);
    grid[Coord::new(1, 1)] = Tile::Rock;
    let pos = Coord::new(1, 0);
    let next = try_move_sand(&grid, pos);
    assert_eq!(next, Some(Coord::new(0, 1))); // Down-left
}

#[test]
fn test_at_rest_when_all_blocked() {
    let mut grid = Grid::new(3, 3, Tile::Rock);
    let pos = Coord::new(1, 0);
    let next = try_move_sand(&grid, pos);
    assert_eq!(next, None); // Can't move
}
```

### Test Termination Separately

```rust
#[test]
fn test_abyss_stops_simulation() {
    let data = parse("/* empty grid */");
    let result = part1(&data);
    assert_eq!(result, 0); // No sand rests, all falls
}

#[test]
fn test_source_blocked_stops_simulation() {
    let data = parse(EXAMPLE);
    let result = part2(&data);
    assert_eq!(result, 93); // Stops when source blocked
}
```

### Test Integration

```rust
#[test]
fn test_actual_input() {
    let input = include_str!("../../inputs/day14.txt");
    let data = parse(input);
    assert_eq!(part1(&data), 763);
    assert_eq!(part2(&data), 23921);
}
```

---

## Performance Considerations

### Optimization Opportunities

**Because physics is separated**:

1. **Memoization**: Cache physics results for repeated states
2. **Vectorization**: Process multiple particles in parallel (if independent)
3. **Early termination**: Check termination before expensive physics
4. **Lazy evaluation**: Only compute physics when needed

**Example - Early Termination**:

```rust
fn simulate(initial: State, should_stop: F) -> Result {
    let mut state = initial;

    loop {
        // Check termination FIRST (cheaper than physics)
        if should_stop(&state) {
            return result;
        }

        // Only compute physics if continuing
        match step(state) {
            Some(next) => state = next,
            None => { /* handle */ }
        }
    }
}
```

### Profiling Points

With separation, you can profile:
- **Time in physics**: Optimize `step` function
- **Time in termination**: Optimize `should_stop` checks
- **Iteration count**: Reduce by changing algorithm

Without separation, profiling is all-or-nothing.

---

## Real-World Applications

### 1. Particle Systems (Game Dev)

```rust
fn update_particle(particle: Particle, forces: &[Force]) -> Option<Particle> {
    // Physics: Apply gravity, wind, etc.
}

fn simulate_particles(
    particles: Vec<Particle>,
    max_time: f32,
) -> Vec<Particle> {
    // Termination: Simulate until max_time
}
```

### 2. Cellular Automata (Conway's Game of Life)

```rust
fn evolve_cell(cell: Cell, neighbors: &[Cell]) -> Cell {
    // Physics: Birth/death rules
}

fn simulate_generations(
    grid: Grid<Cell>,
    num_generations: usize,
) -> Grid<Cell> {
    // Termination: Run N generations
}
```

### 3. Fluid Simulation

```rust
fn advect_particle(particle: FluidParticle, velocity_field: &VelocityField) -> FluidParticle {
    // Physics: Navier-Stokes approximation
}

fn simulate_fluid(
    particles: Vec<FluidParticle>,
    until_stable: bool,
) -> Vec<FluidParticle> {
    // Termination: Until convergence or max iterations
}
```

### 4. Economic Simulations

```rust
fn market_step(state: MarketState, agents: &[Agent]) -> MarketState {
    // Physics: Supply/demand, price adjustments
}

fn simulate_market(
    initial: MarketState,
    equilibrium_threshold: f64,
) -> MarketState {
    // Termination: Until prices stabilize
}
```

---

## Summary

### Key Principles

1. **Separate "what happens" from "when to stop"**
2. **Physics function is pure** (no termination, minimal side effects)
3. **Driver function handles termination** and orchestration
4. **Same physics, different drivers** → different behaviors

### Benefits

- ✅ Code reuse (physics shared across variants)
- ✅ Independent testing (physics and termination separately)
- ✅ Easier reasoning (single responsibility per function)
- ✅ Flexible composition (new terminations without changing physics)
- ✅ Better performance (optimize layers independently)

### When to Use

- Multiple variants with same rules (AoC Part 1 vs Part 2)
- Complex physics worth isolating
- Simulation-based problems
- Want testability and reusability

### Pattern Structure

```rust
// Physics layer (pure, reusable)
fn step(state: State) -> Option<State>

// Driver layer (termination-specific)
fn simulate(initial: State, should_stop: impl Fn(State) -> bool) -> Result
```

---

## Related Notes

- [[single-responsibility-principle]] - SRP applied to simulation
- [[aoc2022-day14]] - Real example of pattern in action
- [[mission6-integration-guide]] - Grid operations (environment for physics)
- [[separation-of-concerns]] - General design principle

---

## References

- AoC 2022 Day 14: `advent_of_code/aoc2022/src/solver/day14.rs`
- Daily Note: [[2026-02-14]] - Pattern discovery and application
- Function Guide: `Problem_Statements/days/day14_function_guide.md`

---

## Key Takeaway

**The best simulation code answers two questions separately:**
1. "What are the rules?" → Physics function
2. "When does it end?" → Driver function

**When these are mixed, you get rigidity. When separated, you get flexibility.**
