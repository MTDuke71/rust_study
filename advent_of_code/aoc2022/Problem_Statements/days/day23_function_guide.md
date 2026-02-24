# Day 23: Unstable Diffusion - Function Guide

**Problem**: Simulate elf spreading on an infinite grid via proposal/collision phases with rotating direction priority.

**Navigation**: [← Day 22](day22_function_guide.md) | [Problem](day23.md) | [Code](../../src/solver/day23.rs) | [Summary](../summary_2022.md) | [Day 24 →](day24_function_guide.md)

---

## Overview

### Problem Summary
- **Part 1**: Simulate 10 rounds of elf movement. Count empty ground tiles in the smallest bounding rectangle containing all elves.
- **Part 2**: Continue simulating until no elf moves. Report that round number.

### Performance
- **Baseline**: 104.62ms
- **Optimized**: 56.38ms (1.86× speedup)
- **Part 1**: ~0.6ms (first 10 rounds, checkpointed mid-stream)
- **Part 2**: ~55.8ms (remaining 920 rounds to quiescence)

### Key Insight
**The grid is infinite and sparse** — elves occupy ~2,500 of potentially millions of positions. A `FxHashSet<i64>` with packed coordinates naturally represents this without wasting memory on empty space. Each round has two phases: (1) build an 8-neighbor occupancy bitmask, test directions with bitwise AND, (2) move only sole proposers. The simulation is O(R × n) but requires ~930 rounds to converge.

---

## Algorithm Analysis

### Parsing: HashSet from Grid

The input is a rectangular grid of `#` (elf) and `.` (empty). Only elf positions are stored as packed `i64` values:

```rust
fn parse(input: &str) -> FxHashSet<Pos> {
    // For each '#', insert pack(row, col) into the set
}
```

Input size: ~73 lines × ~73 cols → ~2,500 elves out of ~5,300 total cells.

### Packed Coordinate Design

Coordinates are packed into a single `i64` for faster hashing — one hash operation instead of two:

```rust
type Pos = i64;  // high 32 bits = row, low 32 bits = col

fn pack(r: i32, c: i32) -> Pos {
    ((r as i64) << 32) | (c as u32 as i64)
}

fn unpack(p: Pos) -> (i32, i32) {
    ((p >> 32) as i32, p as i32)
}
```

**Important**: Packed values cannot be added directly — column overflow creates a carry that corrupts the row bits (e.g., `col=1 + delta=-1` carries into row). All arithmetic uses `unpack → add → repack`.

### Direction Table + Bitmask Design

Three const arrays encode the movement rules compactly:

```
NEIGHBORS[8]: (dr, dc)     — all 8 adjacency offsets
DIR_DELTAS[4]: (dr, dc)    — movement deltas for N, S, W, E
DIR_MASKS[4]: u8 bitmask   — which NEIGHBORS to check per direction
```

The bitmask approach builds an 8-bit occupancy mask once (8 hash lookups), then tests each direction with a single bitwise AND — eliminating up to 12 additional hash lookups per elf:

```
DIR_MASKS[N] = bits 0,1,7  → (1<<0)|(1<<1)|(1<<7) = 0b10000011
DIR_MASKS[S] = bits 4,3,5  → (1<<4)|(1<<3)|(1<<5) = 0b00111000
DIR_MASKS[W] = bits 6,7,5  → (1<<6)|(1<<7)|(1<<5) = 0b11100000
DIR_MASKS[E] = bits 2,1,3  → (1<<2)|(1<<1)|(1<<3) = 0b00001110

if occupied & DIR_MASKS[dir] == 0 { /* direction is clear */ }
```

### Phase 1: Proposals

Elves are collected into a `Vec<Pos>` for cache-friendly iteration (vs random HashSet traversal). For each elf:

1. **Unpack** the `i64` into (row, col) once
2. **Bitmask build**: Check all 8 neighbors, set corresponding bits. If `occupied == 0` → skip (isolated)
3. **Direction search**: Test `occupied & DIR_MASKS[dir] == 0` for each rotated direction. First clear direction wins.
4. **Record**: `FxHashMap<Pos, (Pos, u8)>` stores `(first_proposer, count)` per destination — avoids `Vec` allocations entirely

### Phase 2: Movement

Iterate the proposals map. For each destination with `count == 1`, remove the source from and insert the destination into the HashSet.

### Direction Rotation

Round 0: N, S, W, E → Round 1: S, W, E, N → Round 2: W, E, N, S → ...

