use std::fmt::{self, Debug, Formatter};
use std::mem;

fn main() {
    layout_example::run();
    dispatch_example::run();
    hrtb_example::run();
    marker_types_example::run();
    existential_types_example::run();
}


// --- Listing 2-1: Alignment and Layout ---
// Sources: [1], [2], [3]
mod layout_example {
    use super::*;

    // "The compiler inserts padding... to ensure fields satisfy alignment requirements." [2]
    #[repr(C)]
    pub struct Foo {
        pub tiny: bool,   // 1 byte
        // 3 bytes padding inserted here to align `normal` to 4 bytes
        pub normal: u32,  // 4 bytes
        pub small: u8,    // 1 byte
        // 7 bytes padding inserted here to align `long` to 8 bytes [3]
        pub long: u64,    // 8 bytes
        pub short: u16,   // 2 bytes
        // 6 bytes final padding to align struct size to multiple of 8 [3]
    }

    pub fn run() {
        println!("--- Listing 2-1: Layout & Alignment ---");
        println!("Size of Foo: {} bytes", mem::size_of::<Foo>());
        println!("Alignment of Foo: {} bytes", mem::align_of::<Foo>());
        
        // Verifying offsets described in text [2]-[3]
        // tiny (0) -> normal (4) -> small (8) -> long (16) -> short (24) -> total (32)
        let example = Foo { tiny: true, normal: 1, small: 2, long: 3, short: 4 };
        println!("Offset of tiny: {}", (&example.tiny as *const bool as usize) - (&example as *const Foo as usize));
        println!("Offset of normal: {}", (&example.normal as *const u32 as usize) - (&example as *const Foo as usize));
        println!("Offset of long: {}", (&example.long as *const u64 as usize) - (&example as *const Foo as usize));
    }
}

// --- Listing 2-2 & 2-3: Compilation and Dispatch ---
// Sources: [4], [5]
mod dispatch_example {
    // Defining a dummy Pattern trait to make the examples runnable.
    // The text implies `p.is_contained_in(self)`.
    pub trait Pattern {
        fn is_contained_in(&self, s: &str) -> bool;
    }

    impl Pattern for &str {
        fn is_contained_in(&self, s: &str) -> bool {
            s.contains(*self)
        }
    }

    // We define a wrapper for String to implement the methods shown in the text.
    pub struct MyString(pub String);

    impl MyString {
        // Listing 2-2: A generic method using static dispatch [4]
        // "A copy of this method is made for every distinct pattern type."
        pub fn contains_static(&self, p: impl Pattern) -> bool {
            p.is_contained_in(&self.0)
        }

        // Listing 2-3: A generic method using dynamic dispatch [5]
        // "The caller must give... the address of the pattern and the address of the is_contained_in method (vtable)."
        pub fn contains_dynamic(&self, p: &dyn Pattern) -> bool {
            p.is_contained_in(&self.0)
        }
    }

    pub fn run() {
        println!("\n--- Listing 2-2 & 2-3: Dispatch ---");
        let s = MyString("Hello World".to_string());
        let p = "World";

        // Static Dispatch
        println!("Static match: {}", s.contains_static(p));

        // Dynamic Dispatch
        println!("Dynamic match: {}", s.contains_dynamic(&p));
    }
}

// --- Listing 2-4 & 2-5: Coherence and Orphan Rule ---
// Sources: [6], [7]
mod coherence_example {
    use std::marker::PhantomData;

    #[allow(dead_code)]
    pub struct MyType;
    #[allow(dead_code)]
    pub struct ForeignTrait<T, U>(PhantomData<(T, U)>);

    // Listing 2-4: Valid implementations [7]
    // "Generic type parameters (Ps) are allowed to appear... as long as they are covered by some intermediate type." [6]
    // Note: impl<T> From<T> for MyType conflicts with core's impl<T> From<T> for T
    // Instead, we show a valid covered implementation:
    
    // Simulating foreign types for demonstration
    #[allow(dead_code)]
    pub struct Vec<T>(T); 

    // Valid: MyType is local
    impl<T> From<MyType> for Vec<T> {
        fn from(_: MyType) -> Self { panic!("Mock impl") }
    }

    /* 
    // Listing 2-5: Invalid implementations [7]
    // These would cause compile errors because they violate the orphan rule.
    
    // Invalid: ForeignTrait and T are both foreign (relative to this crate if ForeignTrait were actually foreign)
    impl<T> ForeignTrait<T, T> for T {} 
    
    // Invalid: Uncovered generic T
    impl<T> From<T> for Vec<T> {} 
    */
}

// --- Listing 2-6: Higher-Ranked Trait Bounds (HRTB) ---
// Sources: [8], [9]
mod hrtb_example {
    use super::*;

    // A wrapper struct to allow us to implement a foreign trait (Debug) 
    // effectively on a generic blanket.
    pub struct AnyIterable<T>(pub T);

    // Listing 2-6: An excessively generic implementation of Debug [9]
    // "for any type that can be iterated over and whose elements are Debug."
    impl<T> Debug for AnyIterable<T>
    where
        for<'a> &'a T: IntoIterator,
        for<'a> <&'a T as IntoIterator>::Item: Debug,
    {
        fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
            f.debug_list().entries(&self.0).finish()
        }
    }

    pub fn run() {
        println!("\n--- Listing 2-6: Higher-Ranked Trait Bounds ---");
        let vec = vec![10, 11, 12];
        let iterable = AnyIterable(vec);
        println!("Formatted iterable: {:?}", iterable);
    }
}

// --- Marker Types Example ---
// Context provided in "Marker Traits" section [13]
mod marker_types_example {
    use std::marker::PhantomData;

    // "Marker types are unit types... used for marking a type as being in a particular state." [13]
    struct Unauthenticated;
    struct Authenticated;

    struct SshConnection<State> {
        state: PhantomData<State>,
    }

    impl SshConnection<Unauthenticated> {
        fn new() -> Self {
            SshConnection { state: PhantomData }
        }

        fn connect(self) -> SshConnection<Authenticated> {
            println!("Connecting...");
            SshConnection { state: PhantomData }
        }
    }

    impl SshConnection<Authenticated> {
        fn send_command(&self, cmd: &str) {
            println!("Sending command: {}", cmd);
        }
    }

    pub fn run() {
        println!("\n--- Marker Types Example ---");
        let conn = SshConnection::new(); // State: Unauthenticated
        // conn.send_command("ls"); // This would fail to compile!
        
        let conn = conn.connect(); // State transitions to Authenticated
        conn.send_command("whoami"); // Now allowed
    }
}

// --- Existential Types Example ---
// Context provided in "Existential Types" section [14], [15]
mod existential_types_example {
    // "impl Trait... hints that the function returns some type that implements some set of traits" [14]
    fn make_iter(input: Vec<i32>) -> impl Iterator<Item = i32> {
        // "Using maps and filters over some existing iterator type." [15]
        input.into_iter().map(|x| x * 2).filter(|x| *x > 5)
    }

    pub fn run() {
        println!("\n--- Existential Types (impl Trait) ---");
        let v = vec![1, 2, 3, 10, 16];
        let iter = make_iter(v);
        for i in iter {
            println!("Item: {}", i);
        }
    }
}

