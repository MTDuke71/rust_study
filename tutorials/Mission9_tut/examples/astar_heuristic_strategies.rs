// A* Heuristic Combination Strategies - Tutorial Extension

use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
struct WeightedAstarNode {
    id: usize,
    g: usize,
    h: usize,
    f: usize,
}

impl Ord for WeightedAstarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f).then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for WeightedAstarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Different heuristic combination strategies
#[derive(Debug, Clone)]
pub enum HeuristicStrategy {
    /// Standard A*: f(n) = g(n) + h(n)
    Standard,
    /// Weighted A*: f(n) = g(n) + w × h(n)
    Weighted(f64),
    /// Dynamic weighting: weight decreases with depth
    Dynamic { initial_weight: f64, max_depth: usize },
    /// Focal search: bound suboptimality, then minimize h(n)
    Focal { suboptimality_bound: f64 },
}

/// Calculate f-score based on strategy
fn calculate_f_score(
    g: usize, 
    h: usize, 
    strategy: &HeuristicStrategy,
    current_depth: Option<usize>
) -> usize {
    match strategy {
        HeuristicStrategy::Standard => {
            g + h
        },
        
        HeuristicStrategy::Weighted(weight) => {
            g + (*weight * h as f64) as usize
        },
        
        HeuristicStrategy::Dynamic { initial_weight, max_depth } => {
            let depth = current_depth.unwrap_or(0);
            let depth_factor = depth as f64 / *max_depth as f64;
            let dynamic_weight = 1.0 + (initial_weight - 1.0) * (1.0 - depth_factor);
            g + (dynamic_weight * h as f64) as usize
        },
        
        HeuristicStrategy::Focal { suboptimality_bound: _ } => {
            // For focal search, we still use standard f-score initially
            // The bounding happens in the search algorithm
            g + h
        },
    }
}

/// Demonstration of different strategies
fn main() {
    println!("=== A* Heuristic Combination Strategies ===\n");
    
    // Example coordinates
    let start = (0, 0);
    let goal = (10, 10);
    let current = (5, 3);
    
    let g = 8; // Distance traveled so far
    let h = manhattan_distance(current, goal); // Heuristic estimate
    
    println!("Current position: {:?}", current);
    println!("Goal position: {:?}", goal);
    println!("g(n) = {} (actual distance from start)", g);
    println!("h(n) = {} (Manhattan distance to goal)\n", h);
    
    // Compare different strategies
    let strategies = vec![
        ("Standard A*", HeuristicStrategy::Standard),
        ("Weighted A* (w=1.5)", HeuristicStrategy::Weighted(1.5)),
        ("Weighted A* (w=2.0)", HeuristicStrategy::Weighted(2.0)),
        ("Weighted A* (w=0.5)", HeuristicStrategy::Weighted(0.5)),
        ("Dynamic A*", HeuristicStrategy::Dynamic { 
            initial_weight: 3.0, 
            max_depth: 20 
        }),
        ("Focal A*", HeuristicStrategy::Focal { 
            suboptimality_bound: 1.1 
        }),
    ];
    
    println!("🎯 F-Score Comparison:");
    println!("Strategy                | f(n) | Behavior");
    println!("------------------------|------|--------------------");
    
    for (name, strategy) in strategies {
        let f = calculate_f_score(g, h, &strategy, Some(8));
        let behavior = match strategy {
            HeuristicStrategy::Standard => "Optimal path guaranteed",
            HeuristicStrategy::Weighted(w) if w > 1.0 => "Faster, suboptimal",
            HeuristicStrategy::Weighted(w) if w < 1.0 => "Slower, conservative",
            HeuristicStrategy::Dynamic { .. } => "Adaptive weighting",
            HeuristicStrategy::Focal { .. } => "Bounded suboptimality",
            _ => "Balanced",
        };
        println!("{:<23} | {:<4} | {}", name, f, behavior);
    }
    
    println!("\n📊 Strategy Analysis:");
    
    // Standard A*
    println!("\n🔸 Standard A* (w = 1.0):");
    println!("  • f(n) = g(n) + h(n) = {} + {} = {}", g, h, g + h);
    println!("  • Guarantees optimal path if h(n) is admissible");
    println!("  • Balanced between speed and optimality");
    
    // Weighted A*
    println!("\n🔸 Weighted A* (w > 1.0):");
    let w = 2.0;
    let f_weighted = g + (w * h as f64) as usize;
    println!("  • f(n) = g(n) + {} × h(n) = {} + {} = {}", w, g, (w * h as f64) as usize, f_weighted);
    println!("  • Trades optimality for speed");
    println!("  • Higher weight = more greedy = faster but less optimal");
    println!("  • Solution cost ≤ w × optimal cost");
    
    // Conservative weighting
    println!("\n🔸 Conservative A* (w < 1.0):");
    let w = 0.5;
    let f_conservative = g + (w * h as f64) as usize;
    println!("  • f(n) = g(n) + {} × h(n) = {} + {} = {}", w, g, (w * h as f64) as usize, f_conservative);
    println!("  • More thorough exploration");
    println!("  • Slower but potentially finds better paths");
    
    // Dynamic weighting
    println!("\n🔸 Dynamic Weighted A*:");
    println!("  • Weight decreases as search progresses");
    println!("  • Early: aggressive (fast exploration)");
    println!("  • Later: conservative (accurate convergence)");
    println!("  • w(depth) = 1 + (w₀ - 1) × (1 - depth/max_depth)");
    
    // When to use each strategy
    println!("\n🎯 When to Use Each Strategy:");
    println!("┌─────────────────────┬─────────────────────────────────────┐");
    println!("│ Strategy            │ Best For                            │");
    println!("├─────────────────────┼─────────────────────────────────────┤");
    println!("│ Standard A*         │ Need optimal path, balanced speed   │");
    println!("│ Weighted A* (w>1)   │ Real-time games, speed critical     │");
    println!("│ Conservative (w<1)  │ Safety-critical, exploration        │");
    println!("│ Dynamic A*          │ Unknown environments, adaptability  │");
    println!("│ Focal A*            │ Bounded suboptimality, multiple     │");
    println!("│                     │ objectives                          │");
    println!("└─────────────────────┴─────────────────────────────────────┘");
    
    demonstrate_admissibility();
}

