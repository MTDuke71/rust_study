# State Machines

**Field**: Discrete Mathematics / Automata Theory / Computational Theory

**Prerequisites**: [[set-theory-fundamentals]], [[graph-theory-fundamentals]]

---

## 📐 Definition

A **finite state machine (FSM)** is a mathematical model of computation consisting of:

1. **Finite set of states** $S = \{s_0, s_1, \ldots, s_n\}$
2. **Initial state** $s_0 \in S$
3. **Set of inputs** (alphabet) $\Sigma$
4. **Transition function** $\delta: S \times \Sigma \to S$
5. **Set of accept/final states** $F \subseteq S$ (for acceptors)

**Intuition**: A system that can be in one of a finite number of states, transitioning between states based on inputs/events. The current state determines how the system responds to the next input.

---

## 🔑 Key Properties

### **Deterministic vs Non-Deterministic**
- **Deterministic FSM (DFA)**: Each state + input has exactly one next state
- **Non-Deterministic FSM (NFA)**: Each state + input may have multiple possible next states
- **Conversion**: Any NFA can be converted to an equivalent DFA (potentially exponentially larger)

### **Mealy vs Moore Machines**
- **Mealy Machine**: Outputs depend on current state AND input (transitions labeled with outputs)
- **Moore Machine**: Outputs depend ONLY on current state (states labeled with outputs)
- **Equivalence**: Both models have same computational power

### **State Encapsulation**
Each state can carry internal data:
- **FlipFlop state**: On/off boolean
- **Conjunction state**: Memory of input pulses
- **Counter state**: Numeric accumulator

### **Event-Driven Execution**
State machines react to discrete events:
- Process events in order (often FIFO queue)
- Each event triggers state transition
- Transitions may generate new events (cascading)

---

## 🎯 Common Patterns

### **Pattern 1: Enum-Based State Machines in Rust**

Rust's algebraic data types (enums) are perfect for state machines:

```rust
/// State machine using enum variants
pub enum Module {
    FlipFlop { 
        on: bool,                           // Internal state
        destinations: Vec<String>           // Transition targets
    },
    Conjunction { 
        memory: HashMap<String, Pulse>,     // Complex state
        destinations: Vec<String> 
    },
    Broadcaster { 
        destinations: Vec<String>           // Stateless relay
    },
}

impl Module {
    /// Transition function: (state, input) → (new_state, output)
    fn process(&mut self, from: &str, pulse: Pulse) -> Option<Pulse> {
        match self {
            Module::FlipFlop { on, .. } => {
                // FlipFlop: ignore high pulses, toggle on low
                if pulse == Pulse::Low {
                    *on = !*on;
                    Some(if *on { Pulse::High } else { Pulse::Low })
                } else {
                    None  // High pulses ignored
                }
            }
            Module::Conjunction { memory, .. } => {
                // Conjunction: remember all inputs, send low if all high
                memory.insert(from.to_string(), pulse);
                if memory.values().all(|&p| p == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            Module::Broadcaster { .. } => {
                // Broadcaster: stateless relay
                Some(pulse)
            }
        }
    }
}
```

**Advantages**:
- ✅ **Type safety**: Compiler ensures exhaustive pattern matching
- ✅ **State encapsulation**: Each variant holds its own data
- ✅ **Zero-cost abstraction**: No runtime overhead vs. manual dispatch
- ✅ **Clarity**: State transitions explicit in code

### **Pattern 2: FIFO Event Queue Processing**

State machines often use event queues for cascading transitions:

