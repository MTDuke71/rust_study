use mission3::*;

#[test] // REQ-1: Slice-based binary search
fn req1_slice_basic_search() {
    let data = [1, 3, 5, 7, 9, 11, 13];

    // Found cases
    assert_eq!(binary_search::search_slice(&data, &1), Ok(0));
    assert_eq!(binary_search::search_slice(&data, &7), Ok(3));
    assert_eq!(binary_search::search_slice(&data, &13), Ok(6));

    // Not found cases - should return insertion index
    assert_eq!(binary_search::search_slice(&data, &0), Err(0));
    assert_eq!(binary_search::search_slice(&data, &4), Err(2));
    assert_eq!(binary_search::search_slice(&data, &14), Err(7));
}

#[test] // REQ-1: Edge cases for slice search
fn req1_slice_edge_cases() {
    // Empty slice
    let empty: &[i32] = &[];
    assert_eq!(binary_search::search_slice(empty, &5), Err(0));

    // Single element
    let single = [42];
    assert_eq!(binary_search::search_slice(&single, &42), Ok(0));
    assert_eq!(binary_search::search_slice(&single, &41), Err(0));
    assert_eq!(binary_search::search_slice(&single, &43), Err(1));

    // Two elements
    let two = [10, 20];
    assert_eq!(binary_search::search_slice(&two, &10), Ok(0));
    assert_eq!(binary_search::search_slice(&two, &20), Ok(1));
    assert_eq!(binary_search::search_slice(&two, &15), Err(1));
}

#[test] // REQ-1: Generic type support
fn req1_generic_types() {
    // String slices
    let words = ["apple", "banana", "cherry", "date"];
    assert_eq!(binary_search::search_slice(&words, &"banana"), Ok(1));
    assert_eq!(binary_search::search_slice(&words, &"coconut"), Err(3));

    // Character slices
    let chars = ['a', 'c', 'e', 'g'];
    assert_eq!(binary_search::search_slice(&chars, &'c'), Ok(1));
    assert_eq!(binary_search::search_slice(&chars, &'d'), Err(2));
}

#[test] // REQ-2: Trait-based search on different containers
fn req2_trait_based_search() {
    use searchable::search_in;

    // Test with slice
    let slice_data = [1, 3, 5, 7, 9];
    assert_eq!(search_in(&slice_data, &5), Ok(2));
    assert_eq!(search_in(&slice_data, &6), Err(3));

    // Test with Vec
    let vec_data = vec![1, 3, 5, 7, 9];
    assert_eq!(search_in(&vec_data, &5), Ok(2));
    assert_eq!(search_in(&vec_data, &6), Err(3));

    // Verify trait methods work
    assert_eq!(slice_data.len(), 5);
    assert_eq!(slice_data.get(2), Some(&5));
    assert!(!slice_data.is_empty());
}

#[test] // REQ-2: Custom searchable container
fn req2_custom_container() {
    use searchable::{search_in_by, search_in_by_key};

    let people = vec![(1, "Alice"), (3, "Bob"), (5, "Charlie"), (7, "Diana")];

    // Search by key (ID)
    assert_eq!(search_in_by_key(&people, &3, |p| p.0), Ok(1));
    assert_eq!(search_in_by_key(&people, &4, |p| p.0), Err(2));

    // Search by name with custom comparator
    assert_eq!(
        search_in_by(&people, &(0, "Bob"), |person, target| person
            .1
            .cmp(target.1)),
        Ok(1)
    );
}

#[test] // REQ-3: Iterator integration
fn req3_iterator_integration() {
    use search_iter::{find_all_equal, find_range, SearchExt};

    let data = [1, 2, 2, 2, 3, 4, 5, 5, 6];

    // Find all equal elements
    let twos: Vec<_> = find_all_equal(&data, &2).collect();
    assert_eq!(twos, [&2, &2, &2]);

    // Find range
    let range: Vec<_> = find_range(&data, &3, &5).collect();
    assert_eq!(range, [&3, &4, &5, &5]);

    // Use extension trait
    let fives: Vec<_> = data.find_all_equal(&5).collect();
    assert_eq!(fives, [&5, &5]);
}

#[test] // REQ-3: Iterator size hints and exact size
fn req3_iterator_properties() {
    use search_iter::find_all_equal;

    let data = [1, 2, 2, 2, 3];
    let iter = find_all_equal(&data, &2);

    // Should implement ExactSizeIterator
    assert_eq!(iter.len(), 3);
    assert_eq!(iter.size_hint(), (3, Some(3)));
}

#[test] // REQ-4: Custom ordering support
fn req4_custom_ordering() {
    // Search with custom key extraction
    let items = [(10, "low"), (20, "medium"), (30, "high")];
    assert_eq!(
        binary_search::search_by_key(&items, &20, |item| item.0),
        Ok(1)
    );

    // Search with reverse ordering
    let desc_data = [9, 7, 5, 3, 1];
    assert_eq!(
        binary_search::search_by(&desc_data, &5, |a, b| b.cmp(a)),
        Ok(2)
    );
}

