# Day 10: Balance Bots — Function Guide

**Problem**: Simulate a factory of bots that pass numbered microchips to each other.
Each bot holds up to 2 chips, then distributes its lower-valued chip one way and
its higher-valued chip another way, per its programmed rule.

**Part 1**: Which bot compares chips valued 17 and 61? → **157**
**Part 2**: Product of values in output bins 0, 1, and 2? → **1085**

---

## Performance

| Metric | Value |
|--------|-------|
| Combined | 69.0µs |
| Part 1 | 81.4µs |
| Part 2 | 70.7µs |
| Parse | ~20µs (estimated) |

**Combined < Part 1** because `solve()` calls `simulate()` once to get both answers,
while `solve_part1()` parses + simulates independently.

---

## Architecture

```
Input (232 lines)
    │
    ▼
parse_input()           ──→ ParsedData { assignments, rules }
    │                         assignments: Vec<(value, bot_id)>
    │                         rules: HashMap<bot_id, Rule { low, high }>
    │
    ▼
simulate()              ──→ (part1_bot, part2_product)
    │
    ├── Load initial chip assignments into HashMap<bot_id, Vec<value>>
    ├── Loop: find bot with 2 chips
    │     ├── Sort to determine low/high
    │     ├── Check if this bot compares target values (Part 1)
    │     └── Distribute to destinations (Bot or Output)
    └── Multiply output bins 0 × 1 × 2 (Part 2)
```

---

## Data Structures

### `Destination` (enum)
```rust
enum Destination {
    Bot(usize),      // Send chip to another bot
    Output(usize),   // Send chip to an output bin
}
```
Models the two places a chip can go. Copy-able (2 words: discriminant + id).

### `Rule` (struct)
```rust
struct Rule {
    low: Destination,   // Where to send the lower-valued chip
    high: Destination,  // Where to send the higher-valued chip
}
```
Each bot has exactly one rule, parsed from lines like:
`bot 2 gives low to bot 1 and high to bot 0`

### `ParsedData` (struct)
```rust
struct ParsedData {
    assignments: Vec<(usize, usize)>,  // (chip_value, bot_id)
    rules: HashMap<usize, Rule>,       // bot_id → distribution rule
}
```
Separates the two instruction types cleanly.

---

## Function-by-Function

### `parse_input(input: &str) -> ParsedData`
Splits each line into whitespace-separated words, then dispatches:
- `words[0] == "value"` → extract value and bot_id from fixed positions
- `words[0] == "bot"` → extract bot_id, low destination (words[5..6]), high destination (words[10..11])

**Pattern**: Positional word extraction — no regex needed because the grammar
is completely fixed. Word indices are constants:
```
"value X goes to bot Y"
  [0]  [1] [2] [3][4][5]

"bot X gives low to bot|output Y and high to bot|output Z"
 [0] [1] [2] [3] [4]  [5]     [6][7] [8] [9]  [10]   [11]
```

### `parse_destination(kind: &str, id: &str) -> Destination`
Helper that converts `("bot", "42")` → `Destination::Bot(42)`.

### `simulate(data, target_low, target_high) -> (usize, usize)`
The core simulation loop:

1. **Initialize**: Load chip assignments into `HashMap<bot_id, Vec<value>>`
2. **Loop**: Find any bot holding exactly 2 chips
3. **Sort**: `held.sort()` to identify low and high values
4. **Check**: If `(lo, hi) == (target_low, target_high)`, record this bot (Part 1)
5. **Distribute**: Send `lo` and `hi` to their respective destinations
6. **Terminate**: When no bot has 2 chips, the network has settled
7. **Part 2**: Multiply values in output bins 0, 1, 2

**Key design choice**: The simulation is parameterized by `target_low` and `target_high`
so the example test can search for `(2, 5)` while the actual input searches for `(17, 61)`.

---

## Algorithm: Event-Driven Simulation

This is a **dataflow network** — each bot is a node that fires when it accumulates
2 inputs. The simulation is equivalent to topological processing of a DAG:

```
value 61 ──→ bot 209 ──→ ...
value 17 ──→ bot 48  ──→ ...
                           ↘
                         bot 157 (compares 17 and 61!)
                           ↙
                         ...  ──→ output 0, 1, 2
```

**Convergence guarantee**: Each chip moves exactly once per bot firing. With N chips
and M bots, the simulation terminates in at most N×M steps (in practice, much fewer).

**Order independence**: The problem is deterministic regardless of which ready bot
fires first, because each bot's rule depends only on its own held values, not on
global state.

---

## Concepts for Study

### HashMap as a Mailbox
Each bot's `Vec<usize>` acts like a mailbox — chips arrive asynchronously, and the
bot processes when its mailbox has 2 items. This is a common pattern in event-driven
systems (compare: AUTOSAR runnable triggered by received data element).

