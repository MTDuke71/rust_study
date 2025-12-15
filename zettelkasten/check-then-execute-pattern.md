# Check-Then-Execute Pattern

## Overview

The **Check-Then-Execute Pattern** is a design approach where operations are validated in their entirety before any modifications are made. This ensures atomic success or failure semantics—either the entire operation completes successfully, or nothing changes at all.

This pattern eliminates the need for rollback mechanisms, undo operations, or complex error recovery logic by guaranteeing that modifications only occur when the operation is known to be valid.

## Core Principle

```
1. CHECK Phase (Read-Only)
   ├─ Validate all preconditions
   ├─ Scan for required resources
   ├─ Detect blocking conditions
   └─ Build execution plan

2. EXECUTE Phase (Write Operations)
   ├─ Only runs if CHECK succeeded
   ├─ Applies all changes atomically
   ├─ No error handling needed (already validated)
   └─ Returns success with new state
```

**Key Guarantee**: If CHECK phase completes successfully, EXECUTE phase cannot fail due to validation issues.

## Day 15 Example: Box Pushing

### The Pattern in Action

**Scenario**: Robot wants to push a chain of boxes: `. O O O @>`

```rust
fn try_move(grid: &mut Grid<Tile>, pos: Coord, dir: Direction) -> Option<Coord> {
    // PHASE 1: CHECK - scan forward to find empty space
    let mut check_pos = new_pos;
    loop {
        match grid.get(next_pos) {
            Some(&Tile::Empty) => {
                // ✅ Move is valid! Break to EXECUTE phase
                break;
            }
            Some(&Tile::Box) => {
                // Box in chain, keep scanning
                check_pos = next_pos;
            }
            Some(&Tile::Wall) | None => {
                // ❌ Blocked! Return None with NO changes
                return None;
            }
        }
    }
    
    // PHASE 2: EXECUTE - apply changes (only reached if valid)
    *grid.get_mut(next_pos) = Tile::Box;   // Place at empty
    *grid.get_mut(new_pos) = Tile::Empty;   // Clear first position
    return Some(new_pos);
}
```

### Why This Matters

**Without Check-Then-Execute**:
```rust
// ❌ BAD: Optimistic approach
fn try_move_bad(grid: &mut Grid<Tile>, pos: Coord, dir: Direction) -> Option<Coord> {
    // Start making changes immediately
    *grid.get_mut(pos) = Tile::Empty;
    
    // Oops, hit a wall! Now need to undo...
    if next_tile == Tile::Wall {
        *grid.get_mut(pos) = Tile::Box;  // Rollback!
        return None;
    }
    // More changes, more potential rollback...
}
```

**With Check-Then-Execute**:
- ✅ No partial states—grid never in invalid configuration
- ✅ No rollback logic needed
- ✅ Simpler error handling—just return early
- ✅ Easier to reason about—validation separate from mutation

## Benefits

### 1. **Atomic Semantics**
- Operation either fully succeeds or has no effect
- No intermediate states visible to other code
- Easier to maintain invariants

### 2. **Simplified Error Handling**
```rust
// CHECK phase
if !is_valid() {
    return Err("Validation failed");  // No cleanup needed!
}

// EXECUTE phase - guaranteed to work
apply_changes();
Ok(result)
```

### 3. **Better Testability**
- Can test CHECK phase without side effects
- Can verify EXECUTE phase with known-valid inputs
- Clear separation of concerns

### 4. **Performance Optimization**
- Early exit on validation failure (cheap)
- Expensive operations only run when guaranteed to succeed
- Can optimize EXECUTE phase knowing preconditions hold

## Common Use Cases

### 1. **Transaction Systems**
```rust
// Database transaction
let changes = validate_transaction(&tx)?;  // CHECK
apply_to_database(changes)?;               // EXECUTE
commit()?;
```

### 2. **Resource Allocation**
```rust
// Memory allocation
let required = calculate_memory_needed(data)?;  // CHECK
if !has_enough_memory(required) {
    return Err("Out of memory");
}
let buffer = allocate(required)?;               // EXECUTE
```

### 3. **Game State Updates**
```rust
// Chess move validation
let valid_moves = get_legal_moves(piece, board)?;  // CHECK
if !valid_moves.contains(&target) {
    return Err("Illegal move");
}
apply_move(piece, target, &mut board);             // EXECUTE
```

### 4. **File Operations**
```rust
// Batch file processing
let files = validate_all_paths(&paths)?;      // CHECK - all exist?
for file in files {
    process_file(file)?;                      // EXECUTE - safe!
}
```

## Comparison with Other Patterns

### vs Try-Catch-Rollback
```rust
// Try-Catch-Rollback (complex)
fn update() -> Result<()> {
    let old_state = save_state();
    
    if let Err(e) = try_update() {
        restore_state(old_state);  // Rollback logic
        return Err(e);
    }
    Ok(())
}

// Check-Then-Execute (simple)
fn update() -> Result<()> {
    validate_update()?;  // CHECK - no changes yet
    apply_update();      // EXECUTE - can't fail
    Ok(())
}
```