fn manhattan_distance(from: (usize, usize), to: (usize, usize)) -> usize {
    from.0.abs_diff(to.0) + from.1.abs_diff(to.1)
}

/// Demonstrate admissibility concept
fn demonstrate_admissibility() {
    println!("\n🧠 Understanding Admissible Heuristics:");
    println!("An admissible heuristic NEVER overestimates the actual cost.");
    println!("This is crucial for optimality guarantees.\n");
    
    let positions = vec![
        ((0, 0), "Start"),
        ((2, 3), "Point A"),
        ((5, 1), "Point B"), 
        ((8, 8), "Goal"),
    ];
    
    println!("Example positions:");
    for (pos, name) in &positions {
        println!("  {}: {:?}", name, pos);
    }
    
    // Compare heuristics
    let goal = (8, 8);
    println!("\nHeuristic comparison to goal {:?}:", goal);
    println!("Position | Manhattan | Euclidean | Actual* | Admissible?");
    println!("---------|-----------|-----------|---------|------------");
    
    for (pos, name) in positions {
        let manhattan = manhattan_distance(pos, goal);
        let euclidean = (((pos.0 as f64 - goal.0 as f64).powi(2) + 
                         (pos.1 as f64 - goal.1 as f64).powi(2)).sqrt()) as usize;
        // Simplified "actual" cost (in reality, depends on obstacles)
        let actual_approx = manhattan; // Assuming no obstacles
        
        let manhattan_admissible = manhattan <= actual_approx;
        let euclidean_admissible = euclidean <= actual_approx;
        
        println!("{:<8} | {:<9} | {:<9} | {:<7} | M:{} E:{}", 
                name, manhattan, euclidean, actual_approx,
                if manhattan_admissible { "✅" } else { "❌" },
                if euclidean_admissible { "✅" } else { "❌" });
    }
    
    println!("\n* Actual cost depends on obstacles and allowed movements");
    println!("✅ = Admissible (never overestimates)");
    println!("❌ = Not admissible (may overestimate)");
    
    println!("\n💡 Key Insights:");
    println!("• Manhattan distance: Admissible for 4-way movement");
    println!("• Euclidean distance: Admissible for 8-way movement");
    println!("• Overestimating heuristics can miss optimal paths");
    println!("• Underestimating is safe but may be less efficient");
}