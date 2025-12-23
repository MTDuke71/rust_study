use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GateType {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone)]
struct Gate {
    input1: String,
    input2: String,
    gate_type: GateType,
    output: String,
}

pub fn solve_part1(input: &str) -> i64 {
    let (wire_values, gates) = parse_input(input);
    let final_values = simulate_circuit(wire_values, gates);
    extract_z_value(&final_values)
}

pub fn solve_part2(input: &str) -> String {
    // Part 2: Find the 8 wires involved in swaps (4 pairs)
    // Analyze the adder circuit structure to find anomalies
    let (_wire_values, gates) = parse_input(input);
    
    // Build a lookup: output wire -> gate that produces it
    let mut gate_by_output: HashMap<String, &Gate> = HashMap::new();
    for gate in &gates {
        gate_by_output.insert(gate.output.clone(), gate);
    }
    
    // Find suspicious wires by checking adder structure
    let suspicious = find_swapped_wires(&gates, &gate_by_output);
    
    // Sort and format as comma-separated list
    let mut wires: Vec<String> = suspicious.into_iter().collect();
    wires.sort();
    wires.join(",")
}

fn parse_input(input: &str) -> (HashMap<String, u8>, Vec<Gate>) {
    // Handle both Unix (\n\n) and Windows (\r\n\r\n) line endings
    let input = input.replace("\r\n", "\n");
    let sections: Vec<&str> = input.split("\n\n").collect();
    
    if sections.len() != 2 {
        panic!("Expected 2 sections separated by blank line, got {}", sections.len());
    }
    
    // Parse initial wire values
    let mut wire_values = HashMap::new();
    for line in sections[0].lines() {
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() != 2 {
            panic!("Invalid wire value line: '{}'", line);
        }
        let wire = parts[0].to_string();
        let value = parts[1].parse::<u8>()
            .unwrap_or_else(|_| panic!("Invalid value '{}' for wire '{}'", parts[1], wire));
        wire_values.insert(wire, value);
    }
    
    // Parse gate definitions
    let mut gates = Vec::new();
    for line in sections[1].lines() {
        if line.trim().is_empty() {
            continue;
        }
        // Format: "x00 AND y00 -> z00"
        let parts: Vec<&str> = line.split(" -> ").collect();
        if parts.len() != 2 {
            panic!("Invalid gate line: '{}'", line);
        }
        let output = parts[1].to_string();
        
        let gate_parts: Vec<&str> = parts[0].split_whitespace().collect();
        let input1 = gate_parts[0].to_string();
        let gate_type = match gate_parts[1] {
            "AND" => GateType::And,
            "OR" => GateType::Or,
            "XOR" => GateType::Xor,
            _ => panic!("Unknown gate type: {}", gate_parts[1]),
        };
        let input2 = gate_parts[2].to_string();
        
        gates.push(Gate {
            input1,
            input2,
            gate_type,
            output,
        });
    }
    
    (wire_values, gates)
}

fn simulate_circuit(mut wire_values: HashMap<String, u8>, gates: Vec<Gate>) -> HashMap<String, u8> {
    // Simulate gate propagation until all gates have evaluated
    // Gates can only evaluate when both inputs are available
    
    let mut remaining_gates = gates;
    
    while !remaining_gates.is_empty() {
        let mut evaluated = Vec::new();
        
        for (idx, gate) in remaining_gates.iter().enumerate() {
            // Check if both inputs are available
            if let (Some(&val1), Some(&val2)) = (
                wire_values.get(&gate.input1),
                wire_values.get(&gate.input2),
            ) {
                // Evaluate gate
                let output_value = match gate.gate_type {
                    GateType::And => val1 & val2,
                    GateType::Or => val1 | val2,
                    GateType::Xor => val1 ^ val2,
                };
                
                wire_values.insert(gate.output.clone(), output_value);
                evaluated.push(idx);
            }
        }
        
        // Remove evaluated gates (in reverse to maintain indices)
        for &idx in evaluated.iter().rev() {
            remaining_gates.remove(idx);
        }
        
        // Safety check: if no gates evaluated, we have a problem
        if evaluated.is_empty() && !remaining_gates.is_empty() {
            panic!("Circuit has unresolved dependencies or missing inputs!");
        }
    }
    
    wire_values
}

fn extract_z_value(wire_values: &HashMap<String, u8>) -> i64 {
    // Collect all z wires and sort them by bit position
    let mut z_wires: Vec<(String, u8)> = wire_values
        .iter()
        .filter(|(name, _)| name.starts_with('z'))
        .map(|(name, &value)| (name.clone(), value))
        .collect();
    
    z_wires.sort_by_key(|(name, _)| {
        name[1..].parse::<usize>().unwrap()
    });
    
    // Build binary number (z00 is LSB)
    let mut result = 0i64;
    for (idx, (_name, value)) in z_wires.iter().enumerate() {
        if *value == 1 {
            result |= 1 << idx;
        }
    }
    
    result
}

