// Example 8: Full Chapter - Comprehensive Integration
// Demonstrates all Ch2 concepts working together

use rustaceans_ch02_types::utils::{print_section, print_subsection};
use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;

fn main() {
    print_section("RUST FOR RUSTACEANS - CHAPTER 2: COMPLETE EXAMPLE");
    
    println!("\nThis example demonstrates all chapter concepts:");
    println!("  1. Types in Memory (layout, alignment, DSTs)");
    println!("  2. Traits (dispatch, generics, coherence)");
    println!("  3. Advanced bounds (HRTB)");
    println!("  4. Marker traits and type-state");
    println!("  5. Existential types (impl Trait)");
    
    data_processing_pipeline();
}

// =============================================================================
// Comprehensive Example: Type-Safe Data Processing Pipeline
// =============================================================================

fn data_processing_pipeline() {
    print_section("TYPE-SAFE DATA PROCESSING PIPELINE");
    
    // 1. Create unvalidated data
    let pipeline = DataPipeline::new();
    
    // 2. Validate (type-state transition)
    let pipeline = pipeline.validate(vec![1, 2, 3, 4, 5, 10, 20, 30]);
    
    // 3. Process with static dispatch
    let pipeline = pipeline.process_static();
    
    // 4. Format results
    pipeline.display_results();
}

// --- Type-State Pattern: Pipeline States ---
struct Unvalidated;
struct Validated;
struct Processed;

// --- Pipeline with State ---
struct DataPipeline<State> {
    data: Vec<i32>,
    results: Vec<String>,
    _state: PhantomData<State>,
}

// Unvalidated state
impl DataPipeline<Unvalidated> {
    fn new() -> Self {
        print_subsection("1. Creating Pipeline (Unvalidated State)");
        println!("Type-state ensures proper initialization sequence");
        
        DataPipeline {
            data: Vec::new(),
            results: Vec::new(),
            _state: PhantomData,
        }
    }
    
    fn validate(self, data: Vec<i32>) -> DataPipeline<Validated> {
        print_subsection("2. Validating Data");
        println!("Input: {:?}", data);
        println!("✅ Data validated, transitioning to Validated state");
        
        DataPipeline {
            data,
            results: Vec::new(),
            _state: PhantomData,
        }
    }
}

// Validated state
impl DataPipeline<Validated> {
    fn process_static(self) -> DataPipeline<Processed> {
        print_subsection("3. Processing with Static Dispatch");
        
        // Use impl Trait for iterator chain
        let results = self.process_iterator()
            .map(|x| format!("Result: {}", x))
            .collect();
        
        println!("Processed {} items using static dispatch", self.data.len());
        
        DataPipeline {
            data: self.data,
            results,
            _state: PhantomData,
        }
    }
    
    // impl Trait - existential type
    fn process_iterator(&self) -> impl Iterator<Item = i32> + '_ {
        self.data.iter()
            .map(|&x| x * 2)
            .filter(|&x| x > 5)
    }
}

// Processed state  
impl DataPipeline<Processed> {
    fn display_results(&self) {
        print_subsection("4. Displaying Results");
        
        // Use dynamic dispatch for display
        let formatters: Vec<Box<dyn ResultFormatter>> = vec![
            Box::new(SimpleFormatter),
            Box::new(DetailedFormatter),
        ];
        
        for formatter in &formatters {
            println!("\n{}", formatter.name());
            formatter.format_results(&self.results);
        }
    }
}

// --- Trait with Associated Type ---
trait ResultFormatter {
    fn format_results(&self, results: &[String]);
    fn name(&self) -> &str;
}

struct SimpleFormatter;
struct DetailedFormatter;

impl ResultFormatter for SimpleFormatter {
    fn format_results(&self, results: &[String]) {
        for result in results {
            println!("  {}", result);
        }
    }
    
    fn name(&self) -> &str {
        "Simple Formatter"
    }
}

impl ResultFormatter for DetailedFormatter {
    fn format_results(&self, results: &[String]) {
        println!("  Total results: {}", results.len());
        for (i, result) in results.iter().enumerate() {
            println!("  [{}] {}", i + 1, result);
        }
    }
    
    fn name(&self) -> &str {
        "Detailed Formatter"
    }
}

// --- Memory Layout Demonstration ---
#[allow(dead_code)]
mod memory_layout {
    use std::mem;
    
    #[repr(C)]
    pub struct OptimizedData {
        pub id: u64,        // 8 bytes, align 8
        pub count: u32,     // 4 bytes, align 4
        pub active: bool,   // 1 byte, align 1
        // 3 bytes padding to align struct size to 16
    }
    
    pub fn show_layout() {
        println!("\nMemory Layout (repr(C)):");
        println!("  Size: {} bytes", mem::size_of::<OptimizedData>());
        println!("  Alignment: {} bytes", mem::align_of::<OptimizedData>());
    }
}

