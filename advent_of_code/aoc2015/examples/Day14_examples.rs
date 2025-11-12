//! # AoC 2015 Day 14 Examples and Analysis
//!
//! This example demonstrates various approaches to the Reindeer Olympics problem,
//! including mathematical optimization, visual analysis, and performance comparison.

use aoc2015::solver::day14::*;

// ============================================================================
// Graphics Crate Recommendations & Real-Time Visualization Ideas
// ============================================================================
//
// For this example, we use ASCII graphics for simplicity and no dependencies.
// For production applications, consider these excellent Rust graphics crates:
//
// 🎮 **FUTURE PROJECT: 2D Real-Time Race Scroller**
//   A side-scrolling race visualization showing reindeer in real-time would be amazing!
//   Imagine: reindeer sprites flying/resting, distance markers, live leaderboard,
//   speed indicators, and visual flight/rest cycles. Perfect for education & fun!
//
//   **Implementation Frameworks:**
//   - **macroquad**: Game framework, perfect for 2D animations and sprites
//     Add: macroquad = "0.4", Features: Simple API, cross-platform, web support
//     Usage: Real-time animation loop, sprite rendering, smooth scrolling
//
//   - **bevy**: Full game engine with ECS architecture
//     Add: bevy = "0.12", Features: Entity-component system, plugins, 2D/3D
//     Usage: Entity for each reindeer, animation systems, camera following
//
//   - **ggez**: 2D game framework inspired by LÖVE
//     Add: ggez = "0.9", Features: Simple 2D graphics, audio, input handling
//     Usage: Game loop, sprite batching, smooth animations
//
//   **Animation Features to Implement:**
//   - Side-scrolling track with distance markers every 100km
//   - Reindeer sprites with flight/rest animations (wings flapping/folded)
//   - Speed trails when flying, "ZZZ" when resting
//   - Real-time leaderboard overlay showing current positions
//   - Progress bar showing race completion percentage
//   - Part 2 mode: Points accumulation with sparkle effects for leaders
//   - Playback controls: Play/pause/speed up/slow down time
//   - Camera follows the leader or allows free scrolling
//   - Victory celebration when race ends!
//
//   This would make the algorithm incredibly intuitive and engaging! 🏁
//
// 🏆 **RECOMMENDED: plotters** - Pure Rust plotting library
//   - Add to Cargo.toml: plotters = "0.3"
//   - Features: PNG/SVG output, multiple backends, excellent documentation
//   - Usage: plotters::prelude::*; ChartBuilder, LineSeries, etc.
//   - Perfect for: Scientific plots, data visualization, publication-quality graphs
//
// 📊 **Alternative: textplots** - Terminal-based plotting
//   - Add to Cargo.toml: textplots = "0.8"
//   - Features: Scatter plots, histograms, direct terminal output
//   - Usage: textplots::{Chart, Plot, Shape}
//   - Perfect for: Quick CLI visualizations, debugging, simple analysis
//
// 🎨 **Advanced: egui + eframe** - Immediate mode GUI with plotting
//   - Add to Cargo.toml: egui = "0.24", eframe = "0.24", egui_plot = "0.24"
//   - Features: Interactive plots, real-time updates, cross-platform GUI
//   - Perfect for: Interactive analysis tools, real-time monitoring
//
// 🚀 **Web-based: wasm-bindgen + web-sys + d3** - Browser visualization
//   - Compile to WebAssembly for interactive web-based analysis
//   - Perfect for: Shareable analysis, interactive demonstrations
//
// For this educational example, ASCII provides universal compatibility and
// demonstrates the core algorithmic concepts without external dependencies.

/// Example reindeer data for demonstrations
fn get_example_reindeer() -> Vec<Reindeer> {
    vec![
        Reindeer::new("Comet".to_string(), 14, 10, 127),
        Reindeer::new("Dancer".to_string(), 16, 11, 162),
        Reindeer::new("Prancer".to_string(), 18, 6, 103),
        Reindeer::new("Vixen".to_string(), 20, 2, 60),
    ]
}