### Enum for Tagged Destinations
`Destination::Bot(id)` vs `Destination::Output(id)` avoids stringly-typed dispatch.
The compiler ensures every `match` handles both variants — impossible to forget outputs.

### Parse-Once with Shared Simulation
`solve()` calls `simulate()` once, getting both answers from a single run.
`solve_part1()` and `solve_part2()` each parse + simulate independently (for benchmarking).
Combined is faster than the sum of parts because simulation runs once.

---

## Potential Optimizations (Not Applied)

| Optimization | Expected Impact | Why Not |
|-------------|-----------------|---------|
| VecDeque work queue | Avoid O(n) scan for ready bots | 69µs is already fast |
| Pre-sort assignments | Reduce initial HashMap pressure | Negligible with 21 values |
| Fixed-size arrays | Replace HashMap with `[Option<Rule>; 210]` | Marginal; HashMap is fine at this scale |
| Topological sort | Process in dependency order | Same result, more complex code |

At 69µs, this is well under the 100ms threshold. The straightforward HashMap approach
is clear and correct.

---

## Deep Dive: The Hidden Sorting Network

Tracing the full simulation (see `examples/day10_flow.rs` → `days/day10_flow.txt`)
reveals that this bot network is far more elegant than it first appears.

### Every Bot Fires

With 210 bot rules and only 21 input chips, you might expect most bots to be unused.
In fact, **all 210 bots fire exactly once** — zero dead rules. The puzzle author
constructed a fully connected pipeline where every node sits on a path from input
to output.

The math works out: 21 chips enter, 21 output bins collect one chip each. Chips are
conserved — each bot firing consumes 2 and produces 2.

### The Cascade Pattern

Only **one bot** (208) starts with 2 chips. It fires first, sending chips downstream,
which triggers the next bot, then the next — a domino cascade through all 210 bots.

The remaining 18 bots that receive initial values each hold just 1 chip, waiting for
their second chip to arrive from upstream.

### Outputs Are (Almost) All Low Chips

20 out of 21 output deliveries come from the **low** side of a bot's distribution.
The sole exception is the terminal bot (169, step 210), which sends both its chips
to outputs because there are no more bots downstream.

This reveals the network's true structure: a **sorting cascade**. Each bot acts as a
comparator — it peels off the smaller chip to an output bin and passes the larger
chip deeper into the network. This is the same principle behind hardware sorting
networks (compare-and-swap units wired in sequence).

### Chip 37: The Sorting Spine

Chip 37 demonstrates this perfectly — it travels through 11 consecutive bots, always
as the `high` chip, while each bot's `low` chip peels off to an output:

```
bot  97: [ 2, 37] → low(2)  to OUTPUT 13  │ high(37) continues →
bot 152: [ 3, 37] → low(3)  to OUTPUT 20  │ high(37) continues →
bot 186: [ 5, 37] → low(5)  to OUTPUT  1  │ high(37) continues →
bot 104: [ 7, 37] → low(7)  to OUTPUT  2  │ high(37) continues →
bot  43: [11, 37] → low(11) to OUTPUT 12  │ high(37) continues →
bot 194: [13, 37] → low(13) to OUTPUT  9  │ high(37) continues →
bot  74: [17, 37] → low(17) to OUTPUT 16  │ high(37) continues →
bot  57: [19, 37] → low(19) to OUTPUT  4  │ high(37) continues →
bot   5: [23, 37] → low(23) to OUTPUT  7  │ high(37) continues →
bot  60: [29, 37] → low(29) to OUTPUT 10  │ high(37) continues →
bot 165: [31, 37] → low(31) to OUTPUT  0  │ high(37) continues →
bot 159: [37, 41] → low(37) to OUTPUT  8  ← 37 finally becomes the low chip!
```

Chip 37 acts as a "pivot" — every chip smaller than 37 gets sorted into an output
along the way, and 37 itself drops out when it finally meets a larger value (41).

### The Flow Diagram Tool

Run the visualization to see the complete 210-step trace:

```bash
cargo run -p aoc2016 --example day10_flow
```

This generates `days/day10_flow.txt` with:
- All 210 firing steps with chip values and destinations
- The Part 1 answer flagged inline (step 119: bot 157 compares [17, 61])
- Output bin contents and Part 2 calculation
- Individual journeys of chips 17 and 61 (21 and 23 hops respectively)

### AUTOSAR Analogy

This sorting network maps directly to an AUTOSAR concept: a chain of SWCs where each
runnable triggers on receiving 2 data elements, processes (sorts), and sends results
to the next SWC in the chain. The "low to output" path is like writing to an NV block
(final storage), while "high to bot" is an inter-runnable variable triggering the next
computation stage.
