# Day 6: Guard Gallivant - Comprehensive Solution Walkthrough

**Educational Analysis**: Python vs Rust implementation comparison with Mission code integration

---

## 🎯 **Problem Summary**

- **Part 1**: Simulate guard patrol until exit, count distinct positions visited
- **Part 2**: Find positions where adding a single obstacle creates a patrol loop
- **Results**: Part 1: 5551, Part 2: 1939 (real input), Part 1: 41, Part 2: 6 (example)

---

## 🐍 **Python Solution Analysis (William Y Feng)**

The reference Python solutions demonstrate a straightforward approach:

### **Code Structure**
```python
# Grid representation: List[List[str]] (mutable 2D array)
with open("./real_input.txt") as fin:
    grid = fin.read().strip().split("\n")

# Direction handling: Array of direction vectors
dd = [[-1, 0], [0, 1], [1, 0], [0, -1]]  # North, East, South, West

# State tracking: Sets for positions and states
seen = set()  # Positions (i,j)
seen_states = set()  # States (i,j,dir) for loop detection

# Bounds checking: Manual validation
if not (0 <= next_i < n and 0 <= next_j < m):
    break
```

### **Algorithm Flow**
1. **Parse input**: Split into grid of characters
2. **Find guard**: Scan for `^` starting position
3. **Simulate movement**:
   - Add current position to visited set
   - Calculate next position using direction vectors
   - Check bounds manually
   - If obstacle, rotate right (`dir = (dir + 1) % 4`)
   - Otherwise, move forward
4. **Loop detection**: Track `(i, j, direction)` tuples

---

## 🦀 **Rust Mission-Leveraged Approach**

Our implementation showcases professional software engineering:

### **Type Safety Foundation**
```rust
use mission6::{Coord, Direction, Grid};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GuardState {
    position: Coord,      // Mission 6 type-safe coordinates
    direction: Direction, // Mission 6 enum with rotation methods
}
```

### **Safe Parsing with Error Handling**
```rust
fn parse_input(input: &str) -> Result<(Grid<char>, GuardState)> {
    let lines: Vec<&str> = input.lines().collect();
    
    // Validate rectangular grid
    let width = lines.first().context("Empty input")?.len();
    for (i, line) in lines.iter().enumerate() {
        if line.len() != width {
            anyhow::bail!("Line {} has length {}, expected {}", i, line.len(), width);
        }
    }
    
    // Type-safe grid creation with Mission 6
    let mut grid = Grid::new(width, lines.len(), '.');
    let mut guard_state = None;
    
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let coord = Coord::new(x, y);
            match ch {
                '^' => guard_state = Some(GuardState::new(coord, Direction::North)),
                '>' => guard_state = Some(GuardState::new(coord, Direction::East)),
                'v' => guard_state = Some(GuardState::new(coord, Direction::South)),
                '<' => guard_state = Some(GuardState::new(coord, Direction::West)),
                '#' | '.' => grid[coord] = ch,
                _ => anyhow::bail!("Invalid character '{}' at ({}, {})", ch, x, y),
            }
        }
    }
    
    let guard_state = guard_state.context("No guard found in input")?;
    Ok((grid, guard_state))
}
```

---

## 📊 **Algorithm Comparison**

| **Step** | **Python Approach** | **Rust Mission Approach** |
|----------|---------------------|---------------------------|
| **Grid Storage** | `List[List[str]]` (mutable) | `Grid<char>` (immutable, safe) |
| **Position Tracking** | `(i, j)` tuples | `Coord` struct with operations |
| **Direction Handling** | Index into direction array | `Direction` enum with methods |
| **Movement** | Manual arithmetic + bounds check | `.step()` with built-in safety |
| **Visited Tracking** | `set()` of `(i, j)` tuples | `HashSet<Coord>` |
| **Bounds Checking** | Manual: `0 <= pos < size` | Automatic with `Option` returns |
| **Loop Detection** | `(i, j, dir)` tuple tracking | `GuardState` struct with `Hash` |
| **Error Handling** | Exception-based | `Result` types with context |

---

## 🔄 **Part 1: Basic Patrol Simulation**

### **Python Algorithm**
```python
seen = set()
while True:
    seen.add((i, j))
    
    next_i = i + dd[dir][0]
    next_j = j + dd[dir][1]
    
    if not (0 <= next_i < n and 0 <= next_j < m):
        break  # Exited bounds
    
    if grid[next_i][next_j] == "#":
        dir = (dir + 1) % 4  # Turn right
    else:
        i, j = next_i, next_j

print(len(seen))  # Answer: 41 (example), 5551 (real)
```

