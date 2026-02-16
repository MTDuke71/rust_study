# A Philosophy of Software Design

**Category**: Software Engineering, Design Principles, Complexity Management
**Tags**: #software-design #complexity #information-hiding #module-design #interfaces

**Author**: John Ousterhout (Stanford University)
**Book**: *A Philosophy of Software Design* (2018, 2nd edition 2021)

---

## Overview

**Central Thesis**: The greatest limitation in writing software is our ability to understand the systems we create. The primary goal of software design is to **reduce complexity** - making systems easier to understand and modify.

**Core Philosophy**:
- **Problem**: Software complexity grows incrementally, making systems harder to understand
- **Solution**: Design modules with simple interfaces that hide complexity
- **Metric**: Good design = minimal cognitive load to understand and modify

**Relationship to DDD**: See [[domain-driven-design]]
- **PoSD**: *How* to design modules (interfaces, abstraction, information hiding)
- **DDD**: *What* to model (domain concepts, ubiquitous language)
- **Together**: DDD gives vocabulary, PoSD gives grammar

---

## Core Concepts

### 1. Complexity is Incremental

**Definition**: Complexity accumulates from hundreds of small dependencies and obscurities, not from single catastrophic decisions.

**Three Symptoms**:
1. **Change amplification**: Simple change requires modifications in many places
2. **Cognitive load**: How much you need to know to complete a task
3. **Unknown unknowns**: Not obvious which code must be modified

**Rust Example** (AoC Day 15):
```rust
// ❌ BAD: Complexity accumulates - every caller needs to know implementation
fn calculate_coverage_at_row(sensors: &[Sensor], y: i32) -> Vec<(i32, i32)> {
    sensors.iter()
        .filter_map(|s| {
            let radius = (s.x - s.beacon_x).abs() + (s.y - s.beacon_y).abs();  // Duplicated!
            let vertical_dist = (s.y - y).abs();
            let remaining = radius - vertical_dist;
            if remaining >= 0 {
                Some((s.x - remaining, s.x + remaining))
            } else {
                None
            }
        })
        .collect()
}

// Every caller must remember: radius = Manhattan distance
// Change formula? Update every caller!

// ✅ GOOD: Complexity hidden in Sensor
impl Sensor {
    pub fn radius(&self) -> i32 {
        (self.x - self.beacon_x).abs() + (self.y - self.beacon_y).abs()
    }

    pub fn coverage_at_row(&self, y: i32) -> Option<(i32, i32)> {
        let vertical_dist = (self.y - y).abs();
        let remaining = self.radius() - vertical_dist;

        if remaining >= 0 {
            Some((self.x - remaining, self.x + remaining))
        } else {
            None
        }
    }
}

// Caller doesn't know about Manhattan distance
// Change formula? One place to update!
let intervals: Vec<_> = sensors.iter()
    .filter_map(|s| s.coverage_at_row(y))
    .collect();
```

**Key Insight**: Each abstraction (`radius()`, `coverage_at_row()`) prevents complexity from spreading.

---

### 2. Deep vs Shallow Modules

**Deep Module**: Simple interface, powerful functionality
- Small surface area (few methods, simple signatures)
- Large implementation (handles complexity internally)
- High functionality-to-complexity ratio

**Shallow Module**: Complex interface, minimal functionality
- Large surface area (many methods, complex parameters)
- Small implementation (just delegates or trivial logic)
- Low functionality-to-complexity ratio

**Visualization**:
```
Deep Module:              Shallow Module:
┌────────┐                ┌──────────────────┐
│ Simple │                │ Complex Interface│
│Interface│               └──────────────────┘
└────────┘                        ││
    ││                            ││
    ││                        ┌───┴┴────┐
    ││                        │  Tiny   │
    ││                        │  Impl   │
┌───┴┴──────┐               └─────────┘
│           │
│  Large    │
│   Impl    │
│           │
└───────────┘
```

