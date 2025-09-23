use mission5::{Dictionary, Counter, SetOperations, MemoCache};

#[test] // REQ-1: Enhanced dictionary wrapper around HashMap<K, V>
fn req1_dictionary_enhanced_operations() {
    let mut dict = Dictionary::new();
    
    // Bulk insertion
    dict.insert_many([("rust", "systems"), ("python", "scripting"), ("javascript", "web")]);
    assert_eq!(dict.len(), 3);
    
    // Default value handling
    assert_eq!(dict.get_or_default(&"rust", "unknown"), "systems");
    assert_eq!(dict.get_or_default(&"go", "systems"), "systems");
    
    // Iteration and querying
    let languages: Vec<_> = dict.keys().collect();
    assert_eq!(languages.len(), 3);
    assert!(languages.contains(&&"rust"));
    assert!(languages.contains(&&"python"));
    assert!(languages.contains(&&"javascript"));
}

#[test] // REQ-1: Dictionary coordinate mapping for AoC
fn req1_coordinate_mapping_aoc_style() {
    let mut grid = Dictionary::new();
    
    // Build a simple maze
    let maze = [
        "###",
        "#.#",
        "###",
    ];
    
    for (y, row) in maze.iter().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            grid.insert((x as i32, y as i32), cell);
        }
    }
    
    // Query specific positions
    assert_eq!(grid.get(&(0, 0)), Some(&'#')); // Wall
    assert_eq!(grid.get(&(1, 1)), Some(&'.')); // Empty
    assert_eq!(grid.get(&(3, 3)), None);       // Out of bounds
    
    // Find all empty spaces
    let empty_spaces: Vec<_> = grid.iter()
        .filter(|(_, &cell)| cell == '.')
        .map(|(pos, _)| *pos)
        .collect();
    
    assert_eq!(empty_spaces.len(), 1);
    assert_eq!(empty_spaces[0], (1, 1));
}

#[test] // REQ-2: Set operations for membership testing
fn req2_set_operations_comprehensive() {
    let evens = SetOperations::from_iter((0..10).filter(|x| x % 2 == 0));
    let primes = SetOperations::from_iter([2, 3, 5, 7]);
    let squares = SetOperations::from_iter([0, 1, 4, 9]);
    
    // Basic set operations
    let even_primes = evens.intersection(&primes);
    assert_eq!(even_primes.len(), 1);
    assert!(even_primes.contains(&2));
    
    let even_or_prime = evens.union(&primes);
    assert!(even_or_prime.contains(&2)); // Both even and prime
    assert!(even_or_prime.contains(&4)); // Even only
    assert!(even_or_prime.contains(&3)); // Prime only
    
    // Set relationships
    let small_evens = SetOperations::from_iter([0, 2, 4]);
    assert!(small_evens.is_subset(&evens));
    assert!(evens.is_superset(&small_evens));
    
    assert!(!evens.is_disjoint(&squares)); // Share 0, 4
    assert!(primes.is_disjoint(&SetOperations::from_iter([10, 12, 14])));
}

#[test] // REQ-2: Set deduplication for AoC state tracking
fn req2_state_deduplication() {
    let mut visited_states = SetOperations::new();
    
    // Simulate game states as (x, y, direction) tuples
    type GameState = (i32, i32, char);
    
    let states: Vec<GameState> = vec![
        (0, 0, 'N'), (0, 1, 'N'), (0, 1, 'E'), (1, 1, 'E'),
        (0, 0, 'N'), // Duplicate - should be ignored
        (1, 1, 'S'), (0, 1, 'N'), // Another duplicate
    ];
    
    for state in states {
        visited_states.insert(state);
    }
    
    // Should have 5 unique states (2 duplicates removed)
    assert_eq!(visited_states.len(), 5);
    assert!(visited_states.contains(&(0, 0, 'N')));
    assert!(visited_states.contains(&(1, 1, 'S')));
}

#[test] // REQ-3: Frequency counting and analysis
fn req3_frequency_analysis_comprehensive() {
    let mut letter_counter = Counter::new();
    
    // Count letters in multiple words
    let words = ["hello", "world", "rust", "programming"];
    for word in words {
        letter_counter.count_chars(word);
    }
    
    // Verify specific counts
    assert_eq!(letter_counter.get(&"r".to_string()), 4); // "world", "rust", "programming" x2
    assert_eq!(letter_counter.get(&"o".to_string()), 3); // "hello", "world", "programming"
    assert_eq!(letter_counter.get(&"l".to_string()), 3); // "hello" x2, "world"
    
    // Statistical analysis
    assert_eq!(letter_counter.total_count(), 25); // Total characters (5+5+4+11)
    assert!(letter_counter.unique_count() >= 10); // At least 10 unique letters
    
    let most_common = letter_counter.most_common(3);
    assert_eq!(most_common[0].1, 4); // 'r' appears most frequently
}

