# Box<T> - Heap Allocation for Recursive Structures

**Tags:** #box #heap-allocation #recursive-types #smart-pointers #rust-book-ch15 #aoc-applications #tree-structures #linked-lists

**Related:** [[deref-trait]], [[drop-trait]], [[recursive-data-structures]], [[heap-vs-stack]], [[ownership]], [[smart-pointers]]

---

## Core Concept

**Box<T>** provides **heap allocation** for data with **known size at compile time**. It's essential for **recursive data structures** where the compiler needs to determine the size of self-referential types.

```rust
// Stack allocation (known size)
let x = 5;

// Heap allocation (pointer to heap)
let y = Box::new(5);
```

### **Why Box<T> is Needed for Recursion**
```rust
// ❌ This won't compile - infinite size
enum List {
    Cons(i32, List),  // How big is List? Infinite recursion!
    Nil,
}

// ✅ This works - Box<T> has known pointer size
enum List {
    Cons(i32, Box<List>),  // Box is pointer-sized (8 bytes on 64-bit)
    Nil,
}
```

---

## Basic Usage Patterns

### **Simple Heap Allocation**
```rust
fn basic_box_usage() {
    let b = Box::new(5);
    println!("b = {}", b);  // Deref coercion: *b automatically
    
    // Box owns the data - moved when box is moved
    let b2 = b;  // b is no longer valid
    // println!("{}", b);  // ❌ Error: b moved
    println!("b2 = {}", b2);  // ✅ Works
} // b2's memory automatically freed when out of scope
```

### **Large Data on Heap**
```rust
// Large array on stack might cause stack overflow
// let large_array = [0; 1_000_000];  // 4MB on stack!

// ✅ Put large data on heap
let large_array = Box::new([0; 1_000_000]);  // 8 bytes on stack, 4MB on heap
```

---

## AoC Applications - Tree Structures

### **Binary Tree for AoC Problems**
```rust
#[derive(Debug, Clone)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
    
    fn insert(&mut self, val: i32) {
        if val <= self.val {
            match &mut self.left {
                Some(node) => node.insert(val),
                None => self.left = Some(Box::new(TreeNode::new(val))),
            }
        } else {
            match &mut self.right {
                Some(node) => node.insert(val),
                None => self.right = Some(Box::new(TreeNode::new(val))),
            }
        }
    }
    
    fn height(&self) -> usize {
        let left_height = self.left.as_ref().map_or(0, |node| node.height());
        let right_height = self.right.as_ref().map_or(0, |node| node.height());
        1 + left_height.max(right_height)
    }
    
    fn sum(&self) -> i32 {
        self.val 
            + self.left.as_ref().map_or(0, |node| node.sum())
            + self.right.as_ref().map_or(0, |node| node.sum())
    }
}

// AoC usage: Tree problems (directory sizes, expression evaluation)
fn solve_tree_problem(input: &[i32]) -> i32 {
    let mut root = TreeNode::new(input[0]);
    
    for &val in &input[1..] {
        root.insert(val);
    }
    
    root.sum()  // Or height(), or any tree operation
}
```

