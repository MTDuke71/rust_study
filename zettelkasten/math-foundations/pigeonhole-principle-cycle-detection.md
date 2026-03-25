# Pigeonhole Principle & Cycle Detection

**Field**: Discrete Mathematics / Combinatorics

**Prerequisites**: [[set-theory-fundamentals]], [[graph-theory-fundamentals]]

---

## 📐 Definition

**Pigeonhole Principle** (Dirichlet's Box Principle):

> If $n$ items are placed into $m$ containers, where $n > m$, then at least one container must contain more than one item.

**Generalized Form**:

> If $n$ items are placed into $m$ containers, then at least one container contains at least $\lceil n/m \rceil$ items.

**Intuition**: You can't fit 10 pigeons into 9 pigeonholes without having at least one pigeonhole contain 2+ pigeons.

---

## 🔑 Key Applications

### **Application 1**: Cycle Detection in Finite State Spaces

**Statement**: Any sequence in a finite state space must eventually repeat (form a cycle).

**Proof**:
1. Let $S$ be a finite set of states with $|S| = m$
2. Consider a sequence $s_0, s_1, s_2, \ldots$ where each $s_i \in S$
3. After generating $m + 1$ states, we have $m + 1$ items (states) and $m$ containers (possible state values)
4. By the Pigeonhole Principle, at least two states must be identical: $s_i = s_j$ for some $i < j$
5. Therefore, the sequence repeats from position $i$ with period $p = j - i$

**Consequence**: For any deterministic process on a finite state space:
- The sequence **must** eventually cycle
- The cycle begins within the first $m$ states
- We can detect the cycle in $O(m)$ space by tracking seen states

### **Application 2**: Birthday Paradox

**Statement**: Among 23 people, there's >50% probability that two share a birthday.

Applies Pigeonhole Principle with probabilistic extension.

---

## 🔄 Cycle Detection Algorithms

### **Floyd's Cycle Detection (Tortoise and Hare)**

**Complexity**: $O(n)$ time, $O(1)$ space

```rust
fn floyd_detect_cycle<T: Eq>(f: impl Fn(&T) -> T, x0: T) -> Option<(usize, usize)> {
    let mut tortoise = x0.clone();
    let mut hare = x0.clone();
    
    // Phase 1: Detect if cycle exists
    loop {
        tortoise = f(&tortoise);
        hare = f(&f(&hare));
        if tortoise == hare {
            break; // Cycle detected
        }
    }
    
    // Phase 2: Find cycle start
    let mut mu = 0; // Start of cycle
    tortoise = x0.clone();
    while tortoise != hare {
        tortoise = f(&tortoise);
        hare = f(&hare);
        mu += 1;
    }
    
    // Phase 3: Find cycle length
    let mut lambda = 1; // Cycle length
    hare = f(&tortoise);
    while tortoise != hare {
        hare = f(&hare);
        lambda += 1;
    }
    
    Some((mu, lambda))
}
```

**Advantages**: $O(1)$ space  
**Disadvantages**: Three passes through sequence

### **HashMap-Based Cycle Detection**

**Complexity**: $O(n)$ time, $O(m)$ space where $m =$ unique states

```rust
use std::collections::HashMap;

fn hashmap_detect_cycle<T: Hash + Eq + Clone>(
    f: impl Fn(&T) -> T, 
    x0: T
) -> Option<(usize, usize)> {
    let mut seen: HashMap<T, usize> = HashMap::new();
    let mut current = x0.clone();
    let mut index = 0;
    
    while let None = seen.get(&current) {
        seen.insert(current.clone(), index);
        current = f(&current);
        index += 1;
    }
    
    let cycle_start = seen[&current];
    let cycle_length = index - cycle_start;
    Some((cycle_start, cycle_length))
}
```

**Advantages**: Single pass, early detection, returns exact cycle parameters  
**Disadvantages**: $O(m)$ space for state storage

**Trade-off**: HashMap approach preferred when:
- State is easily hashable
- Space is available (modern hardware)
- Need exact cycle parameters immediately
- Want simpler implementation

---

## 💻 Rust Implementations

### **AoC 2023 Day 14 - Parabolic Reflector Dish**

**Problem**: Simulate 1 billion spin cycles on a grid with moving rocks.

**Mathematical Insight**: 
- State space: All possible grid configurations (finite)
- Process: Deterministic spin cycle transformation
- By Pigeonhole Principle: Grid configurations **must** repeat

**Implementation**:
```rust
// advent_of_code/aoc2023/src/solver/day14.rs

/// Solve Part 2 using cycle detection
pub fn solve_part2(input: &str) -> Result<String> {
    let mut grid = parse_input(input)?;
    
    // HashMap tracks: state → first occurrence index
    let mut seen: HashMap<String, usize> = HashMap::new();
    let target_cycles = 1_000_000_000;
    
    for cycle in 0..target_cycles {
        let state = grid_to_string(&grid); // Serialize state
        
        if let Some(&first_seen) = seen.get(&state) {
            // Cycle detected! Apply Pigeonhole Principle reasoning:
            // - Cycle starts at `first_seen`
            // - Cycle length = current_cycle - first_seen
            let cycle_length = cycle - first_seen;
            
            // Fast-forward: Where will we be after 1B iterations?
            let remaining = target_cycles - cycle;
            let final_offset = remaining % cycle_length;
            
            // Run only the remaining cycles
            for _ in 0..final_offset {
                spin_cycle(&mut grid);
            }
            
            return Ok(calculate_load(&grid).to_string());
        }
        
        seen.insert(state, cycle);
        spin_cycle(&mut grid);
    }
    
    Ok(calculate_load(&grid).to_string())
}
```

**Analysis**:
- **Without cycle detection**: Would require 1,000,000,000 iterations (intractable)
- **With cycle detection**: 
  - Cycle detected at iteration ~100-200 (varies by input)
  - Remaining computation: ~10-50 iterations
  - Total time: ~13ms instead of hours/days

**Performance Gain**: ~99.9999% reduction in iterations

### **AoC 2022 Day 17 - Pyroclastic Flow**

**Problem**: Simulate 1 trillion Tetris-like rocks falling in a 7-wide chamber.

**Mathematical Insight**:
- State = `(rock_type, jet_index % len, top_30_rows)` — finite state space
- By Pigeonhole Principle: Deterministic simulation must cycle
- Detection: HashMap fingerprint finds repeat within ~3,400 rocks
- Fast-forward: Skip `full_cycles × cycle_height`, simulate remainder only

**Performance**: ~400µs instead of impossible 10^12 iterations

**Link**: `advent_of_code/aoc2022/src/solver/day17.rs`

### **AoC 2023 Day 8 - Haunted Wasteland**

**Problem**: Simultaneous ghost paths on directed graph - find when all ghosts reach 'Z' nodes simultaneously.

**Mathematical Insight**:
- Each ghost follows deterministic path on finite graph → must cycle
- By Pigeonhole Principle: Each ghost's path enters a cycle
- Solution: Find LCM of all cycle lengths

### **AoC 2023 Day 20 - Pulse Propagation**

**Problem**: Digital logic circuit simulation - find when 4 counter modules simultaneously output high pulse to trigger final conjunction.

**Mathematical Insight**:
- Each counter module is a deterministic state machine on finite state space
- By Pigeonhole Principle: Each counter must have periodic behavior (cycle)
- Detection approach: Run simulation while tracking when each counter outputs high pulse
- Cycle detection finds period for each counter in ~4000-5000 iterations
- Solution: Compute LCM of all counter periods to find synchronization point

**Implementation**:
```rust
// advent_of_code/aoc2023/src/solver/day20.rs

/// Find cycle length for a specific counter module
fn find_cycle_length(modules: &HashMap<String, Module>, counter_name: &str) -> u64 {
    let mut modules = modules.clone();
    let mut presses = 0;
    
    // Track when counter sends high pulse
    let mut first_high: Option<u64> = None;
    
    loop {
        presses += 1;
        
        // Simulate button press, detect counter output
        if counter_sends_high_pulse(&mut modules, counter_name) {
            if let Some(first) = first_high {
                // Cycle detected! Period = current - first
                return presses - first;
            } else {
                first_high = Some(presses);
            }
        }
        
        // Pigeonhole guarantees cycle within finite iterations
        if presses > 100_000 {
            panic!("No cycle detected - violates Pigeonhole Principle!");
        }
    }
}

/// Part 2: Find when all counters align
pub fn part2(input: &str) -> u64 {
    let modules = parse_input(input);
    let counter_names = find_counter_modules(&modules);
    
    // Find cycle period for each counter
    let periods: Vec<u64> = counter_names.iter()
        .map(|name| find_cycle_length(&modules, name))
        .collect();
    
    // Synchronization point = LCM of all periods
    periods.iter().fold(1, |acc, &p| lcm(acc, p))
}
```

**Analysis**:
- **Without cycle detection**: Would require 238,920,142,622,879 button presses (impossible)
- **With cycle detection**:
  - 4 cycles detected in ~16,000-20,000 total iterations (4 × ~4000-5000 each)
  - LCM computation: O(1)
  - Total time: ~23.54ms
- **Performance Gain**: >99.99999999% reduction in iterations

**Connection to Day 8**: Identical mathematical pattern - multiple independent cycles must align, solved via LCM

**Link**: [advent_of_code/aoc2023/src/solver/day20.rs](advent_of_code/aoc2023/src/solver/day20.rs)

**See also**: [[state-machines]] for state machine implementation details

**Implementation**: See `advent_of_code/aoc2023/src/solver/day08.rs`

---

## 🎯 Common Patterns in Rust

### **Pattern 1**: HashMap State Tracking
```rust
let mut seen: HashMap<StateType, usize> = HashMap::new();

for iteration in 0..max_iterations {
    let state = compute_state();
    
    if let Some(&first_occurrence) = seen.get(&state) {
        // Cycle detected!
        let cycle_length = iteration - first_occurrence;
        // ... apply modular arithmetic ...
    }
    
    seen.insert(state, iteration);
}
```

**When to use**: 
- Finite state space
- Deterministic state transitions
- Need exact cycle parameters
- Memory available

### **Pattern 2**: Fast-Forward with Modulo
```rust
// After detecting cycle at iteration `cycle_start` with length `cycle_len`:
let remaining = target_iterations - current_iteration;
let offset_in_cycle = remaining % cycle_length;

// Simulate only `offset_in_cycle` more iterations instead of `remaining`
```

**Mathematical Justification**:
- If state repeats every `cycle_length` iterations
- Then state at iteration $n$ equals state at iteration $n + k \cdot \text{cycle\\_length}$ for any integer $k$
- Use modulo to find equivalent position within cycle

---

## 📊 Complexity Analysis

### **Space-Time Tradeoffs**

| Algorithm | Time | Space | Use When |
|-----------|------|-------|----------|
| **Brute Force** | O(target) | O(1) | Small target (<10⁶) |
| **Floyd's** | O(μ + λ) | O(1) | Large target, limited memory |
| **HashMap** | O(μ + λ) | O(m) | Large target, need exact params |

Where:
- $\mu$ = cycle start index (rho-length)
- $\lambda$ = cycle length
- $m$ = number of unique states

### **Expected Cycle Length**

For random mapping on $n$ states:
- Expected cycle length: $\sqrt{\pi n / 2}$ (Birthday Paradox)
- Expected pre-cycle length (rho): $\sqrt{\pi n / 8}$

**Example**: Grid with $10^6$ possible states
- Expected cycle length: ~1,253 iterations
- Expected detection: ~1,880 iterations total
- Compared to 1 billion: **99.9998% reduction**

---

## 🔍 Proof Techniques

### **Direct Application**

**Problem**: Show that among any 13 people, at least two were born in the same month.

**Proof**:
- Let $n = 13$ (people), $m = 12$ (months)
- By Pigeonhole Principle: Since $13 > 12$, at least one month contains $\lceil 13/12 \rceil = 2$ birthdays

### **Contrapositive**

**Problem**: Show that if no cycle exists in first $m$ states, state space has size $>m$.

**Proof**:
- Contrapositive: If state space has size $\leq m$, then cycle must exist within $m+1$ states
- By Pigeonhole Principle with $m+1$ items and $\leq m$ containers
- Therefore, original statement holds

---

## 🎓 Related Concepts

**In this repository**:
- [[graph-theory-fundamentals]] - Cycles in directed graphs
- [[modular-arithmetic]] - Used in fast-forward calculations
- [[set-theory-fundamentals]] - Finite set properties
- [[state-machines]] - Deterministic state transitions guarantee cycles
- [[number-theory-basics]] - LCM for cycle alignment (Day 8, Day 20)
- [[Dynamic Programming]] - Memoization prevents recomputation (related concept)

**Mathematical connections**:
- **Ramsey Theory**: Generalization of Pigeonhole Principle
- **Birthday Problem**: Probabilistic extension
- **Diophantine Equations**: Number-theoretic applications
- **Floyd's Algorithm**: Practical cycle detection

---

## 📚 Further Reading

1. **"Concrete Mathematics"** by Graham, Knuth, Patashnik - Chapter 1 (Recurrence Relations)
2. **"Introduction to Algorithms"** (CLRS) - Section on Floyd's Cycle Detection
3. **Pollard's Rho Algorithm** - Application to integer factorization
4. **Functional Iteration Dynamics** - Study of discrete dynamical systems

---

## ✅ Key Takeaways

1. **Finite + Deterministic = Guaranteed Cycle**: Any deterministic process on finite state space must cycle
2. **Detection before Simulation**: Find cycle parameters first, then fast-forward
3. **HashMap vs Floyd**: Trade space for simplicity and single-pass execution
4. **Modular Arithmetic**: Key to converting cycle detection into fast-forward
5. **Ubiquitous Pattern**: Appears in number theory, graph algorithms, simulations, hash functions

**Mental Model**: "If I have more items than containers, some container must have duplicates."

**Practical Heuristic**: For problems asking to simulate large number of iterations on finite state space, immediately consider cycle detection.

---

*Mathematical Foundation for*: AoC 2022 Day 17, AoC 2023 Day 8, AoC 2023 Day 14, AoC 2023 Day 20  
*Tags*: #discrete-math #combinatorics #algorithms #cycle-detection #pigeonhole-principle  
*Related*: [[linear-feedback-shift-registers]], [[graph-theory-fundamentals]], [[modular-arithmetic]], [[state-machines]], [[number-theory-basics]], [[Dynamic Programming]]
