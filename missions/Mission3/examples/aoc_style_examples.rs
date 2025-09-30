/// Advent of Code style examples using Mission3 binary search
/// 
/// These examples demonstrate real-world AoC problem patterns that benefit from binary search.

use mission3::*;

fn main() {
    println!("=== AoC-Style Binary Search Examples ===\n");
    
    // Example 1: 2D Grid coordinate searching
    coordinate_grid_search();
    
    // Example 2: Time-based event processing
    time_based_event_search();
    
    // Example 3: Range queries for puzzle solving
    range_query_puzzles();
    
    // Example 4: Multi-criteria item searching
    multi_criteria_item_search();
    
    // Example 5: Pathfinding support with coordinates
    pathfinding_coordinate_support();
    
    println!("All AoC examples completed! 🎄✅");
}

fn coordinate_grid_search() {
    println!("🗺️ 2D Grid Coordinate Search");
    println!("Pattern: AoC grid navigation and point location");
    
    use aoc_utils::{Coord, find_coords_within_distance, find_closest_coord, Grid};
    
    // Create a list of interesting points in a grid
    let points = vec![
        Coord::new(2, 3),   // beacon
        Coord::new(8, 7),   // sensor
        Coord::new(0, 11),  // exit
        Coord::new(13, 2),  // treasure
        Coord::new(10, 5),  // monster
        Coord::new(6, 9),   // item
    ];
    
    println!("Grid points: {:?}", points);
    
    // Find all points within range of a sensor at (8, 7)
    let sensor_pos = Coord::new(8, 7);
    let sensor_range = 5;
    
    let detected = find_coords_within_distance(&points, sensor_pos, sensor_range);
    println!("Sensor at {:?} detects {} points within range {}", 
             sensor_pos, detected.len(), sensor_range);
    
    for point in detected {
        let distance = sensor_pos.manhattan_distance(*point);
        println!("  {:?} at distance {}", point, distance);
    }
    
    // Find closest point to player position
    let player_pos = Coord::new(5, 5);
    if let Some(closest) = find_closest_coord(&points, player_pos) {
        let distance = player_pos.manhattan_distance(*closest);
        println!("Closest point to player at {:?} is {:?} (distance: {})", 
                 player_pos, closest, distance);
    }
    
    // Grid operations
    let grid = Grid::new(10, 8);
    let row_3_coords = grid.coords_in_row(3);
    println!("All coordinates in row 3: {} coords", row_3_coords.len());
    
    println!();
}

fn time_based_event_search() {
    println!("⏰ Time-Based Event Processing");
    println!("Pattern: AoC scheduling and temporal queries");
    
    use aoc_utils::{TimedEvent, find_events_in_window};
    
    // Simulate AoC problem with events happening over time
    let events = vec![
        TimedEvent::new(0, "Game starts"),
        TimedEvent::new(15, "First enemy spawn"),
        TimedEvent::new(23, "Player gets sword"),
        TimedEvent::new(45, "Boss battle begins"),
        TimedEvent::new(67, "Boss defeated"),
        TimedEvent::new(89, "Treasure found"),
        TimedEvent::new(120, "Level complete"),
        TimedEvent::new(134, "Next level starts"),
    ];
    
    println!("Game events timeline:");
    for event in &events {
        println!("  t={:3}: {}", event.time, event.data);
    }
    
    // Query: What happened during the main action (time 20-80)?
    let action_window = find_events_in_window(&events, 20, 80);
    println!("\nMain action window (t=20-80):");
    for event in action_window {
        println!("  t={:3}: {}", event.time, event.data);
    }
    
    // Query: Events in the final phase
    let endgame = find_events_in_window(&events, 100, 200);
    println!("\nEndgame events (t=100+):");
    for event in endgame {
        println!("  t={:3}: {}", event.time, event.data);
    }
    
    println!();
}

fn range_query_puzzles() {
    println!("📊 Range Query Puzzles");
    println!("Pattern: AoC problems requiring value range searches");
    
    use search_iter::{find_all_equal, SearchExt};
    
    // Simulate sorted puzzle data (e.g., heights, distances, scores)
    let heights = [1, 2, 3, 3, 3, 4, 5, 5, 6, 7, 8, 8, 8, 9, 10];
    println!("Terrain heights: {:?}", heights);
    
    // Find all positions with height 3 (plateaus)
    let plateaus: Vec<_> = find_all_equal(&heights, &3).collect();
    println!("Plateau positions (height=3): {} locations", plateaus.len());
    
    // Find moderate elevation range (4-6)
    let moderate_terrain: Vec<_> = heights.find_range(&4, &6).collect();
    println!("Moderate terrain (4-6): {:?}", moderate_terrain);
    
    // Find high terrain (8+)
    let high_terrain: Vec<_> = heights.find_range(&8, &10).collect();
    println!("High terrain (8-10): {:?}", high_terrain);
    
    // Count terrain types
    let low_count = heights.find_range(&1, &3).count();
    let mid_count = heights.find_range(&4, &6).count();
    let high_count = heights.find_range(&7, &10).count();
    
    println!("Terrain distribution: {} low, {} mid, {} high", 
             low_count, mid_count, high_count);
    
    // Simulate AoC: "Find all valid landing spots" (height 3-5)
    let landing_spots: Vec<_> = heights.find_range(&3, &5).collect();
    println!("Valid landing spots: {} available", landing_spots.len());
    
    println!();
}

