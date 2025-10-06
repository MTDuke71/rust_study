// Mission1_tut Step 6: Real-World Applications
// Apply stack concepts to practical programming problems

fn main() {
    println!("=== Mission1_tut Step 6: Real-World Applications ===\n");
    
    // 1. Bracket Validation
    println!("1. Bracket and Parentheses Validation:");
    demonstrate_bracket_validation();
    
    // 2. Expression Parsing
    println!("\n2. Expression Parsing and Evaluation:");
    demonstrate_expression_parsing();
    
    // 3. Undo/Redo Systems
    println!("\n3. Undo/Redo System Implementation:");
    demonstrate_undo_redo();
    
    // 4. Function Call Simulation
    println!("\n4. Function Call Stack Simulation:");
    demonstrate_call_stack();
    
    // 5. Depth-First Search
    println!("\n5. Depth-First Search with Stack:");
    demonstrate_dfs();
    
    self_assessment();
}

/// Reusable Stack implementation
#[derive(Debug, Clone)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            items: Vec::new(),
        }
    }
    
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.items.len()
    }
}

fn demonstrate_bracket_validation() {
    println!("   Problem: Validate that brackets are properly matched");
    println!("   Valid: '()' '[]' '{{}}' '([{{}}])'");
    println!("   Invalid: '(' '])' '([)]' '((('");
    
    fn validate_brackets(input: &str) -> bool {
        let mut stack: Stack<char> = Stack::new();
        
        for ch in input.chars() {
            match ch {
                '(' | '[' | '{' => {
                    println!("   Pushing opening bracket: '{}'", ch);
                    stack.push(ch);
                }
                ')' | ']' | '}' => {
                    println!("   Found closing bracket: '{}'", ch);
                    
                    let expected = match ch {
                        ')' => '(',
                        ']' => '[', 
                        '}' => '{',
                        _ => unreachable!(),
                    };
                    
                    match stack.pop() {
                        Some(opening) if opening == expected => {
                            println!("   Matched '{}' with '{}'", opening, ch);
                        }
                        Some(opening) => {
                            println!("   Mismatch: expected '{}' but found '{}'", 
                                   get_closing(opening), ch);
                            return false;
                        }
                        None => {
                            println!("   No opening bracket for '{}'", ch);
                            return false;
                        }
                    }
                }
                _ => {
                    // Ignore non-bracket characters
                }
            }
        }
        
        let is_valid = stack.is_empty();
        if !is_valid {
            println!("   Unmatched opening brackets remaining");
        }
        is_valid
    }
    
    fn get_closing(opening: char) -> char {
        match opening {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            _ => '?',
        }
    }
    
    // Test cases
    let test_cases = vec![
        "((()))",     // Valid
        "([{}])",     // Valid  
        "({[]})",     // Valid
        "(((",        // Invalid - unclosed
        "}))",        // Invalid - wrong order
        "([)]",       // Invalid - interleaved
        "",           // Valid - empty
    ];
    
    for test in test_cases {
        println!("\n   Testing: '{}'", test);
        let result = validate_brackets(test);
        println!("   Result: {}\n", if result { "✅ Valid" } else { "❌ Invalid" });
    }
}

