//! Day 4: Conditional Compilation - Platform-Specific Code
//! 
//! This example demonstrates:
//! - Platform detection (OS, architecture)
//! - Feature-based compilation
//! - cfg combinators (all, any, not)
//! - Target-specific code

/// Cross-platform file path separator
#[cfg(windows)]
pub const PATH_SEPARATOR: char = '\\';

#[cfg(not(windows))]
pub const PATH_SEPARATOR: char = '/';

/// Platform-specific functionality
pub fn platform_info() {
    println!("=== Platform Detection ===\n");
    
    // Operating System
    if cfg!(target_os = "windows") {
        println!("OS: Windows");
    } else if cfg!(target_os = "linux") {
        println!("OS: Linux");
    } else if cfg!(target_os = "macos") {
        println!("OS: macOS");
    } else {
        println!("OS: Other");
    }
    
    // Architecture
    if cfg!(target_arch = "x86_64") {
        println!("Architecture: x86_64 (64-bit)");
    } else if cfg!(target_arch = "aarch64") {
        println!("Architecture: ARM64");
    } else if cfg!(target_arch = "wasm32") {
        println!("Architecture: WebAssembly");
    } else {
        println!("Architecture: {:?}", std::env::consts::ARCH);
    }
    
    // Pointer width
    if cfg!(target_pointer_width = "64") {
        println!("Pointer width: 64-bit");
    } else if cfg!(target_pointer_width = "32") {
        println!("Pointer width: 32-bit");
    }
    
    // Endianness
    if cfg!(target_endian = "little") {
        println!("Endianness: Little-endian");
    } else {
        println!("Endianness: Big-endian");
    }
    
    println!("Path separator: '{}'", PATH_SEPARATOR);
}

/// Unix-specific code
#[cfg(unix)]
pub fn unix_operation() {
    println!("\n=== Unix-Specific Code ===");
    println!("✓ This code only compiles on Unix systems");
    println!("  Available on: Linux, macOS, BSD, etc.");
}

/// Windows-specific code
#[cfg(windows)]
pub fn windows_operation() {
    println!("\n=== Windows-Specific Code ===");
    println!("✓ This code only compiles on Windows");
    println!("  Can use Windows API, registry, etc.");
}

/// Linux-specific code
#[cfg(target_os = "linux")]
pub fn linux_operation() {
    println!("\n=== Linux-Specific Code ===");
    println!("✓ This code only compiles on Linux");
}

/// macOS-specific code
#[cfg(target_os = "macos")]
pub fn macos_operation() {
    println!("\n=== macOS-Specific Code ===");
    println!("✓ This code only compiles on macOS");
}

/// Demonstrates cfg combinators
pub fn combinator_examples() {
    println!("\n=== cfg! Combinators ===\n");
    
    // all() - all conditions must be true
    if cfg!(all(unix, target_pointer_width = "64")) {
        println!("✓ all(unix, 64-bit) - Both conditions true");
    }
    
    // any() - at least one condition must be true
    if cfg!(any(windows, unix)) {
        println!("✓ any(windows, unix) - At least one true");
    }
    
    // not() - condition must be false
    if cfg!(not(target_arch = "wasm32")) {
        println!("✓ not(wasm32) - Not targeting WebAssembly");
    }
    
    // Complex combinations
    if cfg!(all(
        any(target_os = "linux", target_os = "macos"),
        target_pointer_width = "64"
    )) {
        println!("✓ Complex: (Linux OR macOS) AND 64-bit");
    }
}

/// Test-only code
#[cfg(test)]
pub fn test_only_function() {
    println!("This function only exists during testing");
}

/// Debug-only assertions
pub fn debug_checks(value: i32) {
    if cfg!(debug_assertions) {
        println!("\n=== Debug Assertions ===");
        println!("Debug mode: Expensive checks enabled");
        assert!(value >= 0, "Value must be non-negative");
        assert!(value < 100, "Value must be less than 100");
    } else {
        println!("\n=== Release Mode ===");
        println!("Release mode: Debug assertions disabled");
    }
}

/// Architecture-specific optimizations
pub fn arch_specific_code() {
    println!("\n=== Architecture-Specific Code ===\n");
    
    #[cfg(target_arch = "x86_64")]
    {
        println!("x86_64: Can use SIMD instructions (SSE, AVX)");
    }
    
    #[cfg(target_arch = "aarch64")]
    {
        println!("ARM64: Can use NEON SIMD instructions");
    }
    
    #[cfg(target_arch = "wasm32")]
    {
        println!("WebAssembly: Running in browser or WASI");
    }
}

fn main() {
    println!("🎯 Day 4: Conditional Compilation Example\n");
    
    platform_info();
    
    // Platform-specific operations
    #[cfg(unix)]
    unix_operation();
    
    #[cfg(windows)]
    windows_operation();
    
    #[cfg(target_os = "linux")]
    linux_operation();
    
    #[cfg(target_os = "macos")]
    macos_operation();
    
    combinator_examples();
    debug_checks(42);
    arch_specific_code();
    
    println!("\n=== Conditional Dependency Example ===\n");
    println!("In Cargo.toml:");
    println!("[target.'cfg(windows)'.dependencies]");
    println!("winapi = \"0.3\"");
    println!();
    println!("[target.'cfg(unix)'.dependencies]");
    println!("libc = \"0.2\"");
    
    println!("\n=== Common cfg Patterns ===\n");
    
    println!("1. Feature flags:");
    println!("   #[cfg(feature = \"std\")]");
    
    println!("\n2. Platform:");
    println!("   #[cfg(unix)]");
    println!("   #[cfg(windows)]");
    println!("   #[cfg(target_os = \"linux\")]");
    
    println!("\n3. Architecture:");
    println!("   #[cfg(target_arch = \"x86_64\")]");
    println!("   #[cfg(target_arch = \"aarch64\")]");
    
    println!("\n4. Testing:");
    println!("   #[cfg(test)]");
    println!("   #[cfg(debug_assertions)]");
    
    println!("\n5. Combinations:");
    println!("   #[cfg(all(unix, target_pointer_width = \"64\"))]");
    println!("   #[cfg(any(windows, target_os = \"macos\"))]");
    println!("   #[cfg(not(target_env = \"msvc\"))]");
    
    println!("\n📝 This code compiles differently on each platform!");
    println!("   Try: cargo build --target x86_64-pc-windows-msvc");
    println!("   Try: cargo build --target x86_64-unknown-linux-gnu");
    println!("   Try: cargo build --target aarch64-apple-darwin");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_platform_detection() {
        // This test only runs when you execute `cargo test`
        platform_info();
        assert!(cfg!(any(windows, unix, target_os = "macos")));
    }
}
