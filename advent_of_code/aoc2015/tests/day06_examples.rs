use aoc2015::solver::day06;

// Helper function for manual testing during development
#[allow(dead_code)]
#[allow(clippy::needless_range_loop)]
fn print_grid_state(grid: &[[bool; 1000]; 1000], x1: usize, y1: usize, x2: usize, y2: usize) {
    println!("Grid state in region ({},{}) to ({},{}):", x1, y1, x2, y2);
    for (_y, row) in grid
        .iter()
        .enumerate()
        .skip(y1)
        .take(y2.min(y1 + 10) - y1 + 1)
    {
        // Limit output for readability
        for x in x1..=x2.min(x1 + 10) {
            print!("{}", if row[x] { '█' } else { '░' });
        }
        println!();
    }
}

#[cfg(test)]
mod day06_tests {
    use super::*;

    // ====== TDD TESTS FOR DAY 6: PROBABLY A FIRE HAZARD ======
    // Following TDD: Write tests first, then implement to make them pass

    #[test]
    fn test_single_light_turn_on() {
        // RED: This will fail initially - we need to implement the grid and parsing
        let input = "turn on 0,0 through 0,0";
        let result = day06::solve_part1(input).unwrap();
        assert_eq!(result, "1"); // One light should be on
    }

    #[test]
    fn test_single_light_turn_off() {
        // When we turn off a light that's already off, it should stay off
        let input = "turn off 0,0 through 0,0";
        let result = day06::solve_part1(input).unwrap();
        assert_eq!(result, "0"); // No lights should be on
    }

    #[test]
    fn test_single_light_toggle_from_off() {
        // Toggle should turn off light to on
        let input = "toggle 0,0 through 0,0";
        let result = day06::solve_part1(input).unwrap();
        assert_eq!(result, "1"); // One light should be on
    }

    #[test]
    fn test_rectangular_region_turn_on() {
        // Turn on a 2x2 region (4 lights total)
        let input = "turn on 0,0 through 1,1";
        let result = day06::solve_part1(input).unwrap();
        assert_eq!(result, "4"); // Four lights should be on
    }

    #[test]
    fn test_line_of_lights() {
        // Turn on first row of lights (1000 lights)
        let input = "turn on 0,0 through 999,0";
        let result = day06::solve_part1(input).unwrap();
        assert_eq!(result, "1000"); // 1000 lights in first row
    }

    #[test]
    fn test_all_lights() {
        // Turn on all lights in the grid
        let input = "turn on 0,0 through 999,999";
        let result = day06::solve_part1(input).unwrap();
        assert_eq!(result, "1000000"); // All million lights
    }

    #[test]
    fn test_sequential_operations() {
        // Multiple operations in sequence
        let input = "turn on 0,0 through 2,2\nturn off 1,1 through 1,1";
        let result = day06::solve_part1(input).unwrap();
        assert_eq!(result, "8"); // 9 lights on, then turn off center = 8
    }

    #[test]
    fn test_toggle_operations() {
        // Test toggle behavior
        let input = "turn on 0,0 through 1,1\ntoggle 0,0 through 1,0";
        let result = day06::solve_part1(input).unwrap();
        // Start: 4 lights on in 2x2
        // Toggle top row: turns off (0,0) and (1,0)
        // Result: only (0,1) and (1,1) remain on = 2 lights
        assert_eq!(result, "2");
    }

    #[test]
    fn test_overlapping_regions() {
        // Test overlapping turn on operations
        let input = "turn on 0,0 through 1,1\nturn on 1,1 through 2,2";
        let result = day06::solve_part1(input).unwrap();
        // First: 4 lights on (0,0 to 1,1)
        // Second: 4 more lights, but (1,1) overlaps
        // Total: 7 unique lights on
        assert_eq!(result, "7");
    }

    #[test]
    fn test_complex_example() {
        // Example from problem description
        let input = "turn on 0,0 through 999,999\ntoggle 0,0 through 999,0\nturn off 499,499 through 500,500";
        let result = day06::solve_part1(input).unwrap();
        // Start: 1,000,000 lights on
        // Toggle first row: turns off 1,000 lights
        // Turn off middle 4: turns off 4 lights (but they might already be off from toggle)
        // Need to calculate exact result based on implementation
        assert!(result.parse::<u32>().unwrap() < 1000000);
    }

    // ====== PART 2 TESTS (Brightness Control) ======
    // Part 2 changes the meaning: lights have brightness levels, not just on/off

