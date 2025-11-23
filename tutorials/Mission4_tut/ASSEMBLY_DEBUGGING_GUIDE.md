# 🔍 Assembly-Level Debugging Guide for step2_push_front.rs

## 🎯 Goal: Single-step through Rust code while viewing corresponding assembly

You can absolutely debug Rust code while viewing the corresponding assembly instructions! Here's how to set it up and use it effectively.

## 🛠️ Setup (Already Done!)

✅ **CodeLLDB Extension**: Already installed (`vadimcn.vscode-lldb`)  
✅ **Debug Configuration**: Created in `.vscode/launch.json`  
✅ **Assembly Support**: x86-64 assembly highlighting available

## 🚀 How to Debug with Assembly View

### Method 1: Using VS Code Debug Panel

1. **Open the file**: `examples/step2_push_front.rs`
2. **Set breakpoints**: Click in the left margin on these key lines:
   ```rust
   Line 25: let new_node = Box::new(Node {
   Line 28: });
   Line 29: self.head = Some(new_node);
   Line 30: self.length += 1;
   ```
3. **Start debugging**: 
   - Press `F5` or go to Debug panel
   - Select "Debug step2_push_front (with Assembly)"
   - Click the green play button

### Method 2: Debug Console Commands

Once debugging starts, use these LLDB commands in the Debug Console:

```lldb
# Show current assembly around instruction pointer
(lldb) disassemble --pc
(lldb) disassemble --frame

# Show assembly for specific function
(lldb) disassemble --name "step2_push_front::SimpleLinkedList::push_front"

# Step instruction by instruction (assembly level)
(lldb) stepi

# Show registers
(lldb) register read

# Show memory at specific address
(lldb) memory read --size 8 --format x --count 4 $rsp
```

### Method 3: Manual Disassembly View

1. **While debugging**, go to `View → Command Palette`
2. Type: `LLDB: View Disassembly`
3. This opens a dedicated disassembly view
4. Set the view to Intel syntax: `LLDB: Set Disassembly Flavor` → `intel`

## 📊 What You'll See

### Rust Source → Assembly Mapping

**Rust Code:**
```rust
let new_node = Box::new(Node {
    data,
    next: self.head.take(),
});
```

**Corresponding Assembly:**
```assembly
; Allocate memory for Node<i32>
mov     rax, 12          ; Size = 12 bytes  
call    __rust_alloc     ; Heap allocation
mov     [rax], edx       ; Store data field
mov     rcx, [rdi]       ; Load self.head
mov     [rax+8], rcx     ; Store next field
```

### Key Assembly Patterns to Look For:

1. **Memory Allocation**:
   ```assembly
   call    __rust_alloc    ; Box::new() becomes heap allocation
   ```

2. **Field Assignment**:
   ```assembly
   mov     [rax], edx      ; data field assignment  
   mov     [rax+8], rcx    ; next field assignment
   ```

3. **Option Handling**:
   ```assembly
   test    rax, rax        ; Check if pointer is null
   jz      none_case       ; Jump if Option::None
   ```

4. **Length Increment**:
   ```assembly
   incq    8(%rdi)         ; self.length++
   ```

## 🎮 Step-by-Step Debugging Workflow

### Step 1: Set Strategic Breakpoints
```rust
// Set breakpoint here to see memory allocation
let new_node = Box::new(Node {    // <- Breakpoint
    data,
    next: self.head.take(),       // <- Breakpoint  
});
self.head = Some(new_node);       // <- Breakpoint
self.length += 1;                 // <- Breakpoint
```

### Step 2: Debug Session Commands

1. **F5**: Start debugging
2. **F10**: Step over (stays in Rust)
3. **F11**: Step into (goes into Box::new implementation)
4. **Shift+F11**: Step out
5. **F9**: Toggle breakpoint

### Step 3: Assembly Analysis Commands

In Debug Console:
```lldb
# See where you are in assembly
(lldb) disassemble --pc

# Step one assembly instruction
(lldb) stepi

# Show next 10 instructions
(lldb) disassemble --count 10

# See memory layout of your struct
(lldb) memory read --format pointer --size 8 $rdi
```

