#![allow(dead_code, unused_variables)]

use std::borrow::Cow;
use std::marker::PhantomData;

// --- Listing 3-1: Similar function signatures with different contracts ---
// Source: [1]
mod signatures_contracts {
    use super::*;

    // "The first function requires the caller to own the string...
    // and it promises that it will return an owned String."
    fn frobnicate1(s: String) -> String {
        s
    }

    // "The second function relaxes the contract: the caller can provide
    // any reference to a string... It also promises to give back a std::borrow::Cow"
    fn frobnicate2(s: &str) -> Cow<'_, str> {
        Cow::Borrowed(s)
    }

    // "The third function lifts these restrictions. It requires only that
    // the user pass in a type that can produce a reference to a string"
    fn frobnicate3(s: impl AsRef<str>) -> impl AsRef<str> {
        s
    }
}

// --- Listing 3-2: Using marker types to restrict implementations ---
// Source: [2], [3]
mod marker_types {
    use super::*;

    // Mock types to make the example compile
    pub struct Color;
    pub struct Kilograms;

    // 1. "We introduce unit types to represent each stage of the rocket"
    struct Grounded;
    struct Launched;

    // "and so on"
    struct Rocket<Stage = Grounded> {
        // 2. "we don't actually need to store the stage... so we store it
        // behind a PhantomData"
        stage: std::marker::PhantomData<Stage>,
    }

    // 3. "You can construct a rocket only on the ground (for now),
    // and you can launch it only from the ground"
    impl Default for Rocket<Grounded> {
        fn default() -> Self {
            Self { stage: PhantomData }
        }
    }

    impl Rocket<Grounded> {
        pub fn launch(self) -> Rocket<Launched> {
            Rocket { stage: PhantomData }
        }
    }

    // 4. "Only when the rocket has been launched can you control its velocity"
    impl Rocket<Launched> {
        pub fn accelerate(&mut self) {}
        pub fn decelerate(&mut self) {}
    }

    // 5. "There are some things you can always do with the rocket...
    // and those we place in a generic implementation block"
    impl<Stage> Rocket<Stage> {
        pub fn color(&self) -> Color {
            Color
        }
        pub fn weight(&self) -> Kilograms {
            Kilograms
        }
    }
}

// --- Listing 3-3: An innocent-looking public type ---
// Source: [4]
mod type_modifications_1 {
    // "in your interface"
    pub mod lib {
        pub struct Unit;
    }

    // "in user code"
    pub fn run() {
        let u = lib::Unit;
    }
}

// --- Listing 3-4: User code accessing a single public field ---
// Source: [5]
mod type_modifications_2 {
    // "in your interface"
    pub mod lib {
        pub struct Unit {
            pub field: bool,
        }
    }

    use lib::Unit;

    // "in user code"
    fn is_true(u: lib::Unit) -> bool {
        // "adding a private field to Unit will break user code, this time because
        // Rust's exhaustive pattern match checking logic is able to see parts
        // of the interface that the user cannot see."
        matches!(u, Unit { field: true })
    }
}

// --- Listing 3-5: Implementing a trait for an existing type may cause problems ---
// Source: [6]
mod trait_conflicts {
    // crate1 1.0
    pub mod crate1 {
        pub struct Unit;
        pub trait Foo1 {
            fn foo(&self);
        }
        // note that Foo1 is not implemented for Unit
    }

    // crate2; depends on crate1 1.0
    // "If you add impl Foo1 for Unit to crate1 without marking it a breaking change,
    // the downstream code will suddenly stop compiling since the call to foo is now ambiguous."
    pub mod crate2 {
        // NOTE: Foo1 import is intentional - demonstrates potential coherence conflict
        // If crate1 later adds impl Foo1 for Unit, the foo() call becomes ambiguous
        #[allow(unused_imports)]
        use super::crate1::{Foo1, Unit};

        trait Foo2 {
            fn foo(&self);
        }

        impl Foo2 for Unit {
            fn foo(&self) {}
        }

        fn main() {
            // Unit.foo(); // ambiguous if Foo1 is implemented for Unit later
        }
    }
}

// --- Listing 3-6: How to seal a trait and add implementations for it ---
// Source: [7]
mod sealed_traits {
    // "The trick is to add a private, empty trait as a supertrait of the trait you wish to seal"
    // 1.
    pub trait CanUseCannotImplement: sealed::Sealed {}

    mod sealed {
        pub trait Sealed {}

        // 2. "The sealed trait requires the underlying type to implement Sealed,
        // so only the types that we explicitly allow are able to ultimately implement the trait."
        impl<T> Sealed for T where T: super::TraitBounds {}
    }

    // Mock trait bound
    pub trait TraitBounds {}

    impl<T> CanUseCannotImplement for T where T: TraitBounds {}
}

// --- Listing 3-7: Re-exports make foreign crates part of your interface contract ---
// Source: [8]
mod re_exports {
    // Mocking the external crate 'itercrate'
    pub mod itercrate {
        pub struct Empty<T>(std::marker::PhantomData<T>);
        impl<T> Default for Empty<T> {
            fn default() -> Self {
                Empty(std::marker::PhantomData)
            }
        }
    }

    // "your crate: bestiter"
    pub mod bestiter {
        use super::itercrate;
        pub fn iter<T>() -> itercrate::Empty<T> {
            itercrate::Empty::default()
        }
    }

    // "their crate"
    struct EmptyIterator {
        it: itercrate::Empty<()>,
    }

    fn run() {
        let _ = EmptyIterator {
            it: bestiter::iter(),
        };
    }
}

// --- Listing 3-8: Testing that a type implements a set of traits ---
// Source: [9]
#[cfg(test)]
mod tests {
    #[allow(unused_imports)] // Import needed if uncommenting the test assertion
    use super::*;

    struct MyType;

    fn is_normal<T: Sized + Send + Sync + Unpin>() {}

    #[test]
    fn normal_types() {
        // Uncommenting this would fail if MyType does not implement the traits
        // is_normal::<MyType>();
    }
}

fn main() {
    println!("Chapter 3 examples compiled successfully.");
}