```rust
use std::collections::VecDeque;

/// Process events in FIFO order
fn simulate_button_press(modules: &mut HashMap<String, Module>) -> (u64, u64) {
    let mut queue: VecDeque<Event> = VecDeque::new();
    let mut low_count = 0;
    let mut high_count = 0;
    
    // Initial event
    queue.push_back(Event { 
        from: "button".to_string(), 
        to: "broadcaster".to_string(), 
        pulse: Pulse::Low 
    });
    
    // Process until queue empty
    while let Some(event) = queue.pop_front() {
        // Count pulses
        match event.pulse {
            Pulse::Low => low_count += 1,
            Pulse::High => high_count += 1,
        }
        
        // Process event (state transition)
        if let Some(module) = modules.get_mut(&event.to) {
            if let Some(output_pulse) = module.process(&event.from, event.pulse) {
                // Transition may generate new events (cascading)
                for dest in module.destinations() {
                    queue.push_back(Event {
                        from: event.to.clone(),
                        to: dest.clone(),
                        pulse: output_pulse,
                    });
                }
            }
        }
    }
    
    (low_count, high_count)
}
```

**Key Insight**: FIFO queue ensures events are processed in arrival order, modeling real-world signal propagation in digital circuits.

### **Pattern 3: State Space Analysis**

For complex systems, analyze state machine structure:

```rust
/// Analyze circuit structure to identify independent components
fn analyze_circuit(modules: &HashMap<String, Module>) -> Vec<String> {
    // Find final conjunction feeding target output
    let conjunction_inputs = modules.iter()
        .filter(|(name, module)| {
            matches!(module, Module::Conjunction { .. }) 
            && module.destinations().contains(&"rx".to_string())
        })
        .flat_map(|(_, module)| {
            // Get inputs to this conjunction
            module.memory_keys()
        })
        .collect();
    
    conjunction_inputs
}
```

**Application**: Instead of simulating 238 trillion steps, identify that 4 independent counters feed a conjunction, find each counter's period, compute LCM.

---

## 💻 Rust Implementations

### **AoC 2023 Day 20 - Pulse Propagation**

**Problem**: Digital logic circuit simulation with flip-flops, conjunctions, and broadcasters.

**What**: Simulate button presses sending pulses through interconnected modules
- **Part 1**: Count high/low pulses in 1000 button presses
- **Part 2**: Find when specific output receives low pulse (238 trillion iterations)

**Mathematical Foundation**:
- **State machine**: Each module is a state machine with transition rules
- **Event-driven**: Pulses are events processed in FIFO order
- **Cycle detection**: Identify periodic behavior in counter modules
- **Number theory**: Use LCM to find when cycles align

**Implementation Highlights**:
```rust
// advent_of_code/aoc2023/src/solver/day20.rs

/// Enum captures three state machine types
pub enum Module {
    FlipFlop { on: bool, destinations: Vec<String> },
    Conjunction { memory: HashMap<String, Pulse>, destinations: Vec<String> },
    Broadcaster { destinations: Vec<String> },
}

/// Part 1: Simulate 1000 iterations, count pulses
pub fn part1(input: &str) -> u64 {
    let mut modules = parse_input(input);
    let (total_low, total_high) = (0..1000)
        .map(|_| simulate_button_press(&mut modules))
        .fold((0, 0), |(l1, h1), (l2, h2)| (l1 + l2, h1 + h2));
    total_low * total_high
}

/// Part 2: Cycle detection + LCM for synchronization
pub fn part2(input: &str) -> u64 {
    let modules = parse_input(input);
    
    // Analyze structure: 4 counters → conjunction → rx
    let counter_names = find_counter_modules(&modules);
    
    // Find cycle period for each counter
    let periods: Vec<u64> = counter_names.iter()
        .map(|name| find_cycle_length(&modules, name))
        .collect();
    
    // Answer: When do all cycles align? → LCM
    periods.iter().fold(1, |acc, &p| lcm(acc, p))
}
```

**Performance**: 
- Part 1: 5.70ms (straightforward simulation)
- Part 2: 23.54ms (cycle detection avoids 238T iterations)

**Connections**:
- Uses [[number-theory-basics]] for GCD/LCM
- Uses [[pigeonhole-principle-cycle-detection]] for cycle detection
- Demonstrates Rust enum dispatch for polymorphic state machines

