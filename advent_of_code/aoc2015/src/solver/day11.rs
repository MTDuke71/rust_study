use anyhow::Result;

/// AoC 2015 Day 11: Corporate Policy
/// Password generation with validation rules
/// 
/// ## Problem Description
/// Santa needs a new password that follows corporate policy:
/// 1. Must be exactly 8 lowercase letters
/// 2. Incrementing wraps: xx → xy → xz → ya → yb (like base-26 counting)
/// 3. Must include one increasing straight of at least 3 letters (abc, bcd, cde, etc.)
/// 4. May NOT contain 'i', 'o', or 'l' (confusing characters)
/// 5. Must contain at least two different, non-overlapping pairs (aa, bb, zz, etc.)
/// 
/// ## Examples
/// - "hijklmmn": Has straight (hij) but contains 'i' and 'l' ❌
/// - "abbceffg": Has pairs (bb, ff) but no straight ❌
/// - "abbcegjk": Has only one pair (bb) ❌
/// - "abcdefgh" → next valid: "abcdffaa" ✅
/// - "ghijklmn" → next valid: "ghjaabcc" ✅ (skips 'i')
/// 
/// ## Part 1
/// Starting with your puzzle input, find the next valid password.
/// 
/// ## Part 2
/// Find the next valid password after the one from Part 1.

// TODO: Implement solve_part1
// 1. Parse the input to get the current password
// 2. Increment the password repeatedly until you find a valid one
// 3. Return the new valid password
pub fn solve_part1(input: &str) -> Result<String> {
    println!("🎄 Day 11 Part 1: Find next valid password");
    
    let mut password = input.trim().to_string();
    
    // TODO: Implement the password search
    // Loop: increment → validate → return when valid
    
    loop {
        password = increment_password(&password);
        
        // OPTIMIZATION: Skip entire ranges with forbidden chars
        if !has_no_forbidden_chars(&password) {
            password = skip_forbidden_char(&password);
            continue;
        }
        
        if is_valid_password(&password) {
            return Ok(password);
        }
    }
}

// TODO: Implement solve_part2
// Same as part1, but start from the password found in part1
// Find the NEXT valid password after that one
pub fn solve_part2(input: &str) -> Result<String> {
    println!("🎄 Day 11 Part 2: Find next valid password after Part 1");
    
    // TODO: Get the Part 1 result, then find the next valid password
    
    let part1_password = solve_part1(input)?;
    
    // Increment once to move past Part 1 answer
    let mut password = increment_password(&part1_password);
    
    // Find next valid password
    loop {
        if is_valid_password(&password) {
            return Ok(password);
        }
        password = increment_password(&password);
    }
}

// ============================================================================
// Password Incrementing (Base-26 Counting)
// ============================================================================

/// Increments a password string like counting in base-26
/// 
/// ## Algorithm
/// 1. Start from the rightmost character
/// 2. If it's not 'z', increment it and done
/// 3. If it's 'z', wrap to 'a' and carry to the left
/// 4. Repeat until no carry needed
/// 
/// ## Examples
/// ```
/// # use aoc2015::solver::day11::increment_password;
/// assert_eq!(increment_password("xx"), "xy");
/// assert_eq!(increment_password("xy"), "xz");
/// assert_eq!(increment_password("xz"), "ya");  // Wrap!
/// assert_eq!(increment_password("zz"), "aa");  // Double wrap!
/// ```
/// 
/// ## TODO: Implementation Steps
/// 1. Convert string to Vec<char> for mutability
/// 2. Start from the last character (rightmost)
/// 3. Loop backwards through characters:
///    - If char < 'z': increment and return
///    - If char == 'z': set to 'a' and continue (carry)
/// 4. Join chars back into string
pub fn increment_password(password: &str) -> String {
    let mut chars: Vec<char> = password.chars().collect();
    // Start from rightmost character
    for i in (0..chars.len()).rev() {
        if chars[i] != 'z' {
            // Simple increment: 'a' → 'b', 'b' → 'c', etc.
            chars[i] = ((chars[i] as u8) + 1) as char;
            break;  // Done!
        } else {
            // Wrap 'z' → 'a' and carry to next position
            chars[i] = 'a';
            // Continue loop to carry left
        }
    }
    chars.into_iter().collect()
}
    // TODO: Implement base-26 incrementing
    // 
    // Hints:
    // - Use password.chars().collect::<Vec<char>>()
    // - Loop from len-1 down to 0
    // - Handle wrapping 'z' → 'a' with carry
    // - Use iter().collect::<String>() to convert back
    