#[test] // REQ-4: Complex custom ordering
fn req4_complex_ordering() {
    #[derive(Debug, PartialEq)]
    struct Person {
        age: u32,
        name: String,
    }

    let people = vec![
        Person {
            age: 25,
            name: "Alice".to_string(),
        },
        Person {
            age: 30,
            name: "Bob".to_string(),
        },
        Person {
            age: 35,
            name: "Charlie".to_string(),
        },
    ];

    // Search by age
    assert_eq!(binary_search::search_by_key(&people, &30, |p| p.age), Ok(1));

    // Search by name length
    assert_eq!(
        binary_search::search_by_key(&people, &3, |p| p.name.len()),
        Ok(1) // "Bob" has 3 characters
    );
}

#[test] // REQ-5: Lifetime-safe borrowing
fn req5_lifetime_safety() {
    let data = vec![1, 3, 5, 7, 9];

    // Test that we can safely return references
    let found_ref = {
        let result = binary_search::search_slice(&data, &5);
        match result {
            Ok(idx) => Some(&data[idx]),
            Err(_) => None,
        }
    };

    assert_eq!(found_ref, Some(&5));

    // Test with iterator
    let range_refs: Vec<_> = {
        use search_iter::find_range;
        find_range(&data, &3, &7).collect()
    };

    assert_eq!(range_refs, [&3, &5, &7]);
}

#[test] // REQ-5: Slice lifetime annotations work correctly
fn req5_slice_lifetimes() {
    fn search_and_return(slice: &[i32], target: i32) -> Option<&i32> {
        match binary_search::search_slice(slice, &target) {
            Ok(idx) => slice.get(idx),
            Err(_) => None,
        }
    }

    let data = [1, 3, 5, 7, 9];
    let result = search_and_return(&data, 5);
    assert_eq!(result, Some(&5));
}

#[test] // REQ-6: AoC-style coordinate search
fn req6_coordinate_search() {
    use aoc_utils::{find_closest_coord, find_coords_within_distance, Coord};

    let coords = vec![
        Coord::new(0, 0),
        Coord::new(1, 1),
        Coord::new(3, 4),
        Coord::new(10, 10),
    ];

    let target = Coord::new(0, 0);

    // Find coordinates within Manhattan distance
    let nearby = find_coords_within_distance(&coords, target, 3);
    assert_eq!(nearby.len(), 2); // (0,0) and (1,1)

    // Find closest coordinate
    let closest = find_closest_coord(&coords, Coord::new(2, 2));
    assert_eq!(closest, Some(&Coord::new(1, 1)));
}

#[test] // REQ-6: AoC-style time-based search
fn req6_time_based_search() {
    use aoc_utils::{find_events_in_window, TimedEvent};

    let events = vec![
        TimedEvent::new(10, "event1"),
        TimedEvent::new(20, "event2"),
        TimedEvent::new(30, "event3"),
        TimedEvent::new(40, "event4"),
        TimedEvent::new(50, "event5"),
    ];

    let window_events = find_events_in_window(&events, 15, 35);
    assert_eq!(window_events.len(), 2);
    assert_eq!(window_events[0].data, "event2");
    assert_eq!(window_events[1].data, "event3");
}

#[test] // REQ-6: AoC-style multi-criteria search
fn req6_multi_criteria_search() {
    use aoc_utils::{find_item_by_value, search_items_multi_criteria, AocItem};

    let items = vec![
        AocItem::new(1, 100, "type_a".to_string()),
        AocItem::new(2, 200, "type_b".to_string()),
        AocItem::new(3, 200, "type_a".to_string()),
        AocItem::new(4, 300, "type_b".to_string()),
    ];

    // Multi-criteria search
    let matches = search_items_multi_criteria(&items, 200, "type_a");
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].id, 3);

    // Value-based binary search (assumes sorted by value)
    let sorted_items = vec![
        AocItem::new(1, 100, "type_a".to_string()),
        AocItem::new(2, 200, "type_b".to_string()),
        AocItem::new(4, 300, "type_b".to_string()),
    ];

    assert_eq!(find_item_by_value(&sorted_items, 200), Ok(1));
}

// Property-based tests with random data
#[test] // Cross-verification with std library
fn property_test_against_std_library() {
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();

    for _ in 0..100 {
        let mut data: Vec<i32> = (0..50).map(|_| rng.gen_range(-100..100)).collect();
        data.sort();

        let target = rng.gen_range(-100..100);

        let std_result = data.binary_search(&target);
        let our_result = binary_search::search_slice(&data, &target);

        match (std_result, our_result) {
            (Ok(std_idx), Ok(our_idx)) => {
                // Both found - values should be equal (index might differ for duplicates)
                assert_eq!(data[std_idx], data[our_idx]);
            }
            (Err(std_idx), Err(our_idx)) => {
                // Both not found - insertion indices should match
                assert_eq!(std_idx, our_idx);
            }
            _ => panic!(
                "Results should match: std={:?}, ours={:?}",
                std_result, our_result
            ),
        }
    }
}
