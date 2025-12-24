// Example 4: Coherence and the Orphan Rule
// Demonstrates trait implementation rules and orphan rule

use rustaceans_ch02_types::utils::{print_section, print_subsection};

fn main() {
    print_section("4. COHERENCE AND THE ORPHAN RULE");
    
    orphan_rule_basics();
    blanket_implementations();
    fundamental_types();
    covered_implementations();
}

// --- Local types for demonstrations ---
struct LocalType;
struct AnotherLocalType;

// Local trait
trait LocalTrait {
    fn local_method(&self);
}

// --- Orphan Rule Basics ---
fn orphan_rule_basics() {
    print_subsection("a. Orphan Rule - Basic Cases");
    
    println!("The Orphan Rule: Can implement trait for type ONLY if:");
    println!("  - The trait is local to your crate, OR");
    println!("  - The type is local to your crate\n");
    
    // ✅ VALID: Local trait + Local type
    impl LocalTrait for LocalType {
        fn local_method(&self) {
            println!("LocalTrait for LocalType");
        }
    }
    
    // ✅ VALID: Local trait + Foreign type (String)
    impl LocalTrait for String {
        fn local_method(&self) {
            println!("LocalTrait for String: {}", self);
        }
    }
    
    // ✅ VALID: Foreign trait (Display) + Local type
    impl std::fmt::Display for LocalType {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "LocalType instance")
        }
    }
    
    let local = LocalType;
    local.local_method();
    
    let s = String::from("hello");
    s.local_method();
    
    println!("{}", local);
    
    println!("\n✅ Valid implementations:");
    println!("   impl LocalTrait for LocalType    (both local)");
    println!("   impl LocalTrait for String       (trait local)");
    println!("   impl Display for LocalType       (type local)");
    
    println!("\n❌ Invalid implementation (would not compile):");
    println!("   impl Display for String          (both foreign)");
    
    /*
    // This would fail to compile:
    impl std::fmt::Display for String {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self)
        }
    }
    // Error: only traits defined in the current crate can be implemented for types defined outside of the crate
    */
}

// --- Blanket Implementations ---
fn blanket_implementations() {
    print_subsection("b. Blanket Implementations");
    
    println!("Blanket impl<T> Trait for T allowed ONLY by defining crate:\n");
    
    // ✅ VALID: We define LocalTrait, so we CAN do blanket impl
    // BUT: It would conflict with existing String implementation above
    // So this is commented out to demonstrate the concept:
    
    /*
    impl<T: std::fmt::Debug> LocalTrait for T {
        fn local_method(&self) {
            println!("Blanket impl for {:?}", self);
        }
    }
    */
    
    // Instead, let's demonstrate with a different trait
    trait AnotherLocalTrait {
        fn another_method(&self);
    }
    
    impl<T: std::fmt::Debug> AnotherLocalTrait for T {
        fn another_method(&self) {
            println!("Blanket impl for {:?}", self);
        }
    }
    
    let _num = 42;
    _num.another_method();  // Works because i32: Debug
    
    println!("✅ impl<T: Debug> AnotherLocalTrait for T works (no conflicts)");
    println!("   (Original blanket impl commented to avoid String conflict)");
    
    println!("✅ impl<T: Debug> LocalTrait for T works (LocalTrait is local)");
    
    println!("\n❌ Cannot do blanket impl for foreign trait:");
    println!("   impl<T> Display for T {{ ... }}  // ERROR");
    println!("   Only std crate can do this");
    
    println!("\n⚠️  Blanket implementations are BREAKING CHANGES:");
    println!("   Adding impl<T> Trait for T prevents downstream crates");
    println!("   from implementing Trait for specific types");
}

// --- Fundamental Types ---
fn fundamental_types() {
    print_subsection("c. Fundamental Types (&, &mut, Box)");
    
    println!("Fundamental types (#[fundamental]) are erased for orphan rule:\n");
    
    // ✅ VALID: Can implement foreign trait for &LocalType
    impl std::fmt::Debug for &LocalType {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "&LocalType")
        }
    }
    
    // ✅ VALID: Can implement foreign trait for Box<LocalType>
    impl std::fmt::Debug for Box<LocalType> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Box<LocalType>")
        }
    }
    
    let local = LocalType;
    println!("{:?}", &local);
    println!("{:?}", Box::new(local));
    
    println!("\n✅ Fundamental types allow:");
    println!("   impl Debug for &LocalType");
    println!("   impl Debug for Box<LocalType>");
    println!("   impl Debug for &mut LocalType");
    
    println!("\nFundamental types: &, &mut, Box");
    println!("They're 'transparent' for orphan rule purposes");
}

// --- Covered Implementations ---
fn covered_implementations() {
    print_subsection("d. Covered Implementations");
    
    println!("Can impl foreign trait for foreign type if local type 'covers':\n");
    
    // ✅ VALID: LocalType appears in Vec's generic parameter
    impl From<LocalType> for Vec<String> {
        fn from(_: LocalType) -> Vec<String> {
            vec!["covered".to_string(), "implementation".to_string()]
        }
    }
    
    // ✅ VALID: LocalType appears before foreign type in tuple
    impl From<(LocalType, String)> for AnotherLocalType {
        fn from(_: (LocalType, String)) -> AnotherLocalType {
            AnotherLocalType
        }
    }
    
    let local = LocalType;
    let vec: Vec<String> = local.into();
    println!("Converted LocalType to Vec<String>: {:?}", vec);
    
    println!("\n✅ Valid covered implementations:");
    println!("   impl From<LocalType> for Vec<String>");
    println!("     LocalType 'covers' the generic parameter");
    
    println!("\n❌ Invalid (uncovered):");
    println!("   impl From<String> for Vec<String>  // String doesn't cover");
    
    /*
    // This would fail:
    impl From<String> for Vec<String> {
        fn from(s: String) -> Vec<String> {
            vec![s]
        }
    }
    */
    
    println!("\n📊 Orphan Rule Summary:\n");
    println!("┌────────────────────────────────────────────┬─────────┐");
    println!("│ Implementation                             │ Valid?  │");
    println!("├────────────────────────────────────────────┼─────────┤");
    println!("│ impl LocalTrait for LocalType              │   ✅    │");
    println!("│ impl LocalTrait for ForeignType            │   ✅    │");
    println!("│ impl ForeignTrait for LocalType            │   ✅    │");
    println!("│ impl ForeignTrait for ForeignType          │   ❌    │");
    println!("│ impl<T> LocalTrait for T                   │   ✅    │");
    println!("│ impl<T> ForeignTrait for T                 │   ❌    │");
    println!("│ impl ForeignTrait for &LocalType           │   ✅    │");
    println!("│ impl From<LocalType> for Vec<String>       │   ✅    │");
    println!("└────────────────────────────────────────────┴─────────┘");
}