**Rust Example** (Unix file I/O is classic deep module):
```rust
// Deep module: std::fs::File
use std::fs::File;
use std::io::Read;

let mut file = File::open("data.txt")?;  // Simple interface
let mut contents = String::new();
file.read_to_string(&mut contents)?;     // Simple interface

// Hidden complexity:
// - File descriptors, kernel calls
// - Buffering strategies
// - Error handling (disk full, permissions)
// - UTF-8 validation
// - Memory allocation

// ❌ SHALLOW: Exposing every detail
struct ShallowFile {
    fd: RawFd,
    buffer: Vec<u8>,
    buffer_size: usize,
    read_pos: usize,
    write_pos: usize,
    // ... 10 more fields
}

impl ShallowFile {
    pub fn set_buffer_size(&mut self, size: usize) { /* ... */ }
    pub fn flush_buffer(&mut self) -> Result<()> { /* ... */ }
    pub fn get_buffer_usage(&self) -> f64 { /* ... */ }
    pub fn set_read_ahead(&mut self, enabled: bool) { /* ... */ }
    // ... 15 more methods caller must understand
}
```

**AoC Day 15 Example**:
```rust
// ✅ DEEP: interval_merging module
pub fn merge_intervals(intervals: &[(i32, i32)]) -> Vec<(i32, i32)> {
    // Simple interface: take intervals, return merged
    // Hidden complexity:
    // - Sorting strategy (assumed pre-sorted or sort internally)
    // - Adjacent vs overlapping detection
    // - Edge cases (empty, single interval)
    // - Memory allocation strategy
}

// Caller doesn't care HOW merging works, just that it does
```

**AUTOSAR Analogy**:
- **Deep Module**: BSW (Basic Software) services - simple API (`Can_Write()`), massive implementation
- **Shallow Module**: Getter/setter wrappers around data - lots of methods, no real abstraction

---

### 3. Information Hiding

**Principle**: Each module should encapsulate knowledge that represents design decisions. These details should be hidden from other modules.

**What to Hide**:
- Implementation mechanisms
- Data structures
- Algorithms
- Design decisions likely to change

