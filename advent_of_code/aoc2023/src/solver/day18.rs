//! # Day 18: Lavaduct Lagoon
//!
//! ## Part 1
//! Calculate the volume of a lagoon by following dig instructions.
//! Each instruction specifies direction (U/D/L/R), distance, and edge color.
//! We need to find the total area including both the trench boundary and interior.
//!
//! ## Algorithm
//! - **Approach**: Shoelace Formula + Pick's Theorem for polygon area
//! - **Shoelace**: Compute interior area from polygon vertices
//! - **Pick's Theorem**: A = I + B/2 - 1, where A=area, I=interior points, B=boundary points
//! - **Total Area**: Shoelace_Area + Perimeter/2 + 1
//! - **Complexity**: O(n) where n = number of instructions
//!
//! ## Mathematical Foundation
//!
//! See `zettelkasten/math-foundations/computational-geometry-basics.md` for detailed theory.
//!
//! **Shoelace Formula**:
//! $$\text{Area} = \frac{1}{2} \left| \sum_{i=0}^{n-1} (x_i \cdot y_{i+1} - x_{i+1} \cdot y_i) \right|$$
//!
//! **Pick's Theorem**: For lattice polygons (grid-aligned vertices):
//! $$A = I + \frac{B}{2} - 1$$
//! Rearranged: $$I = A - \frac{B}{2} + 1$$
//!
//! **Total cells**: $$I + B = A + \frac{B}{2} + 1$$

use anyhow::Result;

/// Dig instruction from input
#[derive(Debug, Clone)]
struct Instruction {
    direction: char,
    distance: i64,
    color: String,
}

impl Instruction {
    /// Parse a line like "R 6 (#70c710)"
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

    /// Decode hex color into real instruction (Part 2)
    /// Format: #XXXXXD where XXXXX is 5-digit hex distance, D is direction (0=R, 1=D, 2=L, 3=U)
    /// Example: #70c710 -> R 461937
    fn decode_from_hex(&self) -> Result<Instruction> {
        if self.color.len() != 6 {
            anyhow::bail!("Invalid hex color length: {}", self.color);
        }

        // First 5 hex digits = distance
        let distance_hex = &self.color[0..5];
        let distance = i64::from_str_radix(distance_hex, 16)?;

        // Last hex digit = direction (0=R, 1=D, 2=L, 3=U)
        let direction_digit = &self.color[5..6];
        let direction = match direction_digit {
            "0" => 'R',
            "1" => 'D',
            "2" => 'L',
            "3" => 'U',
            _ => anyhow::bail!("Invalid direction digit: {}", direction_digit),
        };

        Ok(Instruction {
            direction,
            distance,
            color: self.color.clone(),
        })
    }
}

/// Parse all instructions from input
fn parse_input(input: &str) -> Result<Vec<Instruction>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(Instruction::parse)
        .collect()
}

/// Trace polygon vertices by following dig instructions
/// Returns (vertices, perimeter_length)
fn trace_polygon(instructions: &[Instruction]) -> (Vec<(i64, i64)>, i64) {
    let mut vertices = vec![(0, 0)];
    let mut current = (0i64, 0i64);
    let mut perimeter = 0;

    for instr in instructions {
        // Move current position based on direction
        current = match instr.direction {
            'U' => (current.0, current.1 - instr.distance), // Up = negative Y
            'D' => (current.0, current.1 + instr.distance), // Down = positive Y
            'L' => (current.0 - instr.distance, current.1), // Left = negative X
            'R' => (current.0 + instr.distance, current.1), // Right = positive X
            _ => current,
        };

        vertices.push(current);
        perimeter += instr.distance;
    }

    (vertices, perimeter)
}

/// Calculate polygon area using Shoelace formula
/// Formula: Area = 1/2 * |Σ(x_i * y_{i+1} - x_{i+1} * y_i)|
fn shoelace_area(vertices: &[(i64, i64)]) -> i64 {
    let n = vertices.len();
    let mut sum = 0i64;

    for i in 0..n - 1 {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[i + 1];
        sum += x1 * y2 - x2 * y1;
    }

    // Close the polygon (last vertex to first)
    let (x1, y1) = vertices[n - 1];
    let (x2, y2) = vertices[0];
    sum += x1 * y2 - x2 * y1;

    sum.abs() / 2
}

