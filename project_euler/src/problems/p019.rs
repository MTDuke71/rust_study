//! # Problem 19: Counting Sundays
//!
//! How many Sundays fell on the first of the month during the twentieth
//! century (1 Jan 1901 to 31 Dec 2000)?
//!
//! ## Mathematical Foundation
//!
//! Pure **modular arithmetic** on day-of-week values. Given a known anchor
//! (1 Jan 1900 = Monday), advance month-by-month:
//!
//! ```text
//! next_first = (current_first + days_in_month) mod 7
//! ```
//!
//! The day-of-week cycles with period 7, so we only track the residue.
//!
//! ### Leap Year Rules
//!
//! A year is a leap year if:
//! - Divisible by 4, **except**
//! - Century years (divisible by 100), **unless**
//! - Also divisible by 400
//!
//! Examples: 1900 = NOT leap (÷100, not ÷400), 2000 = leap (÷400)
//!
//! See [[math-foundations/project-euler-p019]] for full analysis.
//!
//! ## Complexity
//!
//! - Time: O(n) where n = number of months in range (1,200)
//! - Space: O(1)

/// Check if a year is a leap year.
///
/// # Examples
/// ```
/// use project_euler::problems::p019::is_leap_year;
/// assert!(!is_leap_year(1900));  // century, not ÷400
/// assert!(is_leap_year(2000));   // ÷400
/// assert!(is_leap_year(1904));   // ÷4, not century
/// assert!(!is_leap_year(1901));  // not ÷4
/// ```
pub fn is_leap_year(year: u32) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

/// Return the number of days in a given month (1-indexed) for a given year.
///
/// # Examples
/// ```
/// use project_euler::problems::p019::days_in_month;
/// assert_eq!(days_in_month(1, 2000), 31);  // January
/// assert_eq!(days_in_month(2, 2000), 29);  // Feb leap year
/// assert_eq!(days_in_month(2, 1900), 28);  // Feb non-leap
/// assert_eq!(days_in_month(4, 2000), 30);  // April
/// ```
pub fn days_in_month(month: u32, year: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => panic!("invalid month: {month}"),
    }
}

/// Count Sundays that fall on the first of the month in a year range.
///
/// Uses the anchor: 1 Jan 1900 = Monday (day 1, where Sunday = 0).
/// Advances month-by-month tracking `day_of_week % 7`.
///
/// # Examples
/// ```
/// use project_euler::problems::p019::count_first_sundays;
/// assert_eq!(count_first_sundays(1901, 2000), 171);
/// ```
pub fn count_first_sundays(start_year: u32, end_year: u32) -> u32 {
    // Anchor: 1 Jan 1900 = Monday
    // Day encoding: 0=Sun, 1=Mon, 2=Tue, ..., 6=Sat
    let mut day_of_week: u32 = 1; // Monday
    let mut count = 0;

    for year in 1900..=end_year {
        for month in 1..=12 {
            // Count if we're in the target range and it's Sunday
            if year >= start_year && day_of_week == 0 {
                count += 1;
            }
            // Advance to the 1st of the next month
            day_of_week = (day_of_week + days_in_month(month, year)) % 7;
        }
    }

    count
}

/// Solve Problem 19: Sundays on the 1st, Jan 1901 – Dec 2000.
pub fn solve() -> u64 {
    count_first_sundays(1901, 2000) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leap_years() {
        assert!(!is_leap_year(1900)); // century, not ÷400
        assert!(is_leap_year(2000));  // ÷400
        assert!(is_leap_year(1904));  // ÷4
        assert!(!is_leap_year(1901)); // not ÷4
        assert!(!is_leap_year(1800)); // century
        assert!(is_leap_year(1600));  // ÷400
    }

    #[test]
    fn test_days_in_month() {
        assert_eq!(days_in_month(1, 2000), 31);
        assert_eq!(days_in_month(2, 2000), 29); // leap
        assert_eq!(days_in_month(2, 1900), 28); // non-leap
        assert_eq!(days_in_month(4, 2000), 30);
        assert_eq!(days_in_month(12, 2000), 31);
    }

    #[test]
    fn test_jan_1_1900_is_monday() {
        // Verify our anchor: 1 Jan 1900 = Monday
        // If we count 0 Sundays in just Jan 1900, that confirms
        // Jan 1 wasn't Sunday. We can also verify known dates.
        // 7 Jan 1900 should be Sunday (Monday + 6 days)
        // So Feb 1 1900 = Monday + 31 days = Monday + 3 mod 7 = Thursday (day 4)
        let days_jan = days_in_month(1, 1900);
        assert_eq!((1 + days_jan) % 7, 4); // Thursday
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 171);
    }
}
