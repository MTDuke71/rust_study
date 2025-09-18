use mission1::Stack;

fn show_move_error() {
    let mut stack = Stack::new();
    
    let text = String::from("This will be moved");
    println!("Before move: {}", text);
    
    stack.push(text);  // text is moved here
    
    // Uncomment the next line to see the compile error:
    //println!("After move: {}", text);  // ERROR!
    
    println!("Stack now owns the string: {:?}", stack);
}

fn main() {
    show_move_error();
}