//! Library crate that adds one to a number
//!
//! This is a workspace member that can be used by other crates.

/// Adds one to the given number.
///
/// # Examples
///
/// ```
/// use add_one::add_one;
///
/// assert_eq!(add_one(5), 6);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_one() {
        assert_eq!(add_one(5), 6);
        assert_eq!(add_one(0), 1);
        assert_eq!(add_one(-1), 0);
    }
}