### **AoC 2015 Day 7: Circuit Evaluation (Expression Trees)**
```rust
#[derive(Debug, Clone)]
enum Gate {
    Wire(String),
    Value(u16),
    And(Box<Gate>, Box<Gate>),
    Or(Box<Gate>, Box<Gate>),
    Not(Box<Gate>),
    LeftShift(Box<Gate>, u16),
    RightShift(Box<Gate>, u16),
}

impl Gate {
    fn evaluate(&self, wires: &std::collections::HashMap<String, u16>) -> Option<u16> {
        match self {
            Gate::Wire(name) => wires.get(name).copied(),
            Gate::Value(val) => Some(*val),
            Gate::And(left, right) => {
                Some(left.evaluate(wires)? & right.evaluate(wires)?)
            },
            Gate::Or(left, right) => {
                Some(left.evaluate(wires)? | right.evaluate(wires)?)
            },
            Gate::Not(gate) => {
                Some(!gate.evaluate(wires)?)
            },
            Gate::LeftShift(gate, shift) => {
                Some(gate.evaluate(wires)? << shift)
            },
            Gate::RightShift(gate, shift) => {
                Some(gate.evaluate(wires)? >> shift)
            },
        }
    }
}

// Parse AoC input into expression tree
fn parse_gate(instruction: &str) -> (String, Gate) {
    let parts: Vec<&str> = instruction.split(" -> ").collect();
    let output = parts[1].to_string();
    let input = parts[0];
    
    let gate = if input.contains(" AND ") {
        let operands: Vec<&str> = input.split(" AND ").collect();
        Gate::And(
            Box::new(parse_operand(operands[0])),
            Box::new(parse_operand(operands[1])),
        )
    } else if input.contains(" OR ") {
        let operands: Vec<&str> = input.split(" OR ").collect();
        Gate::Or(
            Box::new(parse_operand(operands[0])),
            Box::new(parse_operand(operands[1])),
        )
    } else if input.starts_with("NOT ") {
        Gate::Not(Box::new(parse_operand(&input[4..])))
    } else if input.contains(" LSHIFT ") {
        let operands: Vec<&str> = input.split(" LSHIFT ").collect();
        Gate::LeftShift(
            Box::new(parse_operand(operands[0])),
            operands[1].parse().unwrap(),
        )
    } else if input.contains(" RSHIFT ") {
        let operands: Vec<&str> = input.split(" RSHIFT ").collect();
        Gate::RightShift(
            Box::new(parse_operand(operands[0])),
            operands[1].parse().unwrap(),
        )
    } else {
        parse_operand(input)
    };
    
    (output, gate)
}

fn parse_operand(s: &str) -> Gate {
    if let Ok(val) = s.parse::<u16>() {
        Gate::Value(val)
    } else {
        Gate::Wire(s.to_string())
    }
}

// Solve AoC circuit evaluation problem
fn solve_circuit(instructions: &[String]) -> u16 {
    let mut gates = std::collections::HashMap::new();
    let mut wires = std::collections::HashMap::new();
    
    for instruction in instructions {
        let (output, gate) = parse_gate(instruction);
        gates.insert(output, gate);
    }
    
    // Evaluate recursively with memoization
    fn evaluate_wire(
        wire: &str, 
        gates: &std::collections::HashMap<String, Gate>,
        wires: &mut std::collections::HashMap<String, u16>
    ) -> u16 {
        if let Some(&value) = wires.get(wire) {
            return value;
        }
        
        let gate = gates.get(wire).unwrap();
        let value = gate.evaluate(wires).unwrap_or_else(|| {
            // Recursively evaluate dependencies
            evaluate_dependencies(gate, gates, wires)
        });
        
        wires.insert(wire.to_string(), value);
        value
    }
    
    evaluate_wire("a", &gates, &mut wires)
}
```

---

## AoC Applications - Linked Lists

### **Linked List for Sequence Problems**
```rust
#[derive(Debug)]
struct ListNode<T> {
    val: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    fn new(val: T) -> Self {
        ListNode { val, next: None }
    }
    
    fn append(&mut self, val: T) {
        match &mut self.next {
            Some(node) => node.append(val),
            None => self.next = Some(Box::new(ListNode::new(val))),
        }
    }
    
    fn length(&self) -> usize {
        1 + self.next.as_ref().map_or(0, |node| node.length())
    }
    
    fn find(&self, target: &T) -> bool 
    where 
        T: PartialEq 
    {
        self.val == *target || 
        self.next.as_ref().map_or(false, |node| node.find(target))
    }
    
    fn collect_values(&self) -> Vec<&T> {
        let mut values = vec![&self.val];
        if let Some(node) = &self.next {
            values.extend(node.collect_values());
        }
        values
    }
}

// AoC usage: Sequence processing, state chains
fn process_sequence<T>(items: Vec<T>) -> ListNode<T> 
where 
    T: Clone 
{
    let mut head = ListNode::new(items[0].clone());
    
    for item in &items[1..] {
        head.append(item.clone());
    }
    
    head
}
```