## 🔍 Advanced Debugging Features

### Memory Inspection

```lldb
# View the SimpleLinkedList struct
(lldb) frame variable list
(lldb) memory read --format pointer --size 8 &list

# View a Node after allocation  
(lldb) memory read --format uint32_t --size 4 --count 3 $rax
```

### Performance Analysis

```lldb
# Count instructions in push_front
(lldb) breakpoint set --name "push_front" 
(lldb) breakpoint set --name "push_front" --one-shot
(lldb) continue
(lldb) stepi  # Count how many stepi commands needed
```

### Call Stack with Assembly

```lldb
# Show call stack with assembly addresses
(lldb) thread backtrace --all
(lldb) frame select 0
(lldb) disassemble --frame
```

## 📝 Example Debugging Session

Here's what a typical session looks like:

1. **Start**: Breakpoint at `Box::new(Node {`
2. **Observe**: Rust source shows high-level intent
3. **Step Into**: See `__rust_alloc` call in assembly
4. **Continue**: Return to Rust, see field assignments
5. **Assembly View**: Each field becomes a `mov` instruction
6. **Memory**: Inspect the allocated Node structure
7. **Next Line**: `self.head = Some(new_node)` becomes pointer assignment

## 🎯 Learning Objectives

By debugging this way, you'll understand:

- **Zero-cost abstractions**: How `Option<Box<T>>` becomes simple pointers
- **Memory layout**: How structs are arranged in memory
- **Optimization**: What the compiler optimizes away
- **Performance**: Actual instruction count for operations
- **Safety**: How Rust's guarantees translate to assembly

## ⚡ Pro Tips

1. **Use Intel syntax**: Easier to read than AT&T syntax
2. **Set conditional breakpoints**: `condition 1 data == 42`
3. **Watch expressions**: Monitor variables during execution  
4. **Split view**: Have source and disassembly side-by-side
5. **Save sessions**: Record interesting debugging sessions

This debugging approach gives you deep insight into how Rust's high-level constructs compile to efficient machine code!

---

## 🔗 Related Zettelkasten Concepts

**Debugging Techniques:**
- [[QUICK_DEBUG_START]] - Quick 3-minute debugging setup
- [[lldb-debugging]] - LLDB debugger mastery
- [[assembly-debugging]] - Assembly-level debugging
- [[breakpoint-strategies]] - Advanced breakpoint usage

**Compilation Analysis:**
- [[COMPILATION_BREAKDOWN]] - Detailed compilation stages
- [[VISUAL_COMPILATION_PROCESS]] - Visual compilation guide
- [[disassembly-analysis]] - Reading disassembly output
- [[instruction-stepping]] - Single-step debugging

**Memory & Registers:**
- [[register-inspection]] - CPU register analysis
- [[memory-debugging]] - Memory inspection techniques
- [[stack-frame-analysis]] - Stack frame navigation
- [[heap-tracking]] - Heap allocation tracking

**Performance Investigation:**
- [[zero-cost-abstractions]] - Verifying zero-cost claims
- [[optimization-verification]] - Checking compiler optimizations
- [[performance-profiling]] - Performance analysis tools
- [[instruction-counting]] - Counting actual CPU instructions

**Smart Pointers:**
- [[Box Smart Pointer Patterns]] - Box<T> assembly patterns
- [[option-lowering]] - Option<T> in assembly
- [[memory-layout]] - Struct layout in memory

**Mission Integration:**
- [[mission-4]] - Linked list mission
- [[Mission4_tut Overview]] - Complete tutorial
- `step2_push_front` - Example source file

**Tools & Commands:**
- [[lldb-command-reference]] - LLDB command cheat sheet
- [[vscode-debugging]] - VS Code debugging setup
- [[codelldb-extension]] - CodeLLDB configuration

*Tags: #mission4 #debugging #assembly #lldb #advanced #disassembly #performance-analysis*