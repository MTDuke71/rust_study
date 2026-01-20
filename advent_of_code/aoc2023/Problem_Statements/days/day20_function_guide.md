# Day 20: Pulse Propagation - Function-by-Function Guide

**Problem**: Simulate a network of interconnected digital logic modules (flip-flops, conjunctions, broadcaster) processing high/low pulses in FIFO order.

**Key Insights**: 
- Part 1: State machine simulation with queue-based event processing
- Part 2: Cycle detection + LCM - identical pattern to Day 8 ghost synchronization!

---

## 📋 Overview

### Problem Summary
- **Part 1**: Simulate 1000 button presses → Count high/low pulses sent (Answer: 712,543,680)
- **Part 2**: Find minimum button presses to send low pulse to `rx` → 238 trillion presses via LCM (Answer: 238,920,142,622,879)

### Mathematical Foundation
- **Graph Theory**: Directed graph with typed edges (high/low pulses)
- **State Machine**: Each module type has distinct state transitions
- **Number Theory**: LCM for cycle synchronization (see `math_utils.rs`)
- **Queue Theory**: FIFO processing ensures correct event ordering

### Key Algorithms
1. **State Machine Simulation**: Enum dispatch for module type behavior
2. **FIFO Queue Processing**: VecDeque for pulse propagation (critical for correctness!)
3. **Cycle Detection**: Identify when independent counters send high pulses
4. **LCM Synchronization**: Find when multiple cycles align (same as Day 8)

### Performance
```
Benchmark Results (Criterion):
- Part 1: 5.70 ms (1000 simulations)
- Part 2: 23.54 ms (cycle detection ~4000-5000 iterations)
```

---

## 🏗️ Type Definitions