/// Actual puzzle input reindeer for comprehensive analysis
fn get_actual_reindeer() -> Vec<Reindeer> {
    vec![
        Reindeer::new("Vixen".to_string(), 8, 8, 53),
        Reindeer::new("Blitzen".to_string(), 13, 4, 49),
        Reindeer::new("Rudolph".to_string(), 20, 7, 132),
        Reindeer::new("Cupid".to_string(), 12, 4, 43),
        Reindeer::new("Donner".to_string(), 9, 5, 38),
        Reindeer::new("Dasher".to_string(), 10, 4, 37),
        Reindeer::new("Comet".to_string(), 3, 37, 76),
        Reindeer::new("Prancer".to_string(), 9, 12, 97),
        Reindeer::new("Dancer".to_string(), 37, 1, 36),
    ]
}

/// Optimized mathematical calculation that avoids simulation
///
/// Instead of simulating second-by-second, this calculates distance using
/// mathematical formulas based on complete cycles and remaining time.
fn calculate_distance_optimized(reindeer: &Reindeer, duration: u32) -> u32 {
    let cycle_length = reindeer.cycle_length();
    let distance_per_cycle = reindeer.distance_per_cycle();

    // Calculate complete cycles
    let complete_cycles = duration / cycle_length;
    let remaining_time = duration % cycle_length;

    // Distance from complete cycles
    let complete_cycle_distance = complete_cycles * distance_per_cycle;

    // Distance from partial final cycle (only if still in flight phase)
    let final_distance = if remaining_time <= reindeer.flight_time() {
        remaining_time * reindeer.speed()
    } else {
        reindeer.flight_time() * reindeer.speed()
    };

    complete_cycle_distance + final_distance
}

/// Performance comparison between simulation and mathematical approaches
fn performance_comparison() {
    println!("=== PERFORMANCE COMPARISON ===\n");

    let reindeer = get_example_reindeer();
    let duration = 2503;

    // Test mathematical approach
    let start = std::time::Instant::now();
    for r in &reindeer {
        let _distance = calculate_distance_optimized(r, duration);
    }
    let math_time = start.elapsed();

    // Test simulation approach
    let start = std::time::Instant::now();
    for r in &reindeer {
        let _distance = calculate_distance(r, duration);
    }
    let sim_time = start.elapsed();

    println!("Mathematical approach: {:?}", math_time);
    println!("Simulation approach:   {:?}", sim_time);
    println!(
        "Speedup: {:.2}x\n",
        sim_time.as_nanos() as f64 / math_time.as_nanos() as f64
    );

    // Verify both produce same results
    println!("=== ACCURACY VERIFICATION ===");
    for r in &reindeer {
        let math_result = calculate_distance_optimized(r, duration);
        let sim_result = calculate_distance(r, duration);
        println!(
            "{}: Math={}, Sim={}, Match={}",
            r.name(),
            math_result,
            sim_result,
            math_result == sim_result
        );
    }
    println!();
}

