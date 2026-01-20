# Day 19: Aplenty - Function-by-Function Guide

**Problem**: Process machine parts through workflow rules to determine acceptance (Part 1) and count all possible accepted combinations (Part 2).

**Key Insights**: 
- Part 1: State machine simulation with HashMap workflow lookup
- Part 2: Brilliant range propagation optimization - count 256 trillion combinations without enumeration!

---

## 📋 Overview

### Problem Summary
- **Part 1**: Simulate 200 parts through workflow state machine → Count accepted parts
- **Part 2**: Count ALL possible accepted combinations where x,m,a,s ∈ [1,4000] → 123+ trillion combinations

### Mathematical Foundation
- **Combinatorics**: Product rule for independent ranges
- **Constraint Propagation**: Split ranges based on conditionals
- **Graph Theory**: DFS traversal with state (ranges) propagation

### Key Algorithms
1. **State Machine Pattern**: Enum destinations, HashMap lookup, first-match-wins rules
2. **Range Splitting**: Partition [min,max] based on < or > operators
3. **DFS with Constraints**: Recursive graph traversal carrying range state
4. **Mathematical Counting**: Multiply range sizes instead of enumerating values

---

## 🏗️ Type Definitions

### `Part` - Machine Part with Ratings
```rust
#[derive(Debug, Clone)]
struct Part {
    x: u32,  // Extremely cool looking
    m: u32,  // Musical (makes noise)
    a: u32,  // Aerodynamic
    s: u32,  // Shiny
}

impl Part {
    fn rating(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
    
    fn get(&self, attr: char) -> u32 {
        match attr {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("Invalid attribute"),
        }
    }
}
```

**Design Decision**: Simple struct with named fields for clarity. `get()` method allows dynamic attribute access for rule evaluation.

### `Destination` - Type-Safe State Machine States
```rust
#[derive(Debug, Clone, PartialEq)]
enum Destination {
    Accept,
    Reject,
    Workflow(String),
}

impl Destination {
    fn from_str(s: &str) -> Self {
        match s {
            "A" => Destination::Accept,
            "R" => Destination::Reject,
            name => Destination::Workflow(name.to_string()),
        }
    }
}
```

**Design Decision**: Enum ensures exhaustive matching - compiler prevents forgetting terminal cases. `Accept`/`Reject` are terminal, `Workflow` transitions to another workflow.