    #[test]
    fn test_part2_turn_on_increases_brightness() {
        // "turn on" increases brightness by 1
        let input = "turn on 0,0 through 0,0";
        let result = day06::solve_part2(input).unwrap();
        assert_eq!(result, "1"); // Brightness of 1
    }

    #[test]
    fn test_part2_turn_off_decreases_brightness() {
        // "turn off" decreases brightness by 1 (minimum 0)
        let input = "turn off 0,0 through 0,0";
        let result = day06::solve_part2(input).unwrap();
        assert_eq!(result, "0"); // Can't go below 0
    }

    #[test]
    fn test_part2_toggle_increases_by_2() {
        // "toggle" increases brightness by 2
        let input = "toggle 0,0 through 0,0";
        let result = day06::solve_part2(input).unwrap();
        assert_eq!(result, "2"); // Brightness of 2
    }

    #[test]
    fn test_part2_sequential_brightness() {
        // Multiple operations affecting brightness
        let input = "turn on 0,0 through 0,0\nturn on 0,0 through 0,0\ntoggle 0,0 through 0,0";
        let result = day06::solve_part2(input).unwrap();
        // Start: 0, turn on: 1, turn on: 2, toggle: 4
        assert_eq!(result, "4");
    }

    #[test]
    fn test_part2_brightness_never_negative() {
        // Turn off should not make brightness negative
        let input = "turn on 0,0 through 0,0\nturn off 0,0 through 0,0\nturn off 0,0 through 0,0";
        let result = day06::solve_part2(input).unwrap();
        // Start: 0, turn on: 1, turn off: 0, turn off: still 0
        assert_eq!(result, "0");
    }

    #[test]
    fn test_part2_example_from_problem() {
        // Examples given in problem description
        let input = "turn on 0,0 through 0,0";
        assert_eq!(day06::solve_part2(input).unwrap(), "1");

        let input = "toggle 0,0 through 999,999";
        assert_eq!(day06::solve_part2(input).unwrap(), "2000000"); // 2 * 1,000,000
    }

    // ====== EDGE CASES AND ERROR HANDLING ======

    #[test]
    fn test_empty_input() {
        let input = "";
        let result = day06::solve_part1(input).unwrap();
        assert_eq!(result, "0"); // No operations = no lights on
    }

    #[test]
    fn test_invalid_coordinates_should_not_panic() {
        // This test ensures graceful handling of edge cases
        // Implementation should handle bounds checking
        let input = "turn on 1000,1000 through 1000,1000"; // Out of bounds
                                                           // Should either ignore invalid coordinates or handle gracefully
        let result = day06::solve_part1(input);
        assert!(result.is_ok()); // Should not panic
    }

    #[test]
    fn test_backwards_coordinates() {
        // What if coordinates are backwards? (x2 < x1 or y2 < y1)
        let input = "turn on 2,2 through 0,0";
        let result = day06::solve_part1(input).unwrap();
        // Should handle this case (maybe swap coordinates or ignore)
        assert!(result.parse::<u32>().is_ok());
    }

    #[test]
    fn test_single_coordinate_range() {
        // Same start and end coordinates
        let input = "turn on 5,5 through 5,5";
        let result = day06::solve_part1(input).unwrap();
        assert_eq!(result, "1");
    }

    // ====== PERFORMANCE TESTS ======

    #[test]
    fn test_large_operations() {
        // Test that large operations complete in reasonable time
        let input = "turn on 0,0 through 999,999\nturn off 0,0 through 999,999";
        let result = day06::solve_part1(input).unwrap();
        assert_eq!(result, "0"); // All on, then all off = 0
    }

    // ====== PARSING TESTS ======
    // These test the instruction parsing logic

    #[test]
    fn test_parse_turn_on_instruction() {
        // This will require exposing parsing logic or testing through public interface
        let input = "turn on 123,456 through 789,012";
        let result = day06::solve_part1(input);
        assert!(result.is_ok()); // Should parse without error
    }

    #[test]
    fn test_parse_turn_off_instruction() {
        let input = "turn off 0,0 through 1,1";
        let result = day06::solve_part1(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_toggle_instruction() {
        let input = "toggle 100,200 through 300,400";
        let result = day06::solve_part1(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiline_input() {
        let input = "turn on 0,0 through 1,1\nturn off 0,0 through 0,0\ntoggle 1,1 through 1,1";
        let result = day06::solve_part1(input);
        assert!(result.is_ok());
    }
}
