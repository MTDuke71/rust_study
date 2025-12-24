/// Day 25: Code Chronicle - Lock and Key Matching
/// 
/// Parse lock and key schematics and find compatible pairs

#[derive(Debug, Clone, PartialEq, Eq)]
struct Lock {
    heights: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Key {
    heights: Vec<usize>,
}

impl Lock {
    fn fits_with(&self, key: &Key, max_height: usize) -> bool {
        self.heights.iter()
            .zip(key.heights.iter())
            .all(|(lock_h, key_h)| lock_h + key_h <= max_height)
    }
}

pub fn solve_part1(input: &str) -> i64 {
    let (locks, keys) = parse_input(input);
    
    // Available space is 5 (0-5 inclusive in a 7-row schematic)
    let max_height = 5;
    
    let mut count = 0;
    for lock in &locks {
        for key in &keys {
            if lock.fits_with(key, max_height) {
                count += 1;
            }
        }
    }
    
    count
}

pub fn solve_part2(_input: &str) -> String {
    // Day 25 Part 2 is a free star - just finish all previous days
    "Merry Christmas!".to_string()
}

fn parse_input(input: &str) -> (Vec<Lock>, Vec<Key>) {
    // Normalize line endings and split on double newline
    let normalized = input.replace("\r\n", "\n");
    let schematics: Vec<&str> = normalized.split("\n\n").collect();
    
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    
    for schematic in schematics {
        let lines: Vec<&str> = schematic.lines().collect();
        if lines.is_empty() {
            continue;
        }
        
        // Check if it's a lock (top row all #) or key (bottom row all #)
        let is_lock = lines[0].chars().all(|c| c == '#');
        
        if is_lock {
            locks.push(parse_lock(lines));
        } else {
            keys.push(parse_key(lines));
        }
    }
    
    (locks, keys)
}

fn parse_lock(lines: Vec<&str>) -> Lock {
    let width = lines[0].len();
    let mut heights = vec![0; width];
    
    // For locks, count down from top until we hit empty space
    for col in 0..width {
        let mut height = 0;
        for row in 1..lines.len() {
            if lines[row].chars().nth(col) == Some('#') {
                height += 1;
            } else {
                break;
            }
        }
        heights[col] = height;
    }
    
    Lock { heights }
}

fn parse_key(lines: Vec<&str>) -> Key {
    let width = lines[0].len();
    let mut heights = vec![0; width];
    
    // For keys, count up from bottom until we hit empty space
    for col in 0..width {
        let mut height = 0;
        for row in (0..lines.len()-1).rev() {
            if lines[row].chars().nth(col) == Some('#') {
                height += 1;
            } else {
                break;
            }
        }
        heights[col] = height;
    }
    
    Key { heights }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn test_parse_lock() {
        let schematic = "#####
.####
.####
.####
.#.#.
.#...
.....";
        let lines: Vec<&str> = schematic.lines().collect();
        let lock = parse_lock(lines);
        assert_eq!(lock.heights, vec![0, 5, 3, 4, 3]);
    }

    #[test]
    fn test_parse_key() {
        let schematic = ".....
#....
#....
#...#
#.#.#
#.###
#####";
        let lines: Vec<&str> = schematic.lines().collect();
        let key = parse_key(lines);
        assert_eq!(key.heights, vec![5, 0, 2, 1, 3]);
    }

    #[test]
    fn test_lock_key_fit() {
        let lock = Lock { heights: vec![0, 5, 3, 4, 3] };
        let key1 = Key { heights: vec![5, 0, 2, 1, 3] };
        let key2 = Key { heights: vec![3, 0, 2, 0, 1] };
        
        // First key overlaps in last column (4+3 > 5)
        assert_eq!(lock.fits_with(&key1, 5), false);
        
        // Second key fits
        assert_eq!(lock.fits_with(&key2, 5), true);
    }

    #[test]
    fn test_example_part1() {
        let result = solve_part1(EXAMPLE_INPUT);
        assert_eq!(result, 3);
    }
}
