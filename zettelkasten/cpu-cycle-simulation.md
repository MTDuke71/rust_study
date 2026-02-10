# CPU Cycle Simulation: Modeling Instruction Timing and Execution

*Tags: #cpu #simulation #cycles #emulation #instruction-timing #register-semantics #timing-models*  
*Links: [[zettel-index]] | [[daily-study/Day10]] | [[aoc-2022-summary]] | [[Simulation Patterns]] | [[State Machines]]*

---

## 🎯 Overview

**CPU Cycle Simulation** models how a processor executes instructions over time, tracking cycles, register states, and timing-dependent behavior. Critical for emulators, interpreters, and puzzles like AoC 2022 Day 10.

**Key concept**: Instructions take **time** (cycles) to execute, and effects (register updates, memory writes) happen at **specific points** during or after execution.

**Real-world examples**:
- Chip 8 emulator (8-bit CPU, display timing)
- AoC 2022 Day 10 (simplified CPU with X register)
- Game Boy emulators (Z80-based timing)
- NES emulators (6502 timing)

---

## 📚 Core Concepts

### 1. Cycle-Based Execution

**Definition**: Each instruction consumes a fixed number of clock cycles before completing.

```rust
enum Instruction {
    Noop,        // 1 cycle
    Addx(i32),   // 2 cycles
    Jump(u16),   // 3 cycles (hypothetical)
}

impl Instruction {
    fn cycles(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
            Instruction::Jump(_) => 3,
        }
    }
}
```

**Why this matters**:
- Real CPUs aren't instantaneous
- Multi-cycle instructions create timing dependencies
- Synchronization with external systems (displays, timers)

---

### 2. Register Update Semantics

**Critical distinction**: When do register values change?

#### Option A: During Execution (Immediate)
```rust
// ❌ WRONG for most CPU models
for _ in 0..instruction.cycles() {
    cycle += 1;
    x += value;  // Updates during each cycle (incorrect!)
}
```

#### Option B: After Execution (Deferred)
```rust
// ✅ CORRECT for Day 10 and most real CPUs
for _ in 0..instruction.cycles() {
    cycle += 1;
    // X still holds OLD value during execution
}
// NOW update X after instruction completes
x += value;
```

**Example trace** (AoC Day 10):
```
Start: X = 1

Execute "addx 15" (2 cycles):
  Cycle 1: Executing... X still = 1
  Cycle 2: Executing... X still = 1
  After cycle 2: X becomes 1 + 15 = 16

Execute "noop" (1 cycle):
  Cycle 3: Executing... X still = 16
  After cycle 3: X still = 16 (noop has no effect)

Execute "addx -5" (2 cycles):
  Cycle 4: Executing... X still = 16
  Cycle 5: Executing... X still = 16
  After cycle 5: X becomes 16 - 5 = 11
```

**Key insight**: During instruction execution, previous state persists. Updates happen atomically after completion.

---

### 3. Timing-Dependent Observations

**Problem**: How do we observe state at specific cycles?

**Solution**: Sample state **during** cycle execution, not after.

```rust
pub fn simulate(instructions: &[Instruction]) -> Vec<i32> {
    let mut x = 1;
    let mut cycle = 0;
    let mut observations = Vec::new();
    
    for instruction in instructions {
        for _ in 0..instruction.cycles() {
            cycle += 1;
            
            // Observe DURING cycle (current X value)
            if cycle % 40 == 20 {
                observations.push(cycle * x);  // Signal strength
            }
        }
        
        // Update AFTER all cycles
        if let Instruction::Addx(value) = instruction {
            x += value;
        }
    }
    
    observations
}
```

**Why this pattern**:
- External observers (CRT, timers) see current state
- Register updates happen atomically between cycles
- Matches real hardware behavior

---

## 🖥️ Real-World Example: Chip 8

### Chip 8 Architecture
- **Registers**: V0-VF (16 general-purpose, 8-bit)
- **Special registers**: I (address), PC (program counter), SP (stack pointer)
- **Timers**: Delay timer, sound timer (count down at 60Hz)
- **Display**: 64×32 monochrome pixels

### Cycle Timing Challenges

