//! # Day 7: Some Assembly Required
//! 
//! **Circuit simulation with bitwise logic gates and wire connections.**
//! 
//! ## 📋 **Part 1: Circuit Evaluation**
//! 
//! Simulate Bobby's circuit kit with:
//! - **16-bit signals** (0 to 65535) on named wires
//! - **Bitwise operations**: AND, OR, NOT, LSHIFT, RSHIFT
//! - **Wire connections** with dependency resolution
//! - **Signal propagation** until all wires have values
//! 
//! ## 🎯 **Problem Analysis**
//! 
//! **Input Format**:
//! ```
//! 123 -> x              // Direct value assignment
//! x AND y -> z          // Bitwise AND gate
//! p LSHIFT 2 -> q       // Left shift operation
//! NOT e -> f            // Bitwise NOT gate  
//! ```
//! 
//! **Key Challenges**:
//! - **Dependency resolution**: Wires depend on other wires
//! - **Lazy evaluation**: Gates only activate when inputs are ready
//! - **Memoization**: Avoid recalculating wire values
//! - **16-bit arithmetic**: Handle overflow/underflow correctly
//! 
//! ## 🧪 **Test-Driven Development Approach**
//! 
//! 1. **Parse instructions** into structured representations
//! 2. **Build dependency graph** between wires
//! 3. **Implement bitwise operations** (AND, OR, NOT, shifts)
//! 4. **Resolve wire values** through recursive evaluation
//! 5. **Handle memoization** to avoid redundant calculations
//! 
//! ## 🚀 **Implementation Strategy**
//! 
//! Following **Mission6 Grid** and **Mission5 HashMap** patterns for professional code quality.

use anyhow::Result;
use std::collections::HashMap;

/// Represents different types of inputs to a gate or wire
#[derive(Debug, Clone, PartialEq)]
pub enum WireInput {
    Direct(u16),  //numeric value
    Wire(String)  //reference to another wire
}

/// Represents different bitwise operations available
#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    // TODO: Define operations:
    Assign(WireInput),  // Direct assignment (value -> wire)
    And(WireInput, WireInput),  // Bitwise AND (input1 AND input2 -> wire)
    Or(WireInput, WireInput),   // Bitwise OR (input1 OR input2 -> wire)
    Not(WireInput),   // Bitwise NOT (NOT input -> wire)
    LShift(WireInput, WireInput),  // Left shift (input LSHIFT amount -> wire)
    RShift(WireInput, WireInput)   // Right shift (input RSHIFT amount -> wire)
}

/// Represents a single instruction in the circuit
#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub operation: Operation,
    pub output_wire: String
}

/// Circuit simulator with memoization
#[derive(Debug)]
pub struct Circuit {
    // TODO: Define circuit state:
    instructions: HashMap<String, Instruction>, // (wire -> instruction)
    memo: HashMap<String, u16> // (wire -> cached value)
}

impl Circuit {
    /// Create a new empty circuit
    pub fn new() -> Self {
        Circuit {
            instructions: HashMap::new(),
            memo: HashMap::new(),
        }
    }
    
    /// Parse and add an instruction to the circuit
    pub fn add_instruction(&mut self, line: &str) -> Result<()> {
        // TODO: Parse instruction and add to circuit
        // Examples to handle:
        // - "123 -> x"
        // - "x AND y -> z"  
        // - "NOT e -> f"
        // - "p LSHIFT 2 -> q"
        todo!("Parse and store instruction: {}", line)
    }
    
    /// Get the signal value for a specific wire
    pub fn get_wire_value(&mut self, wire_name: &str) -> Result<u16> {
        // TODO: Implement recursive evaluation with memoization
        // 1. Check if value already computed (memoization)
        // 2. If not, look up instruction for this wire
        // 3. Recursively evaluate input dependencies  
        // 4. Apply operation to get result
        // 5. Cache result and return
        todo!("Evaluate wire value for: {}", wire_name)
    }
}

