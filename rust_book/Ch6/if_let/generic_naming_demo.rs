//! Generic Type Parameter Naming Demo
//! Shows both conventional and descriptive naming

// Conventional single-letter naming
fn process_conventional<T, U>(_value: T) -> Option<U> {
    // T and U are just conventional names
    None
}

// Descriptive naming
fn process_descriptive<Input, Output>(_input: Input) -> Result<Output, String> {
    Err("Not implemented".to_string())
}

// Mixed approach (common in real code)
fn parse_data<Key, Value, ErrorType>(
    _key: Key, 
    _value: Value
) -> Result<String, ErrorType> {
    Ok("Parsed successfully".to_string())
}

// Standard library style
fn standard_style<T, E>(value: T) -> Result<T, E> {
    Ok(value)
}

fn main() {
    println!("Generic naming is flexible!");
    println!("- T, U, V: Conventional single letters");
    println!("- Key, Value: Descriptive names");
    println!("- Both are equally valid in Rust");
    
    // All of these compile fine:
    let _result1 = process_conventional::<i32, String>(42);
    let _result2 = process_descriptive::<&str, i32>("hello");
    let _result3 = parse_data::<String, i32, String>("key".to_string(), 42);
    let _result4 = standard_style::<i32, String>(100);
    
    println!("All generic naming styles work!");
}
