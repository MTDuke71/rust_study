// Memory Location Demonstration
// Run with: cargo run --example memory_location_demo

use std::cell::RefCell;
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<RcNode<T>>>;

#[derive(Debug)]
pub struct RcNode<T> {
    pub data: T,
    pub next: Option<NodeRef<T>>,
}

#[derive(Debug)]
pub struct RcLinkedList<T> {
    head: Option<NodeRef<T>>,
    length: usize,
}

impl<T> RcLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(RcNode {
            data,
            next: self.head.clone(),
        }));
        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn get_node_ref(&self, index: usize) -> Option<NodeRef<T>> {
        if index >= self.length {
            return None;
        }

        let mut current = self.head.clone()?;
        for _ in 0..index {
            let next = current.borrow().next.clone()?;
            current = next;
        }
        Some(current)
    }
}

fn demonstrate_memory_locations() {
    println!("=== Memory Location Analysis ===");

    let mut list = RcLinkedList::new();

    // The list struct itself
    let list_addr = &list as *const _ as usize;
    println!("RcLinkedList struct address (stack): 0x{:x}", list_addr);

    list.push_front(String::from("First"));
    list.push_front(String::from("Second"));

    if let Some(first_node) = list.get_node_ref(0) {
        let heap_addr = first_node.as_ptr() as usize;
        println!("First node heap address: 0x{:x}", heap_addr);

        // Show that the data is also on heap
        let data_addr = &first_node.borrow().data as *const _ as usize;
        println!("First node data address: 0x{:x}", data_addr);

        // Reference count info
        println!("Reference count: {}", Rc::strong_count(&first_node));
    }

    if let Some(second_node) = list.get_node_ref(1) {
        let heap_addr = second_node.as_ptr() as usize;
        println!("Second node heap address: 0x{:x}", heap_addr);

        println!("Reference count: {}", Rc::strong_count(&second_node));
    }

    // Show the difference in addresses
    if let (Some(first), Some(second)) = (list.get_node_ref(0), list.get_node_ref(1)) {
        let addr1 = first.as_ptr() as usize;
        let addr2 = second.as_ptr() as usize;
        let diff = if addr1 > addr2 {
            addr1 - addr2
        } else {
            addr2 - addr1
        };
        println!("Distance between nodes: {} bytes", diff);
    }
}

fn compare_stack_vs_heap_allocation() {
    println!("\n=== Stack vs Heap Allocation Comparison ===");

    // Stack allocation
    let stack_data = [1, 2, 3, 4, 5];
    let stack_addr = &stack_data as *const _ as usize;
    println!("Stack array address: 0x{:x}", stack_addr);

    // Heap allocation with Rc
    let heap_data = Rc::new(RefCell::new(vec![1, 2, 3, 4, 5]));
    let heap_addr = heap_data.as_ptr() as usize;
    println!("Heap Rc<RefCell<Vec>> address: 0x{:x}", heap_addr);

    // The difference shows heap vs stack
    println!("Stack grows down, heap grows up (typically)");
    if heap_addr > stack_addr {
        println!("✅ Heap address > Stack address (as expected)");
    } else {
        println!("⚠️  Unusual memory layout (depends on OS/allocator)");
    }
}

fn demonstrate_heap_allocation_overhead() {
    println!("\n=== Heap Allocation Overhead ===");

    // Simple Box allocation
    let box_node = Box::new(42);
    println!("Box<i32> - Simple heap allocation");
    println!("  Data: {}", *box_node);
    println!("  Address: 0x{:x}", &*box_node as *const _ as usize);

    // Rc allocation with overhead
    let rc_data = Rc::new(RefCell::new(42));
    println!("Rc<RefCell<i32>> - Heap allocation with overhead");
    println!("  Data: {}", *rc_data.borrow());
    println!("  Address: 0x{:x}", rc_data.as_ptr() as usize);
    println!("  Strong count: {}", Rc::strong_count(&rc_data));
    println!("  Weak count: {}", Rc::weak_count(&rc_data));

    // Memory usage comparison
    println!("\nMemory overhead:");
    println!("  Box<i32>: ~8 bytes (just the pointer on stack)");
    println!("  Rc<RefCell<i32>>: ~24+ bytes (Rc metadata + RefCell + data)");
}

fn main() {
    demonstrate_memory_locations();
    compare_stack_vs_heap_allocation();
    demonstrate_heap_allocation_overhead();

    println!("\n📊 Summary:");
    println!("┌─────────────────────┬─────────────┬─────────────────┐");
    println!("│ Component           │ Location    │ Notes           │");
    println!("├─────────────────────┼─────────────┼─────────────────┤");
    println!("│ RcLinkedList struct │ Stack       │ Small metadata  │");
    println!("│ All RcNode data     │ Heap        │ Allocated nodes │");
    println!("│ Rc control blocks   │ Heap        │ Reference count │");
    println!("│ RefCell metadata    │ Heap        │ Borrow flags    │");
    println!("│ User data T         │ Heap        │ Inside nodes    │");
    println!("└─────────────────────┴─────────────┴─────────────────┘");

    println!("\n🔑 Key Points:");
    println!("• The list STRUCTURE is on the stack (like a Vec)");
    println!("• All NODES and DATA are on the heap");
    println!("• Rc adds reference counting overhead on heap");
    println!("• RefCell adds borrow checking overhead on heap");
    println!("• Every node allocation includes control structures");
}