**Benefits**:
- Reduces cognitive load (callers don't need to understand internals)
- Enables evolution (change internals without breaking callers)
- Simplifies testing (fewer dependencies)

**Rust Example** (Mission 6 Grid):
```rust
// ❌ BAD: Exposing internal representation
pub struct Grid<T> {
    pub cells: Vec<Vec<T>>,  // Now callers depend on Vec<Vec<T>>!
    pub width: usize,
    pub height: usize,
}

// Caller writes: grid.cells[y][x] = value;
// Problem: Can't change to flat Vec or HashMap without breaking callers!

// ✅ GOOD: Information hiding
pub struct Grid<T> {
    cells: Vec<T>,      // Private! Could change to HashMap later
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn get(&self, coord: Coord) -> Option<&T> {
        if coord.x < 0 || coord.y < 0 {
            return None;
        }
        let (x, y) = (coord.x as usize, coord.y as usize);
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(&self.cells[y * self.width + x])  // Implementation hidden!
    }

    pub fn set(&mut self, coord: Coord, value: T) -> bool {
        // Implementation hidden - can optimize without breaking callers
    }
}

// Caller writes: grid.set(coord, value);
// Internal representation can change freely!
```

**Evolution Example**:
```rust
// Version 1: Flat Vec
struct Grid<T> {
    cells: Vec<T>,
    width: usize,
}

// Version 2: Sparse HashMap (no API changes!)
struct Grid<T> {
    cells: HashMap<(usize, usize), T>,
    width: usize,
    default: T,
}

// Version 3: Chunked for cache locality (no API changes!)
struct Grid<T> {
    chunks: Vec<Vec<T>>,  // 64x64 chunks
    chunk_size: usize,
    width: usize,
}

// All three expose same API - callers never break!
```

**Key Insight**: The interface (`get`, `set`) is the contract. The implementation is the secret.

---

### 4. General-Purpose vs Special-Purpose Interfaces

**General-Purpose**: Solves a broad class of problems
- Fewer methods, more flexibility
- Supports current AND future uses
- Example: `String::from` (works for any convertible type)

**Special-Purpose**: Solves one specific problem
- More methods, each narrow in scope
- Only supports current use case
- Example: `String::from_user_input_validated_trimmed_lowercase`

**Ousterhout's Advice**: Somewhat general-purpose is usually better
- Not too general (hard to use, like `eval()`)
- Not too specific (proliferation of methods)
- "Make interfaces easy to use for today's needs, but not impossible for tomorrow's"

**Rust Example**:
```rust
// ❌ TOO SPECIFIC: Separate method for each case
impl Sensor {
    pub fn coverage_at_row_2000000(&self) -> Option<(i32, i32)> { /* ... */ }
    pub fn coverage_at_row_3000000(&self) -> Option<(i32, i32)> { /* ... */ }
    // Need infinite methods for infinite rows!
}

// ✅ GENERAL PURPOSE: One method, parameterized
impl Sensor {
    pub fn coverage_at_row(&self, y: i32) -> Option<(i32, i32)> {
        let vertical_dist = (self.y - y).abs();
        let remaining = self.radius() - vertical_dist;

        if remaining >= 0 {
            Some((self.x - remaining, self.x + remaining))
        } else {
            None
        }
    }
}

// ❌ TOO GENERAL: Exposing everything
impl Sensor {
    pub fn calculate(&self, operation: Operation, params: &[Param]) -> Result<Value> {
        // So general it's useless - what operations? what params?
    }
}
```

**AsRef Pattern** (from [[asref-trait-ergonomics]]):
```rust
// ✅ GENERAL PURPOSE: Accept anything convertible to Path
pub fn read_config<P: AsRef<Path>>(path: P) -> Result<Config> {
    let path = path.as_ref();
    // Works with: &Path, PathBuf, &str, String, OsStr, etc.
}

// Caller flexibility:
read_config("config.toml");           // &str
read_config(PathBuf::from("config")); // PathBuf
read_config(&path_variable);          // &Path

// ❌ TOO SPECIFIC: Separate function for each type
pub fn read_config_from_str(path: &str) -> Result<Config> { /* ... */ }
pub fn read_config_from_pathbuf(path: PathBuf) -> Result<Config> { /* ... */ }
pub fn read_config_from_path(path: &Path) -> Result<Config> { /* ... */ }
```

---

### 5. Different Layer, Different Abstraction

**Principle**: If adjacent layers have similar abstractions, they probably should be merged. Each layer should provide a different level of abstraction.

**Warning Signs**:
- Pass-through methods (just delegate to next layer)
- Pass-through variables (parameters passed unchanged through layers)
- Decorators that add minimal value

**Rust Example**:
```rust
// ❌ BAD: Layers provide same abstraction
mod parser {
    pub fn parse_sensor(line: &str) -> Sensor { /* ... */ }
}

mod sensor_parser {  // Unnecessary layer!
    pub fn parse(line: &str) -> Sensor {
        parser::parse_sensor(line)  // Just delegates!
    }
}

mod input {  // Another unnecessary layer!
    pub fn parse_sensor_from_line(line: &str) -> Sensor {
        sensor_parser::parse(line)  // Just delegates!
    }
}

// ✅ GOOD: Each layer adds abstraction
mod parser {
    // Low-level: Extract numbers from text
    pub fn parse_sensor(line: &str) -> Sensor { /* ... */ }
}

mod input_processor {
    // Mid-level: Handle file I/O, line iteration
    pub fn parse_input(input: &str) -> Vec<Sensor> {
        input.lines()
            .map(parser::parse_sensor)
            .collect()
    }
}

mod solver {
    // High-level: Business logic
    pub fn solve(input: &str) -> (usize, i64) {
        let sensors = input_processor::parse_input(input);
        (part1(&sensors), part2(&sensors))
    }
}
```

**DDD Connection**: Bounded contexts (see [[domain-driven-design]]) are different layers with different abstractions!

**AUTOSAR Analogy**:
- ❌ **Bad**: RTE just delegates to OS (same abstraction)
- ✅ **Good**: RTE provides "runnable" abstraction, OS provides "task" abstraction (different!)

---

### 6. Pull Complexity Downward

**Principle**: It's better for a module to be complex internally than to leak complexity to callers.

**Rationale**:
- Module is written once, called many times
- Better to suffer complexity in one place than force every caller to handle it

**Rust Example** (Error handling):
```rust
// ❌ BAD: Pushing complexity upward
pub fn parse_sensor(line: &str) -> Option<Vec<i32>> {
    Some(line.split(|c: char| !c.is_numeric() && c != '-')
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse().ok())
        .collect())
}

// Caller must handle weird edge cases:
let nums = parse_sensor(line)?;
if nums.len() != 4 {
    return Err("Invalid format");  // Every caller checks this!
}
let sensor = Sensor {
    x: nums[0],
    y: nums[1],
    beacon_x: nums[2],
    beacon_y: nums[3]
};

// ✅ GOOD: Pulling complexity downward
pub fn parse_sensor(line: &str) -> Result<Sensor, ParseError> {
    let nums: Vec<i32> = line
        .split(|c: char| !c.is_numeric() && c != '-')
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse().ok())
        .collect();

    if nums.len() != 4 {
        return Err(ParseError::InvalidFormat);  // Handle in one place!
    }

    Ok(Sensor {
        x: nums[0],
        y: nums[1],
        beacon_x: nums[2],
        beacon_y: nums[3],
    })
}

// Caller just uses it:
let sensor = parse_sensor(line)?;  // Clean!
```

**Configuration Example**:
```rust
// ❌ BAD: Pushing defaults to caller
pub struct Config {
    pub timeout: Option<u64>,
    pub retries: Option<u32>,
    pub buffer_size: Option<usize>,
}

// Every caller must handle None:
let timeout = config.timeout.unwrap_or(5000);
let retries = config.retries.unwrap_or(3);
let buffer = vec![0; config.buffer_size.unwrap_or(4096)];

// ✅ GOOD: Pulling defaults downward
pub struct Config {
    timeout: u64,
    retries: u32,
    buffer_size: usize,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder {
            timeout: 5000,     // Defaults here!
            retries: 3,
            buffer_size: 4096,
        }
    }
}

// Caller gets sensible defaults automatically:
let config = Config::builder().timeout(10000).build();
// retries and buffer_size use defaults - caller doesn't worry!
```

---

### 7. Define Errors Out of Existence

**Principle**: Reduce error cases by designing them away, rather than detecting and handling them.

**Techniques**:
1. **Make invalid states unrepresentable** (type system)
2. **Provide defaults** instead of requiring configuration
3. **Be tolerant** of inputs (robustness principle)
4. **Mask exceptions** by handling them internally when safe

**Rust Example 1** (Type safety):
```rust
// ❌ BAD: Invalid states representable
pub struct User {
    pub id: u64,
    pub email: String,  // Could be empty or invalid!
    pub age: i32,       // Could be negative!
}

fn create_user(id: u64, email: String, age: i32) -> Result<User, UserError> {
    if email.is_empty() {
        return Err(UserError::EmptyEmail);  // Runtime error!
    }
    if !email.contains('@') {
        return Err(UserError::InvalidEmail);  // Runtime error!
    }
    if age < 0 || age > 150 {
        return Err(UserError::InvalidAge);  // Runtime error!
    }
    Ok(User { id, email, age })
}

// ✅ GOOD: Invalid states unrepresentable
pub struct Email(String);

impl Email {
    pub fn new(s: String) -> Result<Self, EmailError> {
        if s.is_empty() || !s.contains('@') {
            return Err(EmailError::Invalid);
        }
        Ok(Email(s))
    }
}

pub struct Age(u8);  // 0-255, enforced at construction

impl Age {
    pub fn new(value: u8) -> Result<Self, AgeError> {
        if value > 150 {
            return Err(AgeError::TooOld);
        }
        Ok(Age(value))
    }
}

pub struct User {
    pub id: u64,
    pub email: Email,  // Guaranteed valid!
    pub age: Age,      // Guaranteed valid!
}

fn create_user(id: u64, email: Email, age: Age) -> User {
    User { id, email, age }  // No error handling needed!
}
```

**Rust Example 2** (Tolerant parsing):
```rust
// ❌ BAD: Strict, rejects minor variations
fn parse_bool(s: &str) -> Result<bool, ParseError> {
    match s {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(ParseError::InvalidBool),  // Rejects "True", "yes", "1"
    }
}

// ✅ GOOD: Tolerant, defines away errors
fn parse_bool(s: &str) -> bool {
    matches!(
        s.to_lowercase().trim(),
        "true" | "yes" | "1" | "on" | "enabled"
    )
    // Everything else is false - no error case!
}
```

**Mission 6 Grid Example**:
```rust
// ❌ BAD: Out-of-bounds returns error
impl<T> Grid<T> {
    pub fn get(&self, coord: Coord) -> Result<&T, GridError> {
        if coord.x < 0 || coord.x >= self.width as i32 {
            return Err(GridError::OutOfBounds);  // Caller must handle!
        }
        // ...
    }
}

// ✅ GOOD: Out-of-bounds returns None (error defined away)
impl<T> Grid<T> {
    pub fn get(&self, coord: Coord) -> Option<&T> {
        if coord.x < 0 || coord.x >= self.width as i32 {
            return None;  // Natural, expected case
        }
        // ...
    }
}

// Caller uses natural pattern:
if let Some(tile) = grid.get(coord) {
    // Do something
}
// No .unwrap() or .expect() - None is expected!
```

---

### 8. Strategic vs Tactical Programming

**Tactical Programming**: Get features working as quickly as possible
- Short-term focus
- "Just make it work"
- Complexity accumulates
- Result: Systems become unmaintainable

**Strategic Programming**: Invest time to produce great design
- Long-term focus
- "How can this be designed well?"
- Clean abstractions
- Result: Sustained productivity

**Ousterhout's Recommendation**: Spend ~10-20% extra time on design
- Not doubling development time
- Small investment with huge payoff

**AoC Example**:

**Tactical Approach** (Speed-run AoC):
```rust
// Just solve it!
fn solve(input: &str) -> (usize, i64) {
    let mut sensors = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let x = parts[2].trim_end_matches(',').parse().unwrap();
        let y = parts[3].trim_end_matches(':').parse().unwrap();
        // ... keep hacking until it works
        sensors.push((x, y, bx, by));
    }

    // Giant mess of loops and conditions
    let mut count = 0;
    for x in -1000000..5000000 {  // Brute force!
        // ...
    }
    (count, freq)
}
```

**Strategic Approach** (Production quality):
```rust
// Design clear abstractions
pub struct Sensor {
    pub x: i32,
    pub y: i32,
    pub beacon_x: i32,
    pub beacon_y: i32,
}

impl Sensor {
    pub fn radius(&self) -> i32 { /* domain concept */ }
    pub fn coverage_at_row(&self, y: i32) -> Option<(i32, i32)> { /* ... */ }
}

pub fn parse(input: &str) -> Vec<Sensor> { /* clean parsing */ }
pub fn part1(sensors: &[Sensor]) -> usize { /* clear algorithm */ }
pub fn part2(sensors: &[Sensor]) -> i64 { /* clear algorithm */ }

pub fn solve(input: &str) -> (usize, i64) {
    let sensors = parse(input);
    (part1(&sensors), part2(&sensors))
}
```

**Trade-off**: Strategic took 10% longer (45 min vs 40 min), but:
- Tests are easy to write
- Benchmarks measure each phase
- Documentation writes itself
- Future days can reuse Sensor abstraction
- Code reviews are pleasant

**AUTOSAR Connection**: AUTOSAR IS strategic programming - invest upfront in architecture for long-term maintainability.

---

### 9. Comments Should Describe Things Not Obvious from Code

**Principle**: Comments augment code by providing information at different levels of abstraction.

**What to Comment**:
- **Why**, not **what**: Explain design decisions, not obvious code
- **High-level overview**: What does this module/function do?
- **Non-obvious details**: Edge cases, assumptions, performance considerations
- **Cross-module dependencies**: How does this relate to other parts?

**What NOT to Comment**:
- Redundant with code: `i++; // increment i`
- Out-of-date information
- Excuses for bad code

**Rust Example**:
```rust
// ❌ BAD: Comments repeat code
pub fn merge_intervals(intervals: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut merged = vec![intervals[0]];  // Create vec with first interval

    for &(start, end) in &intervals[1..] {  // Iterate over intervals
        let last = merged.last_mut().unwrap();  // Get last interval

        if start <= last.1 + 1 {  // If overlapping or adjacent
            last.1 = last.1.max(end);  // Extend last interval
        } else {  // Otherwise
            merged.push((start, end));  // Add new interval
        }
    }

    merged  // Return merged intervals
}

// ✅ GOOD: Comments explain WHY and non-obvious details
/// Merges overlapping and adjacent intervals into minimal covering set.
///
/// **Algorithm**: Greedy merging after sorting by start position.
///
/// **Key Insight**: Uses `start <= last.end + 1` to merge ADJACENT intervals:
/// - [1, 5] and [6, 10] are adjacent (gap-free) → merge to [1, 10]
/// - [1, 5] and [7, 10] have gap at x=6 → keep separate
///
/// **Precondition**: Intervals must be sorted by start position.
///
/// **Complexity**: O(n) where n = number of intervals.
pub fn merge_intervals(intervals: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut merged = vec![intervals[0]];

    for &(start, end) in &intervals[1..] {
        let last = merged.last_mut().unwrap();

        if start <= last.1 + 1 {  // Adjacent check critical for integer intervals
            last.1 = last.1.max(end);
        } else {
            merged.push((start, end));
        }
    }

    merged
}
```

**Documentation Example** (Day 15):
```rust
impl Sensor {
    /// Returns the detection radius of this sensor.
    ///
    /// The radius is the Manhattan distance from the sensor to its nearest beacon.
    /// All positions within this distance cannot contain another beacon (would be closer).
    ///
    /// **Domain Concept**: "How far can this sensor detect?"
    /// **Implementation**: Manhattan distance (L₁ metric)
    ///
    /// # Example
    /// ```
    /// let sensor = Sensor { x: 8, y: 7, beacon_x: 2, beacon_y: 10 };
    /// assert_eq!(sensor.radius(), 9);  // |8-2| + |7-10| = 6 + 3 = 9
    /// ```
    pub fn radius(&self) -> i32 {
        manhattan_distance(self.x, self.y, self.beacon_x, self.beacon_y)
    }
}
```

**Rust Advantage**: Doc comments (`///`) become tests and documentation automatically!

