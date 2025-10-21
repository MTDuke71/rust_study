# Trait Design Patterns - Lessons from Mission3

*Key insights from designing the `Searchable` trait and extension traits for binary search algorithms.*

---

## 🎯 **The Challenge**

Mission3 required creating a binary search API that works with:
- Slices (`&[T]`)
- Vectors (`Vec<T>`)
- Arrays (`[T; N]`)
- Any sortable container type

**Goal:** Single trait that provides binary search methods to all these types.

## 📐 **Solution: The Searchable Trait**

### **Initial Design**

```rust
pub trait Searchable<T> {
    fn binary_search(&self, target: &T) -> Result<usize, usize>;
    fn search_left_bound(&self, target: &T) -> usize;
    fn search_right_bound(&self, target: &T) -> usize;
}
```

**Problem:** This doesn't specify that `T` must be orderable!

### **Refined Design with Trait Bounds**

```rust
pub trait Searchable<T: Ord> {
    fn as_slice(&self) -> &[T];
    
    fn binary_search(&self, target: &T) -> Result<usize, usize> {
        self.as_slice().binary_search(target)
    }
    
    fn search_left_bound(&self, target: &T) -> usize {
        // Implementation using as_slice()...
    }
}
```

**Key Insight:** 
- Trait bound `T: Ord` ensures only orderable types work
- Single method `as_slice()` required - everything else has default implementation
- **Deep Module Pattern** - Simple interface (`as_slice`), complex functionality

## 🔧 **Implementation Pattern**

### **1. Slice Implementation (Base Case)**

```rust
impl<T: Ord> Searchable<T> for [T] {
    fn as_slice(&self) -> &[T] {
        self  // Already a slice!
    }
}
```

### **2. Vec Implementation (Deref to Slice)**

```rust
impl<T: Ord> Searchable<T> for Vec<T> {
    fn as_slice(&self) -> &[T] {
        &self[..]  // Deref coercion to slice
    }
}
```

### **3. Array Implementation (Generic Size)**

```rust
impl<T: Ord, const N: usize> Searchable<T> for [T; N] {
    fn as_slice(&self) -> &[T] {
        &self[..]  // Array to slice coercion
    }
}
```

**Pattern:**
- All implementations convert to `&[T]`
- Default trait methods work on slice representation
- **Zero runtime overhead** - slice conversions are compile-time

## 🌟 **Extension Trait Pattern**

Make custom methods feel like native slice methods:

```rust
pub trait SearchExt<T> {
    fn find_all_equal(&self, target: &T) -> RangeIter<'_, T>;
    fn find_range(&self, min: &T, max: &T) -> RangeIter<'_, T>;
    fn find_first_matching<P>(&self, predicate: P) -> Option<&T>
    where
        P: Fn(&T) -> bool;
}

impl<T: Ord> SearchExt<T> for [T] {
    fn find_all_equal(&self, target: &T) -> RangeIter<'_, T> {
        find_all_equal(self, target)
    }
    // ... other methods
}
```

**Benefits:**
1. **Ergonomic API**: `data.find_all_equal(&5)` instead of `find_all_equal(&data, &5)`
2. **IDE Support**: Methods appear in autocomplete
3. **Method Chaining**: `data.find_range(&1, &10).filter(...).map(...)`
4. **Namespacing**: Avoids polluting global namespace with functions

## 🎓 **Trait Bound Patterns**

### **1. Simple Bounds**

```rust
impl<T: Ord> Searchable<T> for [T] { ... }
```
- `T: Ord` means T implements the `Ord` trait
- Allows comparison operations (`<`, `>`, `==`)

### **2. Multiple Bounds**

```rust
impl<T: Ord + Clone> Searchable<T> for CustomContainer<T> { ... }
```
- `T` must implement both `Ord` and `Clone`
- More restrictive but enables more functionality

### **3. Where Clauses (for Readability)**

```rust
impl<T> Searchable<T> for CustomContainer<T>
where
    T: Ord + Clone + Debug,
{
    // Implementation
}
```
- Same as multiple bounds but more readable
- Preferred for complex constraints

### **4. Associated Type Bounds**

```rust
trait SearchableIterator: Iterator
where
    Self::Item: Ord,
{
    fn sorted(&mut self) -> SortedIter<Self> { ... }
}
```
- Constrains the associated type `Item`
- Enables trait methods that depend on item properties

## 💡 **Lifetime Management in Traits**

