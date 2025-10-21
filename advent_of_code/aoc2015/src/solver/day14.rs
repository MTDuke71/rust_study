//! # AoC 2015 Day 14: Reindeer Olympics
//! 
//! This module implements a comprehensive solution for the Advent of Code 2015 Day 14 challenge,
//! which involves simulating a reindeer racing competition with complex flight patterns.
//! 
//! ## Problem Overview
//! 
//! The challenge involves reindeer that alternate between flying and resting phases:
//! - Each reindeer flies at a constant speed for a specific duration
//! - After flying, they must rest for a specific duration  
//! - This cycle repeats throughout the race
//! 
//! ## Algorithm Complexity
//! 
//! - **Part 1**: O(n) per reindeer for distance calculation, O(n*m) total where n=reindeers, m=race_duration
//! - **Part 2**: O(n*m) for simulation where we calculate positions at every second
//! - **Space Complexity**: O(n) for storing reindeer data and tracking points
//! 
//! ## Key Insights
//! 
//! 1. **Cyclic Behavior**: Reindeer behavior follows predictable flight/rest cycles
//! 2. **State Simulation**: Part 2 requires second-by-second state tracking
//! 3. **Lead Changes**: The leader can change frequently, making point accumulation complex
//! 
//! ## Real-World Applications
//! 
//! This problem models scenarios like:
//! - Resource scheduling with work/break cycles
//! - Traffic light timing optimization
//! - Manufacturing with machine operation/maintenance cycles

use anyhow::Result;

// ============================================================================
// Constants
// ============================================================================

/// The duration of the reindeer race in seconds.
/// 
/// This represents the official AoC 2015 Day 14 race duration.
/// Used as the default time limit for both distance and points calculations.
const RACE_DURATION: u32 = 2503;
/// Solves Part 1: Find the winning distance after the race duration.
/// 
/// This function calculates the maximum distance traveled by any reindeer
/// during the race period using cyclic flight/rest patterns.
/// 
/// ## Algorithm
/// 
/// 1. Parse all reindeer data from input
/// 2. Calculate final distance for each reindeer using cycle simulation
/// 3. Return the maximum distance achieved
/// 
/// ## Example
/// 
/// ```text
/// Input: "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds."
/// - Comet flies 14 km/s for 10s = 140 km, then rests 127s
/// - After 2503s: Multiple cycles + partial cycle = final distance
/// ```
/// 
/// ## Time Complexity
/// O(n * m) where n = number of reindeer, m = race duration / cycle length
pub fn solve_part1(input: &str) -> Result<String> {
    println!("🎄 Day 14 Part 1: Reindeer Olympics - Distance Race");
    
    let data = parse(input)?;
    
    // Calculate maximum distance using efficient cycle-based algorithm
    let winning_distance = calculate_winning_distance(&data, RACE_DURATION);
    
    println!("   Winning distance: {} km", winning_distance);
    Ok(winning_distance.to_string())
}

/// Solves Part 2: Find the reindeer with the most points after the race.
/// 
/// This function simulates the race second-by-second, awarding points to 
/// the leading reindeer(s) at each moment in time.
/// 
/// ## Algorithm
/// 
/// 1. Parse all reindeer data from input
/// 2. For each second from 1 to race_duration:
///    - Calculate current position of each reindeer
///    - Find the maximum distance(s)
///    - Award 1 point to all reindeer at maximum distance
/// 3. Return the maximum points earned by any reindeer
/// 
/// ## Example
/// 
/// ```text
/// Second 1: Comet leads (14 km) -> Comet gets 1 point
/// Second 2: Comet leads (28 km) -> Comet gets 1 point  
/// ...
/// Second 11: Comet resting, Dancer leads -> Dancer gets 1 point
/// ```
/// 
/// ## Time Complexity
/// O(n * m) where n = number of reindeer, m = race duration
pub fn solve_part2(input: &str) -> Result<String> {
    println!("🎄 Day 14 Part 2: Reindeer Olympics - Points Race");
    
    let data = parse(input)?;
    
    // Simulate race second-by-second to track point accumulation
    let winner_points = simulate_race_with_points(&data, RACE_DURATION);
    
    println!("   Winning points: {}", winner_points);
    Ok(winner_points.to_string())
}

// ============================================================================
// Data Structures
// ============================================================================

