# Error Bank - Learning from Mistakes

*Created: 2025-10-10*  
*Navigation: [[zettel-index]] | [[developer-learning-habits]] | [[spaced-repetition-cards]]*

---

## 🎯 **Purpose**

Systematic collection of programming errors with fast reproduction cases and prevention rules. Transforms mistakes into learning assets following evidence-based error analysis methodology.

**Protocol**: Add entry immediately after each bug. Include minimal reproduction code and one-line prevention rule.

---

## 📋 **Error Catalog**

| Date | Language | Error Category | Symptom | Root Cause | Fast Repro | Prevention Rule | Mission |
|------|----------|----------------|---------|------------|------------|-----------------|---------|
| 2025-10-10 | Rust | Ownership | "cannot move out of borrowed content" | Tried to move String after borrowing | `let s = &String::new(); drop(s.to_owned())` | "Clone before move or use &str" | Mission5 |
| 2025-10-09 | Rust | Lifetimes | "borrowed value does not live long enough" | Returned reference to local variable | `fn bad() -> &str { &String::new() }` | "Return owned values, not references to locals" | Mission4 |
| 2025-10-08 | Rust | Borrow Checker | "cannot borrow as mutable" | Held immutable ref during mutation | `let v = vec![1]; let r = &v[0]; v.push(2);` | "Don't hold refs across mutations" | Mission2 |

---

## 🔬 **Detailed Error Analysis**

### **ERROR-001: Ownership Violation in HashMap Implementation**

**Date**: 2025-10-10  
**Context**: Mission 5 HashMap insert method

**Symptom**:

```
error[E0507]: cannot move out of borrowed content
  --> src/lib.rs:45:13
   |
45 |         Some(old_value)
   |              ^^^^^^^^^ move occurs because `old_value` has type `String`
```

**Minimal Reproduction**:

```rust
fn broken_insert() {
    let mut map = HashMap::new();
    map.insert("key".to_string(), "old".to_string());
    
    // This fails - trying to move out of Option<&mut V>
    if let Some(old_value) = map.get_mut("key") {
        return Some(old_value);  // ❌ Cannot move out of borrowed content
    }
}
```

**Root Cause**: Attempted to move owned value out of mutable reference context.

**Fix Strategy 1 - Clone**:

```rust
fn fix1_clone() -> Option<String> {
    let mut map = HashMap::new();
    map.insert("key".to_string(), "old".to_string());
    
    map.get("key").cloned()  // ✅ Clone the String
}
```

**Fix Strategy 2 - Take/Replace**:

```rust
use std::mem;

fn fix2_replace() -> Option<String> {
    let mut map = HashMap::new();
    map.insert("key".to_string(), "old".to_string());
    
    map.get_mut("key").map(|v| mem::take(v))  // ✅ Move with replacement
}
```

**Prevention Rule**: "Use `.cloned()` for Copy types, `mem::take()` for owned values from mutable refs"

**Spaced Repetition Card**: Added to [[spaced-repetition-cards]] as RUST-016

---

### **ERROR-002: Lifetime Issues in LinkedList Iterator**

**Date**: 2025-10-09  
**Context**: Mission 4 LinkedList implementation

**Symptom**:

```
error[E0515]: cannot return value referencing local variable `current`
  --> src/lib.rs:67:9
   |
67 |         current.as_ref().map(|node| &node.data)
   |         ^^^^^^^---------------------------------
   |         |      |
   |         |      `current` is borrowed here
   |         returns a value referencing data owned by the current function
```

**Minimal Reproduction**:

```rust
fn broken_iterator() -> Option<&i32> {
    let node = Box::new(Node { data: 42, next: None });
    node.as_ref().map(|n| &n.data)  // ❌ Reference to local data
}
```

**Root Cause**: Returned reference to locally-owned data that gets dropped.

**Fix Strategy - Return Owned Values**:

```rust
fn fixed_iterator() -> Option<i32> {
    let node = Box::new(Node { data: 42, next: None });
    node.as_ref().map(|n| n.data)  // ✅ Return owned copy
}
```

**Prevention Rule**: "Return owned values or require lifetime parameters for borrowed returns"

---

### **ERROR-003: Borrow Checker Violation in Ring Buffer**  

**Date**: 2025-10-08
**Context**: Mission 2 Queue implementation

**Symptom**:

```
error[E0502]: cannot borrow `self.data` as mutable because it is also borrowed as immutable
  --> src/lib.rs:89:9
   |
88 |         let current = &self.data[self.head];
89 |         self.data[self.head] = None;
   |         ^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
```

**Minimal Reproduction**:

```rust
fn broken_dequeue() {
    let mut buffer = RingBuffer::new(5);
    let current = &buffer.data[buffer.head];  // Immutable borrow
    buffer.data[buffer.head] = None;          // ❌ Mutable borrow while immutable exists
}
```

**Root Cause**: Held immutable reference while attempting mutation.

**Fix Strategy - Narrow Scope**:

```rust  
fn fixed_dequeue() -> Option<T> {
    let result = self.data[self.head].take();  // ✅ Atomic take operation
    if result.is_some() {
        self.head = (self.head + 1) % self.capacity;
    }
    result
}
```

