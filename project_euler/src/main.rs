//! CLI runner for Project Euler solutions

use project_euler::ProblemSolver;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: cargo run -p project_euler --bin euler <problem_number>");
        println!("Example: cargo run -p project_euler --bin euler 1");
        return;
    }
    
    let problem_num: usize = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Error: Invalid problem number");
        std::process::exit(1);
    });
    
    match ProblemSolver::solve(problem_num) {
        Some(answer) => println!("Problem {}: {}", problem_num, answer),
        None => println!("Problem {} not yet implemented", problem_num),
    }
}