/// ASCII-based race visualization showing distance over time
fn create_race_graph(
    reindeer: &[Reindeer],
    max_time: u32,
    graph_width: usize,
    graph_height: usize,
) {
    println!("=== RACE PROGRESS GRAPH ===\n");

    // Calculate maximum distance for scaling
    let max_distance = reindeer
        .iter()
        .map(|r| calculate_distance_optimized(r, max_time))
        .max()
        .unwrap_or(0);

    println!("Time Range: 0 to {} seconds", max_time);
    println!("Distance Range: 0 to {} km", max_distance);
    println!("Graph Size: {}x{} characters\n", graph_width, graph_height);

    // Create graph grid
    let mut grid = vec![vec![' '; graph_width]; graph_height];

    // Plot each reindeer's progress
    let symbols = ['*', '#', '@', '%', '&', '+', 'x', 'o'];

    for (idx, reindeer) in reindeer.iter().enumerate() {
        let symbol = symbols[idx % symbols.len()];

        for (x, _) in (0..graph_width).enumerate() {
            let time = (x as f64 / (graph_width - 1) as f64 * max_time as f64) as u32;
            let distance = calculate_distance_optimized(reindeer, time);

            // Scale distance to graph height
            let y = if max_distance > 0 {
                ((graph_height - 1) as f64 * (1.0 - distance as f64 / max_distance as f64)) as usize
            } else {
                graph_height - 1
            };

            if y < graph_height {
                grid[y][x] = symbol;
            }
        }
    }

    // Print graph with axes
    for (y, row) in grid.iter().enumerate() {
        let distance_label = if max_distance > 0 {
            (max_distance as f64 * (1.0 - y as f64 / (graph_height - 1) as f64)) as u32
        } else {
            0
        };

        print!("{:4} |", distance_label);
        for &ch in row {
            print!("{}", ch);
        }
        println!("|");
    }

    // Print time axis
    print!("     +");
    for _ in 0..graph_width {
        print!("-");
    }
    println!("+");

    print!("      ");
    for i in 0..5 {
        let time = (max_time as f64 * i as f64 / 4.0) as u32;
        print!("{:>12}", time);
    }
    println!("\n      Time (seconds)\n");

    // Print legend
    println!("Legend:");
    for (idx, reindeer) in reindeer.iter().enumerate() {
        let symbol = symbols[idx % symbols.len()];
        println!("  {} = {}", symbol, reindeer.name());
    }
    println!();
}

/// Detailed analysis showing cycle calculations
fn cycle_analysis() {
    println!("=== DETAILED CYCLE ANALYSIS ===\n");

    let reindeer = get_example_reindeer();
    let duration = 1000u32; // Use shorter duration for clearer analysis

    for r in &reindeer {
        println!("📊 {} Analysis:", r.name());
        println!("   Speed: {} km/s", r.speed());
        println!("   Flight time: {} seconds", r.flight_time());
        println!("   Rest time: {} seconds", r.rest_time());

        let cycle_length = r.cycle_length();
        let distance_per_cycle = r.distance_per_cycle();
        let complete_cycles = duration / cycle_length;
        let remaining_time = duration % cycle_length;

        println!("   Cycle length: {} seconds", cycle_length);
        println!("   Distance per cycle: {} km", distance_per_cycle);
        println!("   Complete cycles in {}s: {}", duration, complete_cycles);
        println!("   Remaining time: {} seconds", remaining_time);

        let final_flight_time = remaining_time.min(r.flight_time());
        let final_distance = final_flight_time * r.speed();
        let total_distance = complete_cycles * distance_per_cycle + final_distance;

        println!("   Final flight time: {} seconds", final_flight_time);
        println!("   Final cycle distance: {} km", final_distance);
        println!("   📍 TOTAL DISTANCE: {} km", total_distance);

        // Verify with both methods
        let sim_distance = calculate_distance(r, duration);
        let opt_distance = calculate_distance_optimized(r, duration);
        println!("   ✅ Simulation verification: {} km", sim_distance);
        println!("   ✅ Optimized verification: {} km", opt_distance);
        println!("   ✅ Methods match: {}\n", sim_distance == opt_distance);
    }
}

/// Lead changes analysis for Part 2 understanding
fn lead_changes_analysis() {
    println!("=== LEAD CHANGES ANALYSIS ===\n");

    let reindeer = get_example_reindeer();
    let max_time = 50u32; // Short duration to see lead changes clearly

    let mut lead_history: Vec<(u32, String, u32)> = Vec::new(); // (time, leader, distance)
    let mut current_leader = String::new();

    for second in 1..=max_time {
        let distances: Vec<(String, u32)> = reindeer
            .iter()
            .map(|r| {
                (
                    r.name().to_string(),
                    calculate_distance_optimized(r, second),
                )
            })
            .collect();

        let max_distance = distances.iter().map(|(_, d)| *d).max().unwrap_or(0);
        let leaders: Vec<String> = distances
            .iter()
            .filter(|(_, d)| *d == max_distance)
            .map(|(name, _)| name.clone())
            .collect();

        let leader_str = leaders.join(" & ");

        if leader_str != current_leader {
            lead_history.push((second, leader_str.clone(), max_distance));
            current_leader = leader_str;
        }
    }

    println!("Lead changes during first {} seconds:", max_time);
    for (time, leader, distance) in lead_history {
        println!(
            "  Second {}: {} takes lead with {} km",
            time, leader, distance
        );
    }
    println!();

    // Show final standings
    println!("Final standings at {} seconds:", max_time);
    let mut final_distances: Vec<(String, u32)> = reindeer
        .iter()
        .map(|r| {
            (
                r.name().to_string(),
                calculate_distance_optimized(r, max_time),
            )
        })
        .collect();

    final_distances.sort_by(|a, b| b.1.cmp(&a.1));

    for (pos, (name, distance)) in final_distances.iter().enumerate() {
        println!("  {}. {}: {} km", pos + 1, name, distance);
    }
    println!();
}