**Link**: [advent_of_code/aoc2023/src/solver/day20.rs](advent_of_code/aoc2023/src/solver/day20.rs)

**Function Guide**: [Problem_Statements/days/day20_function_guide.md](advent_of_code/aoc2023/Problem_Statements/days/day20_function_guide.md)

---

## 📚 Design Patterns

### **When to Use State Machines**

✅ **Use state machines when**:
- System has discrete, well-defined states
- Behavior changes based on current state
- Transitions triggered by events/inputs
- Need to model reactive systems (UI, protocols, circuits)

❌ **Avoid state machines when**:
- Continuous state space (use differential equations)
- Infinite or unbounded states (use different abstraction)
- Simple conditional logic (if/else sufficient)

### **Rust-Specific Best Practices**

**Pattern 1: Enums for Type-Safe States**
```rust
// ✅ GOOD: Type-safe state variants
enum TrafficLight {
    Red { timer: u32 },
    Yellow { timer: u32 },
    Green { timer: u32 },
}

// ❌ AVOID: Stringly-typed states
struct TrafficLight {
    state: String,  // "red", "yellow", "green" - error-prone!
    timer: u32,
}
```

**Pattern 2: Match for Exhaustive Transitions**
```rust
// Compiler enforces handling all states
match current_state {
    TrafficLight::Red { timer } if timer == 0 => TrafficLight::Green { timer: 30 },
    TrafficLight::Yellow { timer } if timer == 0 => TrafficLight::Red { timer: 45 },
    TrafficLight::Green { timer } if timer == 0 => TrafficLight::Yellow { timer: 5 },
    state => state,  // No transition
}
```

**Pattern 3: Builder for Complex Initialization**
```rust
impl Module {
    fn flip_flop(destinations: Vec<String>) -> Self {
        Module::FlipFlop { on: false, destinations }
    }
    
    fn conjunction(inputs: Vec<String>, destinations: Vec<String>) -> Self {
        let memory = inputs.into_iter()
            .map(|name| (name, Pulse::Low))
            .collect();
        Module::Conjunction { memory, destinations }
    }
}
```

---

## 🌳 Related Concepts

**Prerequisites**:
- [[set-theory-fundamentals]] - States form a set
- [[graph-theory-fundamentals]] - State transitions form a directed graph

**Related**:
- [[complexity-theory]] - State space size affects computational complexity
- [[pigeonhole-principle-cycle-detection]] - Finite state spaces must cycle
- [[number-theory-basics]] - Cycle alignment via LCM
- [[math-foundations/formal-systems-invariants]] - Formal systems generalize state machines with string rewriting rules (MIU puzzle)

**Applications**:
- **Regular expressions**: NFA/DFA for pattern matching
- **Digital circuits**: Logic gates, flip-flops, registers
- **Network protocols**: TCP state machine, HTTP protocol
- **Game AI**: Behavior trees often built on state machines
- **Parsers**: Lexical analysis, parsing with FSMs

---

## 📖 Resources

- [Wikipedia: Finite State Machine](https://en.wikipedia.org/wiki/Finite-state_machine)
- [Introduction to Automata Theory, Languages, and Computation](https://www.amazon.com/Introduction-Automata-Theory-Languages-Computation/dp/0321455363) (Hopcroft, Ullman)
- [Rust Enums for State Machines](https://hoverbear.org/blog/rust-state-machine-pattern/)

---

*Tags: #state-machines #automata-theory #finite-state-machine #fsm #rust-enums #event-driven #computational-theory #math-foundations*

**Related Zettelkasten Links**:
- [[linear-feedback-shift-registers]] - LFSRs are FSMs with feedback (AoC Day 20 circuit structure)
- [[set-theory-fundamentals]] - States as sets
- [[graph-theory-fundamentals]] - Transition graphs
- [[number-theory-basics]] - Cycle synchronization
- [[pigeonhole-principle-cycle-detection]] - Guaranteed cycles in finite state spaces
- [[mission-2]] - Queue data structure for event processing