/// Parse a wire input (either numeric value or wire reference)
fn parse_wire_input(input: &str) -> Result<WireInput> {
    use anyhow::anyhow;
    
    let trimmed = input.trim();
    
    // Check if input is empty
    if trimmed.is_empty() {
        return Err(anyhow!("Wire input cannot be empty"));
    }
    
    // Try to parse as a number first
    if let Ok(value) = trimmed.parse::<u16>() {
        Ok(WireInput::Direct(value))
    } else {
        // If it's not a number, treat as wire name
        // Basic validation: wire names should be alphanumeric
        if trimmed.chars().all(|c| c.is_alphanumeric()) {
            Ok(WireInput::Wire(trimmed.to_string()))
        } else {
            Err(anyhow!("Invalid wire name: '{}'. Wire names must be alphanumeric.", trimmed))
        }
    }
}

/// Parse a complete instruction line
fn parse_instruction(line: &str) -> Result<Instruction> {
    use anyhow::anyhow;
    
    let trimmed = line.trim();
    
    // Split by " -> " to get operation and target wire
    let parts: Vec<&str> = trimmed.split(" -> ").collect();
    if parts.len() != 2 {
        return Err(anyhow!("Invalid instruction format: '{}'", line));
    }
    
    let operation_str = parts[0].trim();
    let output_wire = parts[1].trim().to_string();
    
    // Parse the operation part to determine type and inputs
    let operation = if operation_str.contains(" AND ") {
        // Binary AND: "x AND y"
        let and_parts: Vec<&str> = operation_str.split(" AND ").collect();
        if and_parts.len() != 2 {
            return Err(anyhow!("Invalid AND operation: '{}'", operation_str));
        }
        let input1 = parse_wire_input(and_parts[0])?;  // Call 1
        let input2 = parse_wire_input(and_parts[1])?;  // Call 2
        Operation::And(input1, input2)
        
    } else if operation_str.contains(" OR ") {
        // Binary OR: "x OR y"
        let or_parts: Vec<&str> = operation_str.split(" OR ").collect();
        if or_parts.len() != 2 {
            return Err(anyhow!("Invalid OR operation: '{}'", operation_str));
        }
        let input1 = parse_wire_input(or_parts[0])?;   // Call 1
        let input2 = parse_wire_input(or_parts[1])?;   // Call 2
        Operation::Or(input1, input2)
        
    } else if operation_str.contains(" LSHIFT ") {
        // Left shift: "p LSHIFT 2"
        let shift_parts: Vec<&str> = operation_str.split(" LSHIFT ").collect();
        if shift_parts.len() != 2 {
            return Err(anyhow!("Invalid LSHIFT operation: '{}'", operation_str));
        }
        let input = parse_wire_input(shift_parts[0])?;    // Call 1
        let amount = parse_wire_input(shift_parts[1])?;   // Call 2
        Operation::LShift(input, amount)
        
    } else if operation_str.contains(" RSHIFT ") {
        // Right shift: "x RSHIFT 3"
        let shift_parts: Vec<&str> = operation_str.split(" RSHIFT ").collect();
        if shift_parts.len() != 2 {
            return Err(anyhow!("Invalid RSHIFT operation: '{}'", operation_str));
        }
        let input = parse_wire_input(shift_parts[0])?;    // Call 1
        let amount = parse_wire_input(shift_parts[1])?;   // Call 2
        Operation::RShift(input, amount)
        
    } else if operation_str.starts_with("NOT ") {
        // Unary NOT: "NOT e"
        let not_input = operation_str.strip_prefix("NOT ").unwrap().trim();
        let input = parse_wire_input(not_input)?;         // Call 1
        Operation::Not(input)
        
    } else {
        // Direct assignment: "123" or "x"
        let input = parse_wire_input(operation_str)?;     // Call 1
        Operation::Assign(input)
    };
    
    Ok(Instruction {
        operation,
        output_wire,
    })
}

