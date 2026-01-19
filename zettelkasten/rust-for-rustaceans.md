**Tags:** #rust-for-rustaceans #book #advanced-rust #systems-programming  
**Created:** 2025-12-28  
**Related:** [[rust-book]], [[mission-4]]

# Rust for Rustaceans

*Intermediate to advanced Rust - beyond The Book*

**Author**: Jon Gjengset  
**Status**: Chapter 2 in progress  

---

## 📖 Chapter Progress

### **Chapter 1: Foundations** ✅ COMPLETE
  - Memory regions (stack, heap, static)
  - Ownership and move semantics
  - Borrowing and lifetimes
  - Interior mutability (Cell, RefCell, UnsafeCell)
  - **Zettelkasten**: 
    - [[Clone vs Copy]] - Explicit vs implicit duplication
    - [[static-lifetime]] - 'static lifetime and bounds
    - [[interior-mutability]] - Philosophy of safe mutation

### **Chapter 2: Types** ✅ COMPLETE
  - **2.1: Types in Memory** ✅ COMPLETE
    - Alignment and layout
    - repr attributes (Rust, C, packed, transparent)
    - DSTs and wide pointers
    - **Zettelkasten**: [[alignment-and-layout]]
  - **2.2: Traits and Trait Bounds** ⏭️ NEXT
  - **Additional sections**: TBD

### **Chapter 3: Designing Interfaces** ✅ COMPLETE
  - **3.1: Borrowed vs Owned** ✅ COMPLETE
    - Only require ownership when necessary
    - Use `&T` for read-only access
    - **Zettelkasten**: [[cow-borrowed-vs-owned]]
  - **3.2: Fallible/Blocking Destructors** ✅ COMPLETE
    - Drop cannot return Result
    - Explicit close() methods
    - **Zettelkasten**: [[fallible-destructors]]
  - **3.3: Constrained Interfaces (Hidden Contracts)** ✅ COMPLETE
    - Re-export hazards and wrapping foreign types
    - Auto-traits (Send/Sync) as hidden contracts
    - SemVer trick for dependency migration
    - **Zettelkasten**: [[auto-traits-send-sync]], [[semver-trick]]
  - **Additional sections**: TBD

### **Chapter 4: Error Handling** 🔄 IN PROGRESS
  - Error representation: enums for domain errors, opaque types for API simplicity
  - Propagation: `?` operator, `From` trait conversions, try blocks (unstable)
  - Best practices: implement `Error`, derive `Display`/`Debug`, prefer explicit error types, avoid panics in libraries
  - Zettelkasten: [[fallible-destructors]], [[error-handling-patterns]]
  - Workspace: [[../../rust_for_rustaceans/Ch04/rfr-ch04-summary]]

---

## 🗂️ Workspace Files

- **[[../../rust_for_rustaceans/Ch02/README]]** - Chapter 2 workspace overview and examples
- **[[../../rust_for_rustaceans/Ch02/WORKSPACE_CREATED]]** - Chapter 2 workspace setup documentation
- **[[../../rust_for_rustaceans/Ch02/rfr-ch02-summary]]** - Chapter 2 comprehensive summary
- **[[../../rust_for_rustaceans/Ch03/rfr-ch03-summary]]** - Chapter 3 comprehensive summary (Designing Interfaces)

---

## 🔗 Related Content

### **Book Series**
- **[[rust-book]]** - The Rust Programming Language (Ch1-17 complete)
- **Rust for Rustaceans** - This book (advanced topics)

### **Mission Connections**
- **[[mission-4]]** - LinkedList using Rc<RefCell<T>> (Ch1.4 interior mutability applied)
- **[[mission-10]]** - REST API demonstrating production Rust patterns


### **Concept Pages**
- **[[Clone vs Copy]]** - Created from Ch1.3
- **[[static-lifetime]]** - Created from Ch1.2
- **[[interior-mutability]]** - Deep dive from Ch1.4
- **[[alignment-and-layout]]** - Memory layout from Ch2.1
- **[[cow-borrowed-vs-owned]]** - Conditional ownership from Ch3.1
- **[[fallible-destructors]]** - Resource cleanup patterns from Ch3.2
- **[[auto-traits-send-sync]]** - Auto-traits and thread safety from Ch3.3
- **[[semver-trick]]** - Dependency version migration from Ch3.3
- **[[error-handling-patterns]]** - Error handling patterns and best practices (Ch4)

---

*Rustaceans builds on The Book's foundations with real-world insights, performance considerations, and advanced patterns from production Rust development.*