fn demonstrate_expression_parsing() {
    println!("   Problem: Parse and evaluate mathematical expressions");
    println!("   Example: '3 + 4 * 2' = 11 (respecting operator precedence)");
    
    #[derive(Debug, Clone)]
    enum Token {
        Number(i32),
        Operator(char),
        LeftParen,
        RightParen,
    }
    
    fn tokenize(expr: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = expr.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            match chars[i] {
                ' ' => i += 1,  // Skip whitespace
                '(' => { tokens.push(Token::LeftParen); i += 1; }
                ')' => { tokens.push(Token::RightParen); i += 1; }
                '+' | '-' | '*' | '/' => { 
                    tokens.push(Token::Operator(chars[i])); 
                    i += 1; 
                }
                '0'..='9' => {
                    let mut num = 0;
                    while i < chars.len() && chars[i].is_ascii_digit() {
                        num = num * 10 + (chars[i] as i32 - '0' as i32);
                        i += 1;
                    }
                    tokens.push(Token::Number(num));
                }
                _ => i += 1,  // Skip unknown characters
            }
        }
        
        tokens
    }
    
    // Shunting Yard algorithm using stacks
    fn evaluate_expression(expr: &str) -> Result<i32, String> {
        let tokens = tokenize(expr);
        let mut output: Stack<i32> = Stack::new();
        let mut operators: Stack<char> = Stack::new();
        
        fn precedence(op: char) -> i32 {
            match op {
                '+' | '-' => 1,
                '*' | '/' => 2,
                _ => 0,
            }
        }
        
        fn apply_operator(output: &mut Stack<i32>, op: char) -> Result<(), String> {
            let b = output.pop().ok_or("Missing operand")?;
            let a = output.pop().ok_or("Missing operand")?;
            
            let result = match op {
                '+' => a + b,
                '-' => a - b,
                '*' => a * b,
                '/' => {
                    if b == 0 { return Err("Division by zero".to_string()); }
                    a / b
                }
                _ => return Err(format!("Unknown operator: {}", op)),
            };
            
            output.push(result);
            Ok(())
        }
        
        for token in tokens {
            match token {
                Token::Number(n) => {
                    println!("   Push number: {}", n);
                    output.push(n);
                }
                Token::Operator(op) => {
                    println!("   Process operator: {}", op);
                    
                    // Apply higher precedence operators first
                    while let Some(&top_op) = operators.peek() {
                        if top_op != '(' && precedence(top_op) >= precedence(op) {
                            let popped_op = operators.pop().unwrap();
                            println!("   Apply operator: {}", popped_op);
                            apply_operator(&mut output, popped_op)?;
                        } else {
                            break;
                        }
                    }
                    
                    operators.push(op);
                }
                Token::LeftParen => {
                    println!("   Push left parenthesis");
                    operators.push('(');
                }
                Token::RightParen => {
                    println!("   Process right parenthesis");
                    
                    // Apply operators until left parenthesis
                    while let Some(op) = operators.pop() {
                        if op == '(' {
                            break;
                        }
                        println!("   Apply operator: {}", op);
                        apply_operator(&mut output, op)?;
                    }
                }
            }
        }
        
        // Apply remaining operators
        while let Some(op) = operators.pop() {
            println!("   Apply remaining operator: {}", op);
            apply_operator(&mut output, op)?;
        }
        
        output.pop().ok_or_else(|| "No result".to_string())
    }
    
    // Test expressions
    let expressions = vec![
        "3 + 4 * 2",      // Should be 11
        "(3 + 4) * 2",    // Should be 14
        "10 - 2 * 3",     // Should be 4
        "8 / 2 + 3",      // Should be 7
    ];
    
    for expr in expressions {
        println!("\n   Evaluating: '{}'", expr);
        match evaluate_expression(expr) {
            Ok(result) => println!("   Result: {}", result),
            Err(e) => println!("   Error: {}", e),
        }
    }
}