### **Problem: Returning Borrowed Data**

```rust
fn find_all_equal<'a, T: Ord>(slice: &'a [T], target: &T) -> RangeIter<'a, T>
```

**Key Points:**
1. Lifetime `'a` connects input slice to output iterator
2. Iterator can't outlive the data it borrows
3. Compiler enforces this at compile time

### **Elided Lifetimes in Traits**

```rust
trait SearchExt<T> {
    fn find_all_equal(&self, target: &T) -> RangeIter<'_, T>;
    //                                                   ^^ Elided lifetime
}
```

**Rules:**
- `'_` means "infer the lifetime"
- Typically the lifetime of `&self`
- Makes code more readable

## 🚀 **Design Principles Applied**

### **1. Deep Modules (Clean Code)**
- **Simple interface**: One required method (`as_slice`)
- **Complex functionality**: All binary search variants provided
- **Information hiding**: Implementation details hidden behind trait

### **2. Single Responsibility**
- `Searchable` trait: Provides binary search capability
- `SearchExt` trait: Adds advanced search methods
- Each trait has one clear purpose

### **3. Zero-Cost Abstractions**
```rust
// This high-level code:
let results: Vec<_> = data.find_all_equal(&5).collect();

// Compiles to same assembly as:
let left = search_left_bound(&data, &5);
let right = search_right_bound(&data, &5);
let mut results = Vec::with_capacity(right - left);
for i in left..right {
    results.push(&data[i]);
}
```

## 🔍 **Common Pitfalls and Solutions**

### **Pitfall 1: Over-Generic Traits**

```rust
// ❌ Too generic - T could be anything
trait Searchable<T> {
    fn binary_search(&self, target: &T) -> Result<usize, usize>;
}
```

**Solution:** Add trait bounds
```rust
// ✅ Constrains T to be orderable
trait Searchable<T: Ord> { ... }
```

### **Pitfall 2: Requiring Too Many Methods**

```rust
// ❌ Every implementor must write all methods
trait Searchable<T: Ord> {
    fn binary_search(&self, target: &T) -> Result<usize, usize>;
    fn search_left_bound(&self, target: &T) -> usize;
    fn search_right_bound(&self, target: &T) -> usize;
    fn find_all_equal(&self, target: &T) -> Vec<&T>;
}
```

**Solution:** Provide default implementations
```rust
// ✅ Only as_slice() required, rest have defaults
trait Searchable<T: Ord> {
    fn as_slice(&self) -> &[T];
    
    fn binary_search(&self, target: &T) -> Result<usize, usize> {
        self.as_slice().binary_search(target)
    }
    // ... other methods with default implementations
}
```

### **Pitfall 3: Lifetime Issues**

```rust
// ❌ Compiler error - lifetime not connected
fn find_all<T: Ord>(data: &[T], target: &T) -> Vec<&T> {
    // Returns Vec<&T> but doesn't specify lifetime relationship
}
```

**Solution:** Explicit lifetime annotations
```rust
// ✅ Lifetime 'a connects input to output
fn find_all<'a, T: Ord>(data: &'a [T], target: &T) -> Vec<&'a T> {
    // Now compiler knows references in Vec come from data
}
```

## 🎯 **When to Use Each Pattern**

### **Use Basic Trait When:**
- You need polymorphism across different types
- Multiple implementations with same behavior
- Example: `Searchable` trait for slices, Vecs, arrays

### **Use Extension Trait When:**
- Adding methods to existing types you don't own
- Creating "method-style" APIs
- Example: `SearchExt` adding methods to slices

### **Use Trait Objects When:**
- Runtime polymorphism needed
- Heterogeneous collections
- Example: `Vec<Box<dyn Searchable<i32>>>`

### **Use Generic Traits When:**
- Compile-time polymorphism (zero cost)
- Type-specific optimizations
- Example: `fn process<T: Searchable<i32>>(data: T)`

## 🔗 **Related Concepts**

- [[Binary Search Iterator Patterns]] - Iterator integration with these traits
- [[mission-3]] - Complete implementation example
- [[Zero-Cost Abstractions]] - How trait dispatch is optimized
- [[Lifetime Annotations in Practice]] - Deeper dive into lifetimes

---

*Tags: #traits #generics #lifetimes #design-patterns #mission3 #clean-code #deep-modules*

*Links: [[zettel-index]] | [[mission-3]] | [[Rust Trait System Overview]]*