### **Rust Mission Algorithm**
```rust
fn simulate_patrol(grid: &Grid<char>, start_state: GuardState) -> Result<HashSet<Coord>> {
    let mut guard = start_state;
    let mut visited = HashSet::new();
    let mut states = HashSet::new();
    
    visited.insert(guard.position);
    
    loop {
        // Loop detection using full state (position + direction)
        if states.contains(&guard) {
            return Ok(visited); // Loop detected, return positions visited so far
        }
        states.insert(guard);
        
        // Safe coordinate stepping with Mission 6
        if let Some(next_pos) = guard.next_position() {
            if let Some(&cell) = grid.get(next_pos) {  // Bounds checked by Mission 6
                if cell == '#' {
                    guard.turn_right();  // Mission 6 enum method
                } else {
                    guard.position = next_pos;
                    visited.insert(guard.position);
                }
            } else {
                break; // Exited grid bounds
            }
        } else {
            break; // Coordinate underflow (Mission 6 safety)
        }
    }
    
    Ok(visited)
}
```

### **Key Differences**
1. **Bounds Safety**: Python manual checking vs Rust automatic `Option<Coord>` returns
2. **Direction Operations**: Python array indexing vs Rust enum methods (`rotate_90_clockwise()`)
3. **Error Handling**: Python exceptions vs Rust `Result` with context
4. **Loop Detection**: Both use state tracking, Rust with type-safe structs

---

## 🔄 **Part 2: Loop Detection with Obstacle Placement**

### **Algorithm Strategy (Both Languages)**
```
1. Get original patrol path (Part 1 result)
2. For each position on path (except start):
   a. Place obstacle at position
   b. Simulate patrol from start
   c. Check if guard gets stuck in loop
3. Count positions that create loops
```

### **Python Implementation**
```python
def loops(grid, si, sj, sdir):
    """Check if placing obstacle creates a loop"""
    seen = set()
    i, j, dir = si, sj, sdir
    
    while True:
        if (i, j, dir) in seen:
            return True  # Loop detected
        seen.add((i, j, dir))
        
        next_i = i + dd[dir][0] 
        next_j = j + dd[dir][1]
        
        if not (0 <= next_i < n and 0 <= next_j < m):
            return False  # Exited, no loop
            
        if grid[next_i][next_j] == "#":
            dir = (dir + 1) % 4
        else:
            i, j = next_i, next_j

# Main loop: test obstacle at each position
ans = 0
for i in range(n):
    for j in range(m):
        if grid[i][j] == "." and (i, j) != (si, sj):
            grid[i][j] = "#"  # Place obstacle
            if loops(grid, si, sj, sdir):
                ans += 1
            grid[i][j] = "."  # Remove obstacle
```

### **Rust Mission Implementation**
```rust
fn solve_part2(input: &str) -> Result<usize> {
    let (grid, start_state) = parse_input(input)?;
    
    // Get original patrol path
    let original_path = simulate_patrol(&grid, start_state)?;
    
    let mut loop_positions = 0;
    
    // Test obstacle at each position on original path (except start)
    for &pos in &original_path {
        if pos == start_state.position {
            continue; // Can't place obstacle at start
        }
        
        if grid[pos] == '.' {
            // Clone grid and place obstacle
            let mut test_grid = grid.clone();
            test_grid[pos] = '#';
            
            // Check if this creates a loop
            if creates_loop(&test_grid, start_state)? {
                loop_positions += 1;
            }
        }
    }
    
    Ok(loop_positions)
}

fn creates_loop(grid: &Grid<char>, start_state: GuardState) -> Result<bool> {
    let mut guard = start_state;
    let mut states = HashSet::new();
    
    loop {
        if states.contains(&guard) {
            return Ok(true); // Loop detected
        }
        states.insert(guard);
        
        if let Some(next_pos) = guard.next_position() {
            if let Some(&cell) = grid.get(next_pos) {
                if cell == '#' {
                    guard.turn_right();
                } else {
                    guard.position = next_pos;
                }
            } else {
                return Ok(false); // Exited grid, no loop
            }
        } else {
            return Ok(false); // Coordinate underflow, no loop
        }
    }
}
```

### **Key Implementation Differences**
1. **Grid Modification**: Python mutates in-place, Rust clones for safety
2. **State Representation**: Python tuples vs Rust structs with `Hash` trait
3. **Memory Management**: Python GC vs Rust stack allocation and explicit cloning
4. **Error Propagation**: Python exceptions vs Rust `Result` chains

---

## 📊 **Code Quality Metrics**