### `Pulse` - Binary Signal Type
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pulse {
    Low,
    High,
}
```

**Design Decision**: Simple enum for type safety. Binary signals map directly to digital logic.

**Usage**: Pattern match in module processing, equality checks in cycle detection.

---

### `Module` - Polymorphic State Machine Components

```rust
#[derive(Debug, Clone)]
pub enum Module {
    FlipFlop { 
        on: bool, 
        destinations: Vec<String> 
    },
    Conjunction {
        memory: HashMap<String, Pulse>,
        destinations: Vec<String>,
    },
    Broadcaster { 
        destinations: Vec<String> 
    },
}
```

**Design Decision**: Enum allows type-safe dispatch. Each variant encapsulates its state and behavior.

**State Representation**:
- **FlipFlop**: `on: bool` - Toggle state (off → on → off)
- **Conjunction**: `memory: HashMap<String, Pulse>` - Tracks last pulse from each input
- **Broadcaster**: Stateless relay

**Why Enum?**: 
- Compiler ensures all cases handled in `process()` method
- Each type has different state - struct wouldn't work
- Zero-cost abstraction (no vtable like trait objects)

---

## 🔧 Core Functions

### `Module::destinations()` - Get Module Outputs
```rust
fn destinations(&self) -> &[String] {
    match self {
        Module::FlipFlop { destinations, .. } => destinations,
        Module::Conjunction { destinations, .. } => destinations,
        Module::Broadcaster { destinations } => destinations,
    }
}
```

**Purpose**: Uniform access to destinations regardless of module type.

**Design**: Pattern match extracts destinations slice. Lifetime tied to Module.

**Why?**: Avoids duplication when sending pulses to all destinations.

---

### `Module::process()` - State Transition Logic
```rust
fn process(&mut self, from: &str, pulse: Pulse) -> Option<Pulse> {
    match self {
        Module::FlipFlop { on, .. } => {
            if pulse == Pulse::High {
                None  // Ignore high pulses
            } else {
                *on = !*on;  // Flip state
                Some(if *on { Pulse::High } else { Pulse::Low })
            }
        }
        Module::Conjunction { memory, .. } => {
            memory.insert(from.to_string(), pulse);  // Update memory
            let all_high = memory.values().all(|&p| p == Pulse::High);
            Some(if all_high { Pulse::Low } else { Pulse::High })
        }
        Module::Broadcaster { .. } => Some(pulse),  // Relay
    }
}
```

**Purpose**: Execute state transition, return outgoing pulse if any.

**Critical Details**:
- **FlipFlop**: Returns `None` for high pulse (ignored), `Some(pulse)` after flip
- **Conjunction**: Always returns `Some(_)` - sends low if all inputs high, else high
- **Broadcaster**: Always returns `Some(pulse)` - simple relay

**Why `Option<Pulse>`?**: Flip-flops ignore high pulses (no output) - `None` prevents spurious events.

**State Mutation**: Takes `&mut self` - updates `on` state or `memory` HashMap.

---

### `parse_module()` - Input Parsing
```rust
fn parse_module(line: &str) -> (String, Module) {
    let (name_part, dest_part) = line.split_once(" -> ").expect("invalid format");
    
    let destinations: Vec<String> = dest_part
        .split(", ")
        .map(|s| s.to_string())
        .collect();

    if name_part == "broadcaster" {
        ("broadcaster".to_string(), Module::Broadcaster { destinations })
    } else if let Some(name) = name_part.strip_prefix('%') {
        (name.to_string(), Module::FlipFlop { on: false, destinations })
    } else if let Some(name) = name_part.strip_prefix('&') {
        (name.to_string(), Module::Conjunction {
            memory: HashMap::new(),  // Initialized later
            destinations,
        })
    } else {
        panic!("Unknown module type: {}", name_part);
    }
}
```

**Purpose**: Convert input line to (name, Module) tuple.

**Format Recognition**:
- `broadcaster -> a, b` → Broadcaster module
- `%flipflop -> x` → FlipFlop (prefix `%`)
- `&conjunction -> y` → Conjunction (prefix `&`)

**Initialization**: Flip-flops start off, conjunctions get empty memory (populated by `parse_input`).

---

### `parse_input()` - Build Complete Network
```rust
fn parse_input(input: &str) -> HashMap<String, Module> {
    let mut modules: HashMap<String, Module> = input
        .lines()
        .map(|line| parse_module(line.trim()))
        .collect();

    // Build reverse mapping: track what modules send to each module
    let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
    for (name, module) in &modules {
        for dest in module.destinations() {
            inputs
                .entry(dest.clone())
                .or_insert_with(Vec::new)
                .push(name.clone());
        }
    }

    // Initialize conjunction memory with all inputs (defaulting to Low)
    for (name, module) in modules.iter_mut() {
        if let Module::Conjunction { memory, .. } = module {
            if let Some(input_names) = inputs.get(name) {
                for input_name in input_names {
                    memory.insert(input_name.clone(), Pulse::Low);
                }
            }
        }
    }

    modules
}
```

**Purpose**: Parse all modules and initialize conjunction memory.

**Two-Pass Approach**:
1. Parse all modules (conjunctions have empty memory)
2. Build reverse mapping (who sends to whom?)
3. Initialize conjunction memory with all input sources

**Why Two Passes?**: Conjunctions need to know their inputs, but inputs aren't declared in the module's own line. Must scan entire network to build dependency graph.

**HashMap Choice**: O(1) module lookup by name during simulation.

---

## 🎯 Part 1 Implementation

### `part1()` - Pulse Counting Simulation
```rust
pub fn part1(input: &str) -> u64 {
    let mut modules = parse_input(input);
    
    let mut low_count = 0u64;
    let mut high_count = 0u64;

    // Press button 1000 times
    for _ in 0..1000 {
        let mut queue: VecDeque<(String, String, Pulse)> = VecDeque::new();
        
        // Button press sends low pulse to broadcaster
        queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));
        low_count += 1;

        // Process all pulses in FIFO order
        while let Some((from, to, pulse)) = queue.pop_front() {
            if let Some(module) = modules.get_mut(&to) {
                if let Some(output_pulse) = module.process(&from, pulse) {
                    for dest in module.destinations().to_vec() {
                        queue.push_back((to.clone(), dest, output_pulse));
                        match output_pulse {
                            Pulse::Low => low_count += 1,
                            Pulse::High => high_count += 1,
                        }
                    }
                }
            }
        }
    }

    low_count * high_count
}
```

**Algorithm**:
1. Initialize network (parsing + conjunction memory setup)
2. For each of 1000 button presses:
   - Create empty queue
   - Add button→broadcaster low pulse (count it)
   - Process queue until empty (FIFO critical!)
   - Count each pulse sent

**FIFO Queue Processing**: `VecDeque::pop_front()` ensures pulses processed in send order.

**Why FIFO Matters**: Example from problem - if A sends to B and C, B's response must wait until C receives its pulse. Recursive processing would be incorrect!

**Pulse Counting**: Count when sending (push to queue), not when receiving.

**Edge Case**: Non-existent modules (like `output` in examples) are silently ignored (`if let Some(...)`).

---

## 🎯 Part 2 Implementation

### `part2()` - Cycle Detection & LCM
```rust
pub fn part2(input: &str) -> u64 {
    let mut modules = parse_input(input);
    
    // Find module feeding rx
    let rx_input = modules
        .iter()
        .find(|(_, module)| module.destinations().contains(&"rx".to_string()))
        .map(|(name, _)| name.clone())
        .expect("No module feeds rx");

    // Get inputs to rx feeder (must all be HIGH for rx to get LOW)
    let rx_feeder_inputs: Vec<String> = modules
        .iter()
        .filter(|(_, module)| module.destinations().contains(&rx_input))
        .map(|(name, _)| name.clone())
        .collect();

    // Track cycle length for each input
    let mut cycle_lengths: HashMap<String, u64> = HashMap::new();
    let mut button_presses = 0u64;

    // Keep pressing until all cycles found
    while cycle_lengths.len() < rx_feeder_inputs.len() {
        button_presses += 1;
        
        let mut queue: VecDeque<(String, String, Pulse)> = VecDeque::new();
        queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

        while let Some((from, to, pulse)) = queue.pop_front() {
            // Detect when rx_feeder inputs send HIGH
            if to == rx_input && pulse == Pulse::High && !cycle_lengths.contains_key(&from) {
                cycle_lengths.insert(from.clone(), button_presses);
            }

            if let Some(module) = modules.get_mut(&to) {
                if let Some(output_pulse) = module.process(&from, pulse) {
                    for dest in module.destinations().to_vec() {
                        queue.push_back((to.clone(), dest, output_pulse));
                    }
                }
            }
        }
    }

    // LCM of all cycle lengths
    cycle_lengths.values().copied().reduce(lcm).unwrap_or(0)
}
```

**Problem Analysis**: `rx` receives from conjunction `vr`. For `vr` to send LOW to `rx`, ALL inputs to `vr` must be HIGH simultaneously.

**Circuit Structure** (from puzzle input):
```
&pq → vr → rx
&fg → vr
&dk → vr
&fm → vr
```

**Strategy**: Each input to `vr` is an independent counter circuit with period P. They align at LCM(P1, P2, P3, P4).

**Algorithm**:
1. Find module feeding `rx` (in this case `vr`)
2. Find all modules feeding into `vr` (4 conjunction modules)
3. Simulate button presses, detect when each feeder sends HIGH pulse
4. Record button press number (cycle period)
5. Once all 4 periods found, compute LCM

**Why LCM?**: See Day 8 - identical pattern! Independent cycles synchronize at LCM.

**Cycle Detection**: Track first HIGH pulse from each input. Assumption: period starts at first occurrence (validated by puzzle design).

**Mathematical Insight**: 238 trillion button presses would timeout. LCM gives answer in ~4000-5000 iterations!

---

## 🔗 Shared Utilities

### `math_utils::lcm()` - Least Common Multiple
```rust
// From crate::math_utils
pub fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
```

**Purpose**: Find synchronization point of independent cycles.

**Used In**:
- **Day 8**: Ghost path synchronization (6 ghosts → LCM of 6 periods)
- **Day 20**: Counter circuit alignment (4 counters → LCM of 4 periods)

**Algorithm**: Euclidean algorithm for GCD, then `LCM(a,b) = a×b / GCD(a,b)`.

**See**: `advent_of_code/aoc2023/src/math_utils.rs` for full implementation and tests.

---

## 🧪 Testing Strategy

### Example Tests
```rust
#[test]
fn test_example1() {
    let input = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    assert_eq!(part1(input), 32000000);  // 8000 low × 4000 high
}

