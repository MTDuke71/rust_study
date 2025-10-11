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
    Wire(String), //reference to another wire
}

/// Represents different bitwise operations available
#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    // TODO: Define operations:
    Assign(WireInput),            // Direct assignment (value -> wire)
    And(WireInput, WireInput),    // Bitwise AND (input1 AND input2 -> wire)
    Or(WireInput, WireInput),     // Bitwise OR (input1 OR input2 -> wire)
    Not(WireInput),               // Bitwise NOT (NOT input -> wire)
    LShift(WireInput, WireInput), // Left shift (input LSHIFT amount -> wire)
    RShift(WireInput, WireInput), // Right shift (input RSHIFT amount -> wire)
}

/// Represents a single instruction in the circuit
#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub operation: Operation,
    pub output_wire: String,
}

/// Circuit simulator with memoization
#[derive(Debug)]
pub struct Circuit {
    // TODO: Define circuit state:
    instructions: HashMap<String, Instruction>, // (wire -> instruction)
    memo: HashMap<String, u16>,                 // (wire -> cached value)
    debug: bool,                                // Enable debug output
}

impl Circuit {
    /// Create a new empty circuit
    pub fn new() -> Self {
        Circuit {
            instructions: HashMap::new(),
            memo: HashMap::new(),
            debug: false,
        }
    }

    /// Create a new circuit with debug enabled
    pub fn new_with_debug(debug: bool) -> Self {
        Circuit {
            instructions: HashMap::new(),
            memo: HashMap::new(),
            debug,
        }
    }

    /// Parse and add an instruction to the circuit
    pub fn add_instruction(&mut self, line: &str) -> Result<()> {
        let instruction = parse_instruction(line)?;

        if self.debug {
            println!(
                "📝 Adding instruction: {} -> {}",
                line.trim(),
                instruction.output_wire
            );
            println!("   Parsed as: {:?}", instruction.operation);
        }

        self.instructions
            .insert(instruction.output_wire.clone(), instruction);

        if self.debug {
            println!(
                "🔧 Instructions table now has {} entries",
                self.instructions.len()
            );
            self.print_instructions_table();
        }

        Ok(())
    }

    /// Get the signal value for a specific wire
    pub fn get_wire_value(&mut self, wire_name: &str) -> Result<u16> {
        use anyhow::anyhow;

        if self.debug {
            println!("🔍 Evaluating wire '{}'", wire_name);
        }

        // Check memoization first
        if let Some(&cached) = self.memo.get(wire_name) {
            if self.debug {
                println!("💾 Found cached value for '{}': {}", wire_name, cached);
            }
            return Ok(cached);
        }

        // Look up instruction
        let instruction = self
            .instructions
            .get(wire_name)
            .ok_or_else(|| anyhow!("No instruction found for wire: {}", wire_name))?
            .clone();

        if self.debug {
            println!("⚙️  Executing operation: {:?}", instruction.operation);
        }

        // Evaluate based on operation
        let result = match instruction.operation {
            Operation::Assign(input) => self.evaluate_input(&input)?,
            Operation::And(a, b) => self.evaluate_input(&a)? & self.evaluate_input(&b)?,
            Operation::Or(a, b) => self.evaluate_input(&a)? | self.evaluate_input(&b)?,
            Operation::Not(input) => !self.evaluate_input(&input)?,
            Operation::LShift(input, amount) => {
                self.evaluate_input(&input)? << self.evaluate_input(&amount)?
            }
            Operation::RShift(input, amount) => {
                self.evaluate_input(&input)? >> self.evaluate_input(&amount)?
            }
        };

        if self.debug {
            println!("✅ Wire '{}' evaluated to: {}", wire_name, result);
        }

        // Cache and return
        self.memo.insert(wire_name.to_string(), result);

        if self.debug {
            println!("💾 Cached '{}' = {}", wire_name, result);
            self.print_memo_table();
        }

        Ok(result)
    }

    /// Helper function to evaluate a WireInput (either direct value or wire reference)
    fn evaluate_input(&mut self, input: &WireInput) -> Result<u16> {
        match input {
            WireInput::Direct(value) => {
                if self.debug {
                    println!("    📊 Direct value: {}", value);
                }
                Ok(*value)
            }
            WireInput::Wire(wire_name) => {
                if self.debug {
                    println!("    🔗 Looking up wire: {}", wire_name);
                }
                self.get_wire_value(wire_name)
            }
        }
    }

    /// Print the current instructions table (debug helper)
    fn print_instructions_table(&self) {
        println!("📋 Instructions Table:");
        let mut wires: Vec<_> = self.instructions.keys().collect();
        wires.sort();
        for wire in wires {
            if let Some(instruction) = self.instructions.get(wire) {
                println!("  {} -> {:?}", wire, instruction.operation);
            }
        }
        println!();
    }

    /// Print the current memoization table (debug helper)
    fn print_memo_table(&self) {
        if self.memo.is_empty() {
            println!("💾 Memo Table: (empty)");
        } else {
            println!("💾 Memo Table:");
            let mut wires: Vec<_> = self.memo.keys().collect();
            wires.sort();
            for wire in wires {
                if let Some(&value) = self.memo.get(wire) {
                    println!("  {} = {}", wire, value);
                }
            }
        }
        println!();
    }
}

/// Parse a wire input (either numeric value or wire reference)
pub fn parse_wire_input(input: &str) -> Result<WireInput> {
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
            Err(anyhow!(
                "Invalid wire name: '{}'. Wire names must be alphanumeric.",
                trimmed
            ))
        }
    }
}

