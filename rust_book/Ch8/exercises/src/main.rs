//! # Chapter 8: Collections - Practice Exercises
//!
//! Solutions to the exercises at the end of Chapter 8 in The Rust Book.
//!
//! Exercises:
//! 1. Given a list of integers, return the median and mode
//! 2. Convert strings to pig latin
//! 3. Text interface for adding employees to departments (using HashMap)
//!
//! Run with: `cargo run`

use std::collections::HashMap;
use std::io;

// ==================== Exercise 1: Statistics ====================

fn exercise1_statistics() {
    println!("📊 Exercise 1: Statistics (Median and Mode)");
    println!("===========================================");

    fn median(numbers: &[i32]) -> f64 {
        let mut sorted = numbers.to_vec();
        sorted.sort();

        let len = sorted.len();
        if len == 0 {
            return 0.0;
        }

        if len % 2 == 0 {
            let mid = len / 2;
            (sorted[mid - 1] + sorted[mid]) as f64 / 2.0
        } else {
            sorted[len / 2] as f64
        }
    }

    fn mode(numbers: &[i32]) -> Option<i32> {
        let mut frequency: HashMap<i32, usize> = HashMap::new();

        for &num in numbers {
            *frequency.entry(num).or_insert(0) += 1;
        }

        frequency
            .iter()
            .max_by_key(|(_, &count)| count)
            .map(|(&num, _)| num)
    }

    // Test cases
    let test_data = vec![
        vec![1, 2, 3, 4, 5],
        vec![1, 1, 2, 3, 4, 5],
        vec![1, 2, 2, 2, 3, 4, 5],
        vec![5, 1, 3, 2, 4],
        vec![10, 20, 20, 30, 30, 30, 40],
    ];

    for data in test_data {
        println!("\nData: {:?}", data);
        println!("  Median: {:.1}", median(&data));
        if let Some(m) = mode(&data) {
            println!("  Mode: {}", m);
        } else {
            println!("  Mode: None (empty list)");
        }
    }

    println!();
}

// ==================== Exercise 2: Pig Latin ====================

fn exercise2_pig_latin() {
    println!("🐷 Exercise 2: Convert Strings to Pig Latin");
    println!("===========================================");

    fn to_pig_latin(word: &str) -> String {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

        if word.is_empty() {
            return String::new();
        }

        let first_char = word.chars().next().unwrap();

        if vowels.contains(&first_char) {
            // First letter is a vowel: add "hay" to the end
            format!("{}-hay", word)
        } else {
            // First letter is a consonant: move it to end and add "ay"
            let rest: String = word.chars().skip(1).collect();
            format!("{}-{}ay", rest, first_char.to_lowercase())
        }
    }

    fn sentence_to_pig_latin(sentence: &str) -> String {
        sentence
            .split_whitespace()
            .map(to_pig_latin)
            .collect::<Vec<_>>()
            .join(" ")
    }

    // Test cases
    let test_words = vec![
        "first",       // irst-fay
        "apple",       // apple-hay
        "Hello",       // ello-hay
        "world",       // orld-way
        "Rust",        // ust-ray
        "programming", // rogramming-pay
    ];

    println!("Individual words:");
    for word in test_words {
        println!("  '{}' -> '{}'", word, to_pig_latin(word));
    }

    println!("\nSentences:");
    let sentences = vec![
        "Hello world",
        "The quick brown fox",
        "Rust is awesome",
        "I love programming",
    ];

    for sentence in sentences {
        println!(
            "  '{}'\n  -> '{}'",
            sentence,
            sentence_to_pig_latin(sentence)
        );
    }

    println!();
}

// ==================== Exercise 3: Employee Department System ====================

