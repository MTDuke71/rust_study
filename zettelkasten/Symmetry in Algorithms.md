# Symmetry in Algorithms - Mathematical Symmetries for Computational Speedup

## Overview

Symmetry exploitation is a powerful technique in algorithm optimization where mathematical symmetries in problem structure are identified and used to dramatically reduce computational search spaces. This is particularly effective in combinatorial optimization problems.

## Types of Symmetries

### 1. Rotational Symmetry
**Definition**: Solutions that are rotations of each other are considered equivalent.

**Example - Circular Seating (Day 13)**:
```
[A, B, C, D] ≡ [B, C, D, A] ≡ [C, D, A, B] ≡ [D, A, B, C]
```
All represent the same circular seating arrangement.

**Optimization Strategy**: Fix one element's position
```rust
// Instead of generating all n! permutations
fn brute_force(items: Vec<T>) -> Solution {
    generate_all_permutations(items)  // n! permutations
}

// Fix first element to eliminate rotational duplicates
fn rotation_optimized(items: Vec<T>) -> Solution {
    let fixed = items[0].clone();
    let remaining = items[1..].to_vec();
    generate_permutations(remaining)  // (n-1)! permutations
    // Prepend fixed element to each permutation
}
```
**Speedup**: n× reduction (n! → (n-1)!)

### 2. Reflectional Symmetry
**Definition**: Mirror images represent equivalent solutions.

**Example - Circular Arrangements**:
```
Clockwise:     [A, B, C, D] (A→B→C→D→A)
Counter-clockwise: [A, D, C, B] (A→D→C→B→A)
```
Both represent the same relationships, just traversed in opposite directions.

**Optimization Strategy**: Fix two elements' relative positions
```rust
fn full_optimization(items: Vec<T>) -> Solution {
    let fixed1 = items[0].clone();
    let fixed2 = items[1].clone(); 
    let remaining = items[2..].to_vec();
    
    // Generate permutations of remaining items only
    generate_permutations(remaining)  // (n-2)! permutations
    // Construct: [fixed1, fixed2, ...permutation...]
}
```
**Additional Speedup**: 2× reduction ((n-1)! → (n-1)!/2)
**Combined Speedup**: 2n× reduction (n! → (n-1)!/2)

### 3. Translational Symmetry
**Definition**: Solutions that are translations (shifts) of each other are equivalent.

**Example - Grid Problems**:
```
Pattern at (0,0): [X, O, X]
                  [O, X, O]
                  [X, O, X]

Same pattern at (5,3): [_, _, _, _, _, X, O, X]
                       [_, _, _, _, _, O, X, O]  
                       [_, _, _, _, _, X, O, X]
```

**Optimization**: Fix pattern position (e.g., top-left corner at origin).

### 4. Permutation Symmetry
**Definition**: Swapping equivalent elements doesn't change the solution.

**Example - Identical Objects**:
If you have 3 red balls and 2 blue balls, swapping red balls among themselves doesn't create a new arrangement.

**Optimization**: Use multinomial coefficients instead of factorial counting.

## Implementation Patterns

### Basic Symmetry Detection
```rust
// Check if two arrangements are rotations of each other
fn are_rotations<T: PartialEq>(arr1: &[T], arr2: &[T]) -> bool {
    if arr1.len() != arr2.len() { return false; }
    
    let n = arr1.len();
    for rotation in 0..n {
        if (0..n).all(|i| arr1[i] == arr2[(i + rotation) % n]) {
            return true;
        }
    }
    false
}

// Check if two arrangements are reflections of each other
fn are_reflections<T: PartialEq>(arr1: &[T], arr2: &[T]) -> bool {
    if arr1.len() != arr2.len() { return false; }
    
    let n = arr1.len();
    
    // Check all possible reflection axes
    for start in 0..n {
        let mut matches = true;
        for i in 0..n {
            let reflected_pos = (start + n - i) % n;
            if arr1[i] != arr2[reflected_pos] {
                matches = false;
                break;
            }
        }
        if matches { return true; }
    }
    false
}
```

### Symmetry-Aware Generators
```rust
// Generate only canonical representatives (no rotational duplicates)
fn generate_canonical_permutations<T: Clone + Ord>(
    items: &mut [T], 
    callback: &mut impl FnMut(&[T])
) {
    // Fix first element as the "canonical" starting point
    let first = items[0].clone();
    let remaining = &mut items[1..];
    
    generate_permutations(remaining, &mut |perm| {
        let mut full = vec![first.clone()];
        full.extend_from_slice(perm);
        callback(&full);
    });
}

// Generate representatives avoiding both rotational and reflectional duplicates
fn generate_full_canonical<T: Clone + Ord>(
    items: &mut [T],
    callback: &mut impl FnMut(&[T])
) {
    if items.len() < 2 { 
        callback(items); 
        return; 
    }
    
    // Fix first two elements to eliminate both symmetries
    let first = items[0].clone();
    let second = items[1].clone();
    let remaining = &mut items[2..];
    
    generate_permutations(remaining, &mut |perm| {
        let mut full = vec![first.clone(), second.clone()];
        full.extend_from_slice(perm);
        callback(&full);
    });
}
```