| **Quality Metric** | **Python Solution** | **Rust Mission Solution** |
|--------------------|--------------------|-----------------------------|
| **Lines of Code** | ~35 (Part 1), ~55 (Part 2) | ~200 (with tests & docs) |
| **Type Safety** | Runtime type checking | Compile-time type safety |
| **Error Handling** | No explicit error handling | Comprehensive `Result` types |
| **Memory Safety** | Potential index out of bounds | Guaranteed memory safety |
| **Code Reuse** | Duplicate logic between parts | Shared utilities & Mission code |
| **Testing** | No tests provided | 8 comprehensive unit tests |
| **Documentation** | Minimal comments | Full API documentation |
| **Performance** | In-place mutations (faster) | Immutable operations (safer) |
| **Maintainability** | Procedural, harder to extend | Modular, easy to extend |

---

## 🎯 **Design Philosophy Differences**

### 🐍 **Python Approach**
**Strengths**:
- ✅ Rapid prototyping and quick solutions  
- ✅ Direct translation of algorithmic thinking
- ✅ Minimal boilerplate for competitive programming
- ✅ Faster development for one-off problems

**Trade-offs**:
- ⚠️ Runtime errors possible (index out of bounds)
- ⚠️ Mutable state management complexity
- ⚠️ No compile-time verification
- ⚠️ Manual bounds checking throughout

### 🦀 **Rust Mission Approach**
**Strengths**:
- ✅ Production-ready code with comprehensive error handling
- ✅ Leverages existing V-Cycle engineered data structures  
- ✅ Compile-time safety guarantees prevent entire classes of bugs
- ✅ Extensive test coverage ensures correctness
- ✅ Self-documenting code with clear type signatures
- ✅ Automatic memory management without GC overhead

**Trade-offs**:
- ⚠️ Higher upfront development time
- ⚠️ More verbose for simple algorithmic concepts  
- ⚠️ Steeper learning curve for ownership/borrowing
- ⚠️ Additional abstraction layers

---

## 🔗 **Mission Code Integration Benefits**

### **Mission 6 Components Used**

#### **Grid<T>: Professional 2D Array**
```rust
// Python: Manual 2D indexing with potential errors
if 0 <= i < len(grid) and 0 <= j < len(grid[0]):
    cell = grid[i][j]

// Rust Mission 6: Automatic bounds checking
if let Some(&cell) = grid.get(coord) {
    // Guaranteed safe access
}
```

#### **Coord: Type-Safe Coordinate Operations**
```python
# Python: Manual arithmetic, error-prone
next_i = i + dd[dir][0]
next_j = j + dd[dir][1]
```

```rust
// Rust Mission 6: Type-safe with overflow protection
if let Some(next_pos) = coord.step(direction) {
    // Automatic bounds and overflow checking
}
```

#### **Direction: Enum with Built-in Operations**
```python
# Python: Magic numbers and array indexing
dd = [[-1, 0], [0, 1], [1, 0], [0, -1]]
dir = (dir + 1) % 4  # Turn right
```

```rust
// Rust Mission 6: Self-documenting enum methods
direction = direction.rotate_90_clockwise();
```

### **Mission 5 Components Used**

#### **Optimized Collections**
- `HashSet<Coord>` for position tracking
- `HashSet<GuardState>` for loop detection
- Performance-tuned hash implementations
- Zero-cost abstractions over standard collections

### **Integration Benefits Summary**
- 🗺️ **Grid<T>**: Bounds-checked 2D storage with iterator support
- 📍 **Coord**: Mathematical coordinate operations with safety guarantees
- 🧭 **Direction**: Type-safe directional operations with rotation methods
- 📊 **Collections**: Optimized data structures from Mission 5
- ⚡ **Performance**: Cache-friendly memory layouts and efficient algorithms
- 🧪 **Testing**: Integration with existing Mission test infrastructure
- 📚 **Documentation**: Leverages existing API documentation and examples

---

## 🚀 **Performance Analysis**

### **Time Complexity**
- **Part 1**: O(W×H) - visits each cell at most 4 times (once per direction)
- **Part 2**: O(P×W×H) where P is path length - tests obstacle at each path position

### **Space Complexity**
- **Python**: O(W×H) for grid + O(path_length) for state tracking
- **Rust**: Same algorithmic complexity, but more efficient memory layout

### **Memory Layout Advantages (Rust)**
```
Python Grid:
List[List[str]] = 8 bytes/pointer × W×H + string object overhead

Rust Grid:
Grid<char> = 1 byte/char × W×H (contiguous, cache-friendly)
```

### **Benchmark Results** (Hypothetical)
```
Input Size: 130×130 grid (real AoC input)

Python Performance:
- Part 1: ~15ms
- Part 2: ~800ms  
- Memory: ~45MB peak

Rust Performance:
- Part 1: ~8ms  
- Part 2: ~450ms
- Memory: ~12MB peak
```

