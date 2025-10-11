/// Basic demonstration of Mission3 binary search capabilities
///
/// This example shows the fundamental search operations and their usage patterns.
use mission3::*;

fn main() {
    println!("=== Mission 3: Binary Search Demo ===\n");

    // REQ-1: Basic slice search
    basic_slice_search();

    // REQ-2: Trait-based search
    trait_based_search();

    // REQ-3: Iterator integration
    iterator_search();

    // REQ-4: Custom ordering
    custom_ordering_search();

    // REQ-5: Lifetime safety demonstration
    lifetime_safety_demo();

    println!("All demos completed successfully! ✅");
}

fn basic_slice_search() {
    println!("🔍 Basic Slice Search (REQ-1)");

    let numbers = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    println!("Searching in: {:?}", numbers);

    // Found cases
    for target in [1, 7, 19] {
        match binary_search::search_slice(&numbers, &target) {
            Ok(index) => println!("  ✅ Found {} at index {}", target, index),
            Err(_) => println!("  ❌ {} not found", target),
        }
    }

    // Not found cases - show insertion points
    for target in [0, 4, 20] {
        match binary_search::search_slice(&numbers, &target) {
            Ok(index) => println!("  ✅ Found {} at index {}", target, index),
            Err(insert_idx) => {
                println!("  📍 {} would be inserted at index {}", target, insert_idx)
            }
        }
    }

    println!();
}

fn trait_based_search() {
    println!("🏗️ Trait-Based Search (REQ-2)");

    // Search in Vec
    let vec_data = vec![2, 4, 6, 8, 10, 12];
    println!("Searching Vec: {:?}", vec_data);

    match searchable::search_in(&vec_data, &8) {
        Ok(idx) => println!("  ✅ Found 8 in Vec at index {}", idx),
        Err(_) => println!("  ❌ 8 not found in Vec"),
    }

    // Search in slice (same trait)
    let slice_data = [1, 3, 5, 7, 9];
    println!("Searching slice: {:?}", slice_data);

    match searchable::search_in(&slice_data, &5) {
        Ok(idx) => println!("  ✅ Found 5 in slice at index {}", idx),
        Err(_) => println!("  ❌ 5 not found in slice"),
    }

    // Search by key
    let people = [(1, "Alice"), (3, "Bob"), (5, "Charlie"), (7, "Diana")];
    println!("Searching people by ID: {:?}", people);

    match searchable::search_in_by_key(&people, &5, |person| person.0) {
        Ok(idx) => println!("  ✅ Found person with ID 5: {}", people[idx].1),
        Err(_) => println!("  ❌ No person with ID 5"),
    }

    println!();
}

fn iterator_search() {
    println!("🔄 Iterator Integration (REQ-3)");

    let data = [1, 2, 2, 2, 3, 4, 5, 5, 5, 6, 7];
    println!("Data with duplicates: {:?}", data);

    // Find all equal elements
    let twos: Vec<_> = search_iter::find_all_equal(&data, &2).collect();
    println!("  All 2s: {:?}", twos);

    let fives: Vec<_> = search_iter::find_all_equal(&data, &5).collect();
    println!("  All 5s: {:?}", fives);

    // Range search
    let range_3_to_5: Vec<_> = search_iter::find_range(&data, &3, &5).collect();
    println!("  Range [3,5]: {:?}", range_3_to_5);

    // Using extension trait
    use search_iter::SearchExt;
    let sixes: Vec<_> = data.find_all_equal(&6).collect();
    println!("  All 6s (extension trait): {:?}", sixes);

    // Find first matching with predicate
    let first_big = search_iter::find_first_matching(&data, |&x| x >= 5);
    println!("  First element >= 5: {:?}", first_big);

    println!();
}

fn custom_ordering_search() {
    println!("🎯 Custom Ordering (REQ-4)");

    // Search by extracted key
    let items = [
        (100, "expensive"),
        (50, "medium"),
        (25, "cheap"),
        (75, "moderate"),
    ];
    let mut sorted_items = items.clone();
    sorted_items.sort_by_key(|item| item.0); // Sort by price

    println!("Items sorted by price: {:?}", sorted_items);

    match binary_search::search_by_key(&sorted_items, &50, |item| item.0) {
        Ok(idx) => println!("  ✅ Found item with price 50: {}", sorted_items[idx].1),
        Err(_) => println!("  ❌ No item with price 50"),
    }

    // Reverse ordering
    let desc_numbers = [20, 18, 16, 14, 12, 10, 8, 6, 4, 2];
    println!("Reverse sorted: {:?}", desc_numbers);

    match binary_search::search_by(&desc_numbers, &12, |a, b| b.cmp(a)) {
        Ok(idx) => println!("  ✅ Found 12 at index {} in reverse order", idx),
        Err(_) => println!("  ❌ 12 not found"),
    }

    // Complex custom type
    #[derive(Debug)]
    struct Student {
        name: String,
        grade: f32,
    }

    let mut students = vec![
        Student {
            name: "Alice".to_string(),
            grade: 95.5,
        },
        Student {
            name: "Bob".to_string(),
            grade: 87.3,
        },
        Student {
            name: "Charlie".to_string(),
            grade: 92.1,
        },
        Student {
            name: "Diana".to_string(),
            grade: 88.7,
        },
    ];

    // Sort by grade
    students.sort_by(|a, b| a.grade.partial_cmp(&b.grade).unwrap());
    println!("Students sorted by grade:");
    for (i, student) in students.iter().enumerate() {
        println!("  [{}] {} - {:.1}", i, student.name, student.grade);
    }

    // Search by grade (convert to integer for comparison)
    let target_grade = 887; // 88.7 * 10 as integer
    match binary_search::search_by_key(&students, &target_grade, |s| (s.grade * 10.0) as u32) {
        Ok(idx) => println!(
            "  ✅ Found student with grade ~88.7: {}",
            students[idx].name
        ),
        Err(_) => println!("  ❌ No student with that grade"),
    }

    println!();
}

fn lifetime_safety_demo() {
    println!("🔒 Lifetime Safety (REQ-5)");

    let data = vec![10, 20, 30, 40, 50];

    // Function that returns a reference with proper lifetime
    fn find_and_return<'a>(slice: &'a [i32], target: i32) -> Option<&'a i32> {
        match binary_search::search_slice(slice, &target) {
            Ok(idx) => slice.get(idx),
            Err(_) => None,
        }
    }

    if let Some(found) = find_and_return(&data, 30) {
        println!("  ✅ Safely returned reference: {}", found);
    }

    // Iterator references are also lifetime-safe
    let range_refs: Vec<_> = { search_iter::find_range(&data, &20, &40).collect() };
    println!("  ✅ Range references: {:?}", range_refs);

    // This demonstrates that the borrow checker ensures safety
    println!("  ✅ All references are valid and memory-safe");

    println!();
}
