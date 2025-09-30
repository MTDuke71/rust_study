fn main() {
    let s = "é)";
    println!("String: '{}' (byte length: {})", s, s.len());
    
    for (byte_idx, ch) in s.char_indices() {
        println!("Char '{}' at byte index {}", ch, byte_idx);
    }
    
    // Character positions (not byte positions)
    for (char_idx, ch) in s.chars().enumerate() {
        println!("Char '{}' at character position {}", ch, char_idx);
    }
}