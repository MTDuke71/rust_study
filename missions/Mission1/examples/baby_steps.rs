use mission1::Stack;

fn main() {
    println!("=== BABY STEPS WITH RUST OWNERSHIP ===");
    
    // Step 1: Create and use a simple value
    println!("\n1. Basic ownership:");
    let my_number = 42;
    println!("I own the number: {}", my_number);
    
    // Numbers are "Copy" types - they get copied, not moved
    let mut stack = Stack::new();
    stack.push(my_number);
    println!("After pushing to stack, I still have: {}", my_number);
    println!("Stack also has: {:?}", stack);
    
    // Step 2: Now try with a String (Move type)
    println!("\n2. String ownership (the tricky one):");
    let mut string_stack = Stack::new();  // Make a separate stack for strings
    let my_text = String::from("Hello World");
    println!("I own the text: {}", my_text);
    
    // Uncomment the next two lines to see what happens:
    // string_stack.push(my_text);  // This MOVES the text to the stack
    // println!("After push: {}", my_text);  // This would be an ERROR!
    
    println!("Text is still mine: {}", my_text);
    
    // Step 3: Let's move it and see
    println!("\n3. Actually moving the string:");
    let another_text = String::from("Goodbye");
    println!("Before move: {}", another_text);
    
    string_stack.push(another_text);  // Move it to the stack
    println!("After move: stack contains {:?}", string_stack);
    // println!("another_text is: {}", another_text);  // This would ERROR!
    
    // Step 4: Get it back
    println!("\n4. Getting ownership back:");
    let got_it_back = string_stack.pop().unwrap();
    println!("Got back from stack: {}", got_it_back);
    println!("Now I own it again!");
    
    // Step 5: Just looking (borrowing)
    println!("\n5. Just peeking (borrowing):");
    string_stack.push(String::from("Peek at me"));
    
    let just_looking = string_stack.peek().unwrap();
    println!("I'm just looking: {}", just_looking);
    println!("Stack still has it: {:?}", string_stack);
    
    // Step 6: Multiple peeks (multiple borrows)
    println!("\n6. Multiple people can look:");
    let look1 = string_stack.peek().unwrap();
    let look2 = string_stack.peek().unwrap();
    println!("Person 1 sees: {}", look1);
    println!("Person 2 sees: {}", look2);
    println!("Everyone sees the same thing!");
    
    println!("\n🎉 You just learned the basics of Rust ownership!");
}

// Uncomment these functions one at a time to practice:

// fn practice_move_errors() {
//     let mut stack = Stack::new();
//     let text = String::from("Will be moved");
//     
//     stack.push(text);
//     println!("{}", text);  // ERROR: use after move
// }

// fn practice_borrow_conflicts() {
//     let mut stack = Stack::new();
//     stack.push(String::from("data"));
//     
//     let reader = stack.peek().unwrap();
//     stack.push(String::from("more"));  // ERROR: can't modify while borrowed
//     println!("{}", reader);
// }