#### Challenge 1: Display Refresh (60Hz)
```rust
// Chip 8 display refreshes at 60Hz independent of CPU speed
const REFRESH_RATE: f64 = 60.0;  // Hz
const REFRESH_INTERVAL: f64 = 1.0 / REFRESH_RATE;  // ~16.7ms

// CPU can run much faster than display!
// Need to synchronize:
fn main_loop() {
    let mut last_refresh = Instant::now();
    
    loop {
        // Execute instructions at CPU speed
        cpu.execute_instruction();
        
        // Refresh display at fixed 60Hz
        if last_refresh.elapsed() >= Duration::from_secs_f64(REFRESH_INTERVAL) {
            display.refresh();
            last_refresh = Instant::now();
        }
    }
}
```

**This was the hardest part**: Balancing variable CPU speed with fixed display timing.

#### Challenge 2: Timer Decrement
```rust
// Timers decrement at 60Hz, not per instruction
if timer_tick() {  // Called 60 times per second
    if delay_timer > 0 {
        delay_timer -= 1;
    }
    if sound_timer > 0 {
        sound_timer -= 1;
        beep();  // Play sound while sound_timer > 0
    }
}
```

#### Challenge 3: Cycle-Accurate vs. Instruction-Accurate
- **Cycle-accurate**: Track every CPU cycle (complex, slow)
- **Instruction-accurate**: Track per instruction (simpler, usually sufficient)

**Decision for Chip 8**: Instruction-accurate was sufficient. Most games don't rely on precise cycle counts.

---

## 🎮 AoC 2022 Day 10: Simplified Model

Day 10 is a **teaching example** of CPU simulation with minimal complexity:

### Simplifications
1. **No memory**: Only one register (X)
2. **No branching**: Instructions execute sequentially
3. **No I/O delays**: Immediate effects
4. **Deterministic timing**: Fixed 240 cycles total
5. **No real-time**: Offline simulation, no 60Hz sync

### What It Teaches
✅ **Instruction timing**: Multi-cycle execution  
✅ **Register updates**: After completion, not during  
✅ **State observation**: Sampling during cycles  
✅ **Synchronization**: CRT draws in lockstep with CPU  

❌ **Not covered**: Branching, memory, interrupts, real-time constraints

### Day 10 Implementation Pattern
```rust
pub fn simulate_cpu(instructions: &[Instruction]) -> String {
    let mut x = 1i32;       // Register X
    let mut cycle = 0;       // Cycle counter
    let mut output = String::new();
    
    for instruction in instructions {
        // Execute over multiple cycles
        for _ in 0..instruction.cycles() {
            // During cycle: observe current state
            let crt_col = (cycle % 40) as i32;
            if (x - 1..=x + 1).contains(&crt_col) {
                output.push('#');
            } else {
                output.push('.');
            }
            
            cycle += 1;
            
            if cycle % 40 == 0 {
                output.push('\n');
            }
        }
        
        // After cycles: update state
        if let Instruction::Addx(value) = instruction {
            x += value;
        }
    }
    
    output
}
```

**Pattern**: `for cycle in execution { observe(); } then update();`

---

## 🔍 Common Patterns and Pitfalls

### ✅ Correct Pattern: Observe → Update
```rust
// During execution: observe with OLD state
for _ in 0..cycles {
    cycle_count += 1;
    observe_state(register);  // Uses current value
}

// After execution: update to NEW state
register += delta;
```

### ❌ Pitfall 1: Update Too Early
```rust
// WRONG: Updates during execution
for _ in 0..cycles {
    register += delta;  // ← Value changes mid-instruction!
    cycle_count += 1;
    observe_state(register);  // ← Observing wrong value
}
```

### ❌ Pitfall 2: Update After Each Cycle
```rust
// WRONG: Partial updates per cycle
for _ in 0..cycles {
    register += delta / cycles;  // ← Incorrect semantics
    observe_state(register);
}
```

### ❌ Pitfall 3: Observe After Update
```rust
// WRONG: Observer sees future state
for _ in 0..cycles {
    cycle_count += 1;
}
register += delta;  // ← Updates here
observe_state(register);  // ← Too late! Should observe during cycles
```

**Mnemonic**: **D.U.A.** — **D**uring (observe), **U**pdate (after), **A**tomic (all at once)

---

## 🎯 Design Principles