### **AoC Josephus Problem Variant**
```rust
struct CircularList {
    val: usize,
    next: Option<Box<CircularList>>,
}

impl CircularList {
    fn from_range(n: usize) -> Option<Box<Self>> {
        if n == 0 { return None; }
        
        let mut head = Box::new(CircularList { val: 1, next: None });
        let mut current = &mut head;
        
        for i in 2..=n {
            current.next = Some(Box::new(CircularList { val: i, next: None }));
            current = current.next.as_mut().unwrap();
        }
        
        Some(head)
    }
    
    fn eliminate_every_nth(mut self: Box<Self>, n: usize) -> usize {
        // Convert to circular by finding tail and linking to head
        let mut tail = &mut self;
        while tail.next.is_some() {
            tail = tail.next.as_mut().unwrap();
        }
        
        // Josephus elimination algorithm
        let mut current = self;
        while current.next.as_ref().map_or(false, |node| node.val != current.val) {
            // Skip n-1 positions
            for _ in 0..n-1 {
                if let Some(next) = current.next.take() {
                    current = next;
                }
            }
            
            // Remove the nth person
            if let Some(next) = current.next.take() {
                current.next = next.next;
            }
        }
        
        current.val
    }
}
```

---

## Performance Characteristics

### **Memory Layout and Access**
```rust
// Stack-based access (fast)
let stack_array = [1, 2, 3, 4, 5];
let sum: i32 = stack_array.iter().sum();  // Very fast - cache friendly

// Heap-based access (indirection cost)
let heap_array = Box::new([1, 2, 3, 4, 5]);
let sum: i32 = heap_array.iter().sum();   // One indirection to access
```

### **Allocation Costs**
```rust
use std::time::Instant;

fn compare_allocation_performance() {
    // Stack allocation - essentially free
    let start = Instant::now();
    for _ in 0..1_000_000 {
        let _x = [0; 100];  // Stack allocation
    }
    println!("Stack: {:?}", start.elapsed());
    
    // Heap allocation - more expensive
    let start = Instant::now();
    for _ in 0..1_000_000 {
        let _x = Box::new([0; 100]);  // Heap allocation
    }
    println!("Heap: {:?}", start.elapsed());
}
```

### **Cache Locality Considerations**
```rust
// ❌ Poor cache locality - many heap allocations
struct BadLinkedList {
    val: i32,
    next: Option<Box<BadLinkedList>>,
}

// ✅ Better cache locality - contiguous storage
struct GoodVector(Vec<i32>);

// For AoC: Consider Vec<T> over linked structures when possible
fn process_sequence_efficiently(items: Vec<i32>) -> i32 {
    items.iter().sum()  // Much faster than linked list traversal
}
```

---

## Advanced Box Patterns

### **Box with Custom Types**
```rust
trait AocSolver {
    fn solve(&self, input: &str) -> String;
}

struct Day1Solver;
struct Day2Solver;

impl AocSolver for Day1Solver {
    fn solve(&self, input: &str) -> String {
        // Day 1 logic
        input.lines().count().to_string()
    }
}

impl AocSolver for Day2Solver {
    fn solve(&self, input: &str) -> String {
        // Day 2 logic  
        input.len().to_string()
    }
}

// Store different solvers in uniform way
fn create_solver(day: u32) -> Box<dyn AocSolver> {
    match day {
        1 => Box::new(Day1Solver),
        2 => Box::new(Day2Solver),
        _ => panic!("Unknown day"),
    }
}
```

### **Recursive Data with Shared Ownership**
```rust
use std::rc::Rc;

#[derive(Debug)]
struct SharedTreeNode {
    val: i32,
    left: Option<Rc<RefCell<SharedTreeNode>>>,
    right: Option<Rc<RefCell<SharedTreeNode>>>,
    parent: Option<std::rc::Weak<RefCell<SharedTreeNode>>>,
}

// When you need upward navigation in tree (parent pointers)
// Box<T> for single ownership, Rc<RefCell<T>> for shared
```

