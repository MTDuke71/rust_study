//! Chapter 3: Constrained Interfaces - Hidden Contracts
//! 
//! Topics: Re-exports, SemVer Trick, Auto-Traits (Send/Sync)

use std::marker::PhantomData;

fn main() {
    println!("=== Constrained Interfaces: Hidden Contracts ===\n");
    
    reexport_problem();
    auto_traits_demo();
    semver_trick_explanation();
}

/// Re-exports: Exposing dependency types in public API
fn reexport_problem() {
    println!("--- Re-export Hazards ---");
    
    println!("  ❌ BAD: pub fn process(data: DependencyType)");
    println!("     Problem: Updating dependency breaks downstream");
    println!();
    println!("  ✅ GOOD: Wrap foreign types");
    println!("     pub struct MyWrapper(DependencyType);");
    println!();
    println!("  ✅ GOOD: Use impl Trait to hide implementation");
    println!("     pub fn process() -> impl Iterator<Item = i32>");
    println!();
}

/// Auto-Traits: Send and Sync are automatically implemented
fn auto_traits_demo() {
    println!("--- Auto-Traits (Send/Sync) ---");
    
    let safe = SafeType { value: 42 };
    println!("  SafeType is Send: {}", is_send(&safe));
    println!("  SafeType is Sync: {}", is_sync(&safe));
    
    let _unsafe_type = UnsafeType { 
        value: 42,
        _not_send: PhantomData,
    };
    // Cannot call is_send on UnsafeType - it's not Send (intentional)
    // println!("  UnsafeType is Send: {}", is_send(&_unsafe_type));
    println!("  UnsafeType is NOT Send (intentional - contains raw pointer semantics)");
    // println!("  UnsafeType is Sync: {}", is_sync(&unsafe_type));  // Compile error
    
    println!("\n  ⚠️  Removing Send/Sync is a BREAKING CHANGE");
    println!("  Solution: Test that types remain Send/Sync\n");
}

/// SemVer Trick: Support multiple dependency versions
fn semver_trick_explanation() {
    println!("--- SemVer Trick ---");
    
    println!("  Problem: Users on old dependency version vs new");
    println!();
    println!("  Solution: Re-export old types from new version");
    println!("  pub use new_dep::{{TypeV2 as Type}};");
    println!("  pub use old_dep::{{Type as TypeV1}};");
    println!();
    println!("  Allows users to migrate gradually\n");
}

// === Helper Functions ===

fn is_send<T: Send>(_: &T) -> bool { true }
fn is_sync<T: Sync>(_: &T) -> bool { true }

// === Auto-Trait Examples ===

/// Safe type: Automatically implements Send + Sync
#[derive(Debug)]
#[allow(dead_code)]  // Demonstration struct
struct SafeType {
    value: i32,
}

/// Unsafe marker to prevent Send
/// *const () is a raw pointer - raw pointers do NOT implement Send
/// because they can't be safely sent across thread boundaries
#[allow(dead_code)]  // Demonstration struct
struct NotSend {
    _marker: PhantomData<*const ()>,
}

/// Type that is NOT Send (contains raw pointer semantics)
#[allow(dead_code)]  // Demonstration struct
struct UnsafeType {
    value: i32,
    _not_send: PhantomData<*const ()>,
}

// === Tests for Auto-Traits ===

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn ensure_safe_type_is_send_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        
        assert_send::<SafeType>();
        assert_sync::<SafeType>();
    }
    
    // This test would fail if we accidentally remove Send
    // #[test]
    // fn ensure_unsafe_type_is_not_send() {
    //     fn assert_send<T: Send>() {}
    //     assert_send::<UnsafeType>();  // Compile error: expected!
    // }
}

// === Wrapper Pattern for Foreign Types ===

/// Instead of: pub fn process(data: foreign_crate::Data)
/// Use wrapper to control the interface
pub struct DataWrapper {
    inner: String,  // Could be foreign type
}

impl DataWrapper {
    pub fn new(data: String) -> Self {
        Self { inner: data }
    }
    
    pub fn process(&self) -> String {
        self.inner.clone()
    }
}

// === impl Trait to Hide Dependencies ===

/// Returns iterator without exposing concrete type
pub fn get_numbers() -> impl Iterator<Item = i32> {
    // Implementation can change without breaking API
    vec![1, 2, 3].into_iter()
}

/// Could switch to different implementation
pub fn get_numbers_v2() -> impl Iterator<Item = i32> {
    // Different implementation, same interface
    1..=3
}
