# 🔥 Quick Start: Debug Rust with Assembly View

## ⚡ 3-Minute Setup

### Step 1: Open the Source File
```
📁 Open: examples/step2_push_front.rs
```

### Step 2: Set Breakpoints  
Click in the left margin next to these lines:
- ✅ **Line 25**: `let new_node = Box::new(Node {`
- ✅ **Line 29**: `self.head = Some(new_node);`  
- ✅ **Line 30**: `self.length += 1;`

### Step 3: Start Debugging
1. Press **F5** (or go to Run → Start Debugging)
2. Select: **"Debug step2_push_front (with Assembly)"**
3. Program will build and start

### Step 4: Open Assembly View
While debugging, in the Command Palette (Ctrl+Shift+P):
```
LLDB: View Disassembly
```

### Step 5: Debug Commands to Try

In the **Debug Console**, type these commands:

```lldb
# See current assembly instructions
disassemble --pc

# Step one assembly instruction  
stepi

# Show the push_front function assembly
disassemble --name SimpleLinkedList::push_front

# Show registers (see memory addresses)
register read rdi rax rcx

# Show memory content at a pointer
memory read --format x --size 8 --count 4 $rdi
```

## 🎯 What You'll Discover

### Rust Line → Assembly Translation

**When you hit the first breakpoint:**
```rust
let new_node = Box::new(Node { data, next: self.head.take() });
```

**You'll see assembly like:**
```assembly
mov     rax, 12              ; Size for Node<i32>
call    __rust_alloc         ; Heap allocation  
mov     [rax], edx           ; Store data field
mov     rcx, [rdi]           ; Load self.head
mov     [rax+8], rcx         ; Store next field  
```

### Key Insights You'll Gain:

1. **`Box::new()`** → Direct heap allocation call
2. **Field assignment** → Simple memory stores  
3. **`Option<T>`** → Null pointer checks
4. **`self.length += 1`** → Single increment instruction

## 📊 Expected Assembly Patterns

### Memory Allocation:
```assembly
call    qword ptr [__imp_HeapAlloc]  ; Windows heap allocator
```

### Field Updates:
```assembly
mov     dword ptr [rax], edx         ; data = input_value
mov     qword ptr [rax+8], rcx       ; next = old_head  
```

### Pointer Updates:
```assembly
mov     qword ptr [rdi], rax         ; self.head = new_node
inc     qword ptr [rdi+8]            ; self.length++
```

## 🎮 Interactive Session

Try this debugging sequence:

1. **F5** - Start debugging
2. **Hit breakpoint** at `Box::new`
3. **Debug Console**: `disassemble --pc` 
4. **F11** - Step into Box::new
5. **See allocation**: Look for `call __rust_alloc`
6. **F10** - Step over back to your code
7. **Debug Console**: `memory read $rax` (see allocated memory)
8. **F10** - Step to next line
9. **Repeat**: See each Rust line become assembly

This gives you **real-time insight** into how Rust's safe abstractions compile to efficient machine code!

## 🚀 Pro Tips

- **Split view**: Open assembly view side-by-side with source
- **Intel syntax**: Easier to read than AT&T
- **Watch variables**: Monitor `list.head` and `list.length`
- **Step vs stepi**: `F10` = Rust level, `stepi` = assembly level

You're now ready to see **exactly** how Rust's memory safety features translate to machine code! 🦀