---

## Testing Patterns

### **Box Behavior Tests**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_operations() {
        let mut tree = TreeNode::new(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(1);
        tree.insert(9);
        
        assert_eq!(tree.height(), 3);
        assert_eq!(tree.sum(), 25);
    }
    
    #[test]
    fn test_linked_list_operations() {
        let mut list = ListNode::new(1);
        list.append(2);
        list.append(3);
        
        assert_eq!(list.length(), 3);
        assert!(list.find(&2));
        assert!(!list.find(&5));
        
        let values = list.collect_values();
        assert_eq!(values, vec![&1, &2, &3]);
    }
    
    #[test]
    fn test_memory_safety() {
        let data = Box::new(vec![1, 2, 3, 4, 5]);
        
        // Box owns the data exclusively
        let moved_data = data;
        // data is no longer accessible
        
        assert_eq!(moved_data.len(), 5);
        // When moved_data goes out of scope, memory is freed
    }
}
```

---

## Common AoC Use Cases

### **1. Expression Parsing (AST)**
```rust
#[derive(Debug)]
enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn evaluate(&self) -> i64 {
        match self {
            Expr::Number(n) => *n,
            Expr::Add(left, right) => left.evaluate() + right.evaluate(),
            Expr::Mul(left, right) => left.evaluate() * right.evaluate(),
            Expr::Sub(left, right) => left.evaluate() - right.evaluate(),
            Expr::Div(left, right) => left.evaluate() / right.evaluate(),
        }
    }
}

// Parse "2 + 3 * 4" into AST
fn parse_expression(tokens: &[&str]) -> Expr {
    // Recursive descent parser using Box for sub-expressions
    // ...implementation details...
    Expr::Add(
        Box::new(Expr::Number(2)),
        Box::new(Expr::Mul(
            Box::new(Expr::Number(3)),
            Box::new(Expr::Number(4)),
        ))
    )
}
```

### **2. File System Tree (Directory Traversal)**
```rust
#[derive(Debug)]
struct FileNode {
    name: String,
    size: usize,
    children: Vec<Box<FileNode>>,
}

impl FileNode {
    fn total_size(&self) -> usize {
        self.size + self.children.iter().map(|child| child.total_size()).sum::<usize>()
    }
    
    fn find_directories_over_size(&self, threshold: usize) -> Vec<&str> {
        let mut result = Vec::new();
        
        if self.total_size() >= threshold {
            result.push(&self.name);
        }
        
        for child in &self.children {
            result.extend(child.find_directories_over_size(threshold));
        }
        
        result
    }
}

// AoC 2022 Day 7 style problem
fn solve_filesystem_problem(input: &str) -> usize {
    let root = parse_filesystem(input);
    
    root.find_directories_over_size(100_000)
        .iter()
        .map(|_| root.total_size())
        .sum()
}
```

### **3. Game State Trees (Decision Trees)**
```rust
#[derive(Debug, Clone)]
struct GameState {
    board: Vec<Vec<char>>,
    current_player: u8,
    moves_played: Vec<(usize, usize)>,
}

#[derive(Debug)]
struct GameNode {
    state: GameState,
    children: Vec<Box<GameNode>>,
    score: Option<i32>,
}

impl GameNode {
    fn minimax(&mut self, depth: usize, maximize: bool) -> i32 {
        if depth == 0 || self.is_terminal() {
            return self.evaluate();
        }
        
        if self.children.is_empty() {
            self.generate_children();
        }
        
        if maximize {
            let mut max_eval = i32::MIN;
            for child in &mut self.children {
                let eval = child.minimax(depth - 1, false);
                max_eval = max_eval.max(eval);
            }
            max_eval
        } else {
            let mut min_eval = i32::MAX;
            for child in &mut self.children {
                let eval = child.minimax(depth - 1, true);
                min_eval = min_eval.min(eval);
            }
            min_eval
        }
    }
    
