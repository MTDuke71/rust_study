# Day 4: Security Through Obscurity --- Function Guide

**Problem**: Validate encrypted room names via letter-frequency checksums, then decrypt with Caesar cipher.
**Answers**: Part 1 = **245102**, Part 2 = **324**
**Code**: [day04.rs](../../src/solver/day04.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Data Structures](#data-structures)
3. [parse_input](#parse_input)
4. [is_real](#is_real)
5. [decrypt](#decrypt)
6. [solve_part1_with_data / solve_part2_with_data](#solve_part1_with_data--solve_part2_with_data)
7. [Benchmarks](#benchmarks)
8. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: ~1000 encrypted room entries, each with the format:

```
encrypted-name-sectorID[checksum]
```

Example:
```
aaaaa-bbb-z-y-x-123[abxyz]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]
```

**Part 1**: A room is **real** if its checksum equals the 5 most frequent letters in the name (excluding dashes), with ties broken alphabetically. Sum the sector IDs of all real rooms.

**Part 2**: Decrypt each real room name using a **Caesar cipher** --- shift each letter forward by `sector_id` positions (wrapping z->a). Dashes become spaces. Find the room containing "northpole" and return its sector ID.

---

## Data Structures

### `Room<'a>`

```rust
struct Room<'a> {
    name: &'a str,      // encrypted name portion (e.g., "aaaaa-bbb-z-y-x")
    sector_id: u32,     // numeric sector ID
    checksum: &'a str,  // 5-letter checksum between brackets
}
```

Zero-copy design: `name` and `checksum` are borrowed slices into the original input string. No allocation needed during parsing beyond the `Vec<Room>` itself.

**Why `&'a str` instead of `String`**: The room data is read-only --- we never modify names or checksums. Borrowing avoids ~1000 string allocations.

---

## `parse_input`

```rust
fn parse_input(input: &str) -> Vec<Room<'_>> {
    input.lines().map(|line| {
        let bracket = line.find('[').unwrap();
        let checksum = &line[bracket + 1..line.len() - 1];
        let rest = &line[..bracket];
        let dash = rest.rfind('-').unwrap();
        let name = &rest[..dash];
        let sector_id = rest[dash + 1..].parse().unwrap();
        Room { name, sector_id, checksum }
    }).collect()
}
```

**Parsing strategy**: Work from the outside in.
1. `find('[')` --- locates the checksum bracket
2. Slice checksum between `[` and `]`
3. `rfind('-')` on the remainder --- the **last** dash separates name from sector ID
4. Parse sector ID as `u32`

**Why `rfind` not `find`**: The name contains dashes (e.g., `"not-a-real-room"`). The last dash is always the separator before the sector ID.

---

## `is_real`

```rust
fn is_real(room: &Room<'_>) -> bool {
    let mut counts = [0u32; 26];
    for b in room.name.bytes() {
        if b != b'-' {
            counts[(b - b'a') as usize] += 1;
        }
    }
    let mut letters: Vec<(u32, u8)> = counts.iter()
        .enumerate()
        .filter(|(_, &c)| c > 0)
        .map(|(i, &c)| (c, i as u8))
        .collect();
    letters.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    let computed: String = letters.iter()
        .take(5)
        .map(|&(_, i)| (b'a' + i) as char)
        .collect();
    computed == room.checksum
}
```

**Algorithm**:
1. Count letter frequencies in a fixed `[u32; 26]` array (no HashMap needed --- only lowercase ASCII)
2. Collect non-zero entries as `(count, letter_index)` pairs
3. Sort by count **descending**, then letter index **ascending** (alphabetical tie-break)
4. Take top 5 letters, build string, compare to checksum

**Why `[u32; 26]` not `HashMap`**: Only 26 possible letters. A fixed array is faster (no hashing), cache-friendly, and stack-allocated.

**Sort order trick**: `b.0.cmp(&a.0)` reverses count order (high first), `.then(a.1.cmp(&b.1))` keeps alphabetical order for ties.

---

## `decrypt`

```rust
fn decrypt(name: &str, shift: u32) -> String {
    let shift = (shift % 26) as u8;
    name.bytes().map(|b| {
        if b == b'-' { ' ' }
        else { (b'a' + (b - b'a' + shift) % 26) as char }
    }).collect()
}
```

**Caesar cipher**: Shift each letter forward by `shift % 26`. The modulo prevents overflow for large sector IDs (e.g., 993). Dashes become spaces.

**Example**: `decrypt("qzmt-zixmtkozy-ivhz", 343)` = `"very encrypted name"`
- 343 % 26 = 5
- q + 5 = v, z + 5 = e, m + 5 = r, t + 5 = y, ...

---

## `solve_part1_with_data` / `solve_part2_with_data`

### Part 1 --- Sum real sector IDs

```rust
fn solve_part1_with_data(rooms: &[Room<'_>]) -> u32 {
    rooms.iter().filter(|r| is_real(r)).map(|r| r.sector_id).sum()
}
```

Filter real rooms, sum their sector IDs. Classic filter-map-sum chain.

### Part 2 --- Find northpole room

```rust
fn solve_part2_with_real(real: &[&Room<'_>]) -> u32 {
    real.iter()
        .find(|r| decrypt(r.name, r.sector_id).contains("northpole"))
        .map(|r| r.sector_id)
        .expect("northpole room not found")
}
```

Searches the pre-filtered real rooms for "northpole" in the decrypted name. `find()` short-circuits on first match. No checksum re-validation needed since `real_rooms()` already filtered.

---

## Benchmarks

| Function | Time |
|----------|------|
| `solve_part1` (standalone) | 307us |
| `solve_part2` (standalone) | 335us |
| `solve` (combined) | **332us** |

**Key observations**:
- **Combined is fastest**: Pre-filtering real rooms once means Part 1 just sums and Part 2 just searches --- no redundant checksum work
- **Standalone part2 is slowest**: It must parse + filter all rooms even though it only needs one. The combined path avoids this by sharing the filtered list
- **35% improvement** over initial version (532us -> 332us) by pre-filtering real rooms once
- **Well under 100ms target** --- no further optimization needed

---

## Key Patterns

### Zero-copy parsing with lifetimes

```rust
struct Room<'a> {
    name: &'a str,
    checksum: &'a str,
}
```

The `'a` lifetime ties `Room` to the input string. No allocations during parsing --- just pointer arithmetic into the original `&str`. This is why Rust excels at string-heavy parsing problems.

### Fixed-size frequency array vs HashMap

For small, known character sets (26 lowercase letters), `[u32; 26]` beats `HashMap<char, u32>`:
- **No hashing overhead**: Direct index `(b - b'a')` is O(1) with zero overhead
- **Cache-friendly**: 104 bytes contiguous on the stack
- **No allocation**: Stack-only, no heap interaction

### Custom sort ordering with `.then()`

```rust
letters.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
```

`Ordering::then()` chains sort criteria: primary key descending, secondary key ascending. Cleaner than a multi-branch comparison function.

### Parse-once pattern (continued)

Same pattern as Days 1-3: `solve()` parses once, passes data to both `_with_data` functions. Especially important here since Part 2 also needs the parsed rooms for checksum validation.

---

**Navigation**: [<- Day 3](day03_function_guide.md) | [All Days](../summary_2016.md) | [Day 5 ->](day05.md)
