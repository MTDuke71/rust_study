//! CLI runner for Project Euler solutions

use project_euler::ProblemSolver;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return;
    }
    
    let arg = &args[1];
    
    // Handle special commands
    if arg == "list" || arg == "--list" || arg == "-l" {
        list_implemented();
        return;
    }
    
    if arg == "all" || arg == "--all" {
        solve_all();
        return;
    }
    
    // Parse and solve single problem
    let problem_num: usize = arg.parse().unwrap_or_else(|_| {
        eprintln!("Error: Invalid problem number '{}'", arg);
        std::process::exit(1);
    });
    
    match ProblemSolver::solve(problem_num) {
        Some(answer) => println!("Problem {}: {}", problem_num, answer),
        None => {
            println!("Problem {} not yet implemented", problem_num);
            println!("\nImplemented problems: {:?}", ProblemSolver::implemented());
        }
    }
}

fn print_usage() {
    println!("Project Euler Solutions Runner");
    println!();
    println!("Usage:");
    println!("  cargo run -p project_euler -- <problem_number>");
    println!("  cargo run -p project_euler -- list");
    println!("  cargo run -p project_euler -- all");
    println!();
    println!("Examples:");
    println!("  cargo run -p project_euler -- 1      # Solve problem 1");
    println!("  cargo run -p project_euler -- 2      # Solve problem 2");
    println!("  cargo run -p project_euler -- list   # Show all implemented problems");
    println!("  cargo run -p project_euler -- all    # Solve all implemented problems");
}

fn list_implemented() {
    let problems = ProblemSolver::implemented();
    println!("Implemented problems ({}): {:?}", problems.len(), problems);
}

fn solve_all() {
    let problems = ProblemSolver::implemented();
    println!("Solving {} problems...\n", problems.len());
    
    for &num in &problems {
        if let Some(answer) = ProblemSolver::solve(num) {
            println!("Problem {:3}: {}", num, answer);
        }
    }
}