// --- Generic Trait Example ---
#[allow(dead_code)]
trait Transform<Input> {
    type Output;
    fn transform(&self, input: Input) -> Self::Output;
}

#[allow(dead_code)]
struct Doubler;

impl Transform<i32> for Doubler {
    type Output = i32;
    fn transform(&self, input: i32) -> i32 {
        input * 2
    }
}

impl Transform<String> for Doubler {
    type Output = String;
    fn transform(&self, input: String) -> String {
        input.repeat(2)
    }
}

// --- HRTB Example ---
#[allow(dead_code)]
fn process_with_hrtb<F>(f: F) -> String
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    let s = "hello world";
    f(s).to_string()
}

// --- Coherence Example ---
#[allow(dead_code)]
struct LocalType(i32);

// Local trait + local type = OK
#[allow(dead_code)]
trait LocalTrait {
    fn local_method(&self) -> String;
}

impl LocalTrait for LocalType {
    fn local_method(&self) -> String {
        format!("LocalType({})", self.0)
    }
}

// Foreign trait + local type = OK
impl Display for LocalType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "LocalType {{ value: {} }}", self.0)
    }
}

// --- Marker Type Example ---
#[allow(dead_code)]
struct Marker<T> {
    data: Vec<u8>,
    _phantom: PhantomData<T>,
}

// --- Tests (at root level for individual runnables) ---

#[test]
fn test_type_state_transitions() {
    let pipeline = DataPipeline::new();
    let pipeline = pipeline.validate(vec![1, 2, 3]);
    let pipeline = pipeline.process_static();
    
    // Type system ensures correct order
    assert!(!pipeline.results.is_empty());
}

#[test]
fn test_static_dispatch() {
    let doubler = Doubler;
    assert_eq!(doubler.transform(5), 10);
    assert_eq!(doubler.transform("hi".to_string()), "hihi");
}

#[test]
fn test_impl_trait() {
    let pipeline = DataPipeline::new();
    let pipeline = pipeline.validate(vec![1, 5, 10]);
    
    let iter = pipeline.process_iterator();
    let results: Vec<_> = iter.collect();
    
    assert_eq!(results, vec![10, 20]);  // Doubled and filtered > 5
}

#[test]
fn test_memory_layout() {
    use memory_layout::*;
    use std::mem;
    
    // Demonstrate memory layout
    let data = OptimizedData {
        id: 42,
        count: 100,
        active: true,
    };
    
    println!("\nMemory Layout Demonstration:");
    println!("  Size: {} bytes", mem::size_of::<OptimizedData>());
    println!("  Alignment: {} bytes", mem::align_of::<OptimizedData>());
    println!("  Data: id={}, count={}, active={}", data.id, data.count, data.active);
    
    // Verify expected layout
    assert_eq!(mem::size_of::<OptimizedData>(), 16);  // 8 + 4 + 1 + 3 padding
    assert_eq!(mem::align_of::<OptimizedData>(), 8);
}

#[test]
fn test_generic_trait() {
    let doubler = Doubler;
    
    // Same trait, different input/output types
    let num_result = doubler.transform(21);
    println!("Doubler::transform(21) = {}", num_result);
    assert_eq!(num_result, 42);
    
    let str_result = doubler.transform("Rust".to_string());
    println!("Doubler::transform(\"Rust\") = {}", str_result);
    assert_eq!(str_result, "RustRust");
}

#[test]
fn test_hrtb() {
    // Higher-Rank Trait Bounds: function works with any lifetime
    // Using a helper function that properly handles the lifetime
    fn first_word(s: &str) -> &str {
        s.split_whitespace().next().unwrap_or(s)
    }
    
    let result = process_with_hrtb(first_word);
    println!("HRTB result: {}", result);
    assert_eq!(result, "hello");
}

#[test]
fn test_coherence() {
    let local = LocalType(42);
    
    // Using local trait implementation
    let local_result = local.local_method();
    println!("LocalTrait: {}", local_result);
    assert_eq!(local_result, "LocalType(42)");
    
    // Using Display trait implementation
    let display_result = format!("{}", local);
    println!("Display: {}", display_result);
    assert!(display_result.contains("42"));
}

#[test]
fn test_marker_type() {
    use std::marker::PhantomData;
    
    // PhantomData allows us to "mark" a type parameter without storing it
    let marker: Marker<String> = Marker {
        data: vec![1, 2, 3, 4, 5],
        _phantom: PhantomData,
    };
    
    println!("Marker type with {} bytes", marker.data.len());
    assert_eq!(marker.data.len(), 5);
    
    // The type parameter String is used for type safety but takes no space
    assert_eq!(std::mem::size_of::<Marker<String>>(), std::mem::size_of::<Vec<u8>>());
}
