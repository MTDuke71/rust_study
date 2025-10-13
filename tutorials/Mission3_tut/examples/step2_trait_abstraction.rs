// Step 2: Trait Abstraction
//
// Learning Objectives:
// - Define Searchable trait with associated types
// - Implement trait for multiple container types
// - Use trait bounds for generic functions
// - Understand deep module pattern

fn main() {
    println!("=== Step 2: Trait Abstraction ===\n");

    // Section 1: Why Traits?
    println!("📖 Section 1: The Problem - Code Duplication\n");
    println!("Without traits, we'd need separate functions for:");
    println!("- search_slice(&[T], &T)");
    println!("- search_vec(&Vec<T>, &T)");
    println!("- search_array(&[T; N], &T)");
    println!("\nTraits let us write ONE generic function!\n");

    // Section 2: Defining the Searchable Trait
    println!("📖 Section 2: The Searchable Trait\n");
    
    let slice_data = [1, 3, 5, 7, 9];
    let vec_data = vec![2, 4, 6, 8, 10];
    let array_data = [11, 13, 15, 17, 19];
    
    // All use the same generic search function!
    println!("Slice: {:?}", search_in(&slice_data, &5));
    println!("Vec:   {:?}", search_in(&vec_data, &6));
    println!("Array: {:?}", search_in(&array_data, &15));

    // Section 3: Associated Types
    println!("\n📖 Section 3: Associated Types in Traits\n");
    println!("Associated types let traits specify:");
    println!("- Item type (what elements are stored)");
    println!("- Index type (usize for most containers)\n");
    
    demonstrate_associated_types();

    // Section 4: Custom Implementation
    println!("\n📖 Section 4: Implementing Searchable for Custom Types\n");
    
    let custom = CustomSortedList { data: vec![1, 3, 5, 7, 9] };
    println!("Custom container: {:?}", search_in(&custom, &5));

    // Self-Assessment
    println!("\n✅ Self-Assessment:");
    println!("   □ I can define a trait with associated types");
    println!("   □ I understand trait bounds on generics");
    println!("   □ I can implement traits for different types");
    println!("\nNext: `cargo run --example step3_iterator_patterns`");
}

/// The Searchable trait - abstracts over searchable containers
trait Searchable {
    type Item;
    
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> Option<&Self::Item>;
    
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Implementation for slices
impl<T> Searchable for [T] {
    type Item = T;
    
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
    
    fn get(&self, index: usize) -> Option<&Self::Item> {
        <[T]>::get(self, index)
    }
}

/// Implementation for Vec
impl<T> Searchable for Vec<T> {
    type Item = T;
    
    fn len(&self) -> usize {
        Vec::len(self)
    }
    
    fn get(&self, index: usize) -> Option<&Self::Item> {
        self.as_slice().get(index)
    }
}

/// Implementation for arrays
impl<T, const N: usize> Searchable for [T; N] {
    type Item = T;
    
    fn len(&self) -> usize {
        N
    }
    
    fn get(&self, index: usize) -> Option<&Self::Item> {
        <[T]>::get(self, index)
    }
}

/// Generic search function - works with any Searchable container
fn search_in<S, T>(container: &S, target: &T) -> Result<usize, usize>
where
    S: Searchable<Item = T> + ?Sized,
    T: Ord,
{
    let mut left = 0;
    let mut right = container.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        let mid_value = container.get(mid).expect("mid in bounds");
        
        use std::cmp::Ordering;
        match mid_value.cmp(target) {
            Ordering::Equal => return Ok(mid),
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,
        }
    }
    
    Err(left)
}

/// Custom container implementing Searchable
struct CustomSortedList<T> {
    data: Vec<T>,
}

impl<T> Searchable for CustomSortedList<T> {
    type Item = T;
    
    fn len(&self) -> usize {
        self.data.len()
    }
    
    fn get(&self, index: usize) -> Option<&Self::Item> {
        self.data.get(index)
    }
}

fn demonstrate_associated_types() {
    let data: Vec<i32> = vec![1, 2, 3];
    
    // The associated type Item is i32
    // The index type is usize
    println!("Container length: {}", data.len());
    println!("Item at index 1: {:?}", data.get(1));
}
