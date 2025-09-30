//! Tests for Day 6 command parser functionality
//! 
//! These tests verify that the separated parser functions work correctly
//! and handle both valid and invalid input appropriately.

use aoc2015::solver::day06::*;

#[test]
fn test_parse_light_command_turn_on() {
    let cmd = parse_light_command("turn on 0,0 through 999,999").unwrap();
    assert_eq!(cmd.action, LightAction::TurnOn);
    assert_eq!(cmd.x1, 0);
    assert_eq!(cmd.y1, 0);
    assert_eq!(cmd.x2, 999);
    assert_eq!(cmd.y2, 999);
}

#[test]
fn test_parse_light_command_turn_off() {
    let cmd = parse_light_command("turn off 499,499 through 500,500").unwrap();
    assert_eq!(cmd.action, LightAction::TurnOff);
    assert_eq!(cmd.x1, 499);
    assert_eq!(cmd.y1, 499);
    assert_eq!(cmd.x2, 500);
    assert_eq!(cmd.y2, 500);
}

#[test]
fn test_parse_light_command_toggle() {
    let cmd = parse_light_command("toggle 0,0 through 999,0").unwrap();
    assert_eq!(cmd.action, LightAction::Toggle);
    assert_eq!(cmd.x1, 0);
    assert_eq!(cmd.y1, 0);
    assert_eq!(cmd.x2, 999);
    assert_eq!(cmd.y2, 0);
}

#[test]
fn test_parse_invalid_command() {
    assert!(parse_light_command("invalid command").is_err());
    assert!(parse_light_command("turn").is_err());
    assert!(parse_light_command("toggle").is_err());
    assert!(parse_light_command("turn invalid 0,0 through 1,1").is_err());
}

#[test]
fn test_parse_real_aoc_examples() {
    // Test with actual commands from AoC Day 6
    let commands = [
        "turn on 887,9 through 959,629",
        "turn on 454,398 through 844,448", 
        "turn off 539,243 through 559,965",
        "toggle 720,196 through 897,994",
        "turn off 370,819 through 676,868",
        "turn off 145,40 through 370,997"
    ];
    
    for command_str in &commands {
        let cmd = parse_light_command(command_str).unwrap();
        
        // Verify coordinate ordering makes sense
        assert!(cmd.x1 <= cmd.x2, "x1 should be <= x2 in command: {}", command_str);
        assert!(cmd.y1 <= cmd.y2, "y1 should be <= y2 in command: {}", command_str);
        
        // Verify coordinates are within reasonable bounds
        assert!(cmd.x1 < 1000, "x1 should be < 1000 in command: {}", command_str);
        assert!(cmd.y1 < 1000, "y1 should be < 1000 in command: {}", command_str);
        assert!(cmd.x2 < 1000, "x2 should be < 1000 in command: {}", command_str);
        assert!(cmd.y2 < 1000, "y2 should be < 1000 in command: {}", command_str);
    }
}

#[test]
fn test_parser_with_grid_integration() {
    // Test that parsed commands work correctly with grid operations
    use mission6::Grid;
    
    let mut grid = Grid::new(5, 5, false);
    
    // Turn on a region
    apply_light_command(&mut grid, "turn on 1,1 through 3,3").unwrap();
    
    // Verify the correct cells are on
    for y in 1..=3 {
        for x in 1..=3 {
            assert_eq!(grid.get((x, y).into()), Some(&true), "Light at ({},{}) should be on", x, y);
        }
    }
    
    // Verify other cells are off
    assert_eq!(grid.get((0, 0).into()), Some(&false));
    assert_eq!(grid.get((4, 4).into()), Some(&false));
}

#[test]
fn test_brightness_parser_integration() {
    // Test brightness commands with the parser
    use mission6::Grid;
    
    let mut grid = Grid::new(3, 3, 0u32);
    
    // Turn on increases brightness by 1
    apply_brightness_command(&mut grid, "turn on 1,1 through 1,1").unwrap();
    assert_eq!(grid.get((1, 1).into()), Some(&1));
    
    // Toggle increases by 2
    apply_brightness_command(&mut grid, "toggle 1,1 through 1,1").unwrap();
    assert_eq!(grid.get((1, 1).into()), Some(&3));
    
    // Turn off decreases by 1 (with saturation at 0)
    apply_brightness_command(&mut grid, "turn off 1,1 through 1,1").unwrap();
    apply_brightness_command(&mut grid, "turn off 1,1 through 1,1").unwrap();
    apply_brightness_command(&mut grid, "turn off 1,1 through 1,1").unwrap();
    apply_brightness_command(&mut grid, "turn off 1,1 through 1,1").unwrap(); // Should saturate at 0
    assert_eq!(grid.get((1, 1).into()), Some(&0));
}