---

## Practical Applications in This Codebase

### AoC Day 15 - Multiple Principles Applied

**Deep Module** - `Sensor`:
- Simple interface: `radius()`, `coverage_at_row()`
- Hidden complexity: Manhattan distance, geometry calculations

**Information Hiding** - Implementation:
```rust
// Interface doesn't expose how radius is calculated
pub fn radius(&self) -> i32 { /* ... */ }

// Could change to:
// - Cache the radius
// - Add calibration factor
// - Use different distance metric
// Callers never know!
```

**Pull Complexity Downward** - Error handling:
```rust
// Sensor handles "doesn't reach row" internally
pub fn coverage_at_row(&self, y: i32) -> Option<(i32, i32)> {
    // Returns None instead of making caller check
}
```

**Strategic Programming** - Parse-once pattern:
```rust
// Invested time in clean separation
pub fn solve(input: &str) -> (usize, i64) {
    let sensors = parse(input);  // Reusable
    (part1(&sensors), part2(&sensors))
}
```

---

### Mission 6 Grid - Information Hiding

**Deep Module**:
- Simple interface: `get()`, `set()`, `iter()`
- Hidden complexity: Coordinate mapping, bounds checking, storage layout

**Different Layer, Different Abstraction**:
- Grid layer: 2D spatial abstraction (`Coord`, `get/set`)
- Storage layer: 1D Vec (hidden from users)
- Iterator layer: Traversal patterns (hidden implementation)