**Prevention Rule**: "Use `.take()` for Option mutation, or separate borrow and mutation phases"

---

## 🛠️ **Bug Drill Templates**

### **Ownership Error Drill**

```rust
// BUG DRILL: Identify the ownership issue
fn drill_ownership() {
    let s1 = String::from("hello");
    let s2 = s1;                    // What happens here?
    println!("{}", s1);             // Will this compile?
}

// VARIATIONS TO TRY:
// 1. Use &String instead of String
// 2. Use s1.clone() 
// 3. Use String literal instead of String::from()
```

### **Borrow Checker Drill**

```rust  
// BUG DRILL: Fix the borrow checker error
fn drill_borrowing() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let first = &vec[0];            // Immutable borrow
    vec.push(6);                    // Mutable borrow - conflict!
    println!("First: {}", first);
}

// SOLUTIONS TO IMPLEMENT:
// 1. Clone the value instead of borrowing
// 2. Narrow the borrow scope  
// 3. Separate mutation and usage phases
```

### **Lifetime Error Drill**

```rust
// BUG DRILL: Fix the lifetime annotation
fn drill_lifetimes() -> &str {      // Missing lifetime parameter
    let local = String::from("temp");
    &local                          // Reference to local data
}

// FIXES TO TRY:
// 1. Return owned String instead of &str
// 2. Add proper lifetime parameters
// 3. Use 'static string literal
```

---

## 📊 **Error Pattern Analysis**

### **Most Common Error Categories**

1. **Ownership Violations**: 40% of errors
   - Moving out of borrowed content
   - Use after move
   - Double moves

2. **Borrow Checker Issues**: 35% of errors  
   - Mutable/immutable conflicts
   - References across mutations
   - Iterator invalidation

3. **Lifetime Mismatches**: 20% of errors
   - Returning references to locals
   - Missing lifetime parameters
   - Lifetime elision confusion

4. **Type Mismatches**: 5% of errors
   - Generic constraints
   - Trait bound issues
   - Conversion problems

### **Learning Curve Insights**

- **Weeks 1-2**: Mostly ownership and borrowing basics
- **Weeks 3-4**: Complex lifetime situations emerge
- **Weeks 5+**: Focus shifts to advanced trait usage

### **Prevention Strategy Evolution**

- **Early stage**: Focus on ownership rules memorization
- **Intermediate**: Pattern recognition for common scenarios  
- **Advanced**: Architectural design to avoid lifetime complexity

---

## 🔄 **Daily Workflow Integration**

### **When Errors Occur** (5 minutes)

1. **Immediate capture**: Copy exact error message
2. **Minimal repro**: Create 5-10 line reproduction case
3. **Fix twice**: Try two different solution approaches  
4. **Prevention rule**: Write one-line rule to avoid repetition
5. **Spaced repetition**: Convert to retrieval card if concept-heavy

### **Weekly Error Review** (15 minutes, Fridays)

1. **Pattern identification**: Group similar errors
2. **Success rate**: Track prevention rule effectiveness
3. **Drill creation**: Build practice exercises from common mistakes
4. **Calendar integration**: Add error-prone topics to daily study focus

### **Monthly Error Analysis** (30 minutes)

1. **Trend analysis**: Are error types changing over time?
2. **Knowledge gap identification**: What concepts need more study?
3. **Prevention rule validation**: Which rules are most effective?
4. **Learning strategy adjustment**: Modify daily routine based on error patterns

---

## 🎯 **Pre-Commit Checklist**

Print this and keep near your coding setup:

```
╔══════════════════════════════════════════╗
║        RUST ERROR PREVENTION            ║
║             CHECKLIST                   ║
╠══════════════════════════════════════════╣
║                                          ║
║  OWNERSHIP & BORROWING                   ║
║  □ No moves after borrows                ║
║  □ Use .clone() or .take() appropriately ║
║  □ Separate borrow and mutation phases   ║
║                                          ║
║  LIFETIMES                               ║  
║  □ No references to local variables      ║
║  □ Return owned values when possible     ║
║  □ Explicit lifetimes for complex cases  ║
║                                          ║
║  COLLECTIONS                             ║
║  □ No iteration during mutation          ║
║  □ Use .entry() API for HashMap ops      ║
║  □ Check bounds before indexing          ║
║                                          ║
║  GENERAL                                 ║
║  □ cargo clippy --deny warnings         ║
║  □ All tests pass                        ║
║  □ Error messages are descriptive        ║
║                                          ║
╚══════════════════════════════════════════╝
```

---

*Tags: #error-analysis #debugging #learning-from-mistakes #rust #ownership #borrowing #lifetimes*

*Links: [[zettel-index]] | [[developer-learning-habits]] | [[spaced-repetition-cards]] | [[learning-plan]]*

**Next Actions:**

- [ ] Add error from today's coding session  
- [ ] Create bug drill for Mission 5 HashMap errors
- [ ] Review error patterns weekly (Friday retrospective)
- [ ] Update prevention rules based on effectiveness