/// Apply a bitwise operation to input values
fn apply_operation(operation: &Operation, inputs: &[u16]) -> Result<u16> {
    use anyhow::anyhow;
    
    let result = match operation {
        Operation::Assign(_) => {
            if inputs.len() != 1 {
                return Err(anyhow!("Assign operation requires exactly 1 input, got {}", inputs.len()));
            }
            inputs[0]
        }
        Operation::And(_, _) => {
            if inputs.len() != 2 {
                return Err(anyhow!("And operation requires exactly 2 inputs, got {}", inputs.len()));
            }
            inputs[0] & inputs[1]
        }
        Operation::Or(_, _) => {
            if inputs.len() != 2 {
                return Err(anyhow!("Or operation requires exactly 2 inputs, got {}", inputs.len()));
            }
            inputs[0] | inputs[1]
        }
        Operation::Not(_) => {
            if inputs.len() != 1 {
                return Err(anyhow!("Not operation requires exactly 1 input, got {}", inputs.len()));
            }
            !inputs[0]
        }
        Operation::LShift(_, _) => {
            if inputs.len() != 2 {
                return Err(anyhow!("LShift operation requires exactly 2 inputs, got {}", inputs.len()));
            }
            inputs[0] << inputs[1]
        }
        Operation::RShift(_, _) => {
            if inputs.len() != 2 {
                return Err(anyhow!("RShift operation requires exactly 2 inputs, got {}", inputs.len()));
            }
            inputs[0] >> inputs[1]
        }
    };
    
    Ok(result)
}

/// Day 07 Part 1: Find the signal provided to wire 'a'
pub fn solve_part1(_input: &str) -> Result<String> {
    // TODO: Main solving logic
    // 1. Create new circuit
    // 2. Parse all instructions and add to circuit
    // 3. Evaluate wire 'a' to get final signal value
    // 4. Return as string
    todo!("Solve Part 1: Find signal on wire 'a'")
}

/// Day 07 Part 2: [REDACTED - No spoilers!]
pub fn solve_part2(_input: &str) -> Result<String> {
    // TODO: Part 2 will be implemented after Part 1 is complete
    // No spoilers - wait for Part 2 requirements!
    todo!("Part 2 implementation - TBD")
}

// ============================================================================
// IMPLEMENTATION GUIDANCE NOTES (Remove these when implementing)
// ============================================================================

// 📝 **Key Implementation Tips**:
//
// 1. **Memoization Pattern**: 
//    Use HashMap<String, u16> to cache computed wire values
//    This prevents infinite recursion and improves performance
//
// 2. **16-bit Arithmetic**:
//    All values are 0-65535 (u16 range)
//    Rust handles overflow automatically for u16
//
// 3. **Dependency Resolution**:
//    Use recursive evaluation similar to expression trees
//    Check memoization first, then compute if needed
//
// 4. **Parsing Strategy**:
//    Split on " -> " first to get operation and target
//    Then parse operation side based on keywords (AND, OR, NOT, etc.)
//
// 5. **Error Handling**:
//    Use anyhow::Result for parsing errors
//    Handle undefined wires gracefully
//
// 6. **Testing Approach**:
//    Start with simple direct assignments: "123 -> x"
//    Then add unary operations: "NOT x -> y"  
//    Finally add binary operations: "x AND y -> z"
//
// 7. **Example Expected Flow**:
//    circuit.add_instruction("123 -> x")?;
//    circuit.add_instruction("456 -> y")?; 
//    circuit.add_instruction("x AND y -> d")?;
//    let result = circuit.get_wire_value("d")?; // Should be 72
//
// 8. **Integration with Learning Tracks**:
//    - Uses HashMap for memoization (Mission 5 patterns)
//    - String parsing similar to Day 6 command parsing
//    - Recursive evaluation demonstrates ownership patterns
//
// 9. **Performance Considerations**:
//    Memoization is crucial - some circuits may have deep dependencies
//    Consider using RefCell if borrowing becomes complex
//
// 10. **Real AoC Integration**:
//     The actual input will have hundreds of instructions
//     Final goal is typically wire 'a' value
//     Part 2 usually modifies one wire and asks for recalculation