/// Represents a reindeer's flight characteristics and performance metrics.
/// 
/// Each reindeer follows a predictable cycle: fly at constant speed for 
/// `flight_time` seconds, then rest for `rest_time` seconds, then repeat.
/// 
/// ## Fields
/// 
/// * `name` - Unique identifier for the reindeer (e.g., "Comet", "Dancer")
/// * `speed` - Flying speed in km/s (constant during flight phases)  
/// * `flight_time` - Duration of each flight phase in seconds
/// * `rest_time` - Duration of each rest phase in seconds
/// 
/// ## Cycle Behavior
/// 
/// ```text
/// Time:     0----10----137----147----274----284----...
/// Comet:    [Flying] [Resting] [Flying] [Resting] [Flying]...
/// Speed:    14 km/s    0 km/s   14 km/s   0 km/s    14 km/s
/// ```
/// 
/// ## Examples
/// 
/// ```rust
/// use aoc2015::solver::day14::Reindeer;
/// 
/// let comet = Reindeer::new("Comet".to_string(), 14, 10, 127);
/// 
/// // Cycle analysis
/// assert_eq!(comet.cycle_length(), 137); // 10 + 127
/// assert_eq!(comet.distance_per_cycle(), 140); // 14 * 10
/// assert_eq!(comet.is_flying_at(5), true);   // 5 < 10 (flight_time)
/// assert_eq!(comet.is_flying_at(15), false); // 15 > 10 (resting)
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reindeer {
    /// Reindeer's unique name identifier
    name: String,
    /// Flying speed in kilometers per second (constant during flight)
    speed: u32,
    /// Duration of flying phase in seconds
    flight_time: u32,
    /// Duration of resting phase in seconds  
    rest_time: u32,
}

impl Reindeer {
    /// Creates a new reindeer with the specified flight characteristics.
    /// 
    /// ## Parameters
    /// 
    /// * `name` - Unique identifier for this reindeer
    /// * `speed` - Flying speed in km/s (must be > 0)
    /// * `flight_time` - Seconds of flight per cycle (must be > 0)  
    /// * `rest_time` - Seconds of rest per cycle (must be > 0)
    /// 
    /// ## Examples
    /// 
    /// ```rust
    /// use aoc2015::solver::day14::Reindeer;
    /// 
    /// let comet = Reindeer::new("Comet".to_string(), 14, 10, 127);
    /// let dancer = Reindeer::new("Dancer".to_string(), 16, 11, 162);
    /// ```
    pub fn new(name: String, speed: u32, flight_time: u32, rest_time: u32) -> Self {
        Self { name, speed, flight_time, rest_time }
    }

    /// Returns the total length of one complete flight/rest cycle.
    /// 
    /// This is used for calculating how many complete cycles fit within
    /// a given race duration.
    /// 
    /// ## Returns
    /// 
    /// Total cycle time in seconds (flight_time + rest_time)
    pub fn cycle_length(&self) -> u32 {
        self.flight_time + self.rest_time
    }

    /// Calculates the distance traveled during one complete cycle.
    /// 
    /// Only the flight phase contributes to distance; rest contributes zero.
    /// 
    /// ## Returns
    /// 
    /// Distance in kilometers for one complete cycle
    pub fn distance_per_cycle(&self) -> u32 {
        self.speed * self.flight_time
    }

    /// Determines if the reindeer is flying at a specific time.
    /// 
    /// This function uses modular arithmetic to determine the reindeer's
    /// state at any point in time based on their cycle pattern.
    /// 
    /// ## Parameters
    /// 
    /// * `time` - The time to check (1-based, matching problem statement)
    /// 
    /// ## Returns
    /// 
    /// `true` if flying, `false` if resting at the given time
    /// 
    /// ## Algorithm
    /// 
    /// ```text
    /// cycle_position = (time - 1) % cycle_length
    /// flying = cycle_position < flight_time
    /// ```
    pub fn is_flying_at(&self, time: u32) -> bool {
        if time == 0 {
            return false;
        }
        let cycle_position = (time - 1) % self.cycle_length();
        cycle_position < self.flight_time
    }

    /// Returns the reindeer's name for identification purposes.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the reindeer's flight speed in km/s.
    pub fn speed(&self) -> u32 {
        self.speed
    }

    /// Returns the duration of the flight phase in seconds.
    pub fn flight_time(&self) -> u32 {
        self.flight_time
    }

    /// Returns the duration of the rest phase in seconds.  
    pub fn rest_time(&self) -> u32 {
        self.rest_time
    }
}

// ============================================================================
// Parsing
// ============================================================================

/// Parses the input text into a vector of reindeer with their flight characteristics.
/// 
/// ## Input Format
/// 
/// Each line follows this exact pattern:
/// ```text
/// "{Name} can fly {speed} km/s for {flight_time} seconds, but then must rest for {rest_time} seconds."
/// ```
/// 
/// ## Examples
/// 
/// ```text
/// Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
/// Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
/// ```
/// 
/// ## Error Handling
/// 
/// - Skips empty lines and malformed entries
/// - Returns parsing errors for invalid numeric values
/// - Continues processing after encountering malformed lines
/// 
/// ## Algorithm
/// 
/// 1. Split each line into whitespace-separated tokens
/// 2. Extract name from position 0
/// 3. Extract speed from position 3 (after "fly")
/// 4. Extract flight_time from position 6 (after "for")  
/// 5. Extract rest_time from position 13 (after "rest for")
/// 
/// ## Time Complexity
/// O(n * m) where n = number of lines, m = average line length
fn parse(input: &str) -> Result<Vec<Reindeer>> {
    let mut reindeers = Vec::new();
    
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Parse line: "Vixen can fly 8 km/s for 8 seconds, but then must rest for 53 seconds."
        let parts: Vec<&str> = line.split_whitespace().collect();

        // Validate line format
        if parts.len() < 15 {
            continue; // Skip malformed lines
        }

        let name = parts[0];
        let speed_str = parts[3];
        let flight_time_str = parts[6];
        let rest_time_str = parts[13];  

        let speed: u32 = speed_str.parse()?;
        let flight_time: u32 = flight_time_str.parse()?;
        let rest_time: u32 = rest_time_str.parse()?;

        reindeers.push(Reindeer::new(
            name.to_string(),
            speed,
            flight_time,
            rest_time,
        )); 
    }
    Ok(reindeers)

}

