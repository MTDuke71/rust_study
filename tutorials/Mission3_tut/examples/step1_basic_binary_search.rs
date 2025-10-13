// Step 1: Basic Binary Search
//
// Learning Objectives:
// - Understand binary search algorithm mechanics
// - Implement O(log n) search on sorted slices
// - Use Result<T, E> for found vs not-found cases
// - Master slice borrowing basics

fn main() {
    println!("=== Mission3 Tutorial - Step 1: Basic Binary Search ===\n");

    // Section 1: Understanding Binary Search
    println!("📖 Section 1: Binary Search Fundamentals\n");

    println!("Binary search works on SORTED data by repeatedly dividing");
    println!("the search space in half until the target is found.\n");

    println!("Time Complexity: O(log n) - much faster than O(n) linear search");
    println!("Example: 1 million elements = ~20 comparisons vs 500,000 average\n");

    // Section 2: Minimal Implementation
    println!("\n📖 Section 2: Minimal Binary Search Implementation\n");

    let sorted_data = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];

    println!("Searching in: {:?}", sorted_data);
    println!();

    // Find existing element
    let target = 7;
    match basic_binary_search(&sorted_data, &target) {
        Ok(index) => println!("✅ Found {} at index {}", target, index),
        Err(insert_pos) => println!("❌ Not found. Would insert at {}", insert_pos),
    }

    // Find non-existing element
    let target = 8;
    match basic_binary_search(&sorted_data, &target) {
        Ok(index) => println!("✅ Found {} at index {}", target, index),
        Err(insert_pos) => {
            println!("❌ Not found. Would insert at index {}", insert_pos);
            println!(
                "   This maintains sorted order: before {:?}, after would be {:?}",
                &sorted_data[..insert_pos],
                &sorted_data[insert_pos..]
            );
        }
    }

    // Section 3: Step-by-Step Walkthrough
    println!("\n\n📖 Section 3: How Binary Search Works (Step-by-Step)\n");

    let data = [1, 3, 5, 7, 9, 11, 13, 15];
    let target = 11;

    println!("Searching for {} in {:?}", target, data);
    binary_search_with_trace(&data, &target);

    // Section 4: Edge Cases
    println!("\n\n📖 Section 4: Important Edge Cases\n");

    // Empty slice
    let empty: [i32; 0] = [];
    println!("Empty slice: {:?}", basic_binary_search(&empty, &5));

    // Single element - found
    let single = [42];
    println!(
        "Single element (found): {:?}",
        basic_binary_search(&single, &42)
    );

    // Single element - not found
    println!(
        "Single element (not found): {:?}",
        basic_binary_search(&single, &10)
    );

    // First element
    let data = [1, 2, 3, 4, 5];
    println!("First element: {:?}", basic_binary_search(&data, &1));

    // Last element
    println!("Last element: {:?}", basic_binary_search(&data, &5));

    // Not found (too small)
    println!("Too small: {:?}", basic_binary_search(&data, &0));

    // Not found (too large)
    println!("Too large: {:?}", basic_binary_search(&data, &10));

    // Section 5: Generic Binary Search
    println!("\n\n📖 Section 5: Generic Binary Search (Works with Any Type)\n");

    // Works with different numeric types
    let integers = [1, 2, 3, 4, 5];
    println!("Integer search: {:?}", basic_binary_search(&integers, &3));

    // Works with strings
    let words = ["apple", "banana", "cherry", "date", "elderberry"];
    println!(
        "String search: {:?}",
        basic_binary_search(&words, &"cherry")
    );

    // Works with tuples (sorted by first element)
    let pairs = [(1, "one"), (2, "two"), (3, "three"), (5, "five")];
    println!(
        "Tuple search: {:?}",
        basic_binary_search(&pairs, &(3, "three"))
    );

    // Section 6: Performance Comparison
    println!("\n\n📖 Section 6: Performance Comparison\n");

    let large_data: Vec<i32> = (0..10000).map(|i| i * 2).collect();

    // Binary search
    let target = 5000;
    let mut comparisons = 0;
    let result = binary_search_with_count(&large_data, &target, &mut comparisons);
    println!(
        "Binary search found {} at {:?} with {} comparisons",
        target, result, comparisons
    );

    // Linear search for comparison
    let mut linear_comparisons = 0;
    let linear_result = linear_search_with_count(&large_data, &target, &mut linear_comparisons);
    println!(
        "Linear search found {} at {:?} with {} comparisons",
        target, linear_result, linear_comparisons
    );

    println!(
        "\nSpeedup: {}x fewer comparisons!",
        linear_comparisons / comparisons
    );

    // Section 7: Common Mistakes
    println!("\n\n📖 Section 7: Common Mistakes to Avoid\n");

    println!("❌ Mistake 1: Searching unsorted data");
    let unsorted = [5, 1, 9, 3, 7];
    println!("   Unsorted: {:?}", unsorted);
    println!(
        "   Binary search result: {:?} (WRONG!)",
        basic_binary_search(&unsorted, &7)
    );

    let mut sorted = unsorted.clone();
    sorted.sort();
    println!("   After sorting: {:?}", sorted);
    println!(
        "   Binary search result: {:?} (Correct!)",
        basic_binary_search(&sorted, &7)
    );

    println!("\n❌ Mistake 2: Off-by-one errors (handled in implementation)");
    println!("   Our implementation correctly handles:");
    println!("   - Empty slices");
    println!("   - Single elements");
    println!("   - Elements at boundaries");

    // Self-Assessment
    println!("\n\n✅ Self-Assessment Checklist:");
    println!("   □ I understand why binary search is O(log n)");
    println!("   □ I can explain the Result<Ok, Err> return type");
    println!("   □ I know why data must be sorted");
    println!("   □ I can trace through the algorithm step-by-step");
    println!("   □ I understand the edge cases");
    println!("\nNext Step: Run `cargo run --example step2_trait_abstraction`");
}