---

### Mission 11 - Define Errors Out of Existence

**Type Safety**:
```rust
// Pattern matching returns Result, not panic
fn can_form<'a>(design: &'a str, patterns: &[&str]) -> bool {
    // Returns false for impossible, not error
}
```

**Memoization**:
```rust
// Cache handles all cases, no "cache miss" errors
cache.entry(design).or_insert_with(|| compute(design))
```

---

## Common Pitfalls

### Over-Commenting

**Problem**: Comments that don't add information.

```rust
// ❌ BAD
let x = 5;  // set x to 5
x += 1;     // increment x
```

### Shallow Module Proliferation

**Problem**: Creating many tiny modules that don't hide complexity.

```rust
// ❌ BAD: Shallow modules everywhere
mod getters {
    pub fn get_x(sensor: &Sensor) -> i32 { sensor.x }
    pub fn get_y(sensor: &Sensor) -> i32 { sensor.y }
    // 20 more trivial getters
}
```

### Premature Abstraction

**Problem**: Creating general-purpose interfaces before understanding needs.

```rust
// ❌ BAD: Over-generalized
pub trait SensorLike {
    type Coord;
    type Distance;
    type Coverage;
    fn detect<D: Detector>(&self, detector: D) -> Self::Coverage;
}
// Complexity with no benefit!
```