#[test] // REQ-3: Word frequency for text analysis
fn req3_word_frequency_analysis() {
    let mut word_counter = Counter::new();
    
    let text = "the quick brown fox jumps over the lazy dog the fox is very quick";
    word_counter.count_words(text);
    
    // Check specific word counts
    assert_eq!(word_counter.get(&"the".to_string()), 3);
    assert_eq!(word_counter.get(&"quick".to_string()), 2);
    assert_eq!(word_counter.get(&"fox".to_string()), 2);
    assert_eq!(word_counter.get(&"is".to_string()), 1);
    assert_eq!(word_counter.get(&"missing".to_string()), 0);
    
    // Most and least common
    let most_common = word_counter.most_common(1);
    assert_eq!(most_common[0], ("the".to_string(), 3));
    
    let least_common = word_counter.least_common(2);
    // Should have several words with count 1
    assert!(least_common.iter().any(|(_, count)| *count == 1));
}

#[test] // REQ-4: Multi-value dictionaries for grouping
fn req4_multi_value_dictionary_operations() {
    let mut groups: Dictionary<&str, Vec<String>> = Dictionary::new();
    
    // Build category groupings
    groups.add_to_list("fruits", "apple");
    groups.add_to_list("fruits", "banana");
    groups.add_to_list("fruits", "cherry");
    
    groups.add_to_list("vegetables", "carrot");
    groups.add_to_list("vegetables", "spinach");
    
    groups.add_to_list("grains", "rice");
    groups.add_to_list("grains", "wheat");
    groups.add_to_list("grains", "oats");
    
    // Verify groupings
    let fruits = groups.get_list(&"fruits");
    assert_eq!(fruits.len(), 3);
    assert!(fruits.contains(&"apple".to_string()));
    assert!(fruits.contains(&"banana".to_string()));
    assert!(fruits.contains(&"cherry".to_string()));
    
    let vegetables = groups.get_list(&"vegetables");
    assert_eq!(vegetables.len(), 2);
    
    let grains = groups.get_list(&"grains");
    assert_eq!(grains.len(), 3);
    
    // Non-existent category
    let dairy = groups.get_list(&"dairy");
    assert_eq!(dairy.len(), 0);
    
    // List length checking
    assert_eq!(groups.list_len(&"fruits"), 3);
    assert_eq!(groups.list_len(&"missing"), 0);
}

#[test] // REQ-4: Graph adjacency lists using multi-value dictionary
fn req4_graph_adjacency_representation() {
    let mut graph: Dictionary<&str, Vec<String>> = Dictionary::new();
    
    // Build a simple directed graph
    // A -> B, C
    // B -> C, D  
    // C -> D
    // D -> (no outgoing edges)
    
    graph.add_to_list("A", "B");
    graph.add_to_list("A", "C");
    graph.add_to_list("B", "C");
    graph.add_to_list("B", "D");
    graph.add_to_list("C", "D");
    
    // Verify adjacency lists
    let a_neighbors = graph.get_list(&"A");
    assert_eq!(a_neighbors.len(), 2);
    assert!(a_neighbors.contains(&"B".to_string()));
    assert!(a_neighbors.contains(&"C".to_string()));
    
    let b_neighbors = graph.get_list(&"B");
    assert_eq!(b_neighbors.len(), 2);
    assert!(b_neighbors.contains(&"C".to_string()));
    assert!(b_neighbors.contains(&"D".to_string()));
    
    let d_neighbors = graph.get_list(&"D");
    assert_eq!(d_neighbors.len(), 0); // No outgoing edges
    
    // Count total edges
    let total_edges: usize = ["A", "B", "C", "D"].iter()
        .map(|node| graph.list_len(&node))
        .sum();
    assert_eq!(total_edges, 5);
}

#[test] // REQ-5: Memoization cache for dynamic programming
fn req5_memoization_cache_operations() {
    let mut cache = MemoCache::new();
    
    // Test basic caching
    cache.insert("expensive_computation_1", 42);
    cache.insert("expensive_computation_2", 84);
    
    // Test cache hits and misses
    assert_eq!(cache.get(&"expensive_computation_1"), Some(&42)); // Hit
    assert_eq!(cache.get(&"missing_key"), None);                  // Miss
    assert_eq!(cache.get(&"expensive_computation_2"), Some(&84)); // Hit
    
    // Verify statistics
    assert_eq!(cache.hit_count(), 2);
    assert_eq!(cache.miss_count(), 1);
    assert_eq!(cache.hit_ratio(), 2.0 / 3.0);
    
    // Test peek (doesn't affect statistics)
    assert_eq!(cache.peek(&"expensive_computation_1"), Some(&42));
    assert_eq!(cache.hit_count(), 2); // Should not change
}

