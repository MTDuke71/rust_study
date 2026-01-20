# 🔄 Workflow Pattern Matching

**Rule-based state machines with conditional transitions and pattern matching in Rust**

*Created: 2026-01-19 | AoC 2023 Day 19 - Aplenty*

---

## 🎯 Overview

Workflow pattern matching is a design pattern for processing entities through a series of rule-based decisions that determine their path through a state machine. Each workflow contains conditional rules that evaluate properties and route to other workflows or terminal states.

**Core Concept**: Instead of implementing complex nested if/else chains, workflows encapsulate decision logic into reusable, composable state machines.

---

## 🏗️ Architecture

### Components

```rust
// Entity being processed
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

// Terminal states or next workflow
enum Destination {
    Accept,           // Terminal: success
    Reject,           // Terminal: failure
    Workflow(String), // Transition to another workflow
}

// Individual decision rule
enum Rule {
    Conditional {
        attr: char,         // Which property to check
        op: Op,             // Comparison operator (<, >)
        value: u32,         // Threshold value
        dest: Destination,  // Where to go if true
    },
    Unconditional {
        dest: Destination,  // Fallback destination
    },
}

// Collection of rules forming a workflow
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}
```

### Evaluation Flow

```rust
impl Workflow {
    /// Process entity through this workflow's rules
    fn process(&self, part: &Part) -> &Destination {
        for rule in &self.rules {
            if rule.matches(part) {
                return rule.destination();
            }
        }
        panic!("No matching rule"); // Should never happen with proper unconditional fallback
    }
}
```

---

## 🔄 State Machine Execution

### Simple Workflow Traversal

```rust
fn process_part(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
    let mut current = "in"; // Start at entry workflow
    
    loop {
        let workflow = workflows.get(current).expect("Workflow not found");
        let dest = workflow.process(part);
        
        match dest {
            Destination::Accept => return true,
            Destination::Reject => return false,
            Destination::Workflow(name) => current = name, // Transition
        }
    }
}
```

**Key Insight**: The loop continues until a terminal state (Accept/Reject) is reached. Each workflow transition is deterministic based on the entity's properties.

---

## 📊 Real Example: AoC 2023 Day 19

### Problem Context

Process machine parts through workflows that check properties (x, m, a, s) and decide acceptance:

```
Workflow: px{a<2006:qkq,m>2090:A,rfg}
```

**Translation**:
1. If `a < 2006` → go to workflow `qkq`
2. Else if `m > 2090` → Accept
3. Else → go to workflow `rfg`

### Parsing Workflow Syntax

```rust
fn parse_rule(text: &str) -> Rule {
    if let Some((condition, dest_str)) = text.split_once(':') {
        // Conditional: "x>10:one"
        let attr = condition.chars().next().unwrap();
        let op = if condition.contains('<') {
            Op::LessThan
        } else {
            Op::GreaterThan
        };
        let value: u32 = condition[2..].parse().unwrap();
        let dest = Destination::from_str(dest_str);
        
        Rule::Conditional { attr, op, value, dest }
    } else {
        // Unconditional fallback: "A" or "rfg"
        Rule::Unconditional {
            dest: Destination::from_str(text),
        }
    }
}
```

### Rule Evaluation

```rust
impl Rule {
    fn matches(&self, part: &Part) -> bool {
        match self {
            Rule::Conditional { attr, op, value, .. } => {
                let part_value = part.get(*attr);
                match op {
                    Op::LessThan => part_value < *value,
                    Op::GreaterThan => part_value > *value,
                }
            }
            Rule::Unconditional { .. } => true, // Always matches
        }
    }
}
```

---

## 🚀 Advanced: Range-Based Workflow Analysis

Instead of processing individual entities, analyze **all possible inputs** using constraint ranges:

### Range Splitting