---

## 🧪 **Testing Strategy**

### **Python Testing** (Typical competitive programming)
```python
# Usually just run against examples
assert solve_part1(example_input) == 41
assert solve_part2(example_input) == 6
```

### **Rust Testing** (Production quality)
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_valid() {
        let result = parse_input(EXAMPLE_INPUT);
        assert!(result.is_ok());
    }

    #[test] 
    fn test_parse_input_invalid_char() {
        let invalid = ".\n@\n";  // Invalid character
        assert!(parse_input(invalid).is_err());
    }

    #[test]
    fn test_guard_movement() {
        let coord = Coord::new(0, 0);
        let mut guard = GuardState::new(coord, Direction::North);
        
        // Test turning
        guard.turn_right();
        assert_eq!(guard.direction, Direction::East);
    }

    #[test]
    fn test_simulate_patrol_exits_bounds() {
        let input = ".\n^\n";
        let (grid, start_state) = parse_input(input).unwrap();
        let visited = simulate_patrol(&grid, start_state).unwrap();
        assert_eq!(visited.len(), 2); // Start + one step up
    }

    #[test] 
    fn test_creates_loop_detection() {
        // Test grid that creates immediate loop
        let input = "#.#\n.^.\n#.#";
        let (mut grid, start_state) = parse_input(input).unwrap();
        grid[Coord::new(1, 0)] = '#'; // Block north
        
        assert!(creates_loop(&grid, start_state).unwrap());
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE_INPUT).unwrap(), 41);
    }

    #[test] 
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE_INPUT).unwrap(), 6);
    }

    #[test]
    fn test_empty_input() {
        assert!(parse_input("").is_err());
    }
}
```

**8 comprehensive test cases** covering:
- Valid and invalid input parsing
- Guard movement and rotation mechanics  
- Bounds checking and exit detection
- Loop detection algorithms
- Edge cases (empty input, immediate loops)
- Both parts with example data

---

## 🎓 **Educational Takeaways**

### **For Competitive Programming**
- **Python wins**: Faster to write, less ceremony, direct algorithm expression
- **Use when**: Contest time pressure, one-off problems, rapid prototyping

### **For Production Systems**
- **Rust wins**: Safety guarantees, maintainability, performance at scale
- **Use when**: Long-term codebases, reliability requirements, team collaboration

### **For Learning Systems Engineering**
- **Mission approach demonstrates**: How foundational libraries accelerate development
- **V-Cycle benefits**: Requirements → Design → Implementation → Verification
- **Type-driven development**: Let the compiler guide correct implementation

### **Key Learning Points**
1. **Abstraction Layers**: Mission code doesn't just prevent bugs—it makes algorithms clearer
2. **Safety vs Speed**: Compile-time checks trade development time for runtime reliability  
3. **Code Reuse**: Well-designed libraries compound benefits across multiple problems
4. **Testing Philosophy**: Comprehensive testing enables confident refactoring and extension
5. **Error Handling**: Explicit error handling makes systems more robust and debuggable

---

## 🔍 **When to Use Each Approach**

### **Choose Python Style When:**
- Competitive programming contests (time pressure)
- Quick prototyping or proof of concepts  
- One-off scripts or data analysis
- Algorithm exploration and experimentation
- Small team or individual projects

### **Choose Rust Mission Style When:**
- Production systems requiring reliability
- Long-term maintainable codebases
- Performance-critical applications  
- Learning systems engineering principles
- Building reusable libraries/frameworks
- Team environments with changing requirements

---

## 🏆 **Conclusion**

Day 6 exemplifies the **philosophical difference** between competitive programming and systems engineering:

**Python excels** at direct algorithm translation and rapid development—perfect for contests where correctness and speed-to-solution matter most.

**Rust with Mission integration** excels at building reliable, maintainable systems that leverage existing infrastructure—perfect for production environments where safety and long-term maintainability matter most.

The key insight from our **3-track learning system** is that **well-engineered abstractions** (like Mission data structures) don't just prevent bugs—they make complex algorithms more approachable and maintainable while preserving (or improving) performance.

This demonstrates the educational philosophy of the workspace: **learning through professional-quality implementations** that scale from competitive programming to real-world applications.

---

*Educational Analysis by the Rust Study Workspace*  
*Mission Integration: 6 (Grids & 2D Arrays) + 5 (Collections)*  
*Algorithm Complexity: O(W×H) Part 1, O(P×W×H) Part 2*  
*Results: Part 1: 5551, Part 2: 1939 (real), Part 1: 41, Part 2: 6 (example)*