    fn generate_children(&mut self) {
        // Generate all possible next moves as Box<GameNode>
        for (row, col) in self.get_valid_moves() {
            let mut new_state = self.state.clone();
            new_state.make_move(row, col);
            
            self.children.push(Box::new(GameNode {
                state: new_state,
                children: Vec::new(),
                score: None,
            }));
        }
    }
}
```

---

## Box vs Alternatives

### **When to Use Box<T>**
```rust
// ✅ Recursive data structures
enum List<T> { Cons(T, Box<List<T>>), Nil }

// ✅ Large data on heap
let big_data = Box::new([0; 1_000_000]);

// ✅ Trait objects (single ownership)
let solver: Box<dyn AocSolver> = Box::new(Day1Solver);

// ✅ Transferring ownership to heap
fn move_to_heap<T>(data: T) -> Box<T> { Box::new(data) }
```

### **When to Use Alternatives**
```rust
// ✅ Use Vec<T> instead of linked lists (usually faster)
let sequence = vec![1, 2, 3, 4, 5];  // Not Box<Node>

// ✅ Use Rc<T> for shared ownership
let shared_data = Rc::new(expensive_computation());
let reference1 = shared_data.clone();
let reference2 = shared_data.clone();

// ✅ Use Arc<T> for thread-safe sharing
let shared_across_threads = Arc::new(thread_safe_data);

// ✅ Use references when borrowing is sufficient
fn process_data(data: &[i32]) -> i32 { /* ... */ }  // No Box needed
```

---

## Best Practices

### **✅ Efficient Box Usage**
1. **Prefer Vec<T> over linked lists** for sequential data
2. **Use Box<T> for truly recursive structures** (trees, ASTs)
3. **Consider Rc<T>/Arc<T>** when sharing is needed
4. **Profile heap allocation costs** in performance-critical code
5. **Design for cache locality** when possible

### **✅ AoC-Specific Tips**
1. **Parse input into tree structures** for hierarchical problems
2. **Use Box<dyn Trait>** for polymorphic solver patterns
3. **Build ASTs for expression evaluation** problems
4. **Consider iterative algorithms** to avoid deep recursion
5. **Balance heap usage with performance** requirements

### **❌ Common Pitfalls**
```rust
// ❌ Unnecessary boxing of primitives
let x = Box::new(42);  // Just use: let x = 42;

// ❌ Boxing when references work
fn process_box(data: Box<Vec<i32>>) { /* ... */ }  
// Better: fn process_ref(data: &[i32]) { /* ... */ }

// ❌ Deep recursion causing stack overflow
fn deep_sum(list: &List) -> i32 {
    match list {
        List::Cons(val, next) => val + deep_sum(next),  // Stack grows!
        List::Nil => 0,
    }
}

// ✅ Iterative when possible
fn iterative_sum(mut list: &List) -> i32 {
    let mut sum = 0;
    while let List::Cons(val, next) = list {
        sum += val;
        list = next;
    }
    sum
}
```

---

## Advanced Topics

### **Box and Lifetimes**
```rust
struct Container<'a> {
    data: Box<&'a str>,  // Box can contain borrowed data
}

// Lifetime flows through Box transparently
fn create_container(s: &str) -> Container {
    Container {
        data: Box::new(s),
    }
}
```

### **Custom Allocators (Advanced)**
```rust
use std::alloc::{GlobalAlloc, Layout, System};

// Box can work with custom allocators (unstable feature)
// Useful for specialized memory management in some AoC problems
```

---

**Core Concepts:** [[heap-allocation]] | [[recursive-data-structures]] | [[smart-pointers]] | [[ownership]]  
**AoC Applications:** [[tree-problems]] | [[expression-parsing]] | [[linked-sequences]] | [[game-trees]]  
**Performance:** [[heap-vs-stack]] | [[cache-locality]] | [[allocation-costs]] | [[memory-layout]]  
**Integration:** [[deref-trait]] | [[drop-trait]] | [[trait-objects]] | [[rc-arc-comparison]]

*Links: [[box-deref-behavior]] | [[recursive-type-sizing]] | [[heap-allocation-patterns]] | [[aoc-tree-algorithms]] | [[memory-safety-with-box]]*