fn demonstrate_undo_redo() {
    println!("   Problem: Implement undo/redo functionality for a text editor");
    
    #[derive(Debug, Clone)]
    enum Action {
        Insert { pos: usize, text: String },
        Delete { pos: usize, text: String },
        #[allow(dead_code)]
        Replace { pos: usize, old: String, new: String },
    }
    
    struct TextEditor {
        content: String,
        undo_stack: Stack<Action>,
        redo_stack: Stack<Action>,
    }
    
    impl TextEditor {
        fn new() -> Self {
            TextEditor {
                content: String::new(),
                undo_stack: Stack::new(),
                redo_stack: Stack::new(),
            }
        }
        
        fn insert(&mut self, pos: usize, text: &str) {
            println!("   Insert '{}' at position {}", text, pos);
            
            // Clear redo stack on new action
            self.redo_stack = Stack::new();
            
            // Save action for undo
            self.undo_stack.push(Action::Insert {
                pos,
                text: text.to_string(),
            });
            
            // Apply the action
            self.content.insert_str(pos, text);
            println!("   Content: '{}'", self.content);
        }
        
        fn delete(&mut self, pos: usize, len: usize) {
            if pos + len <= self.content.len() {
                let deleted_text = self.content[pos..pos + len].to_string();
                println!("   Delete '{}' at position {}", deleted_text, pos);
                
                // Clear redo stack on new action
                self.redo_stack = Stack::new();
                
                // Save action for undo
                self.undo_stack.push(Action::Delete {
                    pos,
                    text: deleted_text,
                });
                
                // Apply the action
                self.content.drain(pos..pos + len);
                println!("   Content: '{}'", self.content);
            }
        }
        
        fn undo(&mut self) -> bool {
            if let Some(action) = self.undo_stack.pop() {
                println!("   Undoing: {:?}", action);
                
                match action.clone() {
                    Action::Insert { pos, text } => {
                        // Undo insert by deleting
                        self.content.drain(pos..pos + text.len());
                    }
                    Action::Delete { pos, text } => {
                        // Undo delete by inserting
                        self.content.insert_str(pos, &text);
                    }
                    Action::Replace { pos, old, new: _ } => {
                        // Undo replace by restoring original
                        self.content.replace_range(pos..pos + old.len(), &old);
                    }
                }
                
                // Save for redo
                self.redo_stack.push(action);
                println!("   Content after undo: '{}'", self.content);
                true
            } else {
                println!("   Nothing to undo");
                false
            }
        }
        
        fn redo(&mut self) -> bool {
            if let Some(action) = self.redo_stack.pop() {
                println!("   Redoing: {:?}", action);
                
                match action.clone() {
                    Action::Insert { pos, text } => {
                        self.content.insert_str(pos, &text);
                    }
                    Action::Delete { pos, text } => {
                        self.content.drain(pos..pos + text.len());
                    }
                    Action::Replace { pos, old: _, new } => {
                        self.content.replace_range(pos..pos + new.len(), &new);
                    }
                }
                
                // Save for undo again
                self.undo_stack.push(action);
                println!("   Content after redo: '{}'", self.content);
                true
            } else {
                println!("   Nothing to redo");
                false
            }
        }
    }
    
    // Demo the text editor
    let mut editor = TextEditor::new();
    
    editor.insert(0, "Hello");
    editor.insert(5, " World");
    editor.insert(11, "!");
    editor.delete(5, 6);  // Remove " World"
    
    println!("\n   Undo operations:");
    editor.undo();  // Restore " World"
    editor.undo();  // Remove "!"
    
    println!("\n   Redo operations:");
    editor.redo();   // Add "!" back
}

fn demonstrate_call_stack() {
    println!("   Problem: Simulate function call stack for recursive functions");
    
    #[derive(Debug)]
    struct StackFrame {
        function_name: String,
        local_vars: std::collections::HashMap<String, i32>,
        #[allow(dead_code)]
        return_address: String,
    }
    
    struct CallStack {
        frames: Stack<StackFrame>,
    }
    
    impl CallStack {
        fn new() -> Self {
            CallStack {
                frames: Stack::new(),
            }
        }
        
        fn call_function(&mut self, name: &str, params: Vec<(&str, i32)>) {
            let mut local_vars = std::collections::HashMap::new();
            for (param_name, value) in params {
                local_vars.insert(param_name.to_string(), value);
            }
            
            let frame = StackFrame {
                function_name: name.to_string(),
                local_vars,
                return_address: format!("after_{}", name),
            };
            
            println!("   Calling function: {}", name);
            self.frames.push(frame);
            self.print_stack();
        }
        
        fn return_from_function(&mut self) -> Option<StackFrame> {
            if let Some(frame) = self.frames.pop() {
                println!("   Returning from: {}", frame.function_name);
                self.print_stack();
                Some(frame)
            } else {
                println!("   Stack underflow - no function to return from");
                None
            }
        }
        
        fn print_stack(&self) {
            println!("   Call stack (top to bottom):");
            if self.frames.is_empty() {
                println!("   [empty]");
            } else {
                for (i, frame) in self.frames.items.iter().rev().enumerate() {
                    println!("   {}: {} {:?}", i, frame.function_name, frame.local_vars);
                }
            }
            println!();
        }
    }
    
    // Simulate factorial(3) call stack
    let mut call_stack = CallStack::new();
    
    println!("   Simulating factorial(3) recursive calls:");
    call_stack.call_function("factorial", vec![("n", 3)]);
    call_stack.call_function("factorial", vec![("n", 2)]);
    call_stack.call_function("factorial", vec![("n", 1)]);
    call_stack.call_function("factorial", vec![("n", 0)]);
    
    println!("   Now returning with results:");
    call_stack.return_from_function();  // factorial(0) returns 1
    call_stack.return_from_function();  // factorial(1) returns 1
    call_stack.return_from_function();  // factorial(2) returns 2
    call_stack.return_from_function();  // factorial(3) returns 6
}