// ============================================================================
// Password Validation Rules
// ============================================================================

/// Validates if a password meets all corporate policy requirements
/// 
/// ## Rules
/// 1. Must have an increasing straight (3+ consecutive letters)
/// 2. Must NOT contain 'i', 'o', or 'l'
/// 3. Must have at least 2 different, non-overlapping pairs
/// 
/// ## Examples
/// ```
/// # use aoc2015::solver::day11::is_valid_password;
/// assert_eq!(is_valid_password("hijklmmn"), false);  // Contains 'i', 'l'
/// assert_eq!(is_valid_password("abbceffg"), false);  // No straight
/// assert_eq!(is_valid_password("abbcegjk"), false);  // Only one pair
/// assert_eq!(is_valid_password("abcdffaa"), true);   // All rules pass ✅
/// ```
/// 
/// ## TODO: Implementation
/// Call all three validation functions - all must return true
pub fn is_valid_password(password: &str) -> bool {
    // TODO: Check all three requirements
    // 
    // Optimization tip: Check cheapest rules first!
    // - has_no_forbidden_chars() is O(n) and likely to fail
    // - has_two_pairs() is O(n) 
    // - has_increasing_straight() is O(n)
    
    // Rule 2: Most likely to fail, cheapest to check
    if !has_no_forbidden_chars(password) {
        return false;
    }
    
    // Rule 3: Medium likelihood
    if !has_two_pairs(password) {
        return false;
    }
    
    // Rule 1: Least likely to fail
    if !has_increasing_straight(password) {
        return false;
    }
    
    true
}

/// Rule 1: Password must include one increasing straight of at least 3 letters
/// 
/// ## Examples
/// - "abc" → true (abc is a straight)
/// - "bcd" → true (bcd is a straight)
/// - "abd" → false (skips 'c')
/// - "xyz" → true (xyz is a straight)
/// - "hijklmmn" → true (hij is a straight)
/// 
/// ## TODO: Implementation Steps
/// 1. Convert string to Vec<char> or iterate with windows
/// 2. Check every 3-character window
/// 3. For each window, check if char[i+1] == char[i]+1 && char[i+2] == char[i]+2
/// 4. Return true if any window is a straight
/// 
/// ## Optimization Ideas
/// - Use char as u8 for arithmetic: 'a' as u8 = 97
/// - Early return when straight found
pub fn has_increasing_straight(password: &str) -> bool {
    // TODO: Implement increasing straight detection
    // 
    // Hints:
    // - Use .as_bytes() for easier arithmetic
    // - Check bytes[i+1] == bytes[i] + 1 && bytes[i+2] == bytes[i] + 2
    // - Use windows(3) or manual indexing
    
    let bytes = password.as_bytes();
    for i in 0..bytes.len().saturating_sub(2) {
        if bytes[i + 1] == bytes[i] + 1 && bytes[i + 2] == bytes[i] + 2 {
            return true;
        }
    }
    false
}

/// Rule 2: Password must NOT contain 'i', 'o', or 'l' (confusing characters)
/// 
/// ## Examples
/// - "abcdefgh" → true (no forbidden chars)
/// - "hijklmmn" → false (contains 'i' and 'l')
/// - "ghjaabcc" → true (no forbidden chars)
/// 
/// ## TODO: Implementation
/// Check if any character in the password is 'i', 'o', or 'l'
/// Return true only if NONE of these characters are present
/// 
/// ## Optimization Opportunity
/// This is your BEST place to skip ahead when generating passwords!
/// If you find 'i' at position 3, you can skip to the next valid range
pub fn has_no_forbidden_chars(password: &str) -> bool {
    // TODO: Implement forbidden character check
    // 
    // Hints:
    // - Use .chars().any() or .contains()
    // - Check for 'i', 'o', and 'l'
    // - Return true if NONE are found
    !password.chars().any(|c| c == 'i' || c == 'o' || c == 'l')
}