Implemented as `(round + i) % 4` indexing — zero-cost rotation with no array shifting.

### Worked Example (Small Input)

```
Round 0 (check N first):
  ..##.          ..##.     Northernmost 2 → propose N (no N neighbors)
  ..#..    →     .....     Middle → proposes S, collides with SW elf
  .....          ..#..     SW elf → proposes N, collides with middle
  ..##.          ...#.     SE elf → proposes N (no N neighbors)
  .....          ..#..

Round 1 (check S first):
  ..##.          .....     Both top → propose S
  .....          ..##.     West middle → can't go S, proposes W
  ..#..    →     .#...     East middle → can't go S or W, proposes E
  ...#.          ....#     Bottom → doesn't move (no neighbors)
  ..#..          .....
                 ..#..

Round 2 (check W first):
  .....          ..#..     All 3 with neighbors propose different dirs
  ..##.          ....#     West → proposes W
  .#...    →     #....     East top → proposes E
  ....#          ....#     Middle → proposes N
  .....          .....
  ..#..          ..#..

Round 3: No elf has any neighbors → nobody moves → Part 2 answer = 3
```

### Part 1: Bounding Box

Checkpointed at round 10 during the single simulation run:
```
area = (max_row - min_row + 1) × (max_col - min_col + 1)
empty_tiles = area - num_elves
```

### Part 2: Quiescence Detection

The `step()` function returns `bool` — whether any elf moved. Run rounds until it returns `false`. The answer is that round number (1-indexed).

---

## Implementation Details

### Data Flow
```
Input → parse() → FxHashSet<i64>  (packed coords)
                       ↓
              step() rounds 0-9
                       ↓
              round == 10 → bounding_box_empty() → Part 1
                       ↓
              step() rounds 10+ until !moved → Part 2
```

### Function Signatures

| Function | Input | Output | Purpose |
|----------|-------|--------|---------|
| `pack` | `i32, i32` | `i64` | Pack (row, col) into single hash key |
| `unpack` | `i64` | `(i32, i32)` | Extract row, col from packed key |
| `parse` | `&str` | `FxHashSet<Pos>` | Extract elf positions as packed coords |
| `step` | `&mut elves, round, &mut proposals, &mut elf_vec` | `bool` | One round: propose + move, return if any moved |
| `bounding_box_empty` | `&FxHashSet<Pos>` | `usize` | Count empty tiles in min bounding rectangle |
| `solve` | `&str` | `(usize, usize)` | Single-run both parts with checkpoint |

### Key Design Decisions

**Why packed `i64` instead of `(i32, i32)`?**
FxHash on a single `u64` does one hash operation; on a tuple it does two. With ~18.6M hash lookups total, this saves ~18.6M extra hash ops. The pack/unpack overhead (shift + mask) is negligible compared to the hash savings.

**Why can't packed values be added?**
Column overflow corrupts row bits: `pack(0,1) + pack(0,-1)` = `1 + 0xFFFFFFFF` = `0x1_00000000` → unpacks as (1, 0) instead of (0, 0). Arithmetic must go through `unpack → add → repack`.

**Why `(Pos, u8)` in proposals instead of `Vec<Pos>`?**
We only need the source position when count == 1. Storing `(first_proposer, count)` avoids Vec heap allocations entirely — the `and_modify(|e| e.1 += 1)` path just increments a byte.

**Why collect to `Vec` before iterating?**
HashSet iteration jumps between hash buckets with poor cache locality. Collecting ~2,500 `i64` values into a contiguous Vec gives sequential memory access during the proposal phase. The Vec is reused across rounds with `clear()`.

**Why single run with checkpoint instead of clone + separate runs?**
The baseline cloned the HashSet and ran rounds 0-9 separately, then ran all 930 rounds from scratch for Part 2 — effectively computing rounds 0-9 twice. Checkpointing at round 10 saves ~1ms and simplifies the flow.

**Why reuse proposals HashMap with `clear()`?**
Fresh allocation per round means ~930 HashMap constructions. `clear()` retains the allocated capacity, avoiding repeated allocations. Combined with `(Pos, u8)` values (no inner Vecs), this eliminates ~99% of per-round allocation overhead.

---

## Performance Analysis

### Optimization Journey

