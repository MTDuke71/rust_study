// A generic stack data structure that demonstrates Rust's ownership and borrowing concepts
// 
// This implementation fulfills 5 key requirements:
// REQ-1: Generic type support - works with any type T
// REQ-2: Push in amortized O(1) time
// REQ-3: Pop transfers ownership in O(1) time  
// REQ-4: No access to values after they've been popped (memory safety)
// REQ-5: Safe borrowing with peek() and peek_mut()

/// A Last-In-First-Out (LIFO) stack data structure
/// 
/// # Type Parameters
/// * `T` - The type of elements stored in the stack (REQ-1: Generic support)
/// 
/// # Memory Safety
/// This stack provides compile-time memory safety through Rust's ownership system:
/// - Values are moved in/out, preventing use-after-free bugs
/// - Borrowing rules prevent data races and memory corruption
#[derive(Debug, Default)]
pub struct Stack<T> {
    /// Internal storage using Vec<T> for dynamic sizing and efficient operations
    /// Vec provides the amortized O(1) push and O(1) pop we need
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Creates a new empty stack
    /// 
    /// # Examples
    /// ```
    /// use mission1::Stack;
    /// let mut stack: Stack<i32> = Stack::new();
    /// assert!(stack.is_empty());
    /// ```
    pub fn new() -> Self { 
        Self { items: Vec::new() } 
    }
    
    /// Creates a new stack with pre-allocated capacity
    /// 
    /// This can improve performance by reducing allocations when you know
    /// approximately how many items you'll store
    /// 
    /// # Arguments
    /// * `n` - Initial capacity to allocate
    /// 
    /// # Examples
    /// ```
    /// use mission1::Stack;
    /// let mut stack: Stack<String> = Stack::with_capacity(100);
    /// // Can push 100 items without reallocating memory
    /// ```
    pub fn with_capacity(n: usize) -> Self { 
        Self { items: Vec::with_capacity(n) } 
    }

    /// Pushes a value onto the top of the stack (REQ-2: Amortized O(1))
    /// 
    /// # Ownership
    /// This method takes ownership of `x` - the value is MOVED into the stack.
    /// After calling push, you can no longer use the original variable.
    /// 
    /// # Arguments
    /// * `x` - The value to push (ownership is transferred)
    /// 
    /// # Examples
    /// ```
    /// use mission1::Stack;
    /// let mut stack = Stack::new();
    /// let value = String::from("hello");
    /// stack.push(value);
    /// // `value` can no longer be used - ownership was moved to stack
    /// ```
    pub fn push(&mut self, x: T) {
        self.items.push(x); // Delegate to Vec's push (amortized O(1))
    }

    /// Pops and returns the top value from the stack (REQ-3: O(1) with ownership transfer)
    /// 
    /// # Returns
    /// * `Some(T)` - The top value if stack is not empty (you gain ownership)
    /// * `None` - If the stack is empty
    /// 
    /// # Memory Safety (REQ-4)
    /// Once a value is popped, the stack no longer has any reference to it.
    /// This prevents use-after-free bugs that are common in other languages.
    /// 
    /// # Examples
    /// ```
    /// use mission1::Stack;
    /// let mut stack = Stack::new();
    /// stack.push("hello");
    /// let popped = stack.pop().unwrap(); // You now own "hello"
    /// assert_eq!(popped, "hello");
    /// assert!(stack.pop().is_none()); // Stack is empty
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop() // Vec::pop() transfers ownership out
    }

    /// Returns an immutable reference to the top value without removing it (REQ-5)
    /// 
    /// # Borrowing Rules
    /// - You can have multiple immutable borrows simultaneously
    /// - While immutable borrows exist, you CANNOT mutate the stack
    /// - The borrow checker enforces this at compile time
    /// 
    /// # Returns
    /// * `Some(&T)` - Immutable reference to top value if stack is not empty
    /// * `None` - If the stack is empty
    /// 
    /// # Examples
    /// ```
    /// use mission1::Stack;
    /// let mut stack = Stack::new();
    /// stack.push(42);
    /// 
    /// let peek1 = stack.peek().unwrap(); // First immutable borrow
    /// let peek2 = stack.peek().unwrap(); // Second immutable borrow (OK!)
    /// assert_eq!(*peek1, 42);
    /// assert_eq!(*peek2, 42);
    /// // stack.push(99); // This would cause compile error while borrows exist!
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.items.last() // Vec::last() returns Option<&T>
    }

    /// Returns a mutable reference to the top value without removing it (REQ-5)
    /// 
    /// # Borrowing Rules  
    /// - You can have only ONE mutable borrow at a time
    /// - While a mutable borrow exists, you CANNOT have any other borrows
    /// - This prevents data races and ensures memory safety
    /// 
    /// # Returns
    /// * `Some(&mut T)` - Mutable reference to top value if stack is not empty
    /// * `None` - If the stack is empty
    /// 
    /// # Examples
    /// ```
    /// use mission1::Stack;
    /// let mut stack = Stack::new();
    /// stack.push(42);
    /// 
    /// {
    ///     let mut_ref = stack.peek_mut().unwrap(); // Mutable borrow
    ///     *mut_ref = 99; // Modify the value through the reference
    ///     // let peek = stack.peek(); // This would cause compile error!
    /// } // Mutable borrow ends here
    /// 
    /// assert_eq!(*stack.peek().unwrap(), 99); // Now we can borrow again
    /// ```
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.items.last_mut() // Vec::last_mut() returns Option<&mut T>
    }

    /// Returns the number of elements in the stack
    /// 
    /// This operation is O(1) as Vec tracks its length internally.
    /// 
    /// # Examples
    /// ```
    /// use mission1::Stack;
    /// let mut stack = Stack::new();
    /// assert_eq!(stack.len(), 0);
    /// 
    /// stack.push("item");
    /// assert_eq!(stack.len(), 1);
    /// ```
    pub fn len(&self) -> usize { 
        self.items.len() 
    }

    /// Returns true if the stack contains no elements
    /// 
    /// # Examples
    /// ```
    /// use mission1::Stack;
    /// let mut stack: Stack<i32> = Stack::new();
    /// assert!(stack.is_empty());
    /// 
    /// stack.push(1);
    /// assert!(!stack.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool { 
        self.items.is_empty() 
    }
}