/// Basic binary search implementation
///
/// Returns Ok(index) if found, Err(insert_position) if not found
fn basic_binary_search<T: Ord>(slice: &[T], target: &T) -> Result<usize, usize> {
    let mut left = 0;
    let mut right = slice.len();

    while left < right {
        // Calculate middle index (safe from overflow)
        let mid = left + (right - left) / 2;

        // Compare middle element with target
        let mid_value = &slice[mid];

        if mid_value == target {
            return Ok(mid); // Found!
        } else if mid_value < target {
            left = mid + 1; // Search right half
        } else {
            right = mid; // Search left half
        }
    }

    Err(left) // Not found, return insertion position
}

/// Binary search with step-by-step trace
fn binary_search_with_trace<T: Ord + std::fmt::Debug>(slice: &[T], target: &T) {
    let mut left = 0;
    let mut right = slice.len();
    let mut step = 1;

    println!(
        "Initial: left={}, right={} (search space: {:?})",
        left,
        right,
        &slice[left..right]
    );

    while left < right {
        let mid = left + (right - left) / 2;
        let mid_value = &slice[mid];

        println!("\nStep {}: mid={}, mid_value={:?}", step, mid, mid_value);

        if mid_value == target {
            println!("   ✅ FOUND at index {}!", mid);
            return;
        } else if mid_value < target {
            println!("   {:?} < {:?}, search RIGHT", mid_value, target);
            left = mid + 1;
        } else {
            println!("   {:?} > {:?}, search LEFT", mid_value, target);
            right = mid;
        }

        println!(
            "   New range: left={}, right={} (search space: {:?})",
            left,
            right,
            &slice[left..right.min(slice.len())]
        );
        step += 1;
    }

    println!("\n❌ Not found. Would insert at index {}", left);
}

/// Binary search counting comparisons
fn binary_search_with_count<T: Ord>(
    slice: &[T],
    target: &T,
    count: &mut usize,
) -> Result<usize, usize> {
    let mut left = 0;
    let mut right = slice.len();

    while left < right {
        *count += 1;
        let mid = left + (right - left) / 2;
        let mid_value = &slice[mid];

        if mid_value == target {
            return Ok(mid);
        } else if mid_value < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    Err(left)
}

/// Linear search for comparison
fn linear_search_with_count<T: Ord>(slice: &[T], target: &T, count: &mut usize) -> Option<usize> {
    for (i, item) in slice.iter().enumerate() {
        *count += 1;
        if item == target {
            return Some(i);
        }
    }
    None
}
