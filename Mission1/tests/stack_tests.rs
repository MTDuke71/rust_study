use mission1::Stack;

#[test] // REQ-1
fn req1_generic_support() {
    // Test that Stack works with different types (separate stacks)
    let mut string_stack = Stack::new();
    string_stack.push("str");
    
    let mut int_stack = Stack::new();
    int_stack.push(42);
    
    let mut float_stack = Stack::new();
    float_stack.push(3.14);
    
    // All should compile and work
    assert_eq!(string_stack.pop(), Some("str"));
    assert_eq!(int_stack.pop(), Some(42));
    assert_eq!(float_stack.pop(), Some(3.14));
}

#[test] // REQ-2
fn req2_push_amortized_constant() {
    let mut s = Stack::with_capacity(2);
    for i in 0..100 {
        s.push(i);
    }
    assert_eq!(s.len(), 100);
}

#[test] // REQ-3
fn req3_pop_transfers_ownership() {
    let mut s = Stack::new();
    let val = String::from("owned");
    s.push(val);
    let popped = s.pop().unwrap();
    assert_eq!(popped, "owned");
    assert!(s.pop().is_none());
}

#[test] // REQ-4
fn req4_no_use_after_pop() {
    let mut s = Stack::new();
    let a = String::from("x");
    s.push(a);
    // Using `a` here would not compile — ownership moved.
    let b = s.pop().unwrap();
    assert_eq!(b, "x");
}

#[test] // REQ-5
fn req5_peek_aliasing_rules() {
    let mut s = Stack::new();
    s.push(1);
    {
        let imm = s.peek().unwrap();
        assert_eq!(*imm, 1);
        // s.push(2); // compile error if uncommented
    }
    {
        let mut_ref = s.peek_mut().unwrap();
        *mut_ref = 99;
    }
    assert_eq!(*s.peek().unwrap(), 99);
}
