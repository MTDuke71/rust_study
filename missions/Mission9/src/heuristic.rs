//! Heuristic functions for A* pathfinding algorithm

use crate::{NodeId, Weight};

/// Trait for heuristic functions used in A* search
pub trait Heuristic: std::fmt::Debug + Clone {
    /// Estimate the distance from current node to goal
    /// 
    /// # Requirements
    /// - Must be admissible (never overestimate the true cost)
    /// - Should be consistent/monotonic for optimal performance
    /// 
    /// # Arguments
    /// - `current`: Current node position
    /// - `goal`: Goal node position
    /// - `context`: Additional context (e.g., coordinates, graph info)
    fn estimate_distance(&self, current: NodeId, goal: NodeId, context: &HeuristicContext) -> Weight;
    
    /// Get the name of this heuristic for debugging
    fn name(&self) -> &'static str;
    
    /// Check if this heuristic is admissible for the given problem
    fn is_admissible(&self) -> bool {
        true // Most implemented heuristics should be admissible
    }
}

/// Enum-based heuristic for flexible combination and easy usage
#[derive(Debug, Clone)]
pub enum HeuristicType {
    Manhattan(ManhattanHeuristic),
    Euclidean(EuclideanHeuristic),
    Chebyshev(ChebyshevHeuristic),
    Zero(ZeroHeuristic),
    Weighted(WeightedCombinationHeuristic),
}

impl Heuristic for HeuristicType {
    fn estimate_distance(&self, current: NodeId, goal: NodeId, context: &HeuristicContext) -> Weight {
        match self {
            HeuristicType::Manhattan(h) => h.estimate_distance(current, goal, context),
            HeuristicType::Euclidean(h) => h.estimate_distance(current, goal, context),
            HeuristicType::Chebyshev(h) => h.estimate_distance(current, goal, context),
            HeuristicType::Zero(h) => h.estimate_distance(current, goal, context),
            HeuristicType::Weighted(h) => h.estimate_distance(current, goal, context),
        }
    }
    
    fn name(&self) -> &'static str {
        match self {
            HeuristicType::Manhattan(h) => h.name(),
            HeuristicType::Euclidean(h) => h.name(),
            HeuristicType::Chebyshev(h) => h.name(),
            HeuristicType::Zero(h) => h.name(),
            HeuristicType::Weighted(h) => h.name(),
        }
    }
    
    fn is_admissible(&self) -> bool {
        match self {
            HeuristicType::Manhattan(h) => h.is_admissible(),
            HeuristicType::Euclidean(h) => h.is_admissible(),
            HeuristicType::Chebyshev(h) => h.is_admissible(),
            HeuristicType::Zero(h) => h.is_admissible(),
            HeuristicType::Weighted(h) => h.is_admissible(),
        }
    }
}

/// Context information for heuristic calculations
#[derive(Debug, Clone)]
pub struct HeuristicContext {
    /// Node coordinates (if available)
    pub coordinates: std::collections::HashMap<NodeId, (f64, f64)>,
    /// Additional problem-specific data
    pub metadata: std::collections::HashMap<String, f64>,
}

impl HeuristicContext {
    /// Create a new empty context
    pub fn new() -> Self {
        Self {
            coordinates: std::collections::HashMap::new(),
            metadata: std::collections::HashMap::new(),
        }
    }
    
    /// Add coordinates for a node
    pub fn add_coordinates(&mut self, node: NodeId, x: f64, y: f64) {
        self.coordinates.insert(node, (x, y));
    }
    
    /// Add metadata value
    pub fn add_metadata(&mut self, key: String, value: f64) {
        self.metadata.insert(key, value);
    }
    
    /// Get coordinates for a node
    pub fn get_coordinates(&self, node: NodeId) -> Option<(f64, f64)> {
        self.coordinates.get(&node).copied()
    }
}

impl Default for HeuristicContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Manhattan distance heuristic (L1 norm)
/// Best for grid-based movement with 4-directional movement
#[derive(Debug, Clone)]
pub struct ManhattanHeuristic;

impl Heuristic for ManhattanHeuristic {
    fn estimate_distance(&self, current: NodeId, goal: NodeId, context: &HeuristicContext) -> Weight {
        if let (Some((x1, y1)), Some((x2, y2))) = 
            (context.get_coordinates(current), context.get_coordinates(goal)) {
            (x2 - x1).abs() + (y2 - y1).abs()
        } else {
            0.0 // No coordinates available, fall back to zero heuristic
        }
    }
    
    fn name(&self) -> &'static str {
        "Manhattan Distance"
    }
}

/// Euclidean distance heuristic (L2 norm)  
/// Best for movement in any direction with equal cost
#[derive(Debug, Clone)]
pub struct EuclideanHeuristic;