### 1. Separation of Concerns
```rust
// Parse instructions separately from execution
let instructions = parse_instructions(input);
let result = execute_cpu(&instructions);
```

### 2. Pure Functions for Cycles
```rust
// Cycle logic shouldn't have side effects
fn execute_cycle(state: &CpuState) -> Observation {
    // Read-only observation of current state
    observe(state)
}
```

### 3. Explicit State Transitions
```rust
// Make state changes obvious
fn update_state(state: &mut CpuState, instruction: &Instruction) {
    match instruction {
        Instruction::Addx(val) => state.x += val,
        Instruction::Noop => { /* no change */ }
    }
}
```

### 4. Testable Components
```rust
#[test]
fn test_instruction_timing() {
    // Verify cycle counts
    assert_eq!(Instruction::Noop.cycles(), 1);
    assert_eq!(Instruction::Addx(5).cycles(), 2);
}

#[test]
fn test_register_update_after_completion() {
    let mut cpu = Cpu::new();
    assert_eq!(cpu.x, 1);  // Initial state
    
    cpu.execute(Instruction::Addx(15));
    // After 2 cycles, X should be 16
    assert_eq!(cpu.x, 16);
}
```

---

## 🚀 Advanced Topics

### Multi-Register CPUs
```rust
struct CpuState {
    registers: [u8; 16],  // V0-VF for Chip 8
    i: u16,               // Address register
    pc: u16,              // Program counter
    sp: u8,               // Stack pointer
}
```

### Pipeline Simulation
```rust
// Fetch → Decode → Execute pipeline
struct Pipeline {
    fetch: Option<u16>,    // Fetched address
    decode: Option<Instruction>,
    execute: Option<Instruction>,
}
```

### Interrupt Handling
```rust
// Interrupts can occur mid-instruction
if interrupt_pending() {
    save_state();
    jump_to_interrupt_handler();
}
```

---

## 📊 Performance Considerations

### Cycle Tracking Overhead
```rust
// Minimal: Just integer increment per cycle
cycle += 1;  // ~1 CPU cycle (nanoseconds)
```

### State Observation Cost
```rust
// AoC Day 10: String building dominates
output.push('#');  // Heap allocation if grows
```

### Optimization Strategies
1. **Pre-allocate buffers**: `String::with_capacity(240)`
2. **Avoid unnecessary checks**: Only sample at target cycles
3. **Batch updates**: Collect observations, process later
4. **SIMD**: For parallel cycle simulation (advanced)

---

## 🔗 Related Concepts

### Emulation vs. Simulation
- **Emulation**: Bit-accurate, cycle-accurate, aims for 100% compatibility
- **Simulation**: Functional accuracy, timing accuracy, educational/testing purposes

### Timing Models
- **Cycle-accurate**: Every CPU cycle modeled (complex)
- **Instruction-accurate**: Per-instruction timing (sufficient for most)
- **Functional**: No timing, just effects (fastest, least accurate)

### State Machines
CPU simulation is fundamentally a **finite state machine**:
- States: Register values, memory contents
- Transitions: Instruction execution
- Observations: Read state during transitions

---

## 📝 Summary

**Core Concept**: CPUs execute instructions over time (cycles), and effects happen at specific points.

**Key Patterns**:
1. **Instruction timing**: Each instruction has fixed cycle count
2. **Register semantics**: Update AFTER execution, not during
3. **State observation**: Sample DURING cycles, see current state
4. **Synchronization**: External systems (display, timers) sync with cycles

**Common Models**:
- **Simple**: AoC Day 10 (1 register, sequential, offline)
- **Moderate**: Chip 8 (16 registers, timers, 60Hz display)
- **Complex**: NES/Game Boy (interrupts, memory-mapped I/O, DMA)

**Design Principles**:
- Separate parsing from execution
- Make state transitions explicit
- Test timing and state updates independently
- Choose accuracy level appropriate to use case

**When to use**:
- Emulator development (Chip 8, NES, Game Boy)
- AoC CPU simulation puzzles
- Educational CPU models
- Performance analysis (cycle counting)

---

*Related Links: [[2026-02-10]] | [[State Machines]] | [[Simulation Patterns]] | [[parse-once-pattern]] | [[aoc-optimization-patterns]]*

*Backlinks: [[daily-study/Day10]] | [[Chip 8 Emulator Project]]*