#[test] // REQ-5: Get-or-compute pattern for memoization
fn req5_get_or_compute_memoization() {
    let mut cache = MemoCache::new();
    let mut computation_count = 0;
    
    // First computation should call the function
    let result1 = cache.get_or_compute(5, |&n: &i32| -> i32 {
        computation_count += 1;
        n * n
    });
    assert_eq!(result1, 25);
    assert_eq!(computation_count, 1);
    
    // Second computation should use cache
    let result2 = cache.get_or_compute(5, |&n: &i32| -> i32 {
        computation_count += 1;
        n * n
    });
    assert_eq!(result2, 25);
    assert_eq!(computation_count, 1); // Function not called again
    
    // Different input should call function
    let result3 = cache.get_or_compute(7, |&n: &i32| -> i32 {
        computation_count += 1;
        n * n
    });
    assert_eq!(result3, 49);
    assert_eq!(computation_count, 2);
    
    // Verify cache state
    assert_eq!(cache.len(), 2);
    assert_eq!(cache.hit_count(), 1);  // One cache hit (second call to 5)
    assert_eq!(cache.miss_count(), 2); // Two cache misses (first calls)
}

#[test] // REQ-5: Integer memoization for DP algorithms
fn req5_integer_memoization_patterns() {
    let mut fib_cache = MemoCache::new();
    
    // Set up base cases for fibonacci
    fib_cache.insert(0, 0);
    fib_cache.insert(1, 1);
    
    // Compute fibonacci numbers using get_or_compute_int
    for n in 2..=10 {
        let fib_n = fib_cache.get_or_compute_int(n, |&num| {
            // Simplified fibonacci computation for testing
            // In real implementation, this would be recursive with cache
            match num {
                2 => 1, 3 => 2, 4 => 3, 5 => 5, 6 => 8,
                7 => 13, 8 => 21, 9 => 34, 10 => 55,
                _ => 0,
            }
        });
        
        // Verify known values
        match n {
            5 => assert_eq!(fib_n, 5),
            8 => assert_eq!(fib_n, 21),
            10 => assert_eq!(fib_n, 55),
            _ => {}
        }
    }
    
    assert_eq!(fib_cache.len(), 11); // 0 through 10
    
    // Test get_or_default
    let default_val = fib_cache.get_or_default(100, -1);
    assert_eq!(default_val, -1);
    assert!(fib_cache.contains_key(&100));
}

#[test] // REQ-6: AoC coordinate and state management
fn req6_aoc_coordinate_utilities() {
    let mut world_map = Dictionary::new();
    let mut visited_coords = SetOperations::new();
    let mut position_counter = Counter::new();
    
    // Simulate movement tracking in a 2D world
    let movements = [
        (0, 0), (1, 0), (1, 1), (0, 1), (0, 0), // Square path
        (1, 0), (2, 0), (2, 1), (1, 1),         // Extended path with revisit
    ];
    
    // Track world state and movement
    for (x, y) in movements {
        // Mark position as visited in map
        world_map.insert((x, y), '.');
        
        // Add to visited set (deduplication)
        visited_coords.insert((x, y));
        
        // Count position visits
        position_counter.increment(format!("({},{})", x, y));
    }
    
    // Verify unique positions
    assert_eq!(visited_coords.len(), 6); // Unique positions
    assert!(visited_coords.contains(&(0, 0)));
    assert!(visited_coords.contains(&(2, 1)));
    
    // Verify world map coverage
    assert_eq!(world_map.len(), 6); // Same as unique positions
    assert_eq!(world_map.get(&(1, 1)), Some(&'.'));
    
    // Verify visit frequency
    assert_eq!(position_counter.get(&"(0,0)".to_string()), 2); // Start and return
    assert_eq!(position_counter.get(&"(1,1)".to_string()), 2); // Visited twice
    assert_eq!(position_counter.get(&"(2,0)".to_string()), 1); // Visited once
    assert_eq!(position_counter.total_count(), 9); // Total movements
    
    // Find most visited positions
    let most_visited = position_counter.most_common(2);
    assert_eq!(most_visited.len(), 2);
    assert_eq!(most_visited[0].1, 2); // Two positions tied at 2 visits
}