fn find_swapped_wires(gates: &[Gate], _gate_by_output: &HashMap<String, &Gate>) -> Vec<String> {
    let mut suspicious = Vec::new();
    
    // Determine the highest bit number
    let max_z = gates.iter()
        .filter(|g| g.output.starts_with('z'))
        .map(|g| g.output[1..].parse::<u32>().unwrap_or(0))
        .max()
        .unwrap_or(0);
    let final_z = format!("z{:02}", max_z);
    
    // Rule 1: All z-wires (except the final one) should be produced by XOR gates
    for gate in gates {
        if gate.output.starts_with('z') && gate.output != final_z {
            if gate.gate_type != GateType::Xor {
                suspicious.push(gate.output.clone());
            }
        }
    }
    
    // Rule 2: XOR gates should either:
    // - Take x,y inputs (first level XOR), OR  
    // - Output to z wire (second level XOR)
    // Exception: z00 = x00 XOR y00 (both conditions true)
    for gate in gates {
        if gate.gate_type == GateType::Xor {
            let has_xy_input = gate.input1.starts_with('x') || gate.input1.starts_with('y')
                            || gate.input2.starts_with('x') || gate.input2.starts_with('y');
            let outputs_to_z = gate.output.starts_with('z');
            
            // If it's not from x,y and not to z, that's wrong
            if !has_xy_input && !outputs_to_z {
                suspicious.push(gate.output.clone());
            }
        }
    }
    
    // Rule 3: AND gates (except x00 AND y00) should feed into OR gates (carry chain)
    for gate in gates {
        if gate.gate_type == GateType::And {
            // Skip the first carry generator
            if (gate.input1 == "x00" && gate.input2 == "y00") 
            || (gate.input1 == "y00" && gate.input2 == "x00") {
                continue;
            }
            
            // This AND output should be consumed by an OR gate
            let output_wire = &gate.output;
            let mut found_or_consumer = false;
            
            for consumer in gates {
                if (consumer.input1 == *output_wire || consumer.input2 == *output_wire)
                    && consumer.gate_type == GateType::Or {
                    found_or_consumer = true;
                    break;
                }
            }
            
            if !found_or_consumer {
                suspicious.push(gate.output.clone());
            }
        }
    }
    
    // Rule 4: XOR gates with x,y inputs should NOT directly output to z (except z00)
    // They should output to intermediate wires that feed the second XOR
    for gate in gates {
        if gate.gate_type == GateType::Xor {
            let is_xy_xor = (gate.input1.starts_with('x') || gate.input1.starts_with('y'))
                         && (gate.input2.starts_with('x') || gate.input2.starts_with('y'));
            
            if is_xy_xor && gate.output.starts_with('z') && gate.output != "z00" {
                suspicious.push(gate.output.clone());
            }
        }
    }
    
    // Rule 5: XOR gates with x,y inputs should be consumed by another XOR (not by OR/AND)
    // Exception: x00 XOR y00 goes directly to z00
    for gate in gates {
        if gate.gate_type == GateType::Xor {
            let is_xy_xor = (gate.input1.starts_with('x') || gate.input1.starts_with('y'))
                         && (gate.input2.starts_with('x') || gate.input2.starts_with('y'));
            
            if is_xy_xor && gate.output != "z00" {
                // This XOR output should be consumed by another XOR
                let output_wire = &gate.output;
                let mut found_xor_consumer = false;
                
                for consumer in gates {
                    if (consumer.input1 == *output_wire || consumer.input2 == *output_wire)
                        && consumer.gate_type == GateType::Xor {
                        found_xor_consumer = true;
                        break;
                    }
                }
                
                if !found_xor_consumer {
                    suspicious.push(gate.output.clone());
                }
            }
        }
    }
    
    suspicious.sort();
    suspicious.dedup();
    suspicious
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_example() {
        let input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

        let result = solve_part1(input);
        assert_eq!(result, 4); // Binary 100 = decimal 4
    }

    #[test]
    fn test_larger_example() {
        let input = include_str!("../../inputs/day24_problem_example.txt");
        let result = solve_part1(input);
        
        // From problem: should produce 2024
        assert_eq!(result, 2024);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day24_example.txt");
        let result = solve_part1(input);
        
        // Your actual puzzle answer
        assert_eq!(result, 61495910098126);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day24_example.txt");
        let result = solve_part2(input);
        
        println!("Part 2 result: {}", result);
        
        // Your actual answer: css,cwt,gdd,jmv,pqt,z05,z09,z37
        assert_eq!(result, "css,cwt,gdd,jmv,pqt,z05,z09,z37");
    }
}
