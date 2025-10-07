# Visual Compilation Process for step2_push_front.rs

> **Navigation**: [[zettel-index]] | [[Collections MOC]] | [[Mission4_tut Overview]] | [[Mission4 Overview]]

## 🎯 High-Level Overview

```
 Rust Source Code
┌─────────────────────────────────────────┐
│ struct Node<T> {                        │
│     data: T,                           │  
│     next: Option<Box<Node<T>>>,        │
│ }                                      │
│                                        │
│ impl<T> SimpleLinkedList<T> {          │
│     pub fn push_front(&mut self, ...) │
│ }                                      │
└─────────────────────────────────────────┘
                    ↓
    ┌─────────────────────────────┐
    │    Rust Compiler (rustc)    │
    │  • Parsing & AST building   │
    │  • Type checking           │
    │  • Borrow checking         │
    │  • Monomorphization        │
    └─────────────────────────────┘
                    ↓
┌─────────────────────────────────────────┐
│           LLVM IR Code                  │
│ define void @push_front(...) {          │
│   %alloc = call ptr @__rust_alloc       │
│   store i32 %data, ptr %alloc           │
│   br label %exit                        │
│ }                                       │
└─────────────────────────────────────────┘
                    ↓
    ┌─────────────────────────────┐
    │      LLVM Backend           │
    │  • Optimization passes     │
    │  • Register allocation     │
    │  • Instruction selection   │
    └─────────────────────────────┘
                    ↓
┌─────────────────────────────────────────┐
│        Assembly Code (x86-64)          │
│ push_front:                            │
│     mov  %rcx, %rax                    │
│     call HeapAlloc                     │
│     mov  %eax, (%rdi)                  │
│     ret                                │
└─────────────────────────────────────────┘
                    ↓
    ┌─────────────────────────────┐
    │      Assembler & Linker     │
    │  • Machine code generation │
    │  • Symbol resolution       │
    │  • Library linking         │
    └─────────────────────────────┘
                    ↓
┌─────────────────────────────────────────┐
│      Windows Executable (.exe)         │
│ ┌─────────────────────────────────────┐ │
│ │ PE Header + Import Table           │ │
│ ├─────────────────────────────────────┤ │
│ │ .text section (machine code)       │ │
│ ├─────────────────────────────────────┤ │
│ │ .data section (global variables)   │ │
│ ├─────────────────────────────────────┤ │
│ │ .debug section (source mapping)    │ │
│ └─────────────────────────────────────┘ │
└─────────────────────────────────────────┘
```

## 🔍 Detailed Transformation: push_front() Function

### Stage 1: Rust Source
```rust
pub fn push_front(&mut self, data: T) {
    let new_node = Box::new(Node {
        data,
        next: self.head.take(),
    });
    self.head = Some(new_node);
    self.length += 1;
}
```

### Stage 2: After Type Checking & Monomorphization  
```rust
// T becomes i32 for our example
pub fn push_front(&mut self, data: i32) {
    let new_node = Box::new(Node {
        data: data,           // i32 value
        next: self.head.take(), // Option<Box<Node<i32>>>
    });
    self.head = Some(new_node);
    self.length += 1;  // usize increment
}
```

### Stage 3: LLVM IR (simplified)
```llvm
define void @push_front(ptr %self, i32 %data) {
entry:
  ; Allocate memory for Node<i32> (12 bytes)
  %alloc = call ptr @__rust_alloc(i64 12, i64 4)
  
  ; Store data field
  store i32 %data, ptr %alloc
  
  ; Get current head value  
  %old_head = load ptr, ptr %self
  
  ; Store old head as next pointer
  %next_ptr = getelementptr inbounds %Node, ptr %alloc, i32 0, i32 1
  store ptr %old_head, ptr %next_ptr
  
  ; Update head to point to new node
  store ptr %alloc, ptr %self
  
  ; Increment length
  %length_ptr = getelementptr inbounds %SimpleLinkedList, ptr %self, i32 0, i32 1
  %old_length = load i64, ptr %length_ptr
  %new_length = add i64 %old_length, 1
  store i64 %new_length, ptr %length_ptr
  
  ret void
}
```

### Stage 4: x86-64 Assembly (simplified)
```assembly
push_front:
    ; Function prologue
    push   %rbp
    mov    %rsp, %rbp
    
    ; Allocate memory: call HeapAlloc(heap, 0, 12)
    mov    $12, %r8          ; Size = 12 bytes
    xor    %rdx, %rdx        ; Flags = 0
    mov    %rcx, %r9         ; Save self pointer
    call   qword ptr [rip + __imp_HeapAlloc]
    
    ; Store data in new node
    mov    %edx, (%rax)      ; data field = input data
    
    ; Load current head
    mov    (%r9), %rcx       ; Load self.head
    
    ; Store old head as next pointer
    mov    %rcx, 8(%rax)     ; next field = old head
    
    ; Update head pointer
    mov    %rax, (%r9)       ; self.head = new node
    
    ; Increment length
    incq   8(%r9)            ; self.length++
    
    ; Function epilogue
    mov    %rbp, %rsp
    pop    %rbp
    ret
```