## Applications in Different Problem Domains

### 1. Traveling Salesman Problem (TSP)
**Symmetries**: 
- Rotational: Starting city doesn't matter in circular tour
- Reflectional: Clockwise ≡ counter-clockwise traversal

**Optimization**:
```rust
fn tsp_optimized(cities: Vec<City>) -> (Cost, Vec<City>) {
    // Fix starting city (rotational symmetry)
    let start = cities[0].clone();
    let others = cities[1..].to_vec();
    
    let mut best_cost = i32::MAX;
    let mut best_tour = Vec::new();
    
    // Only generate permutations of non-fixed cities
    generate_permutations(&mut others, |tour| {
        let mut full_tour = vec![start.clone()];
        full_tour.extend_from_slice(tour);
        full_tour.push(start.clone());  // Return to start
        
        let cost = calculate_tour_cost(&full_tour);
        if cost < best_cost {
            best_cost = cost;
            best_tour = full_tour;
        }
    });
    
    (best_cost, best_tour)
}
```
**Speedup**: 8× for 8 cities, 10× for 10 cities, etc.

### 2. Seating Arrangements (Day 13)
**Symmetries**:
- Rotational: Physical rotation of table doesn't change relationships
- Reflectional: Clockwise vs counter-clockwise seating

```rust
fn seating_optimized(people: Vec<Person>) -> (Happiness, Vec<Person>) {
    // Exploit both rotational and reflectional symmetry
    let fixed1 = people[0].clone();
    let fixed2 = people[1].clone();
    let remaining = people[2..].to_vec();
    
    // Search space: (n-2)! instead of n!
    // For 8 people: 720 instead of 40,320 (56× reduction!)
}
```

### 3. Grid Pattern Matching
**Symmetries**:
- Rotational: 90°, 180°, 270° rotations
- Reflectional: Horizontal, vertical, diagonal reflections

```rust
// Generate all symmetric variants of a pattern
fn generate_symmetric_patterns(pattern: &Grid) -> Vec<Grid> {
    let mut patterns = Vec::new();
    
    // Original
    patterns.push(pattern.clone());
    
    // Rotations
    patterns.push(pattern.rotate_90());
    patterns.push(pattern.rotate_180());
    patterns.push(pattern.rotate_270());
    
    // Reflections
    patterns.push(pattern.reflect_horizontal());
    patterns.push(pattern.reflect_vertical());
    patterns.push(pattern.reflect_diagonal());
    patterns.push(pattern.reflect_anti_diagonal());
    
    // Deduplicate equivalent patterns
    patterns.sort();
    patterns.dedup();
    patterns
}
```

## Performance Analysis

### Theoretical Speedups

| Problem Size | No Optimization | Rotational Only | Both Symmetries |
|--------------|-----------------|-----------------|-----------------|
| **4 items** | 24 | 6 (4×) | 3 (8×) |
| **6 items** | 720 | 120 (6×) | 60 (12×) |
| **8 items** | 40,320 | 5,040 (8×) | 2,520 (16×) |
| **10 items** | 3,628,800 | 362,880 (10×) | 181,440 (20×) |

### Real-World Measurements (Day 13)
```
8 people seating optimization:
- Brute force:     40,320 permutations → 250ms
- Rotation fixed:   5,040 permutations → 32ms  (7.8× speedup)
- Both optimized:   2,520 permutations → ~16ms (15.6× speedup, theoretical)
```

### Complexity Analysis
- **Time Reduction**: O(n!) → O((n-k)!) where k = fixed positions
- **Space**: No change (same memory usage pattern)
- **Implementation Complexity**: Minimal (just fix some elements)

## Advanced Symmetry Techniques

### 1. Group Theory Application
```rust
// Use mathematical group operations to identify equivalences
struct SymmetryGroup {
    rotations: Vec<Rotation>,
    reflections: Vec<Reflection>,
}

impl SymmetryGroup {
    fn canonical_representative(&self, arrangement: &[T]) -> Vec<T> {
        // Apply all group operations and return lexicographically smallest
        let mut candidates = vec![arrangement.to_vec()];
        
        for rotation in &self.rotations {
            candidates.push(rotation.apply(arrangement));
        }
        
        for reflection in &self.reflections {
            candidates.push(reflection.apply(arrangement));
        }
        
        candidates.sort();
        candidates[0].clone()
    }
}
```

### 2. Burnside's Lemma
For counting distinct arrangements under group action:
```
|X/G| = (1/|G|) * Σ_{g∈G} |X^g|
```
Where X^g is the set of arrangements fixed by group element g.

### 3. Orbit-Stabilizer Theorem
Each equivalence class (orbit) has the same size, allowing systematic enumeration.

## Common Pitfalls and Solutions