| Version | Time | Speedup | Key Change |
|---------|------|---------|------------|
| Baseline | 104.62ms | 1.00× | `Vec<Pos>` proposals, fresh HashMap/round, clone for Part 1 |
| + Reuse HashMap + `(Pos, u8)` + single run | 67.49ms | 1.55× | Eliminate allocs, reuse map, checkpoint |
| + Bitmask direction checks | ~67.8ms | — | Wash: bitwise AND vs short-circuit `any` |
| + Packed `i64` + Vec iteration | **56.38ms** | **1.86×** | Single-u64 hash, cache-friendly iteration |
| + Sorted Vec proposals (reverted) | 58.7ms | — | Sort O(n log n) worse than HashMap |

### Benchmark Results
```
Baseline:  104.62ms
Optimized:  56.38ms  (1.86× faster)
```

### What Helped
1. **`(Pos, u8)` proposals** (biggest win): Eliminated ~1,000 Vec allocations per round × 930 rounds
2. **HashMap reuse**: Single allocation vs 930 constructions
3. **Single run**: Eliminated redundant rounds 0-9
4. **Packed `i64` hashing**: One hash op per lookup instead of two
5. **Vec iteration**: Sequential memory access for proposal phase

### What Didn't Help
- **Bitmask direction checks**: The 8-neighbor bitmask always does 8 lookups. The old code with `any()` short-circuits to ~3 lookups for isolated elves, plus the `all()` direction check short-circuits too. Net: bitmask removes up to 12 lookups for non-isolated elves but adds overhead for isolated ones (the majority). Wash.
- **Sorted Vec for proposals**: O(n log n) sort overhead outweighed cache locality gain vs HashMap with O(1) amortized insert. Measured 4% slower.

### Complexity

| Component | Complexity | Notes |
|-----------|------------|-------|
| Parse | O(rows × cols) | Single pass through input grid |
| Per-round: bitmask build | O(n × 8) | 8 hash lookups per elf (dominates) |
| Per-round: direction test | O(n × 4) | 4 bitwise ANDs (negligible) |
| Per-round: proposals | O(n) | HashMap insert/update, amortized O(1) |
| Per-round: movement | O(p) | p sole proposers, HashSet remove/insert |
| Part 1 total | O(10 × n) | ~0.6ms |
| Part 2 total | O(R × n) | R ≈ 930, n ≈ 2,500 → ~56ms |

### Where Time Is Spent

~18.6M hash lookups (8 × 2,500 × 930) dominate. FxHash on packed `i64` is among the fastest possible implementations. Further gains would require a fundamentally different data structure (spatial grid, quadtree) at significant code complexity cost.

---

## Edge Cases

- **No neighbors**: `occupied == 0` early-out — the majority of elves in later rounds
- **All directions blocked**: All 4 `DIR_MASKS` tests fail → elf stays put
- **Collision cancellation**: `count > 1` in proposals map → none of those elves move
- **Packed coordinate carry**: Cannot add packed `i64` values directly — must unpack, add, repack
- **Round numbering**: 0-indexed internally, Part 2 reports 1-indexed (first round with no movement)
- **Windows line endings**: Normalized with `.replace("\r\n", "\n")` before parsing

---

## Key Takeaways

1. **Packed `i64` coordinates for faster hashing** — single-u64 FxHash is measurably faster than tuple hashing when millions of lookups dominate runtime. But beware: packed addition doesn't work due to column-to-row carry.
2. **Bitmask neighbor encoding** — compute 8-neighbor occupancy once, test directions with bitwise AND. Theoretically eliminates up to 12 hash lookups per non-isolated elf, but in practice was a wash due to short-circuit optimization in the original code.
3. **`(key, count)` proposals beat `Vec<value>`** — when you only need the value for count==1, storing `(first_value, count)` avoids all inner collection allocations. Combined with HashMap reuse via `clear()`, this was the single biggest optimization.
4. **Vec collection for cache-friendly HashSet iteration** — copying ~2,500 entries to a contiguous Vec before iterating eliminates hash-bucket jumping. Reusable Vec with `clear()` avoids per-round allocation.
5. **Checkpoint mid-stream instead of clone** — when Part 1 and Part 2 share the simulation, checkpoint state at the Part 1 boundary rather than running the early rounds twice.
6. **Measure before committing** — bitmask direction checks and sorted-Vec proposals both seemed promising theoretically but measured neutral/negative. Only packed coords + allocation reduction delivered real gains.

---

**Answer**: Part 1: `3780` | Part 2: `930`

**Related patterns**: [[cellular-automaton]] | [[sparse-grid-simulation]] | [[proposal-collision]] | [[packed-coordinates]]