/// Create comprehensive tables using actual problem input
fn actual_problem_analysis() {
    println!("=== ACTUAL PROBLEM INPUT ANALYSIS ===\n");

    let reindeer = get_actual_reindeer();
    let time_points = vec![
        200, 400, 600, 800, 1000, 1200, 1400, 1600, 1800, 2000, 2200, 2503,
    ];

    // Part 1: Distance Analysis
    println!("🏁 PART 1: DISTANCE RACE ANALYSIS");
    println!("{}", "=".repeat(90));

    // Header row
    print!("{:<12}", "Reindeer");
    for &time in &time_points {
        print!("{:>8}s", time);
    }
    println!();

    // Separator
    print!("{}", "-".repeat(12));
    for _ in &time_points {
        print!("{}", "-".repeat(9));
    }
    println!();

    // Data rows for each reindeer
    let mut reindeer_distances: Vec<Vec<u32>> = Vec::new();

    for r in &reindeer {
        print!("{:<12}", r.name());
        let mut distances = Vec::new();

        for &time in &time_points {
            let distance = calculate_distance_optimized(r, time);
            distances.push(distance);
            print!("{:>8}", distance);
        }
        println!();
        reindeer_distances.push(distances);
    }

    // Winner row
    print!("{}", "-".repeat(12));
    for _ in &time_points {
        print!("{}", "-".repeat(9));
    }
    println!();

    print!("{:<12}", "WINNER");
    for (i, &_time) in time_points.iter().enumerate() {
        let max_distance = reindeer_distances
            .iter()
            .map(|distances| distances[i])
            .max()
            .unwrap_or(0);
        let winners: Vec<&str> = reindeer
            .iter()
            .zip(reindeer_distances.iter())
            .filter(|(_, distances)| distances[i] == max_distance)
            .map(|(r, _)| r.name())
            .collect();

        if winners.len() == 1 {
            print!("{:>8}", max_distance);
        } else {
            print!("{:>7}T", max_distance); // T for tie
        }
    }
    println!("\n");

    // Part 2: Points Analysis
    println!("🏆 PART 2: POINTS RACE ANALYSIS");
    println!("{}", "=".repeat(90));

    // Calculate points at each time interval
    let mut points_data: Vec<Vec<u32>> = vec![vec![0; time_points.len()]; reindeer.len()];

    for (time_idx, &time) in time_points.iter().enumerate() {
        // Calculate cumulative points up to this time using our existing function
        let mut current_points = vec![0u32; reindeer.len()];

        for second in 1..=time {
            let distances: Vec<u32> = reindeer
                .iter()
                .map(|r| calculate_distance_optimized(r, second))
                .collect();

            let max_distance = distances.iter().max().unwrap_or(&0);

            // Award points to all reindeer at max distance this second
            for (reindeer_idx, distance) in distances.iter().enumerate() {
                if distance == max_distance {
                    current_points[reindeer_idx] += 1;
                }
            }
        }

        // Store the points for this time interval
        for (reindeer_idx, &points) in current_points.iter().enumerate() {
            points_data[reindeer_idx][time_idx] = points;
        }
    }

    // Header row
    print!("{:<12}", "Reindeer");
    for &time in &time_points {
        print!("{:>8}s", time);
    }
    println!();

    // Separator
    print!("{}", "-".repeat(12));
    for _ in &time_points {
        print!("{}", "-".repeat(9));
    }
    println!();

    // Data rows for points
    for (idx, r) in reindeer.iter().enumerate() {
        print!("{:<12}", r.name());
        for points in &points_data[idx] {
            print!("{:>8}", points);
        }
        println!();
    }

    // Winner row for points
    print!("{}", "-".repeat(12));
    for _ in &time_points {
        print!("{}", "-".repeat(9));
    }
    println!();

    print!("{:<12}", "WINNER");
    for time_idx in 0..time_points.len() {
        let max_points = points_data
            .iter()
            .map(|points| points[time_idx])
            .max()
            .unwrap_or(0);
        print!("{:>8}", max_points);
    }
    println!();

    // Summary
    println!("\n📊 SUMMARY:");
    let final_distances: Vec<(String, u32)> = reindeer
        .iter()
        .map(|r| (r.name().to_string(), calculate_distance_optimized(r, 2503)))
        .collect();

    let final_points = simulate_race_with_points(&reindeer, 2503);

    let distance_winner = final_distances.iter().max_by_key(|(_, d)| d).unwrap();

    println!(
        "Part 1 Winner: {} with {} km",
        distance_winner.0, distance_winner.1
    );
    println!(
        "Part 2 Winner: {} with {} points",
        reindeer
            .iter()
            .zip(points_data.iter())
            .max_by_key(|(_, points)| points[time_points.len() - 1])
            .map(|(r, _)| r.name())
            .unwrap_or("Unknown"),
        final_points
    );
}