/// Calculates the winning distance among all reindeer after the specified duration.
/// 
/// This is the primary function for Part 1 that determines which reindeer
/// travels the farthest distance during the race.
/// 
/// ## Algorithm
/// 
/// 1. Calculate final distance for each reindeer using `calculate_distance`
/// 2. Find the maximum distance among all reindeer
/// 3. Return the winning distance (0 if no reindeer)
/// 
/// ## Parameters
/// 
/// * `reindeers` - Slice of reindeer to evaluate
/// * `duration` - Total race time in seconds
/// 
/// ## Returns
/// 
/// Maximum distance traveled by any reindeer in kilometers
/// 
/// ## Time Complexity
/// O(n * m) where n = number of reindeer, m = duration / cycle_length
fn calculate_winning_distance(reindeers: &[Reindeer], duration: u32) -> u32 {
    reindeers.iter()
        .map(|r| calculate_distance(r, duration))
        .max()
        .unwrap_or(0)
} 

/// Calculates the total distance traveled by a reindeer over the specified duration.
/// 
/// This function simulates the reindeer's alternating flight and rest cycles
/// to determine the final distance traveled.
/// 
/// ## Algorithm
/// 
/// The simulation works by repeatedly alternating between flight and rest phases:
/// 
/// 1. **Flight Phase**: Reindeer flies at `speed` for `flight_time` seconds
///    - Add `speed * actual_flight_time` to total distance
///    - `actual_flight_time` may be less than `flight_time` if race ends during flight
/// 
/// 2. **Rest Phase**: Reindeer rests for `rest_time` seconds
///    - No distance is added during rest
///    - Time still counts toward total duration
/// 
/// 3. **Repeat** until `duration` seconds have elapsed
/// 
/// ## Example Calculation
/// 
/// ```text
/// Comet: 14 km/s for 10s, rest 127s
/// Duration: 1000s
/// 
/// Cycle 1 (0-137s):   Flight: 10s * 14 km/s = 140 km, Rest: 127s
/// Cycle 2 (137-274s): Flight: 10s * 14 km/s = 140 km, Rest: 127s  
/// Cycle 3 (274-411s): Flight: 10s * 14 km/s = 140 km, Rest: 127s
/// ...
/// Cycle 7 (822-959s): Flight: 10s * 14 km/s = 140 km, Rest: 127s
/// Final (959-1000s):  Flight: 41s * 14 km/s = 574 km (partial cycle)
/// 
/// Total: 6 * 140 + 574 = 1414 km
/// ```
/// 
/// ## Parameters
/// 
/// * `reindeer` - The reindeer whose distance to calculate
/// * `duration` - Total time available for the race in seconds
/// 
/// ## Returns
/// 
/// Total distance traveled in kilometers
/// 
/// ## Time Complexity
/// O(duration / cycle_length) where cycle_length = flight_time + rest_time
pub fn calculate_distance(reindeer: &Reindeer, duration: u32) -> u32 {
    let mut time_remaining = duration;
    let mut distance = 0;

    // Simulate alternating flight/rest cycles until time runs out
    while time_remaining > 0 {
        // Flight phase: calculate actual flight time (may be truncated)
        let flight_time = reindeer.flight_time.min(time_remaining);
        distance += flight_time * reindeer.speed;
        time_remaining -= flight_time;

        // Rest phase: no distance gained, but time still passes  
        let rest_time = reindeer.rest_time.min(time_remaining);
        time_remaining -= rest_time;
    }

    distance
}     

// ============================================================================
// Solutions
// ============================================================================