pub fn solve_part1(input: &str) -> Result<String> {
    let instructions = parse_input(input)?;

    // Step 1: Trace polygon vertices and calculate perimeter
    let (vertices, perimeter) = trace_polygon(&instructions);

    // Step 2: Calculate interior area using Shoelace formula
    let shoelace = shoelace_area(&vertices);

    // Step 3: Apply Pick's theorem to get total area
    // Total = Shoelace_Area + Perimeter/2 + 1
    // Derivation:
    //   Pick's Theorem: A = I + B/2 - 1  (where A=area, I=interior, B=boundary)
    //   Rearrange: I = A - B/2 + 1
    //   Total cells = I + B = (A - B/2 + 1) + B = A + B/2 + 1
    let total_area = shoelace + perimeter / 2 + 1;

    Ok(total_area.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let instructions = parse_input(input)?;

    // Decode hex colors into real instructions
    let decoded: Vec<Instruction> = instructions
        .iter()
        .map(|instr| instr.decode_from_hex())
        .collect::<Result<Vec<_>>>()?;

    // Use the same algorithm as Part 1 - still O(n)!
    let (vertices, perimeter) = trace_polygon(&decoded);
    let shoelace = shoelace_area(&vertices);
    let total_area = shoelace + perimeter / 2 + 1;

    Ok(total_area.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    #[test]
    fn test_parse_instruction() {
        let instruction = Instruction::parse("R 6 (#70c710)").unwrap();
        assert_eq!(instruction.direction, 'R');
        assert_eq!(instruction.distance, 6);
        assert_eq!(instruction.color, "70c710");
    }

    #[test]
    fn test_decode_hex() {
        // #70c710 = R 461937
        let instruction = Instruction::parse("R 6 (#70c710)").unwrap();
        let decoded = instruction.decode_from_hex().unwrap();
        assert_eq!(decoded.direction, 'R');
        assert_eq!(decoded.distance, 461937);

        // #0dc571 = D 56407
        let instruction = Instruction::parse("D 5 (#0dc571)").unwrap();
        let decoded = instruction.decode_from_hex().unwrap();
        assert_eq!(decoded.direction, 'D');
        assert_eq!(decoded.distance, 56407);

        // #8ceee2 = L 577262
        let instruction = Instruction::parse("L 5 (#8ceee2)").unwrap();
        let decoded = instruction.decode_from_hex().unwrap();
        assert_eq!(decoded.direction, 'L');
        assert_eq!(decoded.distance, 577262);

        // #caa173 = U 829975
        let instruction = Instruction::parse("U 2 (#caa173)").unwrap();
        let decoded = instruction.decode_from_hex().unwrap();
        assert_eq!(decoded.direction, 'U');
        assert_eq!(decoded.distance, 829975);
    }

    #[test]
    fn test_parse_input() {
        let instructions = parse_input(EXAMPLE).unwrap();
        assert_eq!(instructions.len(), 14);
        assert_eq!(instructions[0].direction, 'R');
        assert_eq!(instructions[0].distance, 6);
    }

    #[test]
    fn test_trace_polygon() {
        let instructions = parse_input(EXAMPLE).unwrap();
        let (vertices, perimeter) = trace_polygon(&instructions);

        // Should have 15 vertices (start + 14 instructions)
        assert_eq!(vertices.len(), 15);

        // First vertex is origin
        assert_eq!(vertices[0], (0, 0));

        // After R 6, should be at (6, 0)
        assert_eq!(vertices[1], (6, 0));

        // Perimeter should be sum of all distances
        let expected_perimeter: i64 = instructions.iter().map(|i| i.distance).sum();
        assert_eq!(perimeter, expected_perimeter);
    }

    #[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result, "62"); // Example shows 62 cubic meters
    }

    #[test]
    fn test_part2_example() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result, "952408144115"); // Example shows 952408144115 cubic meters
    }
}
