//! # Problem 17: Number Letter Counts
//!
//! If the numbers 1 to 5 are written out in words: one, two, three, four,
//! five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
//!
//! If all the numbers from 1 to 1,000 (one thousand) inclusive were written
//! out in words, how many letters would be used?
//!
//! NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and
//! forty-two) contains 23 letters and 115 (one hundred and fifteen) contains
//! 20 letters. The use of "and" when writing out numbers is in accordance
//! with British usage.
//!
//! ## Mathematical Foundation
//!
//! Rather than generating strings, we use **letter-count lookup tables** and
//! count directly by arithmetic structure (ones, tens, hundreds).
//!
//! ### Letter counts for base words
//! - Ones/teens: one=3, two=3, three=5, four=4, five=4, six=3, seven=5,
//!   eight=5, nine=4, ten=3, eleven=6, twelve=6, thirteen=8, fourteen=8,
//!   fifteen=7, sixteen=7, seventeen=9, eighteen=8, nineteen=8
//! - Tens: twenty=6, thirty=6, forty=5, fifty=5, sixty=5, seventy=7, eighty=6, ninety=6
//! - hundred=7, thousand=8, and=3
//!
//! ### Analytical sum (1–1000)
//! - Sum(1–9)   = 36
//! - Sum(10–19) = 70
//! - Sum(20–99) = tens×10 each + ones(1–9)×8 = 460 + 288 = 748
//! - Sum(1–99)  = 854
//! - Sum(100–999): each hundreds group h contributes
//!   100×(letters(h)+7) + 99×3 ("and") + 854 (remainders 1–99)
//!   = 9×700 + 100×36 + 9×297 + 9×854 = 20,259
//! - 1000 ("one thousand") = 3 + 8 = 11
//! - **Total = 854 + 20,259 + 11 = 21,124**
//!
//! ## Complexity
//!
//! - Time: O(n) — one lookup per number
//! - Space: O(1) — fixed-size lookup tables

/// Letter counts for 0–19 (index = number, 0 is unused/zero).
const ONES: [u32; 20] = [
    0, // 0  (unused)
    3, // 1  one
    3, // 2  two
    5, // 3  three
    4, // 4  four
    4, // 5  five
    3, // 6  six
    5, // 7  seven
    5, // 8  eight
    4, // 9  nine
    3, // 10 ten
    6, // 11 eleven
    6, // 12 twelve
    8, // 13 thirteen
    8, // 14 fourteen
    7, // 15 fifteen
    7, // 16 sixteen
    9, // 17 seventeen
    8, // 18 eighteen
    8, // 19 nineteen
];

/// Letter counts for tens 0–90 (index = tens digit, 0 and 1 are unused).
const TENS: [u32; 10] = [
    0, // 0  (unused)
    0, // 1  (covered by ONES 10–19)
    6, // 2  twenty
    6, // 3  thirty
    5, // 4  forty
    5, // 5  fifty
    5, // 6  sixty
    7, // 7  seventy
    6, // 8  eighty
    6, // 9  ninety
];

/// Count the letters in the English word representation of `n` (1–1000).
///
/// Spaces and hyphens are not counted. Uses British convention: "and" is
/// inserted between hundreds and the remainder (e.g. "one hundred and five").
///
/// # Examples
/// ```
/// use project_euler::problems::p017::letter_count;
/// assert_eq!(letter_count(1), 3);   // "one"
/// assert_eq!(letter_count(342), 23); // "threehundredandfortytwo"
/// assert_eq!(letter_count(115), 20); // "onehundredandfifteen"
/// assert_eq!(letter_count(1000), 11); // "onethousand"
/// ```
pub fn letter_count(n: u32) -> u32 {
    assert!((1..=1000).contains(&n), "n must be in 1..=1000");

    if n == 1000 {
        return 11; // "one" (3) + "thousand" (8)
    }

    let mut count = 0;
    let mut remainder = n;

    if remainder >= 100 {
        count += ONES[(remainder / 100) as usize] + 7; // "X hundred"
        remainder %= 100;
        if remainder > 0 {
            count += 3; // "and"
        }
    }

    if remainder >= 20 {
        count += TENS[(remainder / 10) as usize];
        remainder %= 10;
    }

    if remainder > 0 {
        count += ONES[remainder as usize];
    }

    count
}

/// Count total letters for all numbers from 1 to `limit` inclusive.
///
/// # Examples
/// ```
/// use project_euler::problems::p017::total_letter_count;
/// assert_eq!(total_letter_count(5), 19); // one+two+three+four+five
/// ```
pub fn total_letter_count(limit: u32) -> u64 {
    (1..=limit).map(|n| letter_count(n) as u64).sum()
}

/// Solve Problem 17: total letters for 1 to 1000.
pub fn solve() -> u64 {
    total_letter_count(1000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ones() {
        assert_eq!(letter_count(1), 3); // one
        assert_eq!(letter_count(2), 3); // two
        assert_eq!(letter_count(3), 5); // three
        assert_eq!(letter_count(4), 4); // four
        assert_eq!(letter_count(5), 4); // five
    }

    #[test]
    fn test_teens() {
        assert_eq!(letter_count(11), 6); // eleven
        assert_eq!(letter_count(12), 6); // twelve
        assert_eq!(letter_count(15), 7); // fifteen
        assert_eq!(letter_count(17), 9); // seventeen
    }

    #[test]
    fn test_tens() {
        assert_eq!(letter_count(20), 6); // twenty
        assert_eq!(letter_count(40), 5); // forty
        assert_eq!(letter_count(70), 7); // seventy
    }

    #[test]
    fn test_compound() {
        assert_eq!(letter_count(21), 9);  // twentyone
        assert_eq!(letter_count(99), 10); // ninetynine (6+4)
    }

    #[test]
    fn test_hundreds_from_problem() {
        // From problem statement
        assert_eq!(letter_count(342), 23); // "threehundredandfortytwo"
        assert_eq!(letter_count(115), 20); // "onehundredandfifteen"
    }

    #[test]
    fn test_exact_hundreds() {
        assert_eq!(letter_count(100), 10); // "onehundred" (3+7, no "and")
        assert_eq!(letter_count(200), 10); // "twohundred"
        assert_eq!(letter_count(900), 11); // "ninehundred" (4+7)
    }

    #[test]
    fn test_one_thousand() {
        assert_eq!(letter_count(1000), 11); // "onethousand"
    }

    #[test]
    fn test_example_from_problem() {
        // "one two three four five" = 3+3+5+4+4 = 19
        assert_eq!(total_letter_count(5), 19);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 21124);
    }
}
