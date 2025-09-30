//! # Simple Even/Odd Character Splitter
//! 
//! **Quick Reference**: Clean utility functions for splitting strings by index

fn main() {
    println!("🚀 Quick String Splitting Examples");
    println!("==================================\n");
    
    let text = "Hello, World!";
    
    // Method 1: Simple utility function
    let (even, odd) = split_even_odd(text);
    println!("'{}' splits into:", text);
    println!("  Even indices: '{}'", even);
    println!("  Odd indices:  '{}'", odd);
    
    // Method 2: One-liner with tuples
    let result = text.chars()
        .enumerate()
        .fold((String::new(), String::new()), |(mut even, mut odd), (i, c)| {
            if i % 2 == 0 { even.push(c); } else { odd.push(c); }
            (even, odd)
        });
    
    println!("\nOne-liner result: even='{}', odd='{}'", result.0, result.1);
    
    // Method 3: Using collect with partition (most functional)
    let chars: Vec<_> = text.chars().enumerate().collect();
    let (even_pairs, odd_pairs): (Vec<_>, Vec<_>) = chars.into_iter()
        .partition(|(i, _)| i % 2 == 0);
    
    let even: String = even_pairs.into_iter().map(|(_, c)| c).collect();
    let odd: String = odd_pairs.into_iter().map(|(_, c)| c).collect();
    
    println!("Functional result: even='{}', odd='{}'", even, odd);
    
    // Practical examples
    println!("\n🎯 Practical Examples:");
    practical_examples();
}

/// The simplest and most readable approach
fn split_even_odd(s: &str) -> (String, String) {
    let mut even = String::new();
    let mut odd = String::new();
    
    for (i, c) in s.chars().enumerate() {
        if i % 2 == 0 {
            even.push(c);
        } else {
            odd.push(c);
        }
    }
    
    (even, odd)
}

/// Alternative: Return as vectors if you need further processing
fn split_even_odd_vec(s: &str) -> (Vec<char>, Vec<char>) {
    let mut even = Vec::new();
    let mut odd = Vec::new();
    
    for (i, c) in s.chars().enumerate() {
        if i % 2 == 0 {
            even.push(c);
        } else {
            odd.push(c);
        }
    }
    
    (even, odd)
}

/// Practical examples where this might be useful
fn practical_examples() {
    println!("Example 1: Password obfuscation pattern");
    let password = "MyPassword123";
    let (even, odd) = split_even_odd(password);
    println!("  Original: {}", password);
    println!("  Pattern:  {} | {}", even, odd);
    
    println!("\nExample 2: Simple cipher encoding");
    let message = "ATTACK AT DAWN";
    let (even, odd) = split_even_odd(message);
    println!("  Message: {}", message);
    println!("  Encoded: {}{}", odd, even);  // Swap the order
    
    println!("\nExample 3: Data validation check");
    let data = "ABCD1234";
    let (letters, numbers) = split_even_odd(data);
    println!("  Data: {}", data);
    println!("  Even: {} (should be letters)", letters);
    println!("  Odd:  {} (should be numbers)", numbers);
    
    println!("\nExample 4: Alternating pattern analysis");
    let pattern = "xyxyxyxy";
    let (x_positions, y_positions) = split_even_odd(pattern);
    println!("  Pattern: {}", pattern);
    println!("  X chars: {}", x_positions);
    println!("  Y chars: {}", y_positions);
    
    // Bonus: Works with Unicode too!
    println!("\nExample 5: Unicode handling");
    let unicode = "H🦀e🦀l🦀l🦀o";
    let (even, odd) = split_even_odd(unicode);
    println!("  Unicode: {}", unicode);
    println!("  Even: {}", even);
    println!("  Odd:  {}", odd);
}

/// Extension: Split by any custom predicate
fn split_by_predicate<F>(s: &str, predicate: F) -> (String, String)
where
    F: Fn(usize) -> bool,
{
    let mut first = String::new();
    let mut second = String::new();
    
    for (i, c) in s.chars().enumerate() {
        if predicate(i) {
            first.push(c);
        } else {
            second.push(c);
        }
    }
    
    (first, second)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_split_even_odd() {
        assert_eq!(split_even_odd("abcdef"), ("ace".to_string(), "bdf".to_string()));
        assert_eq!(split_even_odd("12345"), ("135".to_string(), "24".to_string()));
        assert_eq!(split_even_odd("A"), ("A".to_string(), "".to_string()));
        assert_eq!(split_even_odd(""), ("".to_string(), "".to_string()));
    }
    
    #[test]
    fn test_unicode() {
        let (even, odd) = split_even_odd("a🦀b🦀c");
        assert_eq!(even, "abc");  // Characters at indices 0, 2, 4
        assert_eq!(odd, "🦀🦀");   // Characters at indices 1, 3
    }
}