### vs Optimistic Concurrency
- **Optimistic**: Make changes, detect conflicts, retry
- **Check-Then-Execute**: Validate first, make changes once
- **Use Check-Then-Execute when**: Single-threaded or lock-protected
- **Use Optimistic when**: High concurrency, low conflict rate

### vs Two-Phase Commit
- **Two-Phase Commit**: Distributed consensus protocol
- **Check-Then-Execute**: Local validation pattern
- **Similarity**: Both separate preparation from commitment
- **Difference**: 2PC coordinates multiple participants

## Implementation Guidelines

### 1. **Keep CHECK Phase Read-Only**
```rust
// ✅ GOOD - no mutations in CHECK
fn is_valid(&self) -> bool {
    self.data.iter().all(|x| x > 0)
}

// ❌ BAD - mutating during CHECK
fn is_valid(&mut self) -> bool {
    self.counter += 1;  // Side effect!
    self.data.iter().all(|x| x > 0)
}
```

### 2. **Make EXECUTE Phase Infallible**
```rust
// ✅ GOOD - EXECUTE can't fail (already validated)
fn execute(valid_data: ValidatedData) {
    apply(valid_data);  // No Result needed
}

// ❌ BAD - EXECUTE can still fail
fn execute(data: Data) -> Result<()> {
    if !is_valid(data) {  // Validation in EXECUTE!
        return Err("Invalid");
    }
    apply(data)
}
```

### 3. **Use Types to Enforce the Pattern**
```rust
// Type-level guarantee
struct ValidatedCommand(Command);

impl Command {
    // CHECK phase returns wrapped type
    fn validate(self) -> Result<ValidatedCommand> {
        // validation logic...
        Ok(ValidatedCommand(self))
    }
}

// EXECUTE phase requires validated type
fn execute(cmd: ValidatedCommand) {
    // No validation needed - type proves it's valid!
}
```

## Trade-offs

### Advantages
- ✅ Simpler error handling (no rollback)
- ✅ Atomic semantics (no partial states)
- ✅ Easier to reason about (clear phases)
- ✅ Better testability (separate concerns)

### Disadvantages
- ❌ May scan data twice (CHECK + EXECUTE)
- ❌ Not suitable for long-running transactions
- ❌ Assumes validation is cheaper than execution
- ❌ Requires predictable execution environment

### When NOT to Use
- **Distributed systems**: Use two-phase commit instead
- **Expensive validation**: Consider optimistic approach
- **Dynamic environments**: Validation may become stale
- **Read-heavy operations**: Overhead not worth it

## Real-World Examples

### Rust Compiler
The Rust compiler uses check-then-execute:
1. **CHECK**: Type checking, borrow checking (read-only AST traversal)
2. **EXECUTE**: Code generation (only runs if type-check passes)

### Git Commits
Git's staging area implements this pattern:
1. **CHECK**: `git add` validates files can be staged
2. **EXECUTE**: `git commit` applies validated changes to history

### Database Transactions
Modern databases separate:
1. **CHECK**: Parse SQL, validate constraints, plan execution
2. **EXECUTE**: Apply changes to storage engine

## Related Patterns

- [[transaction-pattern]] - Broader concept including rollback mechanisms
- [[two-phase-commit]] - Distributed version for multiple participants
- [[command-pattern]] - Often combined with check-then-execute for undo/redo
- [[validation-pattern]] - CHECK phase is essentially validation
- [[atomic-operations]] - The guarantee check-then-execute provides
- [[mission-6]] - Grid operations that benefit from this pattern

## AoC Applications

### Day 15: Warehouse Woes
- **CHECK**: Scan box chain for empty space
- **EXECUTE**: Update two positions (first and last)
- **Benefit**: No rollback needed when wall blocks push

### Potential Use in Other Days
- **Path finding**: Validate path exists before updating state
- **Resource management**: Check resources available before consumption
- **State transitions**: Validate transition legal before applying

## Key Takeaways

1. **Separate validation from mutation** - keep them in distinct phases
2. **Validation should be read-only** - no side effects in CHECK
3. **Execution should be infallible** - no errors in EXECUTE if CHECK passed
4. **Use types to enforce** - `ValidatedT` wrapper types prevent misuse
5. **Choose wisely** - not appropriate for all scenarios

The check-then-execute pattern shines when you need atomic semantics without complex rollback logic. It trades potential redundant reads for dramatically simplified error handling and clearer code structure.

---

*Created: 2025-12-14*  
*Primary Example: [[advent_of_code/aoc2024/day15]] (box-pushing simulation)*

*Links: [[atomic-operations]] | [[transaction-pattern]] | [[validation-pattern]] | [[mission-6]] | [[rust-book/rust-book-ch15]] (smart pointers for safe mutation)*

*Tags: #design-patterns #algorithms #validation #atomicity #error-handling #aoc2024-day15 #state-management #functional-programming*