impl Heuristic for EuclideanHeuristic {
    fn estimate_distance(&self, current: NodeId, goal: NodeId, context: &HeuristicContext) -> Weight {
        if let (Some((x1, y1)), Some((x2, y2))) = 
            (context.get_coordinates(current), context.get_coordinates(goal)) {
            let dx = x2 - x1;
            let dy = y2 - y1;
            (dx * dx + dy * dy).sqrt()
        } else {
            0.0 // No coordinates available, fall back to zero heuristic
        }
    }
    
    fn name(&self) -> &'static str {
        "Euclidean Distance"
    }
}

/// Chebyshev distance heuristic (L∞ norm)
/// Best for grid-based movement with 8-directional movement
#[derive(Debug, Clone)]
pub struct ChebyshevHeuristic;

impl Heuristic for ChebyshevHeuristic {
    fn estimate_distance(&self, current: NodeId, goal: NodeId, context: &HeuristicContext) -> Weight {
        if let (Some((x1, y1)), Some((x2, y2))) = 
            (context.get_coordinates(current), context.get_coordinates(goal)) {
            (x2 - x1).abs().max((y2 - y1).abs())
        } else {
            0.0 // No coordinates available, fall back to zero heuristic
        }
    }
    
    fn name(&self) -> &'static str {
        "Chebyshev Distance"
    }
}

/// Zero heuristic (turns A* into Dijkstra's algorithm)
/// Guarantees optimal solution but slower than informed search
#[derive(Debug, Clone)]
pub struct ZeroHeuristic;

impl Heuristic for ZeroHeuristic {
    fn estimate_distance(&self, _current: NodeId, _goal: NodeId, _context: &HeuristicContext) -> Weight {
        0.0
    }
    
    fn name(&self) -> &'static str {
        "Zero Heuristic (Dijkstra Mode)"
    }
}

/// Custom heuristic that combines multiple basic heuristics with weights
#[derive(Debug, Clone)]
pub struct WeightedCombinationHeuristic {
    /// List of (heuristic_type, weight) pairs
    heuristics: Vec<(BasicHeuristicType, Weight)>,
}

/// Basic heuristic types that can be combined
#[derive(Debug, Clone)]
pub enum BasicHeuristicType {
    Manhattan,
    Euclidean,
    Chebyshev,
    Zero,
}

impl WeightedCombinationHeuristic {
    /// Create a new combination heuristic
    pub fn new() -> Self {
        Self {
            heuristics: Vec::new(),
        }
    }
    
    /// Add a basic heuristic with its weight to the combination
    pub fn add_heuristic(mut self, heuristic_type: BasicHeuristicType, weight: Weight) -> Self {
        self.heuristics.push((heuristic_type, weight));
        self
    }
    
    /// Add Manhattan heuristic with weight
    pub fn add_manhattan(self, weight: Weight) -> Self {
        self.add_heuristic(BasicHeuristicType::Manhattan, weight)
    }
    
    /// Add Euclidean heuristic with weight
    pub fn add_euclidean(self, weight: Weight) -> Self {
        self.add_heuristic(BasicHeuristicType::Euclidean, weight)
    }
    
    /// Add Chebyshev heuristic with weight
    pub fn add_chebyshev(self, weight: Weight) -> Self {
        self.add_heuristic(BasicHeuristicType::Chebyshev, weight)
    }
}

impl BasicHeuristicType {
    fn estimate_distance(&self, current: NodeId, goal: NodeId, context: &HeuristicContext) -> Weight {
        match self {
            BasicHeuristicType::Manhattan => {
                ManhattanHeuristic.estimate_distance(current, goal, context)
            }
            BasicHeuristicType::Euclidean => {
                EuclideanHeuristic.estimate_distance(current, goal, context)
            }
            BasicHeuristicType::Chebyshev => {
                ChebyshevHeuristic.estimate_distance(current, goal, context)
            }
            BasicHeuristicType::Zero => {
                ZeroHeuristic.estimate_distance(current, goal, context)
            }
        }
    }
    
    fn is_admissible(&self) -> bool {
        match self {
            BasicHeuristicType::Manhattan => ManhattanHeuristic.is_admissible(),
            BasicHeuristicType::Euclidean => EuclideanHeuristic.is_admissible(),
            BasicHeuristicType::Chebyshev => ChebyshevHeuristic.is_admissible(),
            BasicHeuristicType::Zero => ZeroHeuristic.is_admissible(),
        }
    }
}

impl Heuristic for WeightedCombinationHeuristic {
    fn estimate_distance(&self, current: NodeId, goal: NodeId, context: &HeuristicContext) -> Weight {
        self.heuristics
            .iter()
            .map(|(heuristic_type, weight)| {
                heuristic_type.estimate_distance(current, goal, context) * weight
            })
            .sum()
    }
    
    fn name(&self) -> &'static str {
        "Weighted Combination"
    }
    
    fn is_admissible(&self) -> bool {
        // Combination is admissible if all components are admissible and weights are non-negative
        self.heuristics.iter().all(|(heuristic_type, weight)| {
            heuristic_type.is_admissible() && *weight >= 0.0
        })
    }
}