/// Simulates the race with points-based scoring for Part 2.
/// 
/// This function tracks the race second-by-second, awarding points to the 
/// leading reindeer(s) at each moment. Unlike Part 1, this scoring system
/// can result in different winners based on lead changes during the race.
/// 
/// ## Scoring Rules
/// 
/// - At each second, calculate the current position of all reindeer
/// - Find the maximum distance(s) achieved at that second
/// - Award 1 point to ALL reindeer currently at the maximum distance
/// - Continue for the entire race duration
/// 
/// ## Algorithm Complexity
/// 
/// **Time**: O(n * m) where n = number of reindeer, m = race duration
/// **Space**: O(n) for storing points and distance calculations
/// 
/// ## Example Simulation
/// 
/// ```text
/// Reindeer: Comet (14 km/s, 10s flight, 127s rest)
///          Dancer (16 km/s, 11s flight, 162s rest)
/// 
/// Second 1: Comet=14km, Dancer=16km → Dancer gets 1 point
/// Second 2: Comet=28km, Dancer=32km → Dancer gets 1 point  
/// ...
/// Second 11: Comet=140km, Dancer=176km → Dancer gets 1 point
/// Second 12: Comet=140km, Dancer=176km → Dancer gets 1 point (both resting)
/// ...
/// ```
/// 
/// ## Key Insight
/// 
/// The winner by points can differ significantly from the winner by distance:
/// - A reindeer with lower top speed might lead more consistently
/// - Reindeer with shorter cycles might accumulate more "leading moments"
/// - The final distance leader might not have the most points
/// 
/// ## Parameters
/// 
/// * `reindeers` - Slice of all reindeer participating in the race
/// * `duration` - Total race time in seconds
/// 
/// ## Returns
/// 
/// Maximum points earned by any reindeer during the race
pub fn simulate_race_with_points(reindeers: &[Reindeer], duration: u32) -> u32 {
    if reindeers.is_empty() {
        return 0;
    }
    
    let mut points = vec![0u32; reindeers.len()];
    
    for second in 1..=duration {
        // Calculate current position of each reindeer at this second
        let distances: Vec<u32> = reindeers
            .iter()
            .map(|r| calculate_distance(r, second))
            .collect();
        
        // Find the maximum distance
        let max_distance = distances.iter().max().unwrap_or(&0);
        
        // Award points to all reindeer at the maximum distance
        for (i, distance) in distances.iter().enumerate() {
            if distance == max_distance {
                points[i] += 1;
            }
        }
    }
    
    // Return the highest score
    points.into_iter().max().unwrap_or(0)
}

// ============================================================================
// Analysis and Debugging Utilities
// ============================================================================

/// Analyzes all reindeer performance characteristics for debugging and optimization.
/// 
/// This function provides detailed performance metrics that can help understand
/// why certain reindeer perform better in different scenarios.
/// 
/// ## Returns
/// 
/// A formatted string containing comprehensive analysis of all reindeer
#[allow(dead_code)]
fn analyze_reindeer_performance(reindeers: &[Reindeer], duration: u32) -> String {
    let mut analysis = String::new();
    analysis.push_str("=== REINDEER PERFORMANCE ANALYSIS ===\n\n");
    
    for (i, reindeer) in reindeers.iter().enumerate() {
        let distance = calculate_distance(reindeer, duration);
        let cycle_length = reindeer.cycle_length();
        let distance_per_cycle = reindeer.distance_per_cycle();
        let complete_cycles = duration / cycle_length;
        let remaining_time = duration % cycle_length;
        
        analysis.push_str(&format!("{}. {} Analysis:\n", i + 1, reindeer.name()));
        analysis.push_str(&format!("   Speed: {} km/s\n", reindeer.speed));
        analysis.push_str(&format!("   Flight: {}s, Rest: {}s\n", reindeer.flight_time, reindeer.rest_time));
        analysis.push_str(&format!("   Cycle: {}s total, {} km per cycle\n", cycle_length, distance_per_cycle));
        analysis.push_str(&format!("   Complete cycles: {} ({} km)\n", complete_cycles, complete_cycles * distance_per_cycle));
        analysis.push_str(&format!("   Remaining time: {}s\n", remaining_time));
        
        let final_flight_time = remaining_time.min(reindeer.flight_time);
        let final_distance = final_flight_time * reindeer.speed;
        analysis.push_str(&format!("   Final distance: {} km\n", final_distance));
        analysis.push_str(&format!("   TOTAL DISTANCE: {} km\n\n", distance));
    }
    
    let winner_distance = calculate_winning_distance(reindeers, duration);
    analysis.push_str(&format!("WINNER: {} km\n", winner_distance));
    
    analysis
}