/// Rule 3: Password must contain at least two different, non-overlapping pairs
/// 
/// ## Examples
/// - "aabb" → true (aa and bb)
/// - "aabcc" → true (aa and cc, or bb and cc)
/// - "aabaa" → false (aa appears twice but counts as one pair)
/// - "abbceffg" → true (bb and ff)
/// - "abbcegjk" → false (only bb)
/// 
/// ## TODO: Implementation Steps
/// 1. Iterate through password characters
/// 2. When you find a pair (char[i] == char[i+1]), record it
/// 3. Skip the next character to avoid overlapping
/// 4. Count distinct pairs found
/// 5. Return true if count >= 2
/// 
/// ## Edge Cases
/// - "aaa" contains one pair (aa), not two
/// - "aaaa" contains two pairs (aa at position 0, aa at position 2)
pub fn has_two_pairs(password: &str) -> bool {

    let chars: Vec<char> = password.chars().collect();
    let mut pair_count = 0;
    let mut i = 0;
    
    while i < chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            pair_count += 1;
            i += 2;  // Skip next to avoid overlap
        } else {
            i += 1;
        }
    }    
    pair_count >= 2
}
    // TODO: Implement pair counting
    // 
    // Hints:
    // - Use a counter and iterate with index
    // - When pair found, increment counter and skip next char
    // - Use i += 2 to skip overlapping
    // - Return count >= 2
    


// ============================================================================
// Advanced Optimization (Optional)
// ============================================================================

/// OPTIONAL: Skip to the next valid password range when forbidden char found
/// 
/// ## Optimization Strategy
/// If we find a forbidden character ('i', 'o', or 'l') at position N:
/// - Replace it with the next valid character
/// - Set all positions to the right to 'a'
/// 
/// ## Example
/// "abci" → "abcj" + "aaa" → "abcjaaa"
/// This skips thousands of invalid passwords!
/// 
/// ## When to Use
/// Call this in your main loop when has_no_forbidden_chars() returns false
pub fn skip_forbidden_char(password: &str) -> String {
    // TODO: OPTIONAL optimization
    // 
    // 1. Find the first forbidden char
    // 2. Replace with next letter: 'i'→'j', 'o'→'p', 'l'→'m'
    // 3. Set everything to the right to 'a'
    // 
    // This can speed up your solution significantly!
    
    let mut chars: Vec<char> = password.chars().collect();
    
    for i in 0..chars.len() {
        if chars[i] == 'i' || chars[i] == 'o' || chars[i] == 'l' {
            // Replace with next valid letter
            chars[i] = match chars[i] {
                'i' => 'j',
                'o' => 'p',
                'l' => 'm',
                _ => chars[i],
            };
            
            // Reset everything to the right to 'a'
            for j in (i + 1)..chars.len() {
                chars[j] = 'a';
            }
            
            break;
        }
    }
    
    chars.into_iter().collect()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Incrementing tests
    #[test]
    fn test_increment_basic() {
        assert_eq!(increment_password("xx"), "xy");
        assert_eq!(increment_password("xy"), "xz");
    }

    #[test]
    fn test_increment_wrap() {
        assert_eq!(increment_password("xz"), "ya");
        assert_eq!(increment_password("azz"), "baa");
    }

    // Straight tests
    #[test]
    fn test_has_straight() {
        assert!(has_increasing_straight("abc"));
        assert!(has_increasing_straight("hijklmmn"));
        assert!(has_increasing_straight("abcdffaa"));
    }

    #[test]
    fn test_no_straight() {
        assert!(!has_increasing_straight("abd"));  // Skips 'c'
        assert!(!has_increasing_straight("abbceffg"));
    }

    // Forbidden chars tests
    #[test]
    fn test_no_forbidden() {
        assert!(has_no_forbidden_chars("abcdefgh"));
        assert!(has_no_forbidden_chars("ghjaabcc"));
    }

    #[test]
    fn test_has_forbidden() {
        assert!(!has_no_forbidden_chars("hijklmmn"));  // Has 'i' and 'l'
        assert!(!has_no_forbidden_chars("ghijklmn"));  // Has 'i', 'l'
    }

    // Pairs tests
    #[test]
    fn test_has_two_pairs() {
        assert!(has_two_pairs("aabb"));
        assert!(has_two_pairs("abbceffg"));
        assert!(has_two_pairs("ghjaabcc"));
    }

    #[test]
    fn test_not_enough_pairs() {
        assert!(!has_two_pairs("abbcegjk"));  // Only one pair
        assert!(!has_two_pairs("aaa"));       // Only one pair
    }

    // Full validation tests
    #[test]
    fn test_example_failures() {
        assert!(!is_valid_password("hijklmmn"));  // Has straight but forbidden chars
        assert!(!is_valid_password("abbceffg"));  // Has pairs but no straight
        assert!(!is_valid_password("abbcegjk"));  // Only one pair
    }

    #[test]
    fn test_example_success() {
        assert!(is_valid_password("abcdffaa"));  // All rules pass
        assert!(is_valid_password("ghjaabcc"));  // All rules pass
    }
}