fn demonstrate_dfs() {
    println!("   Problem: Depth-First Search traversal using explicit stack");
    
    type Graph = std::collections::HashMap<i32, Vec<i32>>;
    
    fn dfs_iterative(graph: &Graph, start: i32) -> Vec<i32> {
        let mut visited = std::collections::HashSet::new();
        let mut stack: Stack<i32> = Stack::new();
        let mut result = Vec::new();
        
        stack.push(start);
        
        while let Some(node) = stack.pop() {
            if !visited.contains(&node) {
                println!("   Visiting node: {}", node);
                visited.insert(node);
                result.push(node);
                
                // Add neighbors to stack (in reverse order for consistent traversal)
                if let Some(neighbors) = graph.get(&node) {
                    for &neighbor in neighbors.iter().rev() {
                        if !visited.contains(&neighbor) {
                            println!("   Adding to stack: {}", neighbor);
                            stack.push(neighbor);
                        }
                    }
                }
                
                println!("   Stack state: {:?}", stack.items);
            }
        }
        
        result
    }
    
    // Create a simple graph
    let mut graph: Graph = std::collections::HashMap::new();
    graph.insert(1, vec![2, 3]);
    graph.insert(2, vec![4, 5]);
    graph.insert(3, vec![6]);
    graph.insert(4, vec![]);
    graph.insert(5, vec![]);
    graph.insert(6, vec![]);
    
    println!("   Graph: {:?}", graph);
    
    let traversal_result = dfs_iterative(&graph, 1);
    println!("   DFS traversal order: {:?}", traversal_result);
}

fn self_assessment() {
    println!("\n=== Self-Assessment Questions ===");
    println!("After completing Step 6, you should understand:");
    println!("1. How does bracket validation use LIFO behavior?");
    println!("2. Why are stacks essential for parsing expressions?");
    println!("3. How do undo/redo systems leverage stack operations?");
    println!("4. What role do stacks play in function call management?");
    println!("5. How can stacks replace recursion in algorithms like DFS?");
    
    println!("\n=== Real-World Applications ===");
    println!("✅ Compilers and interpreters - expression parsing");
    println!("✅ Text editors - undo/redo functionality");
    println!("✅ Web browsers - back button navigation");
    println!("✅ Function calls - managing local variables and return addresses");
    println!("✅ Graph algorithms - DFS traversal and pathfinding");
    
    println!("\n=== Implementation Patterns ===");
    println!("• Validation: Push opening elements, pop and match closing elements");
    println!("• Parsing: Use operator precedence with two stacks (Shunting Yard)");
    println!("• Undo/Redo: Two stacks for forward and backward action history");
    println!("• DFS: Stack replaces recursive call stack for iterative implementation");
    
    println!("\n=== Next Step Preview ===");
    println!("Step 7 will integrate all concepts into a production-ready stack:");
    println!("• Complete API with comprehensive error handling");
    println!("• Performance optimizations and capacity management");
    println!("• Extensive testing matching Mission1 requirements");
    println!("• Documentation and benchmarking");
    println!("Run: cargo run --example step7_final_project");
}