impl Default for WeightedCombinationHeuristic {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_context() -> HeuristicContext {
        let mut context = HeuristicContext::new();
        context.add_coordinates(0, 0.0, 0.0);
        context.add_coordinates(1, 3.0, 4.0);
        context.add_coordinates(2, 1.0, 1.0);
        context
    }
    
    #[test]
    fn test_manhattan_heuristic() {
        let heuristic = ManhattanHeuristic;
        let context = create_test_context();
        
        // Distance from (0,0) to (3,4) should be |3-0| + |4-0| = 7
        let distance = heuristic.estimate_distance(0, 1, &context);
        assert_eq!(distance, 7.0);
        
        // Distance from (0,0) to (1,1) should be |1-0| + |1-0| = 2  
        let distance = heuristic.estimate_distance(0, 2, &context);
        assert_eq!(distance, 2.0);
        
        assert_eq!(heuristic.name(), "Manhattan Distance");
        assert!(heuristic.is_admissible());
    }
    
    #[test]
    fn test_euclidean_heuristic() {
        let heuristic = EuclideanHeuristic;
        let context = create_test_context();
        
        // Distance from (0,0) to (3,4) should be sqrt(3² + 4²) = 5
        let distance = heuristic.estimate_distance(0, 1, &context);
        assert_eq!(distance, 5.0);
        
        // Distance from (0,0) to (1,1) should be sqrt(1² + 1²) = sqrt(2) ≈ 1.414
        let distance = heuristic.estimate_distance(0, 2, &context);
        assert!((distance - std::f64::consts::SQRT_2).abs() < 1e-10);
        
        assert_eq!(heuristic.name(), "Euclidean Distance");
        assert!(heuristic.is_admissible());
    }
    
    #[test]
    fn test_chebyshev_heuristic() {
        let heuristic = ChebyshevHeuristic;
        let context = create_test_context();
        
        // Distance from (0,0) to (3,4) should be max(|3-0|, |4-0|) = max(3, 4) = 4
        let distance = heuristic.estimate_distance(0, 1, &context);
        assert_eq!(distance, 4.0);
        
        // Distance from (0,0) to (1,1) should be max(|1-0|, |1-0|) = 1
        let distance = heuristic.estimate_distance(0, 2, &context);
        assert_eq!(distance, 1.0);
        
        assert_eq!(heuristic.name(), "Chebyshev Distance");
        assert!(heuristic.is_admissible());
    }
    
    #[test]
    fn test_zero_heuristic() {
        let heuristic = ZeroHeuristic;
        let context = create_test_context();
        
        // Should always return 0 regardless of nodes
        assert_eq!(heuristic.estimate_distance(0, 1, &context), 0.0);
        assert_eq!(heuristic.estimate_distance(0, 2, &context), 0.0);
        assert_eq!(heuristic.estimate_distance(1, 2, &context), 0.0);
        
        assert_eq!(heuristic.name(), "Zero Heuristic (Dijkstra Mode)");
        assert!(heuristic.is_admissible());
    }
    
    #[test]
    fn test_heuristic_context() {
        let mut context = HeuristicContext::new();
        
        assert!(context.get_coordinates(0).is_none());
        
        context.add_coordinates(0, 1.5, 2.5);
        assert_eq!(context.get_coordinates(0), Some((1.5, 2.5)));
        
        context.add_metadata("scale".to_string(), 2.0);
        assert_eq!(context.metadata.get("scale"), Some(&2.0));
    }
    
    #[test]
    fn test_heuristic_fallback_without_coordinates() {
        let heuristic = EuclideanHeuristic;
        let context = HeuristicContext::new(); // Empty context
        
        // Should fall back to 0.0 when coordinates are not available
        let distance = heuristic.estimate_distance(0, 1, &context);
        assert_eq!(distance, 0.0);
    }
    
    #[test]
    fn test_weighted_combination_heuristic() {
        let heuristic = WeightedCombinationHeuristic::new()
            .add_manhattan(0.5)
            .add_euclidean(0.5);
        
        let context = create_test_context();
        
        // Should be weighted average: 0.5 * 7.0 + 0.5 * 5.0 = 6.0
        let distance = heuristic.estimate_distance(0, 1, &context);
        assert_eq!(distance, 6.0);
        
        assert_eq!(heuristic.name(), "Weighted Combination");
        assert!(heuristic.is_admissible());
    }
    
    #[test]
    fn test_heuristic_type_enum() {
        let context = create_test_context();
        
        let manhattan_enum = HeuristicType::Manhattan(ManhattanHeuristic);
        let manhattan_direct = ManhattanHeuristic;
        
        // Both should give the same result
        let enum_result = manhattan_enum.estimate_distance(0, 1, &context);
        let direct_result = manhattan_direct.estimate_distance(0, 1, &context);
        
        assert_eq!(enum_result, direct_result);
        assert_eq!(enum_result, 7.0);
        
        assert_eq!(manhattan_enum.name(), "Manhattan Distance");
        assert!(manhattan_enum.is_admissible());
    }
}