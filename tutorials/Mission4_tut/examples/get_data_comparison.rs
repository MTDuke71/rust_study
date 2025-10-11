// Comparison: get_data() in Box<T> vs Rc<RefCell<T>>
// Run with: cargo run --example get_data_comparison

use std::cell::RefCell;
use std::rc::Rc;

// ==================== Box<T> Implementation ====================

#[derive(Debug)]
struct BoxNode<T> {
    data: T,
    next: Option<Box<BoxNode<T>>>,
}

#[derive(Debug)]
pub struct BoxLinkedList<T> {
    head: Option<Box<BoxNode<T>>>,
    length: usize,
}

impl<T> BoxLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Box::new(BoxNode {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn len(&self) -> usize {
        self.length
    }

    // ✅ YES - get_data() IS possible with Box<T>!
    pub fn get_data(&self, index: usize) -> Option<T>
    where
        T: Clone,
    {
        if index >= self.length {
            return None;
        }

        let mut current = self.head.as_ref()?;
        for _ in 0..index {
            current = current.next.as_ref()?;
        }
        Some(current.data.clone()) // Clone the data
    }

    // Alternative: get_data_ref() - borrows instead of cloning
    pub fn get_data_ref(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            return None;
        }

        let mut current = self.head.as_ref()?;
        for _ in 0..index {
            current = current.next.as_ref()?;
        }
        Some(&current.data)
    }

    // ❌ This is NOT possible with Box<T> - no shared ownership!
    // pub fn get_node_ref(&self, index: usize) -> Option<Box<BoxNode<T>>> {
    //     // Can't return Box<T> because it would move ownership
    //     // Can't return &Box<T> because lifetime issues
    // }
}

// ==================== Rc<RefCell<T>> Implementation ====================

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

    pub fn len(&self) -> usize {
        self.length
    }

    // ✅ get_data() - clones the data (no sharing)
    pub fn get_data(&self, index: usize) -> Option<T>
    where
        T: Clone,
    {
        self.get_node_ref(index)
            .map(|node| node.borrow().data.clone())
    }

    // ✅ get_node_ref() - shares ownership (only possible with Rc!)
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

// ==================== Comparison Functions ====================

fn demonstrate_box_get_data() {
    println!("=== Box<T> get_data() Demo ===");

    let mut list = BoxLinkedList::new();
    list.push_front(String::from("World"));
    list.push_front(String::from("Hello"));

    // ✅ Works! Clones the data
    if let Some(data) = list.get_data(0) {
        println!("Box get_data(0): {}", data);
    }

    if let Some(data) = list.get_data(1) {
        println!("Box get_data(1): {}", data);
    }

    // ✅ Alternative: borrow the data (no clone needed)
    if let Some(data_ref) = list.get_data_ref(0) {
        println!("Box get_data_ref(0): {}", data_ref);
    }

    println!("Box list still works: len = {}", list.len());
}

fn demonstrate_rc_get_data() {
    println!("=== Rc<RefCell<T>> get_data() Demo ===");

    let mut list = RcLinkedList::new();
    list.push_front(String::from("World"));
    list.push_front(String::from("Hello"));

    // ✅ Works! Clones the data
    if let Some(data) = list.get_data(0) {
        println!("Rc get_data(0): {}", data);
    }

    // ✅ BONUS: Can also get shared references to nodes!
    if let Some(node_ref) = list.get_node_ref(0) {
        println!("Rc get_node_ref(0): {:?}", node_ref.borrow().data);
        println!("Reference count: {}", Rc::strong_count(&node_ref));
    }

    println!("Rc list still works: len = {}", list.len());
}

fn demonstrate_key_differences() {
    println!("=== Key Differences ===");

    let mut box_list = BoxLinkedList::new();
    let mut rc_list = RcLinkedList::new();

    box_list.push_front(42);
    rc_list.push_front(42);

    // Both can clone data
    let box_data = box_list.get_data(0).unwrap();
    let rc_data = rc_list.get_data(0).unwrap();

    println!("Both got the same data: {}", box_data == rc_data);

    // But only Rc can share node references
    // let box_node = box_list.get_node_ref(0);  // ❌ Not possible!
    let rc_node = rc_list.get_node_ref(0); // ✅ Works!

    println!("Only Rc can share node ownership: {:?}", rc_node.is_some());
}

fn main() {
    demonstrate_box_get_data();
    println!();
    demonstrate_rc_get_data();
    println!();
    demonstrate_key_differences();

    println!();
    println!("📊 Comparison Summary:");
    println!("┌─────────────────────┬─────────────┬─────────────────┐");
    println!("│ Operation           │ Box<T>      │ Rc<RefCell<T>>  │");
    println!("├─────────────────────┼─────────────┼─────────────────┤");
    println!("│ get_data() clone    │ ✅ Yes      │ ✅ Yes          │");
    println!("│ get_data_ref() borrow│ ✅ Yes      │ ✅ Yes          │");
    println!("│ get_node_ref() share │ ❌ No       │ ✅ Yes          │");
    println!("│ Performance         │ ⚡ Faster   │ 🐌 Slower       │");
    println!("│ Memory usage        │ 💾 Lower    │ 💾 Higher       │");
    println!("│ Flexibility         │ 🔒 Limited  │ 🔓 High         │");
    println!("└─────────────────────┴─────────────┴─────────────────┘");

    println!();
    println!("🔑 Key Insights:");
    println!("• get_data() is possible with BOTH patterns");
    println!("• Box<T> can only clone or borrow data");
    println!("• Rc<RefCell<T>> can ALSO share node ownership");
    println!("• Choose based on whether you need sharing capabilities");
}
