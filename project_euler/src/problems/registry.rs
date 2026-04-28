//! Problem solver registry
//!
//! Central dispatch for routing problem numbers to solutions.

/// Problem solver interface
pub struct ProblemSolver;

impl ProblemSolver {
    /// Solve a specific problem by number
    ///
    /// Returns `Some(answer)` if problem is implemented, `None` otherwise.
    /// Note: Some problems have negative answers (e.g., Problem 27).
    /// These are stored as i64 cast to u64; display with `as i64` when needed.
    pub fn solve(problem_num: usize) -> Option<u64> {
        match problem_num {
            1 => Some(super::p001::solve()),
            2 => Some(super::p002::solve()),
            3 => Some(super::p003::solve()),
            4 => Some(super::p004::solve()),
            5 => Some(super::p005::solve()),
            6 => Some(super::p006::solve()),
            7 => Some(super::p007::solve()),
            8 => Some(super::p008::solve()),
            9 => Some(super::p009::solve() as u64),
            10 => Some(super::p010::solve()),
            11 => Some(super::p011::solve()),
            12 => Some(super::p012::solve()),
            13 => Some(super::p013::solve()),
            14 => Some(super::p014::solve()),
            15 => Some(super::p015::solve()),
            16 => Some(super::p016::solve()),
            17 => Some(super::p017::solve()),
            18 => Some(super::p018::solve()),
            19 => Some(super::p019::solve()),
            20 => Some(super::p020::solve()),
            21 => Some(super::p021::solve()),
            22 => Some(super::p022::solve()),
            23 => Some(super::p023::solve()),
            24 => Some(super::p024::solve()),
            25 => Some(super::p025::solve()),
            26 => Some(super::p026::solve()),
            27 => Some(super::p027::solve() as u64),
            28 => Some(super::p028::solve()),
            29 => Some(super::p029::solve()),
            30 => Some(super::p030::solve()),
            31 => Some(super::p031::solve()),
            32 => Some(super::p032::solve()),
            33 => Some(super::p033::solve()),
            34 => Some(super::p034::solve()),
            35 => Some(super::p035::solve()),
            36 => Some(super::p036::solve()),
            67 => Some(super::p067::solve()),
            _ => None,
        }
    }

    /// Get list of implemented problems
    pub fn implemented() -> Vec<usize> {
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 67]
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
