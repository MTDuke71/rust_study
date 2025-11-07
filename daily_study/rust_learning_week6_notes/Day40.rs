//! Complete example demonstrating publishing best practices
//!
//! Run with: cargo run --example day40_publishing

use std::process::Command;

fn main() {
    println!("╔═══════════════════════════════════════════════╗");
    println!("║  Day 40: Publishing Crates                   ║");
    println!("╚═══════════════════════════════════════════════╝\n");

    // Part 1: Cargo.toml Metadata
    println!("=== Part 1: Cargo.toml Metadata ===");
    demonstrate_cargo_metadata();

    // Part 2: Documentation Standards
    println!("\n=== Part 2: Documentation Standards ===");
    demonstrate_documentation();

    // Part 3: Semantic Versioning
    println!("\n=== Part 3: Semantic Versioning ===");
    demonstrate_semver();

    // Part 4: Pre-Publish Checks
    println!("\n=== Part 4: Pre-Publish Validation ===");
    demonstrate_validation();
}

fn demonstrate_cargo_metadata() {
    println!("Required metadata for publishing:");
    println!("  ✓ name - Unique crate name");
    println!("  ✓ version - SemVer (e.g., '0.1.0')");
    println!("  ✓ edition - Rust edition (2021)");
    println!("  ✓ authors - Your name and email");
    println!("  ✓ license - 'MIT OR Apache-2.0'");
    println!("  ✓ description - One-line summary");
    println!("  ✓ repository - GitHub/GitLab URL");

    println!("\nRecommended metadata:");
    println!("  • readme - 'README.md'");
    println!("  • keywords - 5 max, searchability");
    println!("  • categories - From crates.io list");
    println!("  • homepage - Project website");
    println!("  • documentation - docs.rs URL");
    println!("  • rust-version - Minimum Rust version");

    println!("\nExample:");
    println!(
        r#"
[package]
name = "awesome-crate"
version = "0.1.0"
edition = "2021"
authors = ["Alice <alice@example.com>"]
license = "MIT OR Apache-2.0"
description = "An awesome crate for doing awesome things"
repository = "https://github.com/alice/awesome-crate"
keywords = ["data-structures", "algorithms"]
categories = ["algorithms"]
"#
    );
}

fn demonstrate_documentation() {
    println!("Documentation requirements:");
    println!("  1. Crate-level docs (//! in lib.rs)");
    println!("  2. Module-level docs for all modules");
    println!("  3. Public API docs for all public items");
    println!("  4. Examples in documentation");
    println!("  5. Complexity analysis where relevant");

    println!("\nQuality checklist:");
    println!("  ✓ All public items documented");
    println!("  ✓ Examples compile and run");
    println!("  ✓ Clear and concise descriptions");
    println!("  ✓ Links to related items");
    println!("  ✓ Panic conditions documented");

    println!("\nExample documentation:");
    println!(
        r#"
/// Finds the root of element `x`.
///
/// Uses path compression to flatten the tree.
///
/// # Arguments
///
/// * `x` - Element to find root of
///
/// # Returns
///
/// Root element or error if out of bounds.
///
/// # Examples
///
/// ```
/// use crate::UnionFind;
/// let mut uf = UnionFind::new(5);
/// assert_eq!(uf.find(0).unwrap(), 0);
/// ```
///
/// # Complexity
///
/// O(α(n)) amortized time
pub fn find(&mut self, x: usize) -> Result<usize, Error>
"#
    );
}

fn demonstrate_semver() {
    println!("Semantic Versioning: MAJOR.MINOR.PATCH");

    println!("\nVersion increments:");
    println!("  PATCH (0.1.0 → 0.1.1):");
    println!("    • Bug fixes");
    println!("    • Documentation updates");
    println!("    • Internal refactoring");

    println!("\n  MINOR (0.1.1 → 0.2.0):");
    println!("    • New features (backward compatible)");
    println!("    • New public APIs");
    println!("    • Deprecations");

    println!("\n  MAJOR (0.2.0 → 1.0.0 or 1.0.0 → 2.0.0):");
    println!("    • Breaking changes");
    println!("    • Removed public APIs");
    println!("    • Changed function signatures");
    println!("    • Changed behavior");

    println!("\nPre-1.0 special rules:");
    println!("  • 0.x.y - Initial development");
    println!("  • Breaking changes allowed in MINOR");
    println!("  • 0.1.0 → 0.2.0 can break compatibility");
    println!("  • 1.0.0 signals 'stable API'");

    println!("\nDependency version examples:");
    println!(
        r#"
serde = "1.0"           # ^1.0.0 (>=1.0.0, <2.0.0)
tokio = "~1.20"         # >=1.20.0, <1.21.0
rand = ">= 0.8, < 0.9"  # Explicit range
regex = "1"             # Latest 1.x.x
"#
    );
}

fn demonstrate_validation() {
    println!("Pre-publish validation checklist:");

    let checks = vec![
        ("1. Cargo.toml complete", true),
        ("2. LICENSE files present", true),
        ("3. README.md exists", true),
        ("4. All tests pass", true),
        ("5. Documentation complete", true),
        ("6. No clippy warnings", true),
        ("7. Code formatted", true),
        ("8. No uncommitted changes", false),
    ];

    for (check, passed) in checks {
        let status = if passed { "✅" } else { "❌" };
        println!("  {} {}", status, check);
    }

    println!("\nValidation commands:");
    println!("  cargo fmt --check");
    println!("  cargo clippy -- -D warnings");
    println!("  cargo test");
    println!("  cargo test --doc");
    println!("  cargo doc --no-deps");
    println!("  cargo package --list");
    println!("  cargo package --allow-dirty");

    println!("\nPublishing workflow:");
    println!("  1. Ensure all checks pass");
    println!("  2. Update version in Cargo.toml");
    println!("  3. Update CHANGELOG.md");
    println!("  4. Commit: 'Release v0.1.0'");
    println!("  5. Tag: git tag v0.1.0");
    println!("  6. Dry run: cargo publish --dry-run");
    println!("  7. Publish: cargo publish");
    println!("  8. Push: git push origin main --tags");

    println!("\nPost-publish:");
    println!("  • Verify on crates.io");
    println!("  • Check docs.rs build");
    println!("  • Announce on social media");
    println!("  • Monitor for issues");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_semver_parsing() {
        // In real code, would use semver crate
        let version = "1.2.3";
        let parts: Vec<&str> = version.split('.').collect();
        assert_eq!(parts, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_version_increment() {
        let major = 1;
        let minor = 2;
        let patch = 3;

        // Patch increment
        let new_patch = patch + 1;
        assert_eq!(new_patch, 4); // 1.2.4

        // Minor increment (resets patch)
        let new_minor = minor + 1;
        assert_eq!(format!("{}.{}.0", major, new_minor), "1.3.0");

        // Major increment (resets minor and patch)
        let new_major = major + 1;
        assert_eq!(format!("{}.0.0", new_major), "2.0.0");
    }
}
