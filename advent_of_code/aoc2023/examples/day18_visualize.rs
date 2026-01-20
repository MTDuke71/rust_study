//! Day 18: Lavaduct Lagoon - Polygon Visualization
//!
//! Visualizes the Part 1 polygon shape using ASCII art and creates
//! coordinate output suitable for plotting.
//!
//! Run with: `cargo run -p aoc2023 --example day18_visualize`

use anyhow::Result;
use std::collections::HashSet;
use std::fs;

fn main() -> Result<()> {
    // Load the actual puzzle input
    let input = include_str!("../inputs/day18.txt");

    println!("{}", "=".repeat(80));
    println!("Day 18: Lavaduct Lagoon - Polygon Visualization");
    println!("{}", "=".repeat(80));
    println!();

    // Parse and get vertices
    let instructions = parse_input(input)?;
    let (vertices, perimeter) = trace_polygon(&instructions);

    println!("📊 Polygon Statistics:");
    println!("  Vertices: {}", vertices.len());
    println!("  Perimeter: {} units", perimeter);
    println!();

    // Display vertices
    println!("📍 Polygon Vertices:");
    for (i, (x, y)) in vertices.iter().enumerate() {
        println!("  {:2}. ({:3}, {:3})", i, x, y);
    }
    println!();

    // Create ASCII visualization
    println!("🎨 ASCII Visualization:");
    visualize_polygon(&vertices);
    println!();

    // Create data for external plotting
    create_plot_data(&vertices)?;

    println!("✅ Visualization complete!");
    println!("📁 Created: day18_vertices.csv");
    println!("   Use with Python/gnuplot/Excel for detailed plotting");

    Ok(())
}

/// Parse input (copy from day18.rs for standalone example)
fn parse_input(input: &str) -> Result<Vec<Instruction>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(Instruction::parse)
        .collect()
}

#[derive(Debug, Clone)]
struct Instruction {
    direction: char,
    distance: i64,
    color: String,
}

impl Instruction {
    fn parse(line: &str) -> Result<Self> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let direction = parts[0]
            .chars()
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing direction"))?;

        let distance = parts[1].parse::<i64>()?;

        let color = parts[2]
            .trim_start_matches("(#")
            .trim_end_matches(')')
            .to_string();

        Ok(Instruction {
            direction,
            distance,
            color,
        })
    }
}

fn trace_polygon(instructions: &[Instruction]) -> (Vec<(i64, i64)>, i64) {
    let mut vertices = vec![(0, 0)];
    let mut current = (0i64, 0i64);
    let mut perimeter = 0;

    for instr in instructions {
        current = match instr.direction {
            'U' => (current.0, current.1 - instr.distance),
            'D' => (current.0, current.1 + instr.distance),
            'L' => (current.0 - instr.distance, current.1),
            'R' => (current.0 + instr.distance, current.1),
            _ => current,
        };

        vertices.push(current);
        perimeter += instr.distance;
    }

    (vertices, perimeter)
}

/// Create ASCII art visualization of the polygon
fn visualize_polygon(vertices: &[(i64, i64)]) {
    // Find bounding box
    let min_x = vertices.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = vertices.iter().map(|(x, _)| *x).max().unwrap();
    let min_y = vertices.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = vertices.iter().map(|(_, y)| *y).max().unwrap();

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;

    // Build edge set
    let mut edges = HashSet::new();
    for i in 0..vertices.len() - 1 {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[i + 1];

        // Draw line from (x1, y1) to (x2, y2)
        if x1 == x2 {
            // Vertical line
            let min_y_edge = y1.min(y2);
            let max_y_edge = y1.max(y2);
            for y in min_y_edge..=max_y_edge {
                edges.insert((x1, y));
            }
        } else {
            // Horizontal line
            let min_x_edge = x1.min(x2);
            let max_x_edge = x1.max(x2);
            for x in min_x_edge..=max_x_edge {
                edges.insert((x, y1));
            }
        }
    }

    // Print grid
    println!("  ┌{}┐", "─".repeat(width));
    for y in min_y..=max_y {
        print!("  │");
        for x in min_x..=max_x {
            if edges.contains(&(x, y)) {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!("│");
    }
    println!("  └{}┘", "─".repeat(width));

    println!("\n  Dimensions: {}×{} units", width, height);
}

/// Create CSV file for external plotting
fn create_plot_data(vertices: &[(i64, i64)]) -> Result<()> {
    let mut csv = String::from("x,y\n");

    for (x, y) in vertices {
        csv.push_str(&format!("{},{}\n", x, y));
    }

    // Close the polygon
    csv.push_str(&format!("{},{}\n", vertices[0].0, vertices[0].1));

    fs::write("day18_vertices.csv", csv)?;

    // Also create Python plotting script
    let python_script = r#"#!/usr/bin/env python3
"""
Day 18 Polygon Visualization
Plots the lagoon shape from vertices data.

Usage: python day18_plot.py
"""

import matplotlib.pyplot as plt
import pandas as pd

# Read vertices
df = pd.read_csv('day18_vertices.csv')

# Create plot
fig, ax = plt.subplots(figsize=(10, 8))

# Plot polygon
ax.plot(df['x'], df['y'], 'b-', linewidth=2, label='Trench')
ax.fill(df['x'], df['y'], alpha=0.3, color='lightblue', label='Lagoon Area')

# Plot vertices
ax.scatter(df['x'], df['y'], c='red', s=50, zorder=5, label='Vertices')

# Annotations
ax.set_xlabel('X Coordinate')
ax.set_ylabel('Y Coordinate')
ax.set_title('AoC 2023 Day 18: Lavaduct Lagoon Polygon')
ax.legend()
ax.grid(True, alpha=0.3)
ax.set_aspect('equal')

# Invert y-axis to match grid convention (y increases downward)
ax.invert_yaxis()

plt.tight_layout()
plt.savefig('day18_polygon.png', dpi=150)
print("✅ Plot saved to day18_polygon.png")
plt.show()
"#;

    fs::write("day18_plot.py", python_script)?;

    println!("📊 Created Python plotting script: day18_plot.py");
    println!("   Run with: python day18_plot.py");

    Ok(())
}