```rust
#[derive(Debug, Clone, Copy)]
struct Range {
    min: u64,
    max: u64,
}

/// Split range based on conditional
/// Returns (matching, non_matching)
fn split_range(range: Range, op: Op, value: u64) -> (Range, Range) {
    match op {
        Op::LessThan => {
            // Matching: [min, value-1]
            let matching = Range {
                min: range.min,
                max: range.max.min(value - 1),
            };
            // Non-matching: [value, max]
            let non_matching = Range {
                min: range.min.max(value),
                max: range.max,
            };
            (matching, non_matching)
        }
        Op::GreaterThan => {
            // Matching: [value+1, max]
            let matching = Range {
                min: range.min.max(value + 1),
                max: range.max,
            };
            // Non-matching: [min, value]
            let non_matching = Range {
                min: range.min,
                max: range.max.min(value),
            };
            (matching, non_matching)
        }
    }
}
```

### DFS with Range Constraints

```rust
fn count_accepted(
    workflow_name: &str,
    mut ranges: PartRange,
    workflows: &HashMap<String, Workflow>,
) -> u64 {
    // Terminal cases
    if workflow_name == "A" {
        return ranges.combinations(); // Product of all range sizes
    }
    if workflow_name == "R" {
        return 0;
    }
    
    let workflow = workflows.get(workflow_name).unwrap();
    let mut total = 0;
    
    for rule in &workflow.rules {
        match rule {
            Rule::Conditional { attr, op, value, dest } => {
                let (matching, non_matching) = 
                    split_range(ranges.get(*attr), *op, *value as u64);
                
                // Process matching range → destination
                if !matching.is_empty() {
                    let mut matching_ranges = ranges;
                    matching_ranges.set(*attr, matching);
                    total += count_accepted(dest_name, matching_ranges, workflows);
                }
                
                // Continue with non-matching → next rule
                ranges.set(*attr, non_matching);
            }
            Rule::Unconditional { dest } => {
                total += count_accepted(dest_name, ranges, workflows);
                break;
            }
        }
    }
    
    total
}
```

**Brilliant Insight**: Instead of enumerating 256 trillion combinations (4000^4), we **propagate constraint ranges** through the workflow graph and count valid combinations mathematically!

---

## 🎨 Design Patterns

### 1. Enum-Based Destinations

```rust
enum Destination {
    Accept,
    Reject,
    Workflow(String),
}
```

**Benefits**:
- ✅ Exhaustive matching enforced by compiler
- ✅ Clear terminal vs. transition states
- ✅ Type-safe workflow references

### 2. Rule Ordering Matters

```rust
// Rules are evaluated in order - first match wins
vec![
    Rule::Conditional { attr: 'x', op: Op::GreaterThan, value: 10, dest: "one" },
    Rule::Conditional { attr: 'm', op: Op::LessThan, value: 20, dest: "two" },
    Rule::Unconditional { dest: Accept }, // Fallback
]
```

### 3. Workflow Graph

```
    in
    ├─→ px ──→ qkq ──→ Accept
    │         └──→ crn ──→ Reject
    └─→ qqz ──→ qs ──→ lnx ──→ Accept
              └──→ Reject
```

Each workflow is a node, rules create edges with conditions.

---

## 🔧 Common Use Cases

| **Domain** | **Workflows** | **Rules** | **Terminal States** |
|------------|---------------|-----------|---------------------|
| **Content Moderation** | Review stages | Keyword/sentiment checks | Approve/Reject/Escalate |
| **Order Processing** | Fulfillment steps | Inventory/payment checks | Ship/Cancel/Hold |
| **Loan Approval** | Underwriting stages | Credit score/income thresholds | Approve/Deny/Manual Review |
| **Game AI** | Behavior trees | Condition checks | Action states |
| **Data Validation** | Pipeline stages | Schema/business rule checks | Pass/Fail/Warning |

---

## 🧠 Key Insights

### Pattern Matching Advantages

