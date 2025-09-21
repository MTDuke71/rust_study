/// Advent of Code utilities for common search patterns
/// 
/// # Requirements Satisfied
/// - REQ-6: AoC-style problem support with coordinate search, range queries

use crate::binary_search::search_by_key;
use std::cmp::Ordering;

/// 2D coordinate type commonly used in AoC problems.
///
/// # Requirements Satisfied: REQ-6
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    
    /// Manhattan distance between two coordinates.
    pub fn manhattan_distance(self, other: Coord) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }
    
    /// Euclidean distance squared (avoids floating point).
    pub fn distance_squared(self, other: Coord) -> i32 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2)
    }
}

/// Event with timestamp, useful for AoC scheduling problems.
///
/// # Requirements Satisfied: REQ-6
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimedEvent<T>
where
    T: PartialEq + Eq,
{
    pub time: i64,
    pub data: T,
}

impl<T> TimedEvent<T>
where
    T: PartialEq + Eq,
{
    pub fn new(time: i64, data: T) -> Self {
        Self { time, data }
    }
}

impl<T> PartialOrd for TimedEvent<T>
where
    T: PartialEq + Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for TimedEvent<T>
where
    T: PartialEq + Eq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

/// Searches for coordinates within a Manhattan distance of a target.
///
/// # Requirements Satisfied: REQ-6
///
/// Assumes coordinates are sorted by x-coordinate first, then y-coordinate.
/// This is a common pattern in AoC grid problems.
pub fn find_coords_within_distance(
    coords: &[Coord],
    target: Coord,
    max_distance: u32,
) -> Vec<&Coord> {
    coords
        .iter()
        .filter(|&&coord| coord.manhattan_distance(target) <= max_distance)
        .collect()
}

/// Finds the closest coordinate to a target point.
///
/// # Requirements Satisfied: REQ-6
pub fn find_closest_coord(coords: &[Coord], target: Coord) -> Option<&Coord> {
    coords
        .iter()
        .min_by_key(|&&coord| coord.manhattan_distance(target))
}

/// Searches for events within a time window.
///
/// # Requirements Satisfied: REQ-6
///
/// Assumes events are sorted by timestamp.
pub fn find_events_in_window<T>(
    events: &[TimedEvent<T>],
    start_time: i64,
    end_time: i64,
) -> Vec<&TimedEvent<T>>
where
    T: PartialEq + Eq,
{
    let start_idx = match search_by_key(events, &start_time, |event| event.time) {
        Ok(idx) => idx,
        Err(idx) => idx,
    };
    
    let end_idx = match search_by_key(events, &end_time, |event| event.time) {
        Ok(idx) => idx + 1, // Include the exact match
        Err(idx) => idx,
    };
    
    events[start_idx..end_idx.min(events.len())].iter().collect()
}

/// Multi-criteria search for complex AoC objects.
///
/// # Requirements Satisfied: REQ-6
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AocItem {
    pub id: u32,
    pub value: i32,
    pub category: String,
}

impl AocItem {
    pub fn new(id: u32, value: i32, category: String) -> Self {
        Self { id, value, category }
    }
}

/// Searches AoC items by multiple criteria.
///
/// # Requirements Satisfied: REQ-4, REQ-6
pub fn search_items_multi_criteria<'a>(
    items: &'a [AocItem],
    target_value: i32,
    category: &str,
) -> Vec<&'a AocItem> {
    items
        .iter()
        .filter(|item| item.value == target_value && item.category == category)
        .collect()
}

/// Binary search for items sorted by value.
///
/// # Requirements Satisfied: REQ-6
pub fn find_item_by_value(items: &[AocItem], target_value: i32) -> Result<usize, usize> {
    search_by_key(items, &target_value, |item| item.value)
}

/// Utility for parsing and searching coordinate grids.
///
/// # Requirements Satisfied: REQ-6
pub struct Grid {
    coords: Vec<Coord>,
    width: i32,
    height: i32,
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Self {
        let mut coords = Vec::new();
        for y in 0..height {
            for x in 0..width {
                coords.push(Coord::new(x, y));
            }
        }
        Self { coords, width, height }
    }
    
    /// Find coordinate by linear index (row-major order).
    pub fn coord_at_index(&self, index: usize) -> Option<&Coord> {
        self.coords.get(index)
    }
    
    /// Find coordinates in a specific row.
    pub fn coords_in_row(&self, row: i32) -> Vec<&Coord> {
        self.coords
            .iter()
            .filter(|coord| coord.y == row)
            .collect()
    }
    
    /// Find coordinates in a specific column.
    pub fn coords_in_column(&self, col: i32) -> Vec<&Coord> {
        self.coords
            .iter()
            .filter(|coord| coord.x == col)
            .collect()
    }
    
    /// Check if coordinate is within grid bounds.
    pub fn contains(&self, coord: Coord) -> bool {
        coord.x >= 0 && coord.x < self.width && coord.y >= 0 && coord.y < self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coord_manhattan_distance() {
        let a = Coord::new(0, 0);
        let b = Coord::new(3, 4);
        assert_eq!(a.manhattan_distance(b), 7);
    }

    #[test]
    fn find_coords_within_distance_works() {
        let coords = vec![
            Coord::new(0, 0),  // distance 0
            Coord::new(1, 1),  // distance 2
            Coord::new(5, 5),  // distance 10
            Coord::new(2, 1),  // distance 3
        ];
        
        let target = Coord::new(0, 0);
        let nearby = find_coords_within_distance(&coords, target, 3);
        assert_eq!(nearby.len(), 3); // Should exclude (5,5)
    }

    #[test]
    fn timed_event_ordering() {
        let event1 = TimedEvent::new(100, "first");
        let event2 = TimedEvent::new(200, "second");
        assert!(event1 < event2);
    }

    #[test]
    fn find_events_in_window_works() {
        let events = vec![
            TimedEvent::new(10, "a"),
            TimedEvent::new(20, "b"),
            TimedEvent::new(30, "c"),
            TimedEvent::new(40, "d"),
        ];
        
        let window_events = find_events_in_window(&events, 15, 35);
        assert_eq!(window_events.len(), 2); // Should get "b" and "c"
    }

    #[test]
    fn grid_basic_operations() {
        let grid = Grid::new(3, 3);
        assert_eq!(grid.coords_in_row(0).len(), 3);
        assert_eq!(grid.coords_in_column(1).len(), 3);
        assert!(grid.contains(Coord::new(2, 2)));
        assert!(!grid.contains(Coord::new(3, 3)));
    }
}