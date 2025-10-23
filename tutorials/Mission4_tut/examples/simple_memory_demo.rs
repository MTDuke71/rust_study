// SimpleLinkedList Memory Location Demonstration
// Run with: cargo run --example simple_memory_demo

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

fn demonstrate_simple_memory_locations() {
    println!("=== SimpleLinkedList Memory Analysis ===");

    let mut list = SimpleLinkedList::new();

    // The list struct itself (STACK)
    let list_addr = &list as *const _ as usize;
    println!("SimpleLinkedList struct address (STACK): 0x{:x}", list_addr);
    println!(
        "  Size: {} bytes",
        std::mem::size_of::<SimpleLinkedList<String>>()
    );

    list.push_front(String::from("First"));
    list.push_front(String::from("Second"));

    // Get addresses of heap-allocated nodes
    if let Some(ref first_box) = list.head {
        let heap_addr = &**first_box as *const _ as usize;
        println!("First node address (HEAP): 0x{:x}", heap_addr);
        println!("  Node size: {} bytes", std::mem::size_of::<Node<String>>());

        // The data inside the node
        let data_addr = &first_box.data as *const _ as usize;
        println!("First node data address (HEAP): 0x{:x}", data_addr);

        // Check if there's a second node
        if let Some(ref second_box) = first_box.next {
            let second_heap_addr = &**second_box as *const _ as usize;
            println!("Second node address (HEAP): 0x{:x}", second_heap_addr);

            // Distance between nodes
            let distance = heap_addr.abs_diff(second_heap_addr);
            println!("Distance between nodes: {} bytes", distance);
        }
    }
}

fn compare_stack_vs_heap_for_simple() {
    println!("\n=== Stack vs Heap for SimpleLinkedList ===");

    // Stack data
    let stack_array = [1, 2, 3, 4, 5];
    let stack_addr = &stack_array as *const _ as usize;
    println!("Stack array address: 0x{:x}", stack_addr);

    // SimpleLinkedList struct (stack)
    let mut list = SimpleLinkedList::new();
    let list_struct_addr = &list as *const _ as usize;
    println!("List struct address (STACK): 0x{:x}", list_struct_addr);

    // Add data (heap allocations)
    list.push_front(42);
    if let Some(ref node_box) = list.head {
        let node_heap_addr = &**node_box as *const _ as usize;
        println!("Node data address (HEAP): 0x{:x}", node_heap_addr);

        // Show the pattern
        if node_heap_addr > list_struct_addr {
            println!("✅ Node (heap) address > List struct (stack) address");
        } else {
            println!("⚠️  Unusual memory layout");
        }
    }
}

fn demonstrate_box_allocation_overhead() {
    println!("\n=== Box<T> Allocation Overhead ===");

    // Direct stack allocation
    let stack_value = 42i32;
    println!("Stack i32:");
    println!("  Value: {}", stack_value);
    println!("  Address: 0x{:x}", &stack_value as *const _ as usize);
    println!("  Size: {} bytes", std::mem::size_of::<i32>());

    // Box heap allocation
    let box_value = Box::new(42i32);
    println!("Box<i32>:");
    println!("  Value: {}", *box_value);
    println!(
        "  Heap address: 0x{:x}",
        box_value.as_ref() as *const _ as usize
    );
    println!(
        "  Stack pointer size: {} bytes",
        std::mem::size_of::<Box<i32>>()
    );
    println!("  Heap data size: {} bytes", std::mem::size_of::<i32>());

    // Node allocation
    let node = Box::new(Node {
        data: String::from("Hello"),
        next: None,
    });
    println!("Box<Node<String>>:");
    println!("  Value: {:?}", node.data);
    println!("  Heap address: 0x{:x}", node.as_ref() as *const _ as usize);
    println!("  Node size: {} bytes", std::mem::size_of::<Node<String>>());
}

fn memory_efficiency_comparison() {
    println!("\n=== Memory Efficiency Comparison ===");

    let mut simple_list = SimpleLinkedList::new();
    simple_list.push_front(String::from("A"));
    simple_list.push_front(String::from("B"));

    // Stack usage
    let list_size = std::mem::size_of::<SimpleLinkedList<String>>();
    println!("SimpleLinkedList struct (stack): {} bytes", list_size);

    // Per-node heap usage
    let node_size = std::mem::size_of::<Node<String>>();
    println!("Per Node<String> (heap): {} bytes", node_size);

    // String overhead
    let string_size = std::mem::size_of::<String>();
    println!("String metadata (within node): {} bytes", string_size);

    println!("\nFor 2 nodes:");
    println!("  Stack: {} bytes (list struct)", list_size);
    println!("  Heap: {} bytes (2 nodes)", node_size * 2);
    println!("  Plus: Variable bytes for string contents");
}

fn main() {
    demonstrate_simple_memory_locations();
    compare_stack_vs_heap_for_simple();
    demonstrate_box_allocation_overhead();
    memory_efficiency_comparison();

    println!("\n📊 SimpleLinkedList Memory Summary:");
    println!("┌─────────────────────┬─────────────┬─────────────────┐");
    println!("│ Component           │ Location    │ Notes           │");
    println!("├─────────────────────┼─────────────┼─────────────────┤");
    println!("│ SimpleLinkedList    │ Stack       │ ~16 bytes       │");
    println!("│ All Node<T> data    │ Heap        │ Box allocations │");
    println!("│ User data T         │ Heap        │ Inside nodes    │");
    println!("│ Box pointers        │ Stack       │ Part of nodes   │");
    println!("└─────────────────────┴─────────────┴─────────────────┘");

    println!("\n🔄 Comparison with RcLinkedList:");
    println!("┌─────────────────────┬─────────────────┬─────────────────┐");
    println!("│ Aspect              │ SimpleLinkedList│ RcLinkedList    │");
    println!("├─────────────────────┼─────────────────┼─────────────────┤");
    println!("│ List struct         │ Stack (~16b)    │ Stack (~16b)    │");
    println!("│ Node allocation     │ Heap (Box)      │ Heap (Rc+RefCell)");
    println!("│ Per-node overhead   │ Minimal         │ High (ref count)│");
    println!("│ Cache locality      │ Poor            │ Worse           │");
    println!("│ Sharing capable     │ No              │ Yes             │");
    println!("└─────────────────────┴─────────────────┴─────────────────┘");

    println!("\n🔑 Key Points:");
    println!("• BOTH list types have the same stack/heap pattern");
    println!("• The list STRUCTURE is always on the stack");
    println!("• ALL NODES are always on the heap");
    println!("• Box<T> = simple heap allocation (no overhead)");
    println!("• Rc<RefCell<T>> = heap allocation + reference counting");
    println!("• Neither has good cache locality (scattered heap allocations)");
}
