//! Memory Terminology Examples
//! 
//! Demonstrates values, variables, pointers, and memory regions
//! from Section 1: Talking about Memory

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  Section 1: Memory Terminology                          ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // === VALUES ===
    {
        println!("📚 VALUES: Type + Domain Element");
        println!("   ─────────────────────────────────");
        let num: u8 = 6;  // Value: u8 type with element 6
        let flag: bool = true;  // Value: bool type with element true
        println!("   num (u8): {}", num);
        println!("   flag (bool): {}", flag);
        println!("   ✓ Value = Type + Data\n");
    }

    // === VARIABLES ===
    {
        println!("📚 VARIABLES: Named Value Slots");
        println!("   ──────────────────────────────");
        let x = 42;        // Variable 'x' holds value 42
        let mut y = 10;    // Mutable variable 'y'
        println!("   x (immutable): {}", x);
        println!("   y (mutable, before): {}", y);
        y = 20;            // Can reassign mutable variables
        println!("   y (mutable, after): {}", y);
        println!("   ✓ Variables are slots on the stack\n");
    }

    // === POINTERS ===
    {
        println!("📚 POINTERS: Address Holders");
        println!("   ─────────────────────────────");
        let value = 100;
        let ptr = &value;  // Pointer holds address of 'value'
        let ptr2 = &ptr;   // Pointer to pointer
        
        println!("   value: {}", value);
        println!("   ptr (reference): {}", ptr);
        println!("   ptr2 (reference to reference): {}", ptr2);
        println!("   Address of value: {:p}", ptr);
        println!("   ✓ Pointers store memory addresses\n");
    }

    // === MEMORY REGIONS ===
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  Memory Regions                                          ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // THE STACK
    {
        println!("📚 THE STACK: Function Call Scratch Space");
        println!("   ────────────────────────────────────────");
        fn stack_example(a: i32, b: i32) -> i32 {
              // Local variable on stack
            a + b  // Returns value, stack frame deallocated after
        }
        
        let result = stack_example(10, 20);
        println!("   Function parameters and locals on stack");
        println!("   Result: {}", result);
        println!("   ✓ Fast, automatic cleanup, fixed size\n");
    }

    // THE HEAP
    {
        println!("📚 THE HEAP: Dynamic Memory Pool");
        println!("   ────────────────────────────────");
        let boxed = Box::new(42);     // Heap allocation
        let string = String::from("Hello");  // Heap allocation
        let vec = vec![1, 2, 3, 4, 5];  // Heap allocation
        
        println!("   Box<i32>: {}", boxed);
        println!("   String: {}", string);
        println!("   Vec: {:?}", vec);
        println!("   ✓ Flexible size, manual management (Rust automates)\n");
    }

    // STATIC MEMORY
    {
        println!("📚 STATIC MEMORY: Program-Lifetime Storage");
        println!("   ──────────────────────────────────────────");
        static GLOBAL: i32 = 100;  // Lives for entire program
        const PI: f64 = 3.14159;   // Compile-time constant
        let literal = "hello";     // &'static str
        
        println!("   Static variable: {}", GLOBAL);
        println!("   Constant: {}", PI);
        println!("   String literal: \"{}\"", literal);
        println!("   ✓ Lives for 'static lifetime\n");
    }

    // === HIGH-LEVEL VS LOW-LEVEL MODEL ===
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  Variable Models                                         ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    {
        println!("📚 HIGH-LEVEL MODEL: Names in Data Flow");
        println!("   ──────────────────────────────────────");
        let x = 42;
        let _y = x;  // Value flows from x to y
        println!("   x → y (value flow)");
        println!("   Variables exist as long as they hold valid values\n");
    }

    {
        println!("📚 LOW-LEVEL MODEL: Memory Slots");
        println!("   ────────────────────────────────");
          // Slot exists but uninitialized
        // println!("{}", x);  // ❌ Would fail - no valid value
        let x: i32 = 42;      // Slot now holds valid value
        println!("   Slot allocated → Initialized → Valid: {}", x);
        println!("   Variables are memory locations (may be invalid)\n");
    }

    // === MEMORY SAFETY ===
    {
        println!("📚 MEMORY SAFETY: Rust Guarantees");
        println!("   ──────────────────────────────────");
        println!("   ✓ No null pointers");
        println!("   ✓ No dangling pointers");
        println!("   ✓ No use-after-free");
        println!("   ✓ No double-free");
        println!("   ✓ All enforced at compile time!");
    }

    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║  Memory Terminology Complete!                            ║");
    println!("╚══════════════════════════════════════════════════════════╝");
}