#[test] // REQ-6: AoC-style graph representation and analysis
fn req6_aoc_graph_analysis() {
    let mut adjacency: Dictionary<i32, Vec<String>> = Dictionary::new();
    let mut node_values = Dictionary::new();
    let mut path_cache = MemoCache::new();
    
    // Build a small directed graph with values
    // Node structure: id -> (value, [neighbors])
    // 1(val=10) -> [2, 3]
    // 2(val=20) -> [4]  
    // 3(val=15) -> [4]
    // 4(val=25) -> []
    
    adjacency.add_to_list(1, "2");
    adjacency.add_to_list(1, "3");
    adjacency.add_to_list(2, "4");
    adjacency.add_to_list(3, "4");
    
    node_values.insert(1, 10);
    node_values.insert(2, 20);
    node_values.insert(3, 15);
    node_values.insert(4, 25);
    
    // Analyze graph properties
    let all_nodes = SetOperations::from_iter([1, 2, 3, 4]);
    
    // Find leaf nodes (no outgoing edges)
    let leaf_nodes: Vec<_> = all_nodes.iter()
        .filter(|&node| adjacency.list_len(node) == 0)
        .collect();
    assert_eq!(leaf_nodes, vec![&4]);
    
    // Find root nodes (no incoming edges) by checking all adjacency lists
    let mut has_incoming = SetOperations::new();
    for neighbors in adjacency.values() {
        for neighbor in neighbors {
            has_incoming.insert(neighbor.parse::<i32>().unwrap());
        }
    }
    
    let root_nodes = all_nodes.difference(&has_incoming);
    assert_eq!(root_nodes.len(), 1);
    assert!(root_nodes.contains(&1));
    
    // Cache path computations (simplified)
    path_cache.insert("path_1_to_4_via_2", 2); // 2 hops
    path_cache.insert("path_1_to_4_via_3", 2); // 2 hops
    
    // Verify cached paths
    assert_eq!(path_cache.get(&"path_1_to_4_via_2"), Some(&2));
    assert_eq!(path_cache.get(&"path_1_to_4_via_3"), Some(&2));
    
    // Calculate total node values
    let total_value: i32 = node_values.values().sum();
    assert_eq!(total_value, 70); // 10 + 20 + 15 + 25
}

#[test] // Integration test: All components working together
fn integration_test_aoc_scenario() {
    // Simulate an AoC-style problem using all data structures
    
    // Grid for world representation
    let mut world = Dictionary::new();
    
    // Set for tracking visited positions
    let mut visited = SetOperations::new();
    
    // Counter for collecting items
    let mut inventory = Counter::new();
    
    // Cache for expensive path calculations
    let mut path_cache = MemoCache::new();
    
    // Setup: Create a simple world with items
    let world_data = [
        ((0, 0), ('S', "start")),    // Start position
        ((1, 0), ('.', "empty")),    // Empty space
        ((2, 0), ('$', "coin")),     // Coin
        ((0, 1), ('.', "empty")),    // Empty space
        ((1, 1), ('*', "gem")),      // Gem
        ((2, 1), ('E', "end")),      // End position
    ];
    
    for ((x, y), (cell, item)) in world_data {
        world.insert((x, y), cell);
        if item != "empty" && item != "start" && item != "end" {
            inventory.increment(item.to_string());
            inventory.set(item.to_string(), 0); // Reset to 0, we'll collect later
        }
    }
    
    // Simulate movement and collection
    let path = [(0, 0), (1, 0), (2, 0), (2, 1), (1, 1)]; // S -> . -> $ -> E -> *
    
    for (x, y) in path {
        visited.insert((x, y));
        
        // Collect items based on world state
        match world.get(&(x, y)) {
            Some('$') => inventory.increment("coin".to_string()),
            Some('*') => inventory.increment("gem".to_string()),
            _ => {}
        }
        
        // Cache "distance to end" for this position
        let distance_to_end = (2i32 - x).abs() + (1i32 - y).abs();
        path_cache.insert((x, y), distance_to_end);
    }
    
    // Verify final state
    assert_eq!(visited.len(), 5); // Visited 5 unique positions
    assert_eq!(inventory.get(&"coin".to_string()), 1);
    assert_eq!(inventory.get(&"gem".to_string()), 1);
    assert_eq!(inventory.total_count(), 2);
    
    // Check cached distances
    assert_eq!(path_cache.get(&(0, 0)), Some(&3)); // Start is 3 steps from end
    assert_eq!(path_cache.get(&(2, 1)), Some(&0)); // End is 0 steps from end
    
    // Find shortest cached distance to verify we reached the end
    let min_distance = path_cache.values().min().unwrap();
    assert_eq!(*min_distance, 0);
}