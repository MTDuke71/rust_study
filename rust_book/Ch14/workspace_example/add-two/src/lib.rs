//! Library crate that adds two to a number
//!
//! This workspace member uses another workspace member (add-one).


/// Adds two to the given number.
///
/// This function uses `add_one` twice to demonstrate
/// workspace member dependencies.
///
/// # Examples
///
/// ```
/// use add_two::add_two;
///
/// assert_eq!(add_two(5), 7);
/// ```
pub fn add_two(x: i32) -> i32 {
    add_one::add_one(add_one::add_one(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(5), 7);
        assert_eq!(add_two(0), 2);
        assert_eq!(add_two(-1), 1);
    }
}

