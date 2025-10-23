// Virtual Memory Address Space Analysis
// Explains why stack and heap can be "trillions" of bytes apart

fn main() {
    println!("=== Virtual Memory Address Space Analysis ===\n");

    // Let's examine those "impossible" addresses more carefully
    let stack_var = 42i32;
    let heap_vec = [1, 2, 3, 4, 5];

    let stack_addr = &stack_var as *const i32 as usize;
    let heap_addr = heap_vec.as_ptr() as usize;

    println!("🔍 Raw Address Analysis:");
    println!("   Stack address: 0x{:x} ({})", stack_addr, stack_addr);
    println!("   Heap address:  0x{:x} ({})", heap_addr, heap_addr);

    let distance = heap_addr.abs_diff(stack_addr);

    println!("   Distance: {} bytes", distance);
    println!("   That's {:.2} GB", distance as f64 / 1_073_741_824.0);
    println!("   That's {:.2} TB", distance as f64 / 1_099_511_627_776.0);

    // =======================================================================
    // THE TRUTH: VIRTUAL MEMORY EXPLANATION
    // =======================================================================
    println!("\n💡 Why This Makes Sense - Virtual Memory:");

    println!("\n1. 64-bit Address Space:");
    println!("   - Total addressable space: 2^64 = 18 exabytes");
    println!("   - Most of it is VIRTUAL (not real RAM)");
    println!("   - OS spreads different regions far apart for security");

    println!("\n2. Typical 64-bit Memory Layout (Linux/Windows):");
    println!("   0x0000000000000000 - 0x00007fffffffffff : User space (128 TB)");
    println!("   0x0000000000000000 - 0x0000000000400000 : Program code");
    println!("   0x00007fff00000000 - 0x00007fffffffffff : Stack (grows down)");
    println!("   0x0000000100000000 - 0x00007f0000000000 : Heap (grows up)");
    println!("   0xffff800000000000 - 0xffffffffffffffff : Kernel space");

    println!("\n3. Address Space Layout Randomization (ASLR):");
    println!("   - Security feature that randomizes memory locations");
    println!("   - Stack might start at 0x7fff12345000");
    println!("   - Heap might start at 0x000012ab34000000");
    println!("   - Separation prevents buffer overflow attacks");

    // =======================================================================
    // DEMONSTRATION: MULTIPLE RUNS SHOW DIFFERENT ADDRESSES
    // =======================================================================
    println!("\n🔄 Run this program multiple times - addresses will change!");
    println!("   This proves ASLR (Address Space Layout Randomization) is working");

    // Let's examine the bit patterns
    println!("\n🔢 Bit Pattern Analysis:");
    println!("   Stack addr binary: {:064b}", stack_addr);
    println!("   Heap addr binary:  {:064b}", heap_addr);

    // Count leading zeros to see which part of 64-bit space we're using
    let stack_leading_zeros = stack_addr.leading_zeros();
    let heap_leading_zeros = heap_addr.leading_zeros();

    println!("   Stack using {} of 64 bits", 64 - stack_leading_zeros);
    println!("   Heap using {} of 64 bits", 64 - heap_leading_zeros);

    // =======================================================================
    // PHYSICAL vs VIRTUAL MEMORY
    // =======================================================================
    println!("\n🏭 Physical vs Virtual Reality:");

    println!("   Your computer probably has 8-32 GB RAM");
    println!("   But each process gets a 128 TB virtual address space!");
    println!("   The OS + MMU (Memory Management Unit) maps virtual → physical");

    println!("\n   Example mapping:");
    println!("   Virtual 0x00007fff12345678 → Physical 0x00000000abc12340");
    println!("   Virtual 0x000012ab34567890 → Physical 0x00000000def56780");

    println!("   The 'distance' in virtual space means nothing for physical layout!");

    // =======================================================================
    // PRACTICAL IMPLICATIONS
    // =======================================================================
    println!("\n⚡ Performance Implications:");

    // Show that virtual distance doesn't matter for cache performance
    let array1 = [1u32, 2, 3, 4, 5];
    let array2 = [10u32, 20, 30, 40, 50];

    println!("   Array 1 at: {:p}", &array1);
    println!("   Array 2 at: {:p}", &array2);

    let virtual_distance = (&array2 as *const _ as usize) - (&array1 as *const _ as usize);
    println!("   Virtual distance: {} bytes", virtual_distance);
    println!("   But they're probably adjacent in physical RAM!");
    println!("   Cache performance depends on PHYSICAL proximity, not virtual");

    // =======================================================================
    // DEBUGGING TIPS
    // =======================================================================
    println!("\n🛠️ Debugging Tips:");
    println!("   ✅ Compare addresses within the same allocation");
    println!("   ✅ Use addresses to verify data structure layout");
    println!("   ✅ Check for memory leaks by tracking allocation addresses");
    println!("   ❌ Don't worry about huge virtual address gaps");
    println!("   ❌ Virtual distance != physical distance");

    // Show meaningful address comparison
    println!("\n📏 Meaningful Address Comparisons:");
    let struct_data = (100u32, 200u32, 300u32);
    println!("   Tuple at: {:p}", &struct_data);
    println!("   Field 0: {:p} (offset 0)", &struct_data.0);
    println!(
        "   Field 1: {:p} (offset {})",
        &struct_data.1,
        (&struct_data.1 as *const u32 as usize) - (&struct_data.0 as *const u32 as usize)
    );
    println!(
        "   Field 2: {:p} (offset {})",
        &struct_data.2,
        (&struct_data.2 as *const u32 as usize) - (&struct_data.0 as *const u32 as usize)
    );

    println!("\n🎓 Key Takeaway:");
    println!("   Those 'TB distances' are virtual memory layout, not physical RAM!");
    println!("   Modern OS design puts stack and heap far apart for security.");
    println!("   Your 16GB laptop isn't secretly hiding 2TB of RAM! 😄");
}