fn multi_criteria_item_search() {
    println!("🎒 Multi-Criteria Item Search");
    println!("Pattern: AoC inventory and resource management");
    
    use aoc_utils::{AocItem, search_items_multi_criteria, find_item_by_value};
    
    // Simulate AoC items with multiple properties
    let mut items = vec![
        AocItem::new(1, 100, "weapon".to_string()),
        AocItem::new(2, 50, "potion".to_string()),
        AocItem::new(3, 200, "armor".to_string()),
        AocItem::new(4, 100, "weapon".to_string()),
        AocItem::new(5, 75, "potion".to_string()),
        AocItem::new(6, 150, "armor".to_string()),
        AocItem::new(7, 25, "scroll".to_string()),
        AocItem::new(8, 300, "weapon".to_string()),
    ];
    
    println!("Available items:");
    for item in &items {
        println!("  #{}: {} value, {}", item.id, item.value, item.category);
    }
    
    // Find all weapons worth 100 gold
    let good_weapons = search_items_multi_criteria(&items, 100, "weapon");
    println!("\nWeapons worth 100 gold:");
    for weapon in good_weapons {
        println!("  #{}: {}", weapon.id, weapon.category);
    }
    
    // Find all potions
    let potions = search_items_multi_criteria(&items, 50, "potion");
    println!("\nPotions worth 50 gold:");
    for potion in potions {
        println!("  #{}: {}", potion.id, potion.category);
    }
    
    // Sort by value for binary search
    items.sort_by_key(|item| item.value);
    println!("\nItems sorted by value:");
    for item in &items {
        println!("  #{}: {} value, {}", item.id, item.value, item.category);
    }
    
    // Binary search by value
    match find_item_by_value(&items, 150) {
        Ok(idx) => println!("Found item worth 150: #{} ({})", 
                           items[idx].id, items[idx].category),
        Err(idx) => println!("No item worth exactly 150, would insert at position {}", idx),
    }
    
    println!();
}

fn pathfinding_coordinate_support() {
    println!("🎯 Pathfinding Coordinate Support");
    println!("Pattern: AoC pathfinding and spatial queries");
    
    use aoc_utils::{Coord, find_coords_within_distance};
    
    // Simulate a maze or grid with key locations
    let waypoints = vec![
        Coord::new(0, 0),    // start
        Coord::new(3, 2),    // checkpoint 1
        Coord::new(7, 1),    // checkpoint 2
        Coord::new(5, 5),    // checkpoint 3
        Coord::new(10, 8),   // goal
        Coord::new(2, 4),    // bonus item
        Coord::new(8, 6),    // danger zone
        Coord::new(1, 7),    // safe house
    ];
    
    println!("Waypoints in the maze:");
    for (i, wp) in waypoints.iter().enumerate() {
        println!("  [{}]: {:?}", i, wp);
    }
    
    // Find waypoints reachable within movement range from start
    let start = Coord::new(0, 0);
    let movement_range = 4;
    
    let reachable = find_coords_within_distance(&waypoints, start, movement_range);
    println!("\nFrom start {:?}, reachable within {} moves:", start, movement_range);
    for wp in reachable {
        let distance = start.manhattan_distance(*wp);
        println!("  {:?} (distance: {})", wp, distance);
    }
    
    // Check visibility/interaction range around a checkpoint
    let checkpoint = Coord::new(5, 5);
    let interaction_range = 3;
    
    let nearby = find_coords_within_distance(&waypoints, checkpoint, interaction_range);
    println!("\nNear checkpoint {:?} (range {}):", checkpoint, interaction_range);
    for wp in nearby {
        let distance = checkpoint.manhattan_distance(*wp);
        println!("  {:?} (distance: {})", wp, distance);
    }
    
    // Calculate distances for pathfinding
    println!("\nDistance matrix (Manhattan):");
    println!("From start to each waypoint:");
    for (i, wp) in waypoints.iter().enumerate() {
        let dist = start.manhattan_distance(*wp);
        println!("  to [{}] {:?}: {}", i, wp, dist);
    }
    
    println!();
}