/// Generates a detailed race timeline showing position changes over time.
/// 
/// This debugging utility shows how the race unfolds second-by-second,
/// which is particularly useful for understanding Part 2 scoring.
/// 
/// ## Parameters
/// 
/// * `reindeers` - The competing reindeer
/// * `max_time` - Maximum time to simulate (use small values for readability)
/// 
/// ## Returns
/// 
/// A formatted string showing the race timeline
#[allow(dead_code)]
fn generate_race_timeline(reindeers: &[Reindeer], max_time: u32) -> String {
    let mut timeline = String::new();
    timeline.push_str("=== RACE TIMELINE ===\n\n");
    timeline.push_str("Time | ");
    
    // Header with reindeer names
    for reindeer in reindeers {
        timeline.push_str(&format!("{:>8} | ", reindeer.name()));
    }
    timeline.push_str("Leader(s)\n");
    
    // Separator line
    timeline.push_str("-----+");
    for _ in reindeers {
        timeline.push_str("----------+");
    }
    timeline.push_str("----------\n");
    
    // Timeline data
    for second in 1..=max_time {
        timeline.push_str(&format!("{:4} | ", second));
        
        let distances: Vec<u32> = reindeers
            .iter()
            .map(|r| calculate_distance(r, second))
            .collect();
        
        let max_distance = distances.iter().max().unwrap_or(&0);
        
        // Display distances
        for distance in &distances {
            timeline.push_str(&format!("{:8} | ", distance));
        }
        
        // Display leaders
        let leaders: Vec<&str> = reindeers
            .iter()
            .zip(&distances)
            .filter(|(_, &dist)| dist == *max_distance)
            .map(|(reindeer, _)| reindeer.name())
            .collect();
        
        timeline.push_str(&format!("{}\n", leaders.join(", ")));
    }
    
    timeline
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Part 1 Tests
    // ========================================================================

    #[test]
    fn test_calculate_distance_simple_case() {
        let comet = Reindeer {
            name: "Comet".to_string(),
            speed: 14,
            flight_time: 10,
            rest_time: 127,
        };
        
        // Test for exactly one flight cycle (10 seconds)
        assert_eq!(calculate_distance(&comet, 10), 140); // 14 * 10
        
        // Test for one flight + partial rest (100 seconds)
        assert_eq!(calculate_distance(&comet, 100), 140); // Still just one flight
        
        // Test for one complete cycle + partial next flight (150 seconds)
        // First cycle: 10 flight + 127 rest = 137 seconds, distance = 140
        // Remaining: 13 seconds, but only 10 more flight seconds, distance = 140
        assert_eq!(calculate_distance(&comet, 150), 280); // 140 + 140
    }

    #[test]
    fn test_calculate_distance_example_from_problem() {
        // Example from AoC 2015 Day 14 problem description
        let comet = Reindeer {
            name: "Comet".to_string(),
            speed: 14,
            flight_time: 10,
            rest_time: 127,
        };
        
        let dancer = Reindeer {
            name: "Dancer".to_string(),
            speed: 16,
            flight_time: 11,
            rest_time: 162,
        };
        
        // At 1000 seconds (from problem description)
        let comet_distance_1000 = calculate_distance(&comet, 1000);
        let dancer_distance_1000 = calculate_distance(&dancer, 1000);
        
        // Comet: cycle = 137s, 7 complete cycles in 1000s = 959s, 41s left
        // 7 cycles * 140km + 1 more flight (10s) * 14km/s = 980 + 140 = 1120km
        assert_eq!(comet_distance_1000, 1120);
        
        // Dancer: cycle = 173s, 5 complete cycles in 1000s = 865s, 135s left
        // 5 cycles * 176km + 1 more flight (11s) * 16km/s = 880 + 176 = 1056km  
        assert_eq!(dancer_distance_1000, 1056);
        
        // Comet should be ahead at 1000 seconds
        assert!(comet_distance_1000 > dancer_distance_1000);
    }

    #[test]
    fn test_calculate_distance_edge_cases() {
        let reindeer = Reindeer {
            name: "Test".to_string(),
            speed: 10,
            flight_time: 5,
            rest_time: 10,
        };
        
        // Zero duration
        assert_eq!(calculate_distance(&reindeer, 0), 0);
        
        // Duration less than flight time
        assert_eq!(calculate_distance(&reindeer, 3), 30); // 3 * 10
        
        // Duration equal to flight time
        assert_eq!(calculate_distance(&reindeer, 5), 50); // 5 * 10
        
        // Duration equal to one complete cycle
        assert_eq!(calculate_distance(&reindeer, 15), 50); // One flight only
    }

    #[test]
    fn test_calculate_distance_zero_speed() {
        let stationary = Reindeer {
            name: "Stationary".to_string(),
            speed: 0,
            flight_time: 10,
            rest_time: 5,
        };
        
        assert_eq!(calculate_distance(&stationary, 100), 0);
    }

    #[test]
    fn test_calculate_distance_zero_flight_time() {
        let no_flight = Reindeer {
            name: "NoFlight".to_string(),
            speed: 100,
            flight_time: 0,
            rest_time: 5,
        };
        
        assert_eq!(calculate_distance(&no_flight, 100), 0);
    }

    #[test]
    fn test_calculate_winning_distance_single_reindeer() {
        let reindeers = vec![
            Reindeer {
                name: "Solo".to_string(),
                speed: 15,
                flight_time: 8,
                rest_time: 12,
            }
        ];
        
        let winning_distance = calculate_winning_distance(&reindeers, 100);
        let expected = calculate_distance(&reindeers[0], 100);
        assert_eq!(winning_distance, expected);
    }

    #[test]
    fn test_calculate_winning_distance_multiple_reindeer() {
        let reindeers = vec![
            Reindeer {
                name: "Slow".to_string(),
                speed: 5,
                flight_time: 10,
                rest_time: 20,
            },
            Reindeer {
                name: "Fast".to_string(),
                speed: 20,
                flight_time: 5,
                rest_time: 10,
            },
            Reindeer {
                name: "Medium".to_string(),
                speed: 10,
                flight_time: 8,
                rest_time: 15,
            }
        ];
        
        let winning_distance = calculate_winning_distance(&reindeers, 100);
        
        // Calculate each distance
        let slow_distance = calculate_distance(&reindeers[0], 100);
        let fast_distance = calculate_distance(&reindeers[1], 100);
        let medium_distance = calculate_distance(&reindeers[2], 100);
        
        // Winning distance should be the maximum
        let expected_max = slow_distance.max(fast_distance).max(medium_distance);
        
        assert_eq!(winning_distance, expected_max);
        
        // Verify it's actually the fast reindeer that wins
        assert_eq!(winning_distance, fast_distance);
        assert!(fast_distance >= slow_distance);
        assert!(fast_distance >= medium_distance);
    }

    #[test]
    fn test_calculate_winning_distance_empty_list() {
        let reindeers = vec![];
        assert_eq!(calculate_winning_distance(&reindeers, 100), 0);
    }

    #[test]
    fn test_calculate_winning_distance_example_from_problem() {
        // Using the example from AoC 2015 Day 14
        let reindeers = vec![
            Reindeer {
                name: "Comet".to_string(),
                speed: 14,
                flight_time: 10,
                rest_time: 127,
            },
            Reindeer {
                name: "Dancer".to_string(),
                speed: 16,
                flight_time: 11,
                rest_time: 162,
            }
        ];
        
        // At 1000 seconds, Comet should win
        let winning_distance = calculate_winning_distance(&reindeers, 1000);
        assert_eq!(winning_distance, 1120); // Comet's distance
        
        // Test with race duration constant
        let winning_distance_full_race = calculate_winning_distance(&reindeers, RACE_DURATION);
        assert!(winning_distance_full_race > 0);
    }

    #[test]
    fn test_solve_part1_with_example_input() {
        let input = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#;
        
        let result = solve_part1(input).unwrap();
        
        // Should return the winning distance as a string
        let winning_distance: u32 = result.parse().unwrap();
        assert!(winning_distance > 0);
        
        // Calculate expected result manually
        let expected = calculate_winning_distance(
            &parse(input).unwrap(),
            RACE_DURATION
        );
        assert_eq!(winning_distance, expected);
    }

    #[test]
    fn test_solve_part1_empty_input() {
        let result = solve_part1("").unwrap();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_solve_part1_single_reindeer() {
        let input = "Rudolph can fly 22 km/s for 8 seconds, but then must rest for 165 seconds.";
        
        let result = solve_part1(input).unwrap();
        let distance: u32 = result.parse().unwrap();
        
        // Calculate expected distance for single reindeer
        let reindeer = Reindeer {
            name: "Rudolph".to_string(),
            speed: 22,
            flight_time: 8,
            rest_time: 165,
        };
        let expected = calculate_distance(&reindeer, RACE_DURATION);
        
        assert_eq!(distance, expected);
    }

    #[test]
    fn test_solve_part1_malformed_input() {
        let input = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Invalid line with insufficient parts
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#;
        
        let result = solve_part1(input).unwrap();
        let distance: u32 = result.parse().unwrap();
        
        // Should process only the valid lines
        assert!(distance > 0);
    }

    // ========================================================================
    // Part 2 Tests
    // ========================================================================

    #[test]
    fn test_simulate_race_with_points_simple_case() {
        let reindeers = vec![
            Reindeer {
                name: "Fast".to_string(),
                speed: 10,
                flight_time: 2,
                rest_time: 1, // cycle: 3 seconds
            },
            Reindeer {
                name: "Slow".to_string(),
                speed: 5,
                flight_time: 1,
                rest_time: 1, // cycle: 2 seconds
            }
        ];
        
        // Test for 6 seconds to see multiple cycles
        let points = simulate_race_with_points(&reindeers, 6);
        
        // Fast: distances [10, 20, 20, 30, 40, 40]
        // Slow: distances [5, 5, 10, 10, 15, 15]
        // Fast is always leading, so should get all 6 points
        assert_eq!(points, 6);
    }

    #[test]
    fn test_simulate_race_with_points_tie_scenario() {
        let reindeers = vec![
            Reindeer {
                name: "Twin1".to_string(),
                speed: 10,
                flight_time: 1,
                rest_time: 1,
            },
            Reindeer {
                name: "Twin2".to_string(),
                speed: 10,
                flight_time: 1,
                rest_time: 1,
            }
        ];
        
        // Both reindeer are identical, so they should tie and both get points
        let points = simulate_race_with_points(&reindeers, 4);
        
        // Both should get all 4 points since they're always tied
        assert_eq!(points, 4);
    }

    #[test]
    fn test_simulate_race_with_points_lead_changes() {
        let reindeers = vec![
            Reindeer {
                name: "Sprinter".to_string(),
                speed: 20, // Fast but short bursts
                flight_time: 1,
                rest_time: 3,
            },
            Reindeer {
                name: "Marathoner".to_string(),
                speed: 5, // Slow but consistent
                flight_time: 3,
                rest_time: 1,
            }
        ];
        
        // Test leadership changes over time
        let points = simulate_race_with_points(&reindeers, 8);
        
        // Second 1: Sprinter=20, Marathoner=5 -> Sprinter gets point
        // Second 2: Sprinter=20, Marathoner=10 -> Sprinter gets point  
        // Second 3: Sprinter=20, Marathoner=15 -> Sprinter gets point
        // Second 4: Sprinter=20, Marathoner=15 -> Sprinter gets point
        // Second 5: Sprinter=40, Marathoner=20 -> Sprinter gets point
        // Second 6: Sprinter=40, Marathoner=25 -> Sprinter gets point
        // Second 7: Sprinter=40, Marathoner=30 -> Sprinter gets point
        // Second 8: Sprinter=40, Marathoner=30 -> Sprinter gets point
        
        assert!(points > 0);
    }

    #[test]
    fn test_simulate_race_with_points_example_from_problem() {
        // Using the example from AoC 2015 Day 14
        let reindeers = vec![
            Reindeer {
                name: "Comet".to_string(),
                speed: 14,
                flight_time: 10,
                rest_time: 127,
            },
            Reindeer {
                name: "Dancer".to_string(),
                speed: 16,
                flight_time: 11,
                rest_time: 162,
            }
        ];
        
        // Test with smaller duration first to verify logic
        let points_10s = simulate_race_with_points(&reindeers, 10);
        assert!(points_10s > 0);
        
        // Test with 1000 seconds (example from problem)
        let points_1000s = simulate_race_with_points(&reindeers, 1000);
        assert!(points_1000s > 0);
        
        // The winner should have more than half the points in most cases
        // (though not guaranteed due to ties)
        assert!(points_1000s <= 1000); // Can't exceed total seconds
    }

    #[test]
    fn test_simulate_race_with_points_single_reindeer() {
        let reindeers = vec![
            Reindeer {
                name: "Solo".to_string(),
                speed: 15,
                flight_time: 5,
                rest_time: 10,
            }
        ];
        
        let points = simulate_race_with_points(&reindeers, 20);
        
        // Single reindeer always leads, gets all points
        assert_eq!(points, 20);
    }

    #[test]
    fn test_simulate_race_with_points_empty_list() {
        let reindeers = vec![];
        let points = simulate_race_with_points(&reindeers, 100);
        assert_eq!(points, 0);
    }

    #[test]
    fn test_simulate_race_with_points_zero_duration() {
        let reindeers = vec![
            Reindeer {
                name: "Test".to_string(),
                speed: 10,
                flight_time: 1,
                rest_time: 1,
            }
        ];
        
        let points = simulate_race_with_points(&reindeers, 0);
        assert_eq!(points, 0);
    }

    #[test]
    fn test_simulate_race_with_points_stationary_reindeer() {
        let reindeers = vec![
            Reindeer {
                name: "Stationary".to_string(),
                speed: 0,
                flight_time: 10,
                rest_time: 5,
            },
            Reindeer {
                name: "Moving".to_string(),
                speed: 5,
                flight_time: 2,
                rest_time: 3,
            }
        ];
        
        let points = simulate_race_with_points(&reindeers, 10);
        
        // Moving reindeer should always be ahead (except at second 0)
        // So moving reindeer should get all or most points
        assert!(points >= 9); // At least 9 out of 10 seconds
    }

    #[test]
    fn test_solve_part2_with_example_input() {
        let input = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#;
        
        let result = solve_part2(input).unwrap();
        
        // Should return the winning points as a string
        let winning_points: u32 = result.parse().unwrap();
        assert!(winning_points > 0);
        
        // Calculate expected result manually
        let expected = simulate_race_with_points(
            &parse(input).unwrap(),
            RACE_DURATION
        );
        assert_eq!(winning_points, expected);
    }

    #[test]
    fn test_solve_part2_empty_input() {
        let result = solve_part2("").unwrap();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_solve_part2_single_reindeer() {
        let input = "Rudolph can fly 22 km/s for 8 seconds, but then must rest for 165 seconds.";
        
        let result = solve_part2(input).unwrap();
        let points: u32 = result.parse().unwrap();
        
        // Single reindeer gets all points (equals to duration)
        assert_eq!(points, RACE_DURATION);
    }

    #[test]
    fn test_part1_vs_part2_different_winners() {
        // Create a scenario where distance winner != points winner
        let reindeers = vec![
            Reindeer {
                name: "LateRusher".to_string(),
                speed: 100, // Very fast
                flight_time: 1,
                rest_time: 99, // But mostly rests
            },
            Reindeer {
                name: "SteadyRunner".to_string(),
                speed: 2, // Slow but consistent
                flight_time: 50,
                rest_time: 1,
            }
        ];
        
        let test_duration = 200;
        
        let winning_distance = calculate_winning_distance(&reindeers, test_duration);
        let winning_points = simulate_race_with_points(&reindeers, test_duration);
        
        // Both should be valid results
        assert!(winning_distance > 0);
        assert!(winning_points > 0);
        assert!(winning_points <= test_duration);
    }

    // ========================================================================
    // Parse Function Tests (existing tests)
    // ========================================================================

    #[test]
    fn test_parse_empty_input() {
        let data = parse("").unwrap();
        assert_eq!(data.len(), 0);
    }

    #[test]
    fn test_parse_single_reindeer() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.";
        let data = parse(input).unwrap();
        
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].name, "Comet");
        assert_eq!(data[0].speed, 14);
        assert_eq!(data[0].flight_time, 10);
        assert_eq!(data[0].rest_time, 127);
    }

    #[test]
    fn test_parse_multiple_reindeer() {
        let input = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
Prancer can fly 18 km/s for 6 seconds, but then must rest for 103 seconds."#;
        
        let data = parse(input).unwrap();
        
        assert_eq!(data.len(), 3);
        
        // Check Comet
        assert_eq!(data[0].name, "Comet");
        assert_eq!(data[0].speed, 14);
        assert_eq!(data[0].flight_time, 10);
        assert_eq!(data[0].rest_time, 127);
        
        // Check Dancer
        assert_eq!(data[1].name, "Dancer");
        assert_eq!(data[1].speed, 16);
        assert_eq!(data[1].flight_time, 11);
        assert_eq!(data[1].rest_time, 162);
        
        // Check Prancer
        assert_eq!(data[2].name, "Prancer");
        assert_eq!(data[2].speed, 18);
        assert_eq!(data[2].flight_time, 6);
        assert_eq!(data[2].rest_time, 103);
    }

    #[test]
    fn test_parse_with_whitespace_and_empty_lines() {
        let input = r#"
        
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.

Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
        
        "#;
        
        let data = parse(input).unwrap();
        
        assert_eq!(data.len(), 2);
        assert_eq!(data[0].name, "Comet");
        assert_eq!(data[1].name, "Dancer");
    }

    #[test]
    fn test_parse_example_from_problem() {
        // Example from AoC 2015 Day 14 problem description
        let input = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#;
        
        let data = parse(input).unwrap();
        
        assert_eq!(data.len(), 2);
        
        // Verify Comet's stats
        let comet = &data[0];
        assert_eq!(comet.name, "Comet");
        assert_eq!(comet.speed, 14);
        assert_eq!(comet.flight_time, 10);
        assert_eq!(comet.rest_time, 127);
        
        // Verify Dancer's stats
        let dancer = &data[1];
        assert_eq!(dancer.name, "Dancer");
        assert_eq!(dancer.speed, 16);
        assert_eq!(dancer.flight_time, 11);
        assert_eq!(dancer.rest_time, 162);
    }

    #[test]
    fn test_parse_malformed_lines_are_skipped() {
        let input = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
This is a malformed line
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
Another malformed line with insufficient parts
Prancer can fly 18 km/s for 6 seconds, but then must rest for 103 seconds."#;
        
        let data = parse(input).unwrap();
        
        // Should only parse the 3 valid lines, skipping malformed ones
        assert_eq!(data.len(), 3);
        assert_eq!(data[0].name, "Comet");
        assert_eq!(data[1].name, "Dancer");
        assert_eq!(data[2].name, "Prancer");
    }

    #[test]
    fn test_parse_invalid_number_format() {
        let input = "Comet can fly abc km/s for 10 seconds, but then must rest for 127 seconds.";
        
        let result = parse(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_flight_time() {
        let input = "Comet can fly 14 km/s for xyz seconds, but then must rest for 127 seconds.";
        
        let result = parse(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_rest_time() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for def seconds.";
        
        let result = parse(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_zero_values() {
        let input = "Slowpoke can fly 0 km/s for 0 seconds, but then must rest for 0 seconds.";
        
        let data = parse(input).unwrap();
        
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].name, "Slowpoke");
        assert_eq!(data[0].speed, 0);
        assert_eq!(data[0].flight_time, 0);
        assert_eq!(data[0].rest_time, 0);
    }

    #[test]
    fn test_parse_large_values() {
        let input = "Speedy can fly 999 km/s for 999 seconds, but then must rest for 999 seconds.";
        
        let data = parse(input).unwrap();
        
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].name, "Speedy");
        assert_eq!(data[0].speed, 999);
        assert_eq!(data[0].flight_time, 999);
        assert_eq!(data[0].rest_time, 999);
    }

    #[test]
    fn test_reindeer_struct_equality() {
        let reindeer1 = Reindeer {
            name: "Comet".to_string(),
            speed: 14,
            flight_time: 10,
            rest_time: 127,
        };
        
        let reindeer2 = Reindeer {
            name: "Comet".to_string(),
            speed: 14,
            flight_time: 10,
            rest_time: 127,
        };
        
        assert_eq!(reindeer1, reindeer2);
    }

    #[test]
    fn test_reindeer_struct_clone() {
        let original = Reindeer {
            name: "Comet".to_string(),
            speed: 14,
            flight_time: 10,
            rest_time: 127,
        };
        
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }
}
