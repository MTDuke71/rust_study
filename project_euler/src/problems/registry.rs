//! Problem solver registry
//!
//! Central dispatch for routing problem numbers to solutions.

/// Problem solver interface
pub struct ProblemSolver;

impl ProblemSolver {
    /// Solve a specific problem by number
    ///
    /// Returns `Some(answer)` if problem is implemented, `None` otherwise.
    pub fn solve(problem_num: usize) -> Option<u64> {
        match problem_num {
            1 => Some(super::p001::solve()),
            2 => Some(super::p002::solve()),
            3 => Some(super::p003::solve()),
            _ => None,
        }
    }
    
    /// Get list of implemented problems
    pub fn implemented() -> Vec<usize> {
        vec![1, 2, 3]
    }
    
    /// Check if a problem is implemented
    pub fn is_implemented(problem_num: usize) -> bool {
        Self::implemented().contains(&problem_num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unimplemented_problem() {
        assert_eq!(ProblemSolver::solve(999), None);
    }
}
