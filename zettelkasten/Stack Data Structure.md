# Stack Data Structure

**Tags:** #data-structures #stack #algorithms #lifo #rust
**Created:** 2025-10-22
**Related:** [[Vec Type]], [[Generic Programming]], [[Brackets Basic]], [[Mission1]], [[Collections MOC]]

## Overview

A **stack** is a Last-In-First-Out (LIFO) data structure where elements are added and removed from the same end, called the "top" of the stack. It's fundamental to many algorithms, especially for parsing, expression evaluation, and backtracking.

## Core Operations

### Essential Stack Operations

1. **Push** - Add element to top
2. **Pop** - Remove and return top element  
3. **Peek/Top** - View top element without removing
4. **IsEmpty** - Check if stack is empty
5. **Size** - Get number of elements

### LIFO Principle

```
Push(1) → [1]
Push(2) → [1, 2]  
Push(3) → [1, 2, 3]
Pop()   → returns 3, stack becomes [1, 2]
Pop()   → returns 2, stack becomes [1]
```

## Implementation in Rust

### Basic Stack Using Vec

```rust
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            items: Vec::new(),
        }
    }
    
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    fn len(&self) -> usize {
        self.items.len()
    }
}
```

### Usage Example

```rust
let mut stack = Stack::new();

// Push elements
stack.push(10);
stack.push(20);
stack.push(30);

// Pop elements (LIFO order)
assert_eq!(stack.pop(), Some(30));
assert_eq!(stack.pop(), Some(20));
assert_eq!(stack.peek(), Some(&10));
assert_eq!(stack.pop(), Some(10));
assert_eq!(stack.pop(), None);
```

### Direct Vec as Stack

Rust's `Vec<T>` already implements stack operations efficiently:

```rust
let mut stack: Vec<i32> = Vec::new();

// Stack operations using Vec methods
stack.push(1);
stack.push(2);
stack.push(3);

let top = stack.pop();        // Some(3)
let peek = stack.last();      // Some(&2) 
let is_empty = stack.is_empty();  // false
```

## Common Stack Applications

### 1. Bracket/Parentheses Matching

```rust
fn is_balanced(s: &str) -> bool {
    let mut stack = Vec::new();
    
    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') { return false; }
            },
            ']' => {
                if stack.pop() != Some('[') { return false; }
            },
            '}' => {
                if stack.pop() != Some('{') { return false; }
            },
            _ => {} // Ignore other characters
        }
    }
    
    stack.is_empty()
}

// Examples
assert_eq!(is_balanced("()[]{}"), true);
assert_eq!(is_balanced("([)]"), false);
assert_eq!(is_balanced("((()))"), true);
```

### 2. Expression Evaluation

```rust
// Evaluate postfix expression: "3 4 + 2 *" = 14
fn eval_postfix(expr: &str) -> i32 {
    let mut stack = Vec::new();
    
    for token in expr.split_whitespace() {
        match token {
            "+" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a + b);
            },
            "-" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a - b);
            },
            "*" => {
                let b = stack.pop().unwrap();  
                let a = stack.pop().unwrap();
                stack.push(a * b);
            },
            num => stack.push(num.parse().unwrap()),
        }
    }
    
    stack.pop().unwrap()
}
```

### 3. Function Call Stack (Conceptual)

```rust
fn factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)  // Each call pushed onto call stack
    }
}

// Call stack for factorial(3):
// factorial(3) → 3 * factorial(2)
//                ↳ factorial(2) → 2 * factorial(1) 
//                                  ↳ factorial(1) → 1
// Result bubbles back up: 1 → 2*1 → 3*2 → 6
```

### 4. Undo Operations

```rust
struct UndoableEditor {
    content: String,
    history: Vec<String>,
}

impl UndoableEditor {
    fn new() -> Self {
        UndoableEditor {
            content: String::new(),
            history: Vec::new(),
        }
    }
    
    fn edit(&mut self, new_content: String) {
        // Save current state before changing
        self.history.push(self.content.clone());
        self.content = new_content;
    }
    
    fn undo(&mut self) -> bool {
        if let Some(previous_state) = self.history.pop() {
            self.content = previous_state;
            true
        } else {
            false  // Nothing to undo
        }
    }
}
```

### 5. Depth-First Search (DFS)

```rust
fn dfs_iterative(graph: &[Vec<usize>], start: usize) -> Vec<usize> {
    let mut visited = vec![false; graph.len()];
    let mut stack = Vec::new();
    let mut result = Vec::new();
    
    stack.push(start);
    
    while let Some(node) = stack.pop() {
        if !visited[node] {
            visited[node] = true;
            result.push(node);
            
            // Add neighbors to stack (reverse order for left-to-right traversal)
            for &neighbor in graph[node].iter().rev() {
                if !visited[neighbor] {
                    stack.push(neighbor);
                }
            }
        }
    }
    
    result
}
```

## Performance Characteristics

### Time Complexity

- **Push:** O(1) amortized (Vec may need to resize)
- **Pop:** O(1)
- **Peek:** O(1)
- **IsEmpty:** O(1)