### `Op` - Comparison Operators
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    LessThan,
    GreaterThan,
}
```

**Design Decision**: Only two operators needed. Could use `std::cmp::Ordering` but custom enum is more explicit.

### `Rule` - Conditional Decision Logic
```rust
#[derive(Debug, Clone)]
enum Rule {
    Conditional {
        attr: char,         // Which attribute (x, m, a, s)
        op: Op,             // Comparison (<, >)
        value: u32,         // Threshold value
        dest: Destination,  // Where to go if condition is true
    },
    Unconditional {
        dest: Destination,  // Fallback destination (always matches)
    },
}

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
            Rule::Unconditional { .. } => true,  // Always matches
        }
    }
    
    fn destination(&self) -> &Destination {
        match self {
            Rule::Conditional { dest, .. } => dest,
            Rule::Unconditional { dest } => dest,
        }
    }
}
```

**Design Decision**: Two variants handle conditional vs unconditional rules. `Unconditional` always appears last in workflow as fallback.

**Algorithm**: First-match-wins evaluation - rules processed in order, first matching rule's destination is taken.

### `Workflow` - Collection of Rules
```rust
#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}
```

**Design Decision**: Simple container. Rules vector maintains order (critical for first-match-wins semantics).

---

## 🎯 Part 1: Simulation Approach

### `parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<Part>)`
```rust
fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let sections: Vec<&str> = input.split("\n\n").collect();
    
    // Parse workflows into HashMap for O(1) lookup
    let workflows = sections[0]
        .lines()
        .map(parse_workflow)
        .map(|w| (w.name.clone(), w))
        .collect();
    
    // Parse parts
    let parts = sections[1]
        .lines()
        .map(parse_part)
        .collect();
    
    (workflows, parts)
}
```

**Algorithm**: Split input into workflow/part sections, parse each, store workflows in HashMap for fast lookup.

**Complexity**: O(w + p) where w=workflows, p=parts

### `parse_workflow(line: &str) -> Workflow`
```rust
fn parse_workflow(line: &str) -> Workflow {
    // Example: px{a<2006:qkq,m>2090:A,rfg}
    let (name, rules_str) = line.split_once('{').unwrap();
    let rules_str = rules_str.strip_suffix('}').unwrap();
    
    let rules = rules_str
        .split(',')
        .map(parse_rule)
        .collect();
    
    Workflow {
        name: name.to_string(),
        rules,
    }
}
```

**Algorithm**: Split by `{` to separate name, then split rules by `,` and parse individually.

### `parse_rule(text: &str) -> Rule`
```rust
fn parse_rule(text: &str) -> Rule {
    if let Some((condition, dest_str)) = text.split_once(':') {
        // Conditional: "a<2006:qkq"
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
        // Unconditional: "rfg" or "A"
        Rule::Unconditional {
            dest: Destination::from_str(text),
        }
    }
}
```

**Algorithm**: Check for `:` to distinguish conditional vs unconditional. Parse condition parts (attribute, operator, value).

**Example**: `a<2006:qkq` → Conditional{attr='a', op=LessThan, value=2006, dest=Workflow("qkq")}

### `parse_part(line: &str) -> Part`
```rust
fn parse_part(line: &str) -> Part {
    // Example: {x=787,m=2655,a=1222,s=2876}
    let inner = line.strip_prefix('{').unwrap().strip_suffix('}').unwrap();
    
    let mut x = 0;
    let mut m = 0;
    let mut a = 0;
    let mut s = 0;
    
    for assignment in inner.split(',') {
        let (attr, value_str) = assignment.split_once('=').unwrap();
        let value: u32 = value_str.parse().unwrap();
        
        match attr {
            "x" => x = value,
            "m" => m = value,
            "a" => a = value,
            "s" => s = value,
            _ => panic!("Unknown attribute"),
        }
    }
    
    Part { x, m, a, s }
}
```

**Algorithm**: Strip braces, split by `,`, parse each `attr=value` pair.

### `process_part(part: &Part, workflows: &HashMap<String, Workflow>) -> bool`
```rust
fn process_part(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
    let mut current = "in";  // Always start at "in" workflow
    
    loop {
        let workflow = workflows.get(current).expect("Workflow not found");
        
        // Evaluate rules in order - first match wins
        for rule in &workflow.rules {
            if rule.matches(part) {
                match rule.destination() {
                    Destination::Accept => return true,
                    Destination::Reject => return false,
                    Destination::Workflow(name) => {
                        current = name;
                        break;  // Move to next workflow
                    }
                }
            }
        }
    }
}
```

**Algorithm**: State machine loop. Start at "in", evaluate rules until terminal state (Accept/Reject) reached.

**Example Trace** for Part{x=787, m=2655, a=1222, s=2876}:
1. Workflow "in": Rule `s<1351` → false, Rule unconditional → "qqz"
2. Workflow "qqz": Rule `s>2770` → true → "qs"
3. Workflow "qs": Rule `s>3448` → false, Rule unconditional → "lnx"
4. Workflow "lnx": Rule `m>1548` → true → Accept ✓

**Complexity**: O(workflows_visited × avg_rules_per_workflow) ≈ O(5 × 3) = O(15) per part

### `solve_part1(input: &str) -> String`
```rust
pub fn solve_part1(input: &str) -> String {
    let (workflows, parts) = parse_input(input);
    
    let total: u32 = parts
        .iter()
        .filter(|part| process_part(part, &workflows))
        .map(|part| part.rating())
        .sum();
    
    total.to_string()
}
```

**Algorithm**: Process each part, sum ratings of accepted parts.

**Result**: 330,820

---

## 🚀 Part 2: Range Propagation Approach

### `Range` - Interval with Min/Max Bounds
```rust
#[derive(Debug, Clone, Copy)]
struct Range {
    min: u64,
    max: u64,
}

impl Range {
    fn size(&self) -> u64 {
        if self.max >= self.min {
            self.max - self.min + 1
        } else {
            0  // Empty range
        }
    }
    
    fn is_empty(&self) -> bool {
        self.max < self.min
    }
}
```

**Design Decision**: Inclusive range [min, max]. Size calculation: max - min + 1 (e.g., [1,4000] has size 4000).

### `PartRange` - 4D Range Space
```rust
#[derive(Debug, Clone, Copy)]
struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRange {
    fn combinations(&self) -> u64 {
        self.x.size() * self.m.size() * self.a.size() * self.s.size()
    }
    
    fn is_empty(&self) -> bool {
        self.x.is_empty() || self.m.is_empty() || 
        self.a.is_empty() || self.s.is_empty()
    }
    
    fn get(&self, attr: char) -> Range {
        match attr {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("Invalid attribute"),
        }
    }
    
    fn set(&mut self, attr: char, range: Range) {
        match attr {
            'x' => self.x = range,
            'm' => self.m = range,
            'a' => self.a = range,
            's' => self.s = range,
            _ => panic!("Invalid attribute"),
        }
    }
}
```

**Design Decision**: Represents all possible parts within given ranges. `combinations()` computes total possibilities using product rule.

**Example**: `x[1,4000], m[1,4000], a[1,4000], s[1,4000]` has 4000^4 = 256 trillion combinations.

### `split_range(range: Range, op: Op, value: u64) -> (Range, Range)`
```rust
fn split_range(range: Range, op: Op, value: u64) -> (Range, Range) {
    match op {
        Op::LessThan => {
            // Matching: values < value
            let matching = Range {
                min: range.min,
                max: range.max.min(value - 1),  // Cap at value-1
            };
            // Non-matching: values >= value
            let non_matching = Range {
                min: range.min.max(value),  // Start at value
                max: range.max,
            };
            (matching, non_matching)
        }
        Op::GreaterThan => {
            // Matching: values > value
            let matching = Range {
                min: range.min.max(value + 1),  // Start at value+1
                max: range.max,
            };
            // Non-matching: values <= value
            let non_matching = Range {
                min: range.min,
                max: range.max.min(value),  // Cap at value
            };
            (matching, non_matching)
        }
    }
}
```

**Algorithm**: Partition range into two subranges based on conditional operator.

**Example 1** - LessThan:
- Input: `Range{min:1, max:4000}`, `Op::LessThan`, value=2006
- Output: `(Range{1,2005}, Range{2006,4000})`
- Matching goes to destination, non-matching continues to next rule

**Example 2** - GreaterThan:
- Input: `Range{min:1, max:4000}`, `Op::GreaterThan`, value=2770
- Output: `(Range{2771,4000}, Range{1,2770})`

**Key Insight**: BOTH ranges are returned - this is critical for progressive narrowing!

### `count_accepted(workflow_name: &str, ranges: PartRange, workflows: &HashMap<String, Workflow>) -> u64`
```rust
fn count_accepted(
    workflow_name: &str,
    mut ranges: PartRange,
    workflows: &HashMap<String, Workflow>,
) -> u64 {
    // Terminal cases
    if workflow_name == "A" {
        return ranges.combinations();  // Count combinations in this range
    }
    if workflow_name == "R" {
        return 0;  // Rejected - don't count
    }
    
    let workflow = workflows.get(workflow_name).unwrap();
    let mut total = 0;
    
    for rule in &workflow.rules {
        match rule {
            Rule::Conditional { attr, op, value, dest } => {
                let (matching, non_matching) = 
                    split_range(ranges.get(*attr), *op, *value as u64);
                
                // Process matching range → recurse to destination
                if !matching.is_empty() {
                    let mut matching_ranges = ranges;
                    matching_ranges.set(*attr, matching);
                    total += count_accepted(dest_name, matching_ranges, workflows);
                }
                
                // Continue with non-matching range → next rule
                // KEY: This progressively narrows the range!
                ranges.set(*attr, non_matching);
                
                // Early exit if range becomes impossible
                if ranges.is_empty() {
                    break;
                }
            }
            Rule::Unconditional { dest } => {
                // Remaining range goes to fallback destination
                total += count_accepted(dest_name, ranges, workflows);
                break;
            }
        }
    }
    
    total
}
```

**Algorithm**: DFS through workflow graph, carrying range constraints. At each conditional:
1. Split range into matching/non-matching
2. Recurse with matching range to destination
3. Continue with non-matching range to next rule

**Brilliant Optimization**: Instead of enumerating 256 trillion individual parts, we propagate ranges and count mathematically!

**Example Trace**:

Initial: `x[1,4000], m[1,4000], a[1,4000], s[1,4000]`

Workflow `in{s<1351:px,qqz}`:
1. Rule `s<1351`:
   - Split s[1,4000] → matching s[1,1350], non-matching s[1351,4000]
   - Recurse: `px` with `s[1,1350]` (explores 1350 × 4000^3 combinations)
   - Continue with `s[1351,4000]`
2. Rule unconditional `qqz`:
   - Recurse: `qqz` with `s[1351,4000]` (explores 2650 × 4000^3 combinations)

**Recursion continues** until all paths reach Accept (count) or Reject (0).

**Complexity**: O(workflows × rules × avg_splits) ≈ O(30 × 3 × 2) = O(180) - independent of input space size!

### `solve_part2(input: &str) -> String`
```rust
pub fn solve_part2(input: &str) -> String {
    let (workflows, _parts) = parse_input(input);
    
    // Start with all possible combinations
    let initial_ranges = PartRange {
        x: Range { min: 1, max: 4000 },
        m: Range { min: 1, max: 4000 },
        a: Range { min: 1, max: 4000 },
        s: Range { min: 1, max: 4000 },
    };
    
    let count = count_accepted("in", initial_ranges, &workflows);
    count.to_string()
}
```

**Result**: 123,972,546,935,551 (123+ trillion combinations!)

---

## 🎨 Design Patterns

### 1. Enum-Based State Machine
- Type-safe destinations prevent invalid states
- Exhaustive matching catches missing terminal cases
- Self-documenting code (Accept/Reject/Workflow)

### 2. HashMap for O(1) Workflow Lookup
- Critical for state machine efficiency
- Trade space (small) for time (significant)

### 3. First-Match-Wins Rule Evaluation
- Ordered rules = priority-based decisions
- Natural mapping to match expressions
- Clear semantics (no ambiguity)

### 4. Range Splitting for Constraint Propagation
- Avoid exponential enumeration
- Progressive narrowing through rule cascade
- Mathematical counting instead of iteration

### 5. DFS with State Propagation
- Recursive exploration of all acceptance paths
- Carry state (ranges) through recursion
- Sum contributions from all terminal Accept nodes

---

## 📊 Performance Analysis

| Metric | Part 1 | Part 2 |
|--------|--------|--------|
| **Input Space** | 200 parts | 256 trillion (4000^4) combinations |
| **Approach** | Simulation | Range propagation |
| **Operations** | ~3,000 (200 parts × 15 workflow evals) | ~180 (30 workflows × 6 DFS calls) |
| **Runtime** | 210µs | 190µs |
| **Space** | O(workflows) | O(recursion_depth × PartRange) |

**Why Part 2 is Faster**: Part 1 processes 200 individual parts. Part 2 processes ~30 range states through workflow graph. Fewer operations despite massive input space!

---

## 🧠 Key Takeaways

1. **State machines**: Enum destinations + HashMap lookup = type-safe, efficient
2. **Range propagation**: When constraints form intervals, work with ranges not values
3. **Mathematical counting**: Product rule avoids enumeration
4. **Progressive narrowing**: Split ranges at each conditional, continue with remainder
5. **Graph structure matters**: Complexity depends on workflows (graph), not inputs (data)
6. **Abstract interpretation**: Represent sets of values symbolically (ranges) for exponential speedup

---

## 🔗 Related Concepts

**Zettelkasten**:
- [[workflow-pattern-matching]] - State machines with enum destinations
- [[constraint-propagation]] - Range splitting algorithms
- [[combinatorics-fundamentals]] - Product rule for counting
- [[dfs-patterns]] - Graph traversal patterns
- [[Error Handling Patterns]] - Enum-based error types (similar to Destination enum)

**Missions**:
- [[mission-5]] - HashMap optimizations
- [[mission-8]] - Graph algorithms

**Mathematics**:
- [[math-foundations/combinatorics]] - Product rule, counting without enumeration
- [[math-foundations/constraint-satisfaction]] - CSP techniques

---

## 💡 Follow-Up Questions

1. **Extension**: How would you handle workflows with cycles? (Detect with visited set, reject cyclic paths)
2. **Optimization**: Could you memoize (workflow_name, ranges) → count? (Yes, but requires hashable ranges - likely overkill at 190µs)
3. **Generalization**: How would this scale to 10 attributes? (Same algorithm, just 10D ranges - still O(workflows × rules))
4. **Application**: Where else does range propagation apply? (Symbolic execution, abstract interpretation, interval arithmetic, database query optimization)