### Stage 5: Machine Code (hexadecimal)
```
55                    ; push %rbp
48 89 E5              ; mov %rsp, %rbp
41 B8 0C 00 00 00     ; mov $12, %r8
48 31 D2              ; xor %rdx, %rdx
4C 89 C9              ; mov %r9, %rcx
FF 15 XX XX XX XX     ; call HeapAlloc
89 10                 ; mov %edx, (%rax)
48 8B 09              ; mov (%r9), %rcx
48 89 48 08           ; mov %rcx, 8(%rax)
48 89 01              ; mov %rax, (%r9)
48 FF 41 08           ; incq 8(%r9)
48 89 EC              ; mov %rbp, %rsp
5D                    ; pop %rbp
C3                    ; ret
```

## 📊 Memory Layout During Execution

### Before push_front(42):
```
Stack:
┌─────────────────────┐
│ list: SimpleLinked  │
│ ├─ head: None       │  ← 8 bytes (null pointer)
│ └─ length: 0        │  ← 8 bytes  
└─────────────────────┘

Heap: (empty)
```

### After push_front(42):
```
Stack:
┌─────────────────────┐
│ list: SimpleLinked  │
│ ├─ head: Some(ptr)  │  ← Points to heap
│ └─ length: 1        │  ← Incremented
└─────────────────────┘
                │
                ↓
Heap:
┌─────────────────────┐
│ Node<i32>           │  ← 12 bytes allocated
│ ├─ data: 42         │  ← 4 bytes
│ └─ next: None       │  ← 8 bytes (null)
└─────────────────────┘
```

### After push_front(24):
```
Stack:
┌─────────────────────┐
│ list: SimpleLinked  │
│ ├─ head: Some(ptr)  │  ← Points to new node
│ └─ length: 2        │
└─────────────────────┘
                │
                ↓
Heap:
┌─────────────────────┐    ┌─────────────────────┐
│ Node<i32> (new)     │    │ Node<i32> (old)     │
│ ├─ data: 24         │    │ ├─ data: 42         │
│ └─ next: Some(ptr)──┼────→ └─ next: None       │
└─────────────────────┘    └─────────────────────┘
```

## 🎯 Key Insights

1. **Generic Monomorphization**: `<T>` becomes concrete `i32` type
2. **Zero-cost Option**: `Option<Box<T>>` becomes simple null pointer check
3. **Memory Safety**: Automatic allocation/deallocation tracking
4. **Performance**: Direct memory operations with bounds checking
5. **Debugging**: Source line mapping preserved through all stages

This transformation shows how Rust's high-level safety features compile into efficient, safe machine code that's competitive with hand-optimized C!

---

## 🔗 **Related Documentation**

### **Mission4 Tutorial Materials**
- **[[Mission4_tut Overview]]** - Main linked list tutorial guide
- **[[Mission4_tut/examples/README.md]]** - Progressive linked list examples (7 steps)
- **[[COMPILE_ERROR_ANALYSIS]]** - Common compilation errors and solutions
- **[[TYPE_BREAKDOWN]]** - Deep dive into `Box<T>` and `Rc<RefCell<T>>` types
- **[[COMPLETE_ANALYSIS]]** - Complete compilation stages overview

### **Core Concepts**
- **[[Day 02 - Ownership Basics]]** - Ownership fundamentals needed for Box<T>
- **[[Day 03 - Borrowing]]** - Reference rules in linked structures
- **[[Day 04 - Lifetimes]]** - Lifetime management in recursive types
- **[[Box Smart Pointer Patterns]]** - Box<T> usage patterns and heap allocation
- **[[Performance Optimization Guide]]** - Compilation level optimizations

### **Main Mission**
- **[[Mission4 Overview]]** - Mission 4: Linked Lists implementation
- **[[Collections MOC]]** - Data structures hub with Mission 4 integration

---

*Tags: #mission4 #compilation #assembly #llvm #box #linked-list #tutorial #visualization #zero-cost-abstractions #monomorphization #memory-layout*

*Links: [[zettel-index]] | [[Mission4_tut Overview]] | [[Collections MOC]] | [[Box Smart Pointer Patterns]] | [[Performance Optimization Guide]]*