/// Racing state at specific moments
fn racing_state_snapshots() {
    println!("=== RACING STATE SNAPSHOTS ===\n");

    let reindeer = get_example_reindeer();
    let snapshot_times = vec![10, 50, 100, 300, 1000];

    for &time in &snapshot_times {
        println!("📸 Snapshot at {} seconds:", time);

        for r in &reindeer {
            let distance = calculate_distance_optimized(r, time);
            let is_flying = r.is_flying_at(time);
            let cycle_pos = (time - 1) % r.cycle_length() + 1;
            let status = if is_flying { "FLYING" } else { "RESTING" };

            println!(
                "  {}: {} km, {} (cycle position: {}/{})",
                r.name(),
                distance,
                status,
                cycle_pos,
                r.cycle_length()
            );
        }
        println!();
    }
}

fn main() {
    println!("🦌 AoC 2015 Day 14: Reindeer Olympics Analysis\n");
    println!("{}", "=".repeat(60));

    // 1. Performance comparison
    performance_comparison();

    // 2. Detailed cycle analysis
    cycle_analysis();

    // 3. Visual race graph
    let reindeer = get_example_reindeer();
    create_race_graph(&reindeer, 300, 60, 20);

    // 4. Lead changes analysis
    lead_changes_analysis();

    // 5. Racing state snapshots
    racing_state_snapshots();

    // 6. Actual problem analysis with comprehensive tables
    actual_problem_analysis();

    println!("🎯 Analysis complete! This demonstrates:");
    println!("  • Mathematical optimization vs simulation");
    println!("  • Visual race progression analysis");
    println!("  • Lead change dynamics for Part 2");
    println!("  • Detailed cycle behavior understanding");

    println!("\n📈 Future Enhancement Ideas:");
    println!("  🎮 2D Real-Time Race Scroller:");
    println!("    • Side-scrolling animation with reindeer sprites");
    println!("    • Flight/rest animations, speed trails, live leaderboard");
    println!("    • Playback controls, camera following, victory celebrations");
    println!("    • Perfect for education - makes the algorithm intuitive!");
    println!("    • Frameworks: macroquad, bevy, or ggez (see comments above)");
    println!();
    println!("  📊 Static Plotting with `plotters` crate:");
    println!("    • Add to Cargo.toml: plotters = \"0.3\"");
    println!("    • Use the example code in the comments below");
    println!("    • Generates PNG/SVG files with publication-quality graphs");
    println!("    • Perfect for detailed analysis and presentations!");
}

