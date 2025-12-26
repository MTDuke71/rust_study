// Example 1: Types in Memory
// Demonstrates alignment, layout, repr attributes, and wide pointers

use rustaceans_ch02_types::utils::{print_section, print_subsection, show_type_info};
use std::mem;

fn main() {
    print_section("1. TYPES IN MEMORY");
    
    alignment_examples();
    layout_examples();
    repr_examples();
    complex_types();
    wide_pointers();
}

// --- Alignment Examples ---
fn alignment_examples() {
    print_subsection("a. Alignment");
    
    println!("Alignment prevents misaligned memory access:");
    show_type_info::<bool>("bool");
    show_type_info::<u8>("u8");
    show_type_info::<u16>("u16");
    show_type_info::<u32>("u32");
    show_type_info::<u64>("u64");
    show_type_info::<usize>("usize");
    
    println!("\nCompound types:");
    show_type_info::<(u8, u32)>("(u8, u32)");
    show_type_info::<(u32, u8)>("(u32, u8)");
    show_type_info::<[u8; 4]>("[u8; 4]");
    show_type_info::<[u32; 2]>("[u32; 2]");
}

// --- Layout with Different Repr ---
fn layout_examples() {
    print_subsection("b. Layout - repr(Rust) vs repr(C)");
    
    // repr(Rust) - compiler optimizes field order
    #[repr(Rust)]
    #[allow(dead_code)]
    struct OptimizedLayout {
        tiny: bool,    // 1 byte
        normal: u32,   // 4 bytes
        small: u8,     // 1 byte
        long: u64,     // 8 bytes
        short: u16,    // 2 bytes
    }
    
    // repr(C) - preserves field order
    #[repr(C)]
    struct CLayout {
        tiny: bool,    // 1 byte + 3 padding
        normal: u32,   // 4 bytes
        small: u8,     // 1 byte + 7 padding
        long: u64,     // 8 bytes
        short: u16,    // 2 bytes + 6 padding
    }
    
    println!("Field order impact on size:");
    show_type_info::<OptimizedLayout>("OptimizedLayout (Rust)");
    show_type_info::<CLayout>("CLayout (C)");
    
    // Demonstrate field offsets with repr(C)
    let example = CLayout {
        tiny: true,
        normal: 42,
        small: 7,
        long: 1000,
        short: 99,
    };
    
    println!("\nField offsets in CLayout:");
    println!("  tiny:   offset {}", offset_of(&example, &example.tiny));
    println!("  normal: offset {}", offset_of(&example, &example.normal));
    println!("  small:  offset {}", offset_of(&example, &example.small));
    println!("  long:   offset {}", offset_of(&example, &example.long));
    println!("  short:  offset {}", offset_of(&example, &example.short));
}

// Helper to calculate field offset
fn offset_of<T, U>(base: &T, field: &U) -> usize {
    (field as *const U as usize) - (base as *const T as usize)
}

// --- Different Repr Attributes ---
fn repr_examples() {
    print_subsection("c. repr Attributes");
    
    #[repr(C)]
    struct ReprC {
        a: u8,
        b: u32,
    }
    
    // Note: Using #[repr(packed)] for demonstration purposes
    // Clippy warns about missing ABI qualifier, but this is intentional
    // to show the basic packed representation concept
    #[repr(packed)]
    #[allow(dead_code)]
    #[allow(clippy::repr_packed_without_abi)]
    struct ReprPacked {
        a: u8,
        b: u32,  // No padding!
    }
    
    #[repr(transparent)]
    struct ReprTransparent(u64);
    
    println!("Impact of repr attributes:");
    show_type_info::<ReprC>("repr(C)");
    show_type_info::<ReprPacked>("repr(packed)");
    show_type_info::<ReprTransparent>("repr(transparent)");
    show_type_info::<u64>("u64 (inner type)");
    
    println!("\nNote: repr(packed) removes padding (5 bytes vs 8 bytes)");
    println!("      repr(transparent) matches inner type exactly");
}

// --- Complex Types ---
fn complex_types() {
    print_subsection("d. Complex Types - Enums, Tuples, Arrays");
    
    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
    }
    
    println!("Enums (discriminant + largest variant):");
    show_type_info::<Message>("Message enum");
    show_type_info::<Option<u8>>("Option<u8>");
    show_type_info::<Result<u32, ()>>("Result<u32, ()>");
    
    println!("\nTuples and Arrays:");
    show_type_info::<(u8, u16, u32)>("(u8, u16, u32)");
    show_type_info::<[u8; 10]>("[u8; 10]");
    show_type_info::<[u64; 10]>("[u64; 10]");
}

// --- Wide Pointers (DSTs) ---
fn wide_pointers() {
    print_subsection("e. Dynamically Sized Types & Wide Pointers");
    
    println!("Regular (thin) pointers:");
    show_type_info::<&u32>("&u32");
    show_type_info::<&String>("&String");
    show_type_info::<*const u8>("*const u8");
    
    println!("\nWide (fat) pointers (2x usize):");
    show_type_info::<&[u8]>("&[u8] (slice)");
    show_type_info::<&str>("&str (string slice)");
    show_type_info::<&dyn std::fmt::Debug>("&dyn Debug (trait object)");
    
    println!("\nWide pointer demonstration:");
    let data = vec![1, 2, 3, 4, 5];
    let slice: &[i32] = &data;
    
    // Wide pointer = (data_ptr, length)
    // Note: Taking size of the reference itself (not slice contents) to show pointer size
    #[allow(clippy::size_of_ref)]
    {
        println!("  Slice &[i32]:");
        println!("    data pointer: {:p}", slice.as_ptr());
        println!("    length: {}", slice.len());
        println!("    total size: {} bytes (2 × usize)", mem::size_of_val(&slice));
    }
    
    // Trait object = (data_ptr, vtable_ptr)
    #[allow(clippy::size_of_ref)]
    {
        let debug_obj: &dyn std::fmt::Debug = &42u32;
        println!("\n  Trait object &dyn Debug:");
        println!("    size: {} bytes (2 × usize)", mem::size_of_val(&debug_obj));
        println!("    Contains: data pointer + vtable pointer");
    }
}