✅ **Declarative**: Rules describe WHAT to check, not HOW to implement  
✅ **Composable**: Workflows reference other workflows  
✅ **Testable**: Each workflow can be unit tested independently  
✅ **Auditable**: Rule evaluation creates clear decision trails  
✅ **Extensible**: Add new workflows without modifying existing ones

### Performance Considerations

- **HashMap lookup**: O(1) workflow retrieval by name
- **Rule evaluation**: O(n) where n = rules per workflow (typically small)
- **Graph traversal**: Average case O(k) transitions before terminal state
- **Range analysis**: O(workflows × rules) - processes entire space once

### Error Handling Integration

Workflow pattern matching pairs naturally with [[Error Handling Patterns]]:

```rust
enum ProcessingError {
    WorkflowNotFound(String),
    InvalidRule { workflow: String, rule: String },
    CyclicDependency { path: Vec<String> },
}

fn process_with_errors(
    part: &Part,
    workflows: &HashMap<String, Workflow>,
) -> Result<bool, ProcessingError> {
    let mut current = "in";
    let mut visited = HashSet::new();
    
    loop {
        // Detect cycles
        if !visited.insert(current.to_string()) {
            return Err(ProcessingError::CyclicDependency {
                path: visited.into_iter().collect(),
            });
        }
        
        let workflow = workflows
            .get(current)
            .ok_or_else(|| ProcessingError::WorkflowNotFound(current.to_string()))?;
        
        // ... rest of logic
    }
}
```

---

## 📚 Implementation Checklist

When implementing workflow pattern matching:

- [ ] Define clear terminal states (Accept/Reject/etc.)
- [ ] Use enums for type-safe destinations
- [ ] Ensure every workflow has unconditional fallback rule
- [ ] Store workflows in HashMap for fast lookup
- [ ] Consider cycle detection for complex graphs
- [ ] Log workflow transitions for debugging
- [ ] Test edge cases (empty workflows, missing workflows)
- [ ] Document workflow dependencies and expected behavior

---

## 🔗 Related Concepts

**Core Patterns**:
- [[Error Handling Patterns]] - Enum-based error types for workflow failures
- [[state-machine-rust]] - Finite state machines with typestate pattern
- [[pattern-matching]] - Rust pattern matching in match expressions

**AoC Applications**:
- [[aoc2023-day19]] - Aplenty: Workflow processing (this example)
- [[aoc-parsing-patterns]] - Parsing workflow syntax
- [[aoc-state-tracking]] - State machines in puzzles

**Mission Integration**:
- [[mission-5]] - HashMap for workflow storage
- [[mission-8]] - Graph traversal patterns

**Error Handling**:
- [[rust-for-rustaceans-ch4]] - Chapter 4: Error enumeration patterns
- [[anyhow and thiserror]] - Production error handling

**Advanced Topics**:
- [[math-foundations/constraint-propagation]] - Range splitting, interval arithmetic, forward propagation
- [[math-foundations/combinatorics-fundamentals]] - Product rule for counting without enumeration
- [[dfs-patterns]] - Depth-first search with constraints

---

## 💡 Key Takeaways

1. **Workflows = State Machines**: Each workflow is a state with conditional transitions
2. **Rules = Guards**: Conditionals determine which transition to take
3. **Enums = Safety**: Type-safe destinations prevent invalid states
4. **Ranges = Optimization**: Analyze entire input space without enumeration
5. **Pattern Matching**: Natural fit for Rust's exhaustive match expressions

---

*Tags: #workflow #state-machine #pattern-matching #rule-based #aoc2023 #day19 #aplenty #conditional-logic #graph-traversal #constraint-propagation*

*Links: [[zettel-index]] | [[Error Handling Patterns]] | [[aoc2023]] | [[rust-for-rustaceans-ch4]] | [[state-machine-rust]] | [[pattern-matching]] | [[mission-5]] | [[mission-8]]*