// ============================================================================
// Example plotters integration (requires: plotters = "0.3" in Cargo.toml)
// ============================================================================
//
// ```rust
// use plotters::prelude::*;
//
// fn create_plotters_graph(reindeer: &[Reindeer], max_time: u32) -> Result<(), Box<dyn std::error::Error>> {
//     let root = BitMapBackend::new("reindeer_race.png", (800, 600)).into_drawing_area();
//     root.fill(&WHITE)?;
//
//     let max_distance = reindeer.iter()
//         .map(|r| calculate_distance_optimized(r, max_time))
//         .max()
//         .unwrap_or(0);
//
//     let mut chart = ChartBuilder::on(&root)
//         .caption("Reindeer Race Progress", ("Arial", 40).into_font())
//         .margin(10)
//         .x_label_area_size(40)
//         .y_label_area_size(50)
//         .build_cartesian_2d(0u32..max_time, 0u32..max_distance)?;
//
//     chart.configure_mesh()
//         .x_desc("Time (seconds)")
//         .y_desc("Distance (km)")
//         .draw()?;
//
//     let colors = [&RED, &BLUE, &GREEN, &MAGENTA];
//
//     for (idx, reindeer) in reindeer.iter().enumerate() {
//         let color = colors[idx % colors.len()];
//         let series: Vec<(u32, u32)> = (0..=max_time)
//             .step_by(max_time as usize / 100)
//             .map(|t| (t, calculate_distance_optimized(reindeer, t)))
//             .collect();
//
//         chart.draw_series(LineSeries::new(series, color))?
//             .label(reindeer.name())
//             .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], color));
//     }
//
//     chart.configure_series_labels().draw()?;
//     root.present()?;
//
//     println!("Graph saved to reindeer_race.png");
//     Ok(())
// }
// ```