fn exercise3_employee_system() {
    println!("👥 Exercise 3: Employee Department System");
    println!("=========================================");

    // Using HashMap to store departments and their employees
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    // Helper function to add employee
    fn add_employee(company: &mut HashMap<String, Vec<String>>, name: String, department: String) {
        company
            .entry(department.clone())
            .or_default()
            .push(name.clone());
        println!("✅ Added {} to {}", name, department);
    }

    // Helper function to list employees in department
    fn list_department(company: &HashMap<String, Vec<String>>, department: &str) {
        if let Some(employees) = company.get(department) {
            println!("\n{} Department:", department);
            let mut sorted = employees.clone();
            sorted.sort();
            for (i, employee) in sorted.iter().enumerate() {
                println!("  {}. {}", i + 1, employee);
            }
        } else {
            println!("Department '{}' not found", department);
        }
    }

    // Helper function to list all employees by department
    fn list_all(company: &HashMap<String, Vec<String>>) {
        println!("\nAll Employees by Department:");
        println!("============================");

        let mut departments: Vec<_> = company.keys().collect();
        departments.sort();

        for dept in departments {
            if let Some(employees) = company.get(dept) {
                println!("\n{} ({} employees):", dept, employees.len());
                let mut sorted = employees.clone();
                sorted.sort();
                for employee in sorted {
                    println!("  • {}", employee);
                }
            }
        }
    }

    // Demonstrate the system with sample data
    println!("Sample Usage:\n");

    add_employee(
        &mut company,
        String::from("Alice"),
        String::from("Engineering"),
    );
    add_employee(&mut company, String::from("Bob"), String::from("Sales"));
    add_employee(
        &mut company,
        String::from("Charlie"),
        String::from("Engineering"),
    );
    add_employee(&mut company, String::from("Diana"), String::from("HR"));
    add_employee(&mut company, String::from("Eve"), String::from("Sales"));
    add_employee(
        &mut company,
        String::from("Frank"),
        String::from("Engineering"),
    );
    add_employee(&mut company, String::from("Grace"), String::from("HR"));

    // List specific department
    list_department(&company, "Engineering");

    // List all employees
    list_all(&company);

    println!("\n📝 Interactive Mode Instructions:");
    println!("==================================");
    println!("Commands you could implement:");
    println!("  • 'Add Sally to Engineering'");
    println!("  • 'List Engineering'");
    println!("  • 'List all'");
    println!("  • 'Quit'");

    println!();
}

// ==================== Bonus: Interactive Mode ====================

#[allow(dead_code)]
fn interactive_mode() {
    println!("🎮 Bonus: Interactive Employee System");
    println!("=====================================");
    println!("Commands:");
    println!("  add <name> to <department> - Add an employee");
    println!("  list <department>          - List employees in department");
    println!("  list all                   - List all employees");
    println!("  quit                       - Exit");
    println!();

    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Enter command:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim().to_lowercase();

        if input == "quit" {
            println!("Goodbye!");
            break;
        } else if input.starts_with("add ") {
            // Parse: "add <name> to <department>"
            let parts: Vec<&str> = input.split(" to ").collect();
            if parts.len() == 2 {
                let name = parts[0].trim_start_matches("add ").trim();
                let department = parts[1].trim();

                company
                    .entry(department.to_string())
                    .or_default()
                    .push(name.to_string());

                println!("✅ Added {} to {}\n", name, department);
            } else {
                println!("❌ Invalid format. Use: add <name> to <department>\n");
            }
        } else if input == "list all" {
            println!("\nAll Employees by Department:");
            let mut departments: Vec<_> = company.keys().collect();
            departments.sort();

            for dept in departments {
                if let Some(employees) = company.get(dept) {
                    println!("\n{}:", dept);
                    let mut sorted = employees.clone();
                    sorted.sort();
                    for employee in sorted {
                        println!("  • {}", employee);
                    }
                }
            }
            println!();
        } else if input.starts_with("list ") {
            let department = input.trim_start_matches("list ").trim();
            if let Some(employees) = company.get(department) {
                println!("\n{} Department:", department);
                let mut sorted = employees.clone();
                sorted.sort();
                for employee in sorted {
                    println!("  • {}", employee);
                }
                println!();
            } else {
                println!("❌ Department '{}' not found\n", department);
            }
        } else {
            println!("❌ Unknown command. Try: add, list, list all, or quit\n");
        }
    }
}

// ==================== Main ====================

fn main() {
    println!("=== Chapter 8: Collections - Practice Exercises ===\n");

    exercise1_statistics();
    println!("\n");

    exercise2_pig_latin();
    println!("\n");

    exercise3_employee_system();
    println!("\n");

    // Uncomment to try interactive mode
    println!("💡 To try interactive mode, uncomment the following line in main()");
    println!("   and run again:\n");
    println!("   // interactive_mode();\n");

    // interactive_mode();  // Uncomment to enable interactive mode

    println!("✅ All exercises completed!");
}