### Space Complexity

- **O(n)** where n is the number of elements

### Memory Layout

```rust
// Vec<T> provides optimal stack performance
// Elements stored contiguously in memory
// Only grows at one end (top of stack)

let mut stack = Vec::with_capacity(100);  // Pre-allocate to avoid resizing
```

## Advanced Stack Patterns

### Stack with Maximum Tracking

```rust
struct MaxStack {
    items: Vec<i32>,
    max_stack: Vec<i32>,  // Parallel stack tracking maximums
}

impl MaxStack {
    fn new() -> Self {
        MaxStack {
            items: Vec::new(),
            max_stack: Vec::new(),
        }
    }
    
    fn push(&mut self, val: i32) {
        self.items.push(val);
        
        let new_max = if let Some(&current_max) = self.max_stack.last() {
            val.max(current_max)
        } else {
            val
        };
        self.max_stack.push(new_max);
    }
    
    fn pop(&mut self) -> Option<i32> {
        self.max_stack.pop();
        self.items.pop()
    }
    
    fn max(&self) -> Option<i32> {
        self.max_stack.last().copied()
    }
}
```

### Generic Stack with Bounds

```rust
use std::fmt::Debug;

struct BoundedStack<T: Debug> {
    items: Vec<T>,
    max_size: usize,
}

impl<T: Debug> BoundedStack<T> {
    fn new(max_size: usize) -> Self {
        BoundedStack {
            items: Vec::with_capacity(max_size),
            max_size,
        }
    }
    
    fn push(&mut self, item: T) -> Result<(), T> {
        if self.items.len() >= self.max_size {
            Err(item)  // Stack overflow
        } else {
            self.items.push(item);
            Ok(())
        }
    }
    
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}
```

## Use Cases in Rust Study Projects

### Advanced Examples - Brackets

- **Basic bracket matching:** Simple parentheses validation
- **Extended brackets:** Multiple bracket types with nesting rules
- **Syntax validation:** Ensuring proper bracket pairing in expressions

### Mission 1 - Fundamentals

- **Learning progression:** Stack as introductory data structure
- **Problem solving:** Using stacks for algorithm challenges
- **Memory understanding:** Stack vs heap concepts

### Competitive Programming

- **Parsing problems:** Expression evaluation and syntax checking
- **Backtracking algorithms:** Maintaining state for exploration
- **Tree traversal:** Iterative depth-first search implementation

### Real-world Applications

- **Compiler design:** Managing scope and syntax parsing
- **Browser history:** Back/forward navigation
- **Game development:** Undo systems and state management

## Common Patterns and Idioms

### Pattern: Processing with Temporary Storage

```rust
fn process_nested_structure(input: &[char]) -> Vec<String> {
    let mut stack = Vec::new();
    let mut results = Vec::new();
    let mut current = String::new();
    
    for &ch in input {
        match ch {
            '(' => {
                stack.push(current);
                current = String::new();
            },
            ')' => {
                results.push(current);
                current = stack.pop().unwrap_or_default();
            },
            c => current.push(c),
        }
    }
    
    results
}
```

### Pattern: State Machine with Backtracking

```rust
fn find_path(maze: &[Vec<char>], start: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let mut stack = vec![(start, vec![start])];
    let mut visited = std::collections::HashSet::new();
    
    while let Some((pos, path)) = stack.pop() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        
        if maze[pos.0][pos.1] == 'E' {  // Found exit
            return Some(path);
        }
        
        // Add neighbors to stack with extended path
        for next_pos in get_neighbors(maze, pos) {
            if !visited.contains(&next_pos) {
                let mut new_path = path.clone();
                new_path.push(next_pos);
                stack.push((next_pos, new_path));
            }
        }
    }
    
    None
}
```

## Testing Stack Implementations

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_operations() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());
        
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.peek(), Some(&2));
        
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        assert!(stack.is_empty());
    }
    
    #[test]  
    fn test_lifo_order() {
        let mut stack = Stack::new();
        for i in 1..=5 {
            stack.push(i);
        }
        
        let mut result = Vec::new();
        while let Some(item) = stack.pop() {
            result.push(item);
        }
        
        assert_eq!(result, vec![5, 4, 3, 2, 1]);
    }
}
```

## Related Concepts

- [[Vec Type]] - Primary implementation backing for Rust stacks
- [[Generic Programming]] - Creating type-generic stack implementations
- [[Brackets Basic]] - Practical application in bracket matching
- [[Mission1]] - Learning foundation and problem-solving context
- [[Collections MOC]] - Relationship to other data structures

## Quick Reference

```rust
// Using Vec<T> as stack
let mut stack = Vec::new();
stack.push(item);           // Add to top
stack.pop()                 // Remove from top (returns Option<T>)
stack.last()                // Peek at top (returns Option<&T>)
stack.is_empty()            // Check if empty
stack.len()                 // Get size

// Custom stack wrapper
struct Stack<T> { items: Vec<T> }
// Implement push(), pop(), peek(), is_empty(), len()
```

---

*LIFO data structure essential for parsing, backtracking, and maintaining program state.*