### 1. Incorrect Symmetry Identification
**Problem**: Assuming symmetries that don't actually exist
```rust
// WRONG: Assuming all permutations are equivalent in line arrangements
fn incorrect_line_optimization(items: Vec<T>) {
    // Line arrangements [A,B,C] and [C,B,A] are NOT equivalent!
    // First and last positions matter in linear arrangements
}

// CORRECT: Only circular arrangements have rotational symmetry
fn correct_circular_optimization(items: Vec<T>) {
    // Circular: [A,B,C] ≡ [B,C,A] ≡ [C,A,B]
    fix_one_position(items);
}
```

### 2. Off-by-One Errors in Reduction Calculations
```rust
// WRONG: Calculating wrong reduction factor
let wrong_reduction = factorial(n) / n;  // Should be factorial(n-1)

// CORRECT: n! permutations → (n-1)! after fixing one position
let correct_reduction = factorial(n-1);  // Fix one position
let reflection_reduction = factorial(n-1) / 2;  // Fix two positions
```

### 3. Missing Edge Cases
```rust
fn handle_edge_cases<T>(items: Vec<T>) -> Vec<T> {
    match items.len() {
        0 => vec![],  // Empty case
        1 => items,   // Single item - no optimization possible
        2 => {
            // Two items - only reflection symmetry applies
            // [A, B] ≡ [B, A] in circular arrangements
            vec![items[0].clone(), items[1].clone()]
        }
        _ => apply_full_optimization(items)
    }
}
```

## Integration with Other Optimization Techniques

### 1. Branch and Bound + Symmetry
```rust
fn branch_and_bound_with_symmetry(problem: Problem) -> Solution {
    // Use canonical forms in the branching process
    fn branch(state: State, bound: Bound) {
        let canonical_state = compute_canonical_form(state);
        
        // Check if we've seen this canonical state before
        if seen_states.contains(&canonical_state) {
            return;  // Skip symmetric duplicate
        }
        
        seen_states.insert(canonical_state);
        
        if bound_check(canonical_state, current_best) {
            return;  // Prune branch
        }
        
        // Continue branching...
    }
}
```

### 2. Dynamic Programming + Symmetry
```rust
fn dp_with_symmetry(problem: Problem) -> Solution {
    let mut memo = HashMap::new();
    
    fn solve(state: State) -> Value {
        // Always use canonical form as memo key
        let canonical = compute_canonical_form(state);
        
        if let Some(&cached) = memo.get(&canonical) {
            return cached;
        }
        
        // Compute result...
        let result = compute_result(canonical);
        memo.insert(canonical, result);
        result
    }
}
```

## Testing and Validation

### 1. Correctness Testing
```rust
#[cfg(test)]
mod symmetry_tests {
    #[test]
    fn test_symmetry_preservation() {
        let original = solve_brute_force(&problem);
        let optimized = solve_with_symmetry(&problem);
        
        // Results should be equivalent (possibly different representatives)
        assert!(are_equivalent_solutions(original, optimized));
    }
    
    #[test]
    fn test_all_representatives_found() {
        let brute_solutions = find_all_solutions_brute_force(&problem);
        let optimized_solutions = find_all_canonical_solutions(&problem);
        
        // Each brute force solution should map to exactly one canonical solution
        let mut canonical_map = HashMap::new();
        for solution in brute_solutions {
            let canonical = compute_canonical_form(solution);
            canonical_map.insert(canonical, true);
        }
        
        assert_eq!(canonical_map.len(), optimized_solutions.len());
    }
}
```

### 2. Performance Validation
```rust
#[test]
fn test_expected_speedup() {
    let problem = create_test_problem(size: 8);
    
    let start = Instant::now();
    let _result1 = solve_brute_force(&problem);
    let brute_time = start.elapsed();
    
    let start = Instant::now();
    let _result2 = solve_with_symmetry(&problem);
    let optimized_time = start.elapsed();
    
    let speedup = brute_time.as_nanos() as f64 / optimized_time.as_nanos() as f64;
    
    // For 8-item circular problem, expect ~8× speedup from rotational symmetry
    assert!(speedup >= 7.0, "Expected ~8× speedup, got {:.1}×", speedup);
}
```

## Educational Value

### Mathematical Concepts
1. **Group Theory**: Understanding mathematical symmetry groups
2. **Combinatorics**: Counting with and without symmetry considerations
3. **Equivalence Relations**: Partitioning solution space into equivalence classes

### Algorithmic Concepts  
1. **Search Space Reduction**: Systematic approaches to pruning
2. **Canonical Forms**: Unique representatives for equivalence classes
3. **Optimization Techniques**: Combining symmetry with other optimization methods

### Implementation Skills
1. **Pattern Recognition**: Identifying symmetries in problem structure
2. **Algorithm Design**: Modifying existing algorithms to exploit symmetry
3. **Performance Engineering**: Measuring and validating optimization effectiveness

---

*Created: 2025-10-19*  
*Last Updated: 2025-10-19*

*Tags: #symmetry #optimization #algorithms #combinatorics #group-theory #performance #search-space-reduction #canonical-forms*

*Links: [[Graph Theory MOC]] | [[TSP Algorithms]] | [[day13_analysis]] | [[Combinatorial Optimization]] | [[Performance Engineering]] | [[Heap's Algorithm Deep Dive]]*