#[test]
fn test_example2() {
    let input = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    assert_eq!(part1(input), 11687500);  // 4250 low × 2750 high
}
```

**Coverage**:
- Example 1: Simple cycle (flip-flops → conjunction → loop back)
- Example 2: Multi-output flip-flop, multi-input conjunction

**What to Test**:
- Flip-flop state toggling (low pulse flips, high pulse ignored)
- Conjunction memory updates (tracks all inputs)
- Pulse counting accuracy
- Queue processing order (FIFO)

---

## 🎨 Design Patterns

### 1. **State Machine with Enum Dispatch**
- Each `Module` variant has type-specific behavior
- Compiler enforces exhaustive matching
- Zero-cost abstraction (no vtable overhead)

### 2. **Queue-Based Event Processing**
- FIFO ensures correct event ordering
- Prevents incorrect recursive propagation
- Natural fit for digital logic simulation

### 3. **Two-Pass Initialization**
- First pass: Parse modules
- Second pass: Wire up dependencies (conjunction inputs)
- Required when dependencies are implicit in input

### 4. **Cycle Detection for Optimization**
- Recognize when brute force is impossible (238 trillion iterations)
- Identify independent periodic components
- Apply number theory (LCM) for synchronization

### 5. **Code Reuse via Utility Modules**
- Extract common patterns (GCD/LCM)
- Shared between Day 8 and Day 20
- Comprehensive tests ensure correctness

---

## 💡 Key Insights

### Digital Logic Simulation
- **Flip-flops**: Binary memory (on/off state)
- **Conjunctions**: NAND-like behavior (output low only when ALL inputs high)
- **Broadcaster**: Signal splitter (1-to-many fanout)

### Queue Processing is Critical
- Problem explicitly states: "process in order they are sent"
- Recursive DFS would give wrong results
- BFS with queue matches problem semantics exactly

### Pattern Recognition: Day 8 = Day 20
Both are **cycle synchronization problems**:
- Day 8: Multiple ghost paths reach Z nodes
- Day 20: Multiple counters send high pulses
- Solution: Find cycle periods, compute LCM

### Circuit Analysis
The puzzle input is a **binary counter circuit**:
- 4 independent counter chains
- Each feeds into final conjunction `vr`
- `vr` outputs LOW only when all counters align
- Classic synchronization problem from digital design

---

## 🚀 Optimization Opportunities

### Current Performance
- Part 1: 5.70 ms (acceptable for 1000 simulations)
- Part 2: 23.54 ms (cycle detection in ~4000 iterations)

### Potential Improvements
1. **String Interning**: Use `&str` or integer IDs instead of `String` keys
2. **State Packing**: Flip-flop state fits in 1 bit, could pack into bitset
3. **Conjunction Optimization**: Use bitset for memory (4 inputs = 4 bits)
4. **Queue Preallocation**: Reuse VecDeque across button presses

### Trade-offs
- **Readability vs Speed**: Current code prioritizes clarity
- **Memory vs Time**: String keys use more memory but HashMap is fast
- **Engineering Time**: Optimizations unlikely worth it for <30ms runtime

**Verdict**: Current implementation is clean, correct, and fast enough. Don't optimize prematurely!

---

## 📚 References

**Related Days**:
- **Day 8**: Ghost path synchronization (LCM pattern)
- See `day08_function_guide.md` for cycle detection details

**Mission Integration**:
- **Mission 2**: Queue (FIFO processing)
- **Mission 5**: HashMap (module lookup)
- **Mission 8**: Graph algorithms (conceptual - directed graph)

**Zettelkasten Links**:
- `[[state-machine-rust]]` - Enum-based state machines
- `[[cycle-detection]]` - Period finding in iterative systems
- `[[mission-2]]` - Queue data structure
- `[[math-foundations/number-theory-basics]]` - GCD/LCM theory

**Mathematical Foundation**:
- See `zettelkasten/math-foundations/number-theory-basics.md` for GCD/LCM theory
- See `advent_of_code/aoc2023/src/math_utils.rs` for implementation

---

## 🎯 Complexity Analysis

### Part 1
- **Time**: O(N × M × D) where:
  - N = button presses (1000)
  - M = average queue size per press (~50-100)
  - D = average destinations per module (~2-3)
  - Total: ~5-6ms for real input
- **Space**: O(K) for K modules in HashMap

### Part 2
- **Time**: O(P × M × D) where P = max cycle period (~4000-5000)
- **Space**: O(K) for modules, O(C) for cycle tracking (C = 4 in this input)

### Overall
- Linear in number of modules
- Linear in simulation iterations
- Constant per pulse processed (O(1) HashMap lookup)

---

**Navigation**: [← Day 19](day19_function_guide.md) | [Day 21 →](day21_function_guide.md)
