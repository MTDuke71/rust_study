# 🔥 Mission 1 (V-cycle style): Stack

## 🌟 **Foundational Mission**

Mission1 is where this entire learning journey began. It established:
- The **ownership mental model** (library analogy)
- The **V-cycle methodology** used throughout
- The **testing discipline** with requirements traceability
- The **mental models** that make Rust intuitive

**See:** [FOUNDATIONAL_CONCEPTS.md](FOUNDATIONAL_CONCEPTS.md) for the complete origin story and key insights.

---

## 📚 **Deep Dive: Knowledge Resources**

This mission's insights have been preserved in the zettelkasten system:

### **[Ownership Mental Model - The Library Analogy](../../zettelkasten/Ownership%20Mental%20Model%20-%20The%20Library%20Analogy.md)** 📖
*The powerful metaphor that makes ownership intuitive*

Learn how memory is like a library, values are books, owners are librarians, and borrows are reading/editing passes.

### **[V-Cycle in Rust Development](../../zettelkasten/V-Cycle%20in%20Rust%20Development.md)** 🔄
*The requirements-driven methodology Mission1 established*

Understand how formal software engineering integrates with Rust's compiler-enforced guarantees.

### **[Rust Learning Roadmap - The Master Plan](../../zettelkasten/Rust%20Learning%20Roadmap%20-%20The%20Master%20Plan.md)** 🗺️
*The original phase-by-phase strategy*

See where Mission1 fits in the complete learning journey from basics to capstone.

### **[Project Origin Story](../../zettelkasten/PROJECT_ORIGIN.md)** 🎯
*How this workspace began*

Read the founding conversation that defined the interactive agent mode and learning philosophy.

---

## 1. Requirements

REQ-1: The stack shall store values of any generic type T.

REQ-2: The stack shall allow push in amortized O(1).

REQ-3: The stack shall allow pop in O(1), transferring ownership of the top element.

REQ-4: The stack shall not allow access to values after they have been popped.

REQ-5: The stack shall allow immutable (peek) and mutable (peek_mut) access to the top element, enforcing Rust’s aliasing rules.

2. Design Specification

Data structure: struct Stack<T> { items: Vec<T> }.

API contracts:

fn push(&mut self, x: T) — moves ownership into stack.

fn pop(&mut self) -> Option<T> — moves ownership out.

fn peek(&self) -> Option<&T> — immutable borrow.

fn peek_mut(&mut self) -> Option<&mut T> — mutable borrow.

Invariant: len == number of pushes - pops.

Safety contract: After pop, no alias to popped element remains.

3. Implementation

src/stack.rs:

#[derive(Debug, Default)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self { Self { items: Vec::new() } }
    pub fn with_capacity(n: usize) -> Self { Self { items: Vec::with_capacity(n) } }

    pub fn push(&mut self, x: T) {
        self.items.push(x); // REQ-2
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop() // REQ-3
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last() // REQ-5 (immutable borrow)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.items.last_mut() // REQ-5 (mutable borrow)
    }

    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
}

4. Verification (Unit Tests)

tests/stack_test.rs:

use algolab::stack::Stack;

#[test] // REQ-1
fn req1_generic_support() {
    let mut s = Stack::new();
    s.push("str");
    s.push(42);
    s.push(3.14); // all compile (different stacks)
}

#[test] // REQ-2
fn req2_push_amortized_constant() {
    let mut s = Stack::with_capacity(2);
    for i in 0..100 {
        s.push(i);
    }
    assert_eq!(s.len(), 100);
}

#[test] // REQ-3
fn req3_pop_transfers_ownership() {
    let mut s = Stack::new();
    let val = String::from("owned");
    s.push(val);
    let popped = s.pop().unwrap();
    assert_eq!(popped, "owned");
    assert!(s.pop().is_none());
}

#[test] // REQ-4
fn req4_no_use_after_pop() {
    let mut s = Stack::new();
    let a = String::from("x");
    s.push(a);
    // Using `a` here would not compile — ownership moved.
    let b = s.pop().unwrap();
    assert_eq!(b, "x");
}

#[test] // REQ-5
fn req5_peek_aliasing_rules() {
    let mut s = Stack::new();
    s.push(1);
    {
        let imm = s.peek().unwrap();
        assert_eq!(*imm, 1);
        // s.push(2); // compile error if uncommented
    }
    {
        let mut_ref = s.peek_mut().unwrap();
        *mut_ref = 99;
    }
    assert_eq!(*s.peek().unwrap(), 99);
}

5. Validation

Compare behavior to Vec (Rust standard dynamic array).

Integration test: push and pop a sequence, check order matches LIFO.

Performance check (later with Criterion): confirm push/pop scale as expected.

6. Traceability Matrix
Requirement	Design Element	Test Case
REQ-1	Stack<T> generic type	req1_generic_support
REQ-2	Vec<T> push semantics	req2_push_amortized_constant
REQ-3	pop -> Option<T>	req3_pop_transfers_ownership
REQ-4	Ownership move rules	req4_no_use_after_pop
REQ-5	peek / peek_mut	req5_peek_aliasing_rules

---

## � **Further Learning**

### **Foundational Documents:**
- **[FOUNDATIONAL_CONCEPTS.md](FOUNDATIONAL_CONCEPTS.md)** - Key insights from the founding conversation
- **[Introchat.md](Introchat.md)** - Complete original conversation (1,152 lines)

### **Zettelkasten Knowledge Pages:**
- **[Ownership Mental Model - The Library Analogy](../../zettelkasten/Ownership%20Mental%20Model%20-%20The%20Library%20Analogy.md)** - Mental models for ownership
- **[V-Cycle in Rust Development](../../zettelkasten/V-Cycle%20in%20Rust%20Development.md)** - Requirements-driven methodology
- **[Rust Learning Roadmap - The Master Plan](../../zettelkasten/Rust%20Learning%20Roadmap%20-%20The%20Master%20Plan.md)** - Complete learning strategy

### **Project Context:**
- **[PROJECT_ORIGIN.md](../../PROJECT_ORIGIN.md)** - How this workspace began

---

## 📝 Documentation Standards

This mission follows the workspace documentation standards:
- **Code Documentation**: [RUST_DOCUMENTATION_STANDARDS.md](../../_github/RUST_DOCUMENTATION_STANDARDS.md)
- **Test Documentation**: [RUST_TEST_DOCUMENTATION_STANDARDS.md](../../_github/RUST_TEST_DOCUMENTATION_STANDARDS.md)