/// Parse a complete instruction line
pub fn parse_instruction(line: &str) -> Result<Instruction> {
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
        let input1 = parse_wire_input(and_parts[0])?; // Call 1
        let input2 = parse_wire_input(and_parts[1])?; // Call 2
        Operation::And(input1, input2)
    } else if operation_str.contains(" OR ") {
        // Binary OR: "x OR y"
        let or_parts: Vec<&str> = operation_str.split(" OR ").collect();
        if or_parts.len() != 2 {
            return Err(anyhow!("Invalid OR operation: '{}'", operation_str));
        }
        let input1 = parse_wire_input(or_parts[0])?; // Call 1
        let input2 = parse_wire_input(or_parts[1])?; // Call 2
        Operation::Or(input1, input2)
    } else if operation_str.contains(" LSHIFT ") {
        // Left shift: "p LSHIFT 2"
        let shift_parts: Vec<&str> = operation_str.split(" LSHIFT ").collect();
        if shift_parts.len() != 2 {
            return Err(anyhow!("Invalid LSHIFT operation: '{}'", operation_str));
        }
        let input = parse_wire_input(shift_parts[0])?; // Call 1
        let amount = parse_wire_input(shift_parts[1])?; // Call 2
        Operation::LShift(input, amount)
    } else if operation_str.contains(" RSHIFT ") {
        // Right shift: "x RSHIFT 3"
        let shift_parts: Vec<&str> = operation_str.split(" RSHIFT ").collect();
        if shift_parts.len() != 2 {
            return Err(anyhow!("Invalid RSHIFT operation: '{}'", operation_str));
        }
        let input = parse_wire_input(shift_parts[0])?; // Call 1
        let amount = parse_wire_input(shift_parts[1])?; // Call 2
        Operation::RShift(input, amount)
    } else if operation_str.starts_with("NOT ") {
        // Unary NOT: "NOT e"
        let not_input = operation_str.strip_prefix("NOT ").unwrap().trim();
        let input = parse_wire_input(not_input)?; // Call 1
        Operation::Not(input)
    } else {
        // Direct assignment: "123" or "x"
        let input = parse_wire_input(operation_str)?; // Call 1
        Operation::Assign(input)
    };

    Ok(Instruction {
        operation,
        output_wire,
    })
}

/// Day 07 Part 1: Find the signal provided to wire 'a'
pub fn solve_part1(input: &str) -> Result<String> {
    solve_part1_with_debug(input, false)
}

/// Day 07 Part 1 with debug output
pub fn solve_part1_with_debug(input: &str, debug: bool) -> Result<String> {
    let mut circuit = Circuit::new_with_debug(debug);

    if debug {
        println!("🚀 Starting Day 7 Part 1 with DEBUG enabled");
        println!("📥 Processing {} lines of input", input.lines().count());
        println!();
    }

    for line in input.lines().filter(|line| !line.is_empty()) {
        // Parse and apply each instruction directly to the circuit
        circuit.add_instruction(line)?;
    }

    if debug {
        println!("🎯 Evaluating final result for wire 'a':");
    }

    let result = circuit.get_wire_value("a")?;

    if debug {
        println!("🏁 Final result: {}", result);
    }

    Ok(result.to_string())
}

/// Day 07 Part 2: Override wire 'b' with Part 1's result and recalculate
pub fn solve_part2(input: &str) -> Result<String> {
    solve_part2_with_debug(input, false)
}

/// Day 07 Part 2 with debug output
pub fn solve_part2_with_debug(input: &str, debug: bool) -> Result<String> {
    if debug {
        println!("🚀 Starting Day 7 Part 2 with DEBUG enabled");
        println!("📥 First, calculating Part 1 result...");
        println!();
    }

    // Step 1: Get the signal from wire 'a' in the original circuit (Part 1 result)
    let part1_result = {
        let mut circuit = Circuit::new_with_debug(debug && false); // Don't debug Part 1 calculation
        for line in input.lines().filter(|line| !line.is_empty()) {
            circuit.add_instruction(line)?;
        }
        circuit.get_wire_value("a")?
    };

    if debug {
        println!("✅ Part 1 result: {}", part1_result);
        println!(
            "🔄 Now creating modified circuit with b = {}...",
            part1_result
        );
        println!();
    }

    // Step 2: Create a new circuit with modified input
    let mut circuit = Circuit::new_with_debug(debug);

    // Step 3: Process all instructions, but override any instruction targeting wire 'b'
    for line in input.lines().filter(|line| !line.is_empty()) {
        // Check if this instruction targets wire 'b'
        if line.trim().ends_with(" -> b") {
            // Override: use Part 1's result as the new value for wire 'b'
            let override_instruction = format!("{} -> b", part1_result);
            if debug {
                println!(
                    "🔄 Overriding: '{}' with '{}'",
                    line.trim(),
                    override_instruction
                );
            }
            circuit.add_instruction(&override_instruction)?;
        } else {
            // Use original instruction
            circuit.add_instruction(line)?;
        }
    }

    if debug {
        println!("🎯 Evaluating final result for wire 'a' with modified circuit:");
    }

    // Step 4: Calculate the new signal on wire 'a'
    let result = circuit.get_wire_value("a")?;

    if debug {
        println!("🏁 Part 2 final result: {}", result);
    }

    Ok(result.to_string())
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