---

## Relationship to Other Concepts

### Complements Domain-Driven Design

See [[domain-driven-design]]

**DDD Provides**:
- *What* to model (domain concepts)
- Ubiquitous language
- Business alignment

**PoSD Provides**:
- *How* to design modules
- Information hiding strategies
- Complexity management

**Together**: Model the right things (DDD) with clean modules (PoSD)

### Complements Clean Code

See [[Clean Code Principles]]

**Clean Code**: Make code readable (naming, formatting, small functions)
**PoSD**: Make systems understandable (modules, interfaces, abstractions)

Both aim to reduce cognitive load, at different scales.

---

## Summary

**A Philosophy of Software Design** provides principles for managing complexity:

1. **Complexity is incremental** - Many small decisions accumulate
2. **Deep modules** - Simple interfaces, powerful implementations
3. **Information hiding** - Encapsulate design decisions
4. **General-purpose interfaces** - Support current and future needs
5. **Different layer, different abstraction** - Each layer adds value
6. **Pull complexity downward** - Module complexity > caller complexity
7. **Define errors out of existence** - Design away error cases
8. **Strategic programming** - Invest 10-20% extra in good design
9. **Comments augment code** - Explain WHY and non-obvious details

**Rust Advantage**: Type system enables defining errors out of existence better than most languages!

**Key Insight**: The `Sensor::radius()` method from Day 15 demonstrates multiple principles:
- Deep module (simple interface, hidden calculation)
- Information hiding (Manhattan distance is implementation detail)
- Ubiquitous language (DDD: "radius" is domain term)
- Strategic design (invested time in clean abstraction)

---

## Related Notes

- [[domain-driven-design]] - Complementary approach (what to model)
- [[Clean Code Principles]] - Complementary approach (code readability)
- [[asref-trait-ergonomics]] - General-purpose interface example
- [[common-traits-pattern]] - Deep module pattern (derive does the work)
- [[mission-6]] - Deep module example (Grid abstraction)
- [[software-architecture-patterns]] - Different layer, different abstraction

---

## Further Reading

**Book**: *A Philosophy of Software Design* by John Ousterhout
- 1st edition: 2018
- 2nd edition: 2021 (recommended)

**Related Books**:
- *Domain-Driven Design* by Eric Evans (complements PoSD)
- *Clean Code* by Robert Martin (complements PoSD)
- *The Pragmatic Programmer* by Hunt & Thomas

---

**Last Updated**: 2026-02-15

**Note**: This summary created while reading the book. Will be updated as understanding deepens!