// ============================================================================
// Future Project: 2D Real-Time Race Scroller Framework Example
// ============================================================================
//
// ```rust
// // Add to Cargo.toml: macroquad = "0.4"
// use macroquad::prelude::*;
//
// struct ReindeerSprite {
//     reindeer: Reindeer,
//     x: f32,
//     y: f32,
//     is_flying_animation: bool,
//     wing_flap_timer: f32,
//     speed_trail: Vec<Vec2>,
// }
//
// struct RaceVisualization {
//     reindeer_sprites: Vec<ReindeerSprite>,
//     current_time: f32,
//     time_speed: f32,  // Multiplier for animation speed
//     camera_x: f32,
//     paused: bool,
//     leaderboard: Vec<(String, u32, u32)>, // name, distance, points
// }
//
// impl RaceVisualization {
//     fn update(&mut self, dt: f32) {
//         if !self.paused {
//             self.current_time += dt * self.time_speed;
//
//             // Update each reindeer's position and animation
//             for sprite in &mut self.reindeer_sprites {
//                 let distance = calculate_distance_optimized(&sprite.reindeer, self.current_time as u32);
//                 sprite.x = distance as f32 * 0.1; // Scale for screen
//
//                 // Update flight/rest animation state
//                 sprite.is_flying_animation = sprite.reindeer.is_flying_at(self.current_time as u32);
//
//                 if sprite.is_flying_animation {
//                     sprite.wing_flap_timer += dt * 10.0; // Fast wing flapping
//                     // Add to speed trail
//                     sprite.speed_trail.push(vec2(sprite.x, sprite.y));
//                     if sprite.speed_trail.len() > 20 {
//                         sprite.speed_trail.remove(0);
//                     }
//                 } else {
//                     sprite.speed_trail.clear(); // No trail when resting
//                 }
//             }
//
//             // Update camera to follow leader
//             let leader_x = self.reindeer_sprites.iter()
//                 .map(|s| s.x)
//                 .max_by(|a, b| a.partial_cmp(b).unwrap())
//                 .unwrap_or(0.0);
//             self.camera_x = leader_x - screen_width() * 0.3;
//         }
//     }
//
//     fn draw(&self) {
//         clear_background(SKYBLUE);
//
//         // Draw scrolling background with distance markers
//         for i in 0..50 {
//             let marker_x = i as f32 * 100.0 * 0.1 - self.camera_x;
//             if marker_x > -50.0 && marker_x < screen_width() + 50.0 {
//                 draw_line(marker_x, 0.0, marker_x, screen_height(), 2.0, DARKGRAY);
//                 draw_text(&format!("{}km", i * 100), marker_x + 5.0, 30.0, 20.0, BLACK);
//             }
//         }
//
//         // Draw each reindeer sprite
//         for (idx, sprite) in self.reindeer_sprites.iter().enumerate() {
//             let screen_x = sprite.x - self.camera_x;
//             let screen_y = 100.0 + idx as f32 * 60.0;
//
//             // Draw speed trail
//             for (i, trail_pos) in sprite.speed_trail.iter().enumerate() {
//                 let trail_screen_x = trail_pos.x - self.camera_x;
//                 let alpha = i as f32 / sprite.speed_trail.len() as f32;
//                 draw_circle(trail_screen_x, screen_y, 3.0, Color::new(1.0, 1.0, 0.0, alpha * 0.5));
//             }
//
//             // Draw reindeer sprite
//             let color = if sprite.is_flying_animation { BROWN } else { DARKBROWN };
//             draw_circle(screen_x, screen_y, 15.0, color);
//
//             // Draw wings (animated if flying)
//             if sprite.is_flying_animation {
//                 let wing_offset = (sprite.wing_flap_timer.sin() * 5.0).abs();
//                 draw_circle(screen_x - 10.0, screen_y - wing_offset, 8.0, LIGHTGRAY);
//                 draw_circle(screen_x + 10.0, screen_y - wing_offset, 8.0, LIGHTGRAY);
//             } else {
//                 // Draw "ZZZ" when resting
//                 draw_text("ZZZ", screen_x + 20.0, screen_y - 10.0, 16.0, PURPLE);
//             }
//
//             // Draw name tag
//             draw_text(&sprite.reindeer.name(), screen_x - 20.0, screen_y + 30.0, 16.0, BLACK);
//         }
//
//         // Draw live leaderboard overlay
//         draw_rectangle(10.0, 10.0, 300.0, 200.0, Color::new(0.0, 0.0, 0.0, 0.7));
//         draw_text("🏆 Live Leaderboard", 20.0, 35.0, 20.0, WHITE);
//         draw_text(&format!("Time: {:.1}s", self.current_time), 20.0, 60.0, 16.0, YELLOW);
//
//         // Show top 3 positions
//         for i in 0..3.min(self.leaderboard.len()) {
//             let (name, distance, points) = &self.leaderboard[i];
//             draw_text(&format!("{}. {}: {}km", i + 1, name, distance),
//                      20.0, 85.0 + i as f32 * 20.0, 14.0, WHITE);
//         }
//
//         // Draw controls
//         draw_text("Controls: SPACE=Pause, ↑↓=Speed, ←→=Camera", 10.0, screen_height() - 20.0, 16.0, WHITE);
//     }
// }
//
// #[macroquad::main("Reindeer Race 2D")]
// async fn main() {
//     let mut race = RaceVisualization::new(get_actual_reindeer());
//
//     loop {
//         // Handle input
//         if is_key_pressed(KeyCode::Space) {
//             race.paused = !race.paused;
//         }
//         if is_key_down(KeyCode::Up) {
//             race.time_speed *= 1.01;
//         }
//         if is_key_down(KeyCode::Down) {
//             race.time_speed /= 1.01;
//         }
//
//         race.update(get_frame_time());
//         race.draw();
//
//         next_frame().await;
//     }
// }
// ```
//
// This framework would create an incredibly engaging and educational visualization!
// Features: Real-time animation, interactive controls, visual flight/rest cycles,
// speed trails, live leaderboard, and smooth camera following. Perfect for
// understanding the algorithm dynamics and making AoC problems come alive! 🎮
