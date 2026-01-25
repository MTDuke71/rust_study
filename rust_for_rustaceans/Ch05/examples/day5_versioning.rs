//! Day 5: Versioning - SemVer, MSRV, and Dependency Management
//! 
//! This example demonstrates:
//! - Semantic versioning concepts
//! - MSRV (Minimum Supported Rust Version)
//! - Dependency version requirements
//! - Changelog best practices

use std::fmt;

/// Represents a semantic version
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemVer {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre_release: Option<String>,
}

impl SemVer {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            pre_release: None,
        }
    }
    
    pub fn with_pre_release(mut self, pre: &str) -> Self {
        self.pre_release = Some(pre.to_string());
        self
    }
    
    /// Check if this is a breaking change from another version
    pub fn is_breaking_change_from(&self, other: &SemVer) -> bool {
        self.major > other.major
    }
    
    /// Check if this is a feature addition from another version
    pub fn is_feature_addition_from(&self, other: &SemVer) -> bool {
        self.major == other.major && self.minor > other.minor
    }
    
    /// Check if this is a bug fix from another version
    pub fn is_bug_fix_from(&self, other: &SemVer) -> bool {
        self.major == other.major && 
        self.minor == other.minor && 
        self.patch > other.patch
    }
}

impl fmt::Display for SemVer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(pre) = &self.pre_release {
            write!(f, "-{}", pre)?;
        }
        Ok(())
    }
}

/// Demonstrates semantic versioning rules
fn semver_examples() {
    println!("=== Semantic Versioning (SemVer) ===\n");
    
    let v1_0_0 = SemVer::new(1, 0, 0);
    let v1_1_0 = SemVer::new(1, 1, 0);
    let v1_1_1 = SemVer::new(1, 1, 1);
    let v2_0_0 = SemVer::new(2, 0, 0);
    
    println!("Version progression:");
    println!("  {} -> {} (feature addition)", v1_0_0, v1_1_0);
    println!("  {} -> {} (bug fix)", v1_1_0, v1_1_1);
    println!("  {} -> {} (breaking change)", v1_1_1, v2_0_0);
    
    println!("\nChange detection:");
    println!("  {}.is_breaking_change_from({}) = {}", 
             v2_0_0, v1_1_1, v2_0_0.is_breaking_change_from(&v1_1_1));
    println!("  {}.is_feature_addition_from({}) = {}", 
             v1_1_0, v1_0_0, v1_1_0.is_feature_addition_from(&v1_0_0));
    println!("  {}.is_bug_fix_from({}) = {}", 
             v1_1_1, v1_1_0, v1_1_1.is_bug_fix_from(&v1_1_0));
}

/// Demonstrates pre-release versioning
fn prerelease_examples() {
    println!("\n=== Pre-Release Versions ===\n");
    
    let versions = vec![
        SemVer::new(2, 0, 0).with_pre_release("alpha.1"),
        SemVer::new(2, 0, 0).with_pre_release("alpha.2"),
        SemVer::new(2, 0, 0).with_pre_release("beta.1"),
        SemVer::new(2, 0, 0).with_pre_release("rc.1"),
        SemVer::new(2, 0, 0),
    ];
    
    println!("Development progression:");
    for version in versions {
        println!("  {}", version);
    }
    
    println!("\nWorkflow after release:");
    println!("  1. Release: 2.0.3");
    println!("  2. Immediately bump to: 2.0.4-alpha.1");
    println!("  3. Continue development");
    println!("  4. Before next release: 2.0.4");
}

/// Demonstrates dependency version requirements
fn dependency_versions() {
    println!("\n=== Dependency Version Requirements ===\n");
    
    let examples = vec![
        ("1.7.3", ">= 1.7.3"),
        ("^1.7.3", ">= 1.7.3, < 2.0.0 (caret - default)"),
        ("~1.7.3", ">= 1.7.3, < 1.8.0 (tilde - patch updates)"),
        ("1.7.*", ">= 1.7.0, < 1.8.0 (wildcard)"),
        (">=1.7.3, <2.0.0", "Explicit range"),
        ("1.7.3-alpha.1", "Exact pre-release version"),
    ];
    
    for (requirement, meaning) in examples {
        println!("  {} → {}", requirement, meaning);
    }
}

/// Demonstrates MSRV considerations
fn msrv_examples() {
    println!("\n=== Minimum Supported Rust Version (MSRV) ===\n");
    
    println!("In Cargo.toml:");
    println!("  [package]");
    println!("  rust-version = \"1.70\"");
    
    println!("\nMSRV Policies:");
    println!("  1. Conservative: Latest stable - 2 releases");
    println!("  2. Moderate: Latest stable - 6 months");
    println!("  3. Aggressive: Latest stable");
    
    println!("\nWhen to bump MSRV:");
    println!("  • Need new language features");
    println!("  • Dependency requires newer version");
    println!("  • Security fixes in newer compiler");
    
    println!("\nHow to bump MSRV:");
    println!("  • Document in CHANGELOG");
    println!("  • Consider bumping minor version");
    println!("  • Test with CI on multiple Rust versions");
}

/// Demonstrates minimal dependency versions
fn minimal_versions() {
    println!("\n=== Minimal Dependency Versions ===\n");
    
    println!("Problem:");
    println!("  Your Cargo.toml: serde = \"1.7.3\" (means >= 1.7.3)");
    println!("  Cargo resolves to: serde 1.0.150 (latest)");
    println!("  Your code uses: API added in 1.0.100");
    println!("  ❌ Breaks for users with serde 1.7.3");
    
    println!("\nSolution:");
    println!("  Test with minimal versions:");
    println!("    cargo +nightly update -Zminimal-versions");
    println!("    cargo build");
    
    println!("\nBest Practice:");
    println!("  Add to CI to catch this automatically");
}

/// Example changelog structure
fn changelog_example() {
    println!("\n=== Changelog Best Practices ===\n");
    
    println!("## [Unreleased]");
    println!("### Added");
    println!("- New feature X for improved performance");
    println!("### Fixed");
    println!("- Bug in edge case Y");
    println!();
    println!("## [2.0.0] - 2024-01-15");
    println!("### Breaking");
    println!("- Changed API for better ergonomics");
    println!("### Added");
    println!("- Support for async operations");
    println!("### Fixed");
    println!("- Memory leak in resource cleanup");
    println!();
    println!("## [1.5.0] - 2023-12-01");
    println!("### Added");
    println!("- New configuration options");
    
    println!("\nFollow: https://keepachangelog.com/");
}

fn main() {
    println!("🎯 Day 5: Versioning Example\n");
    
    semver_examples();
    prerelease_examples();
    dependency_versions();
    msrv_examples();
    minimal_versions();
    changelog_example();
    
    println!("\n=== Version Bump Decision Tree ===\n");
    
    println!("Breaking changes (API incompatibility):");
    println!("  → Bump MAJOR version (1.x.x → 2.0.0)");
    
    println!("\nNew features (backwards compatible):");
    println!("  → Bump MINOR version (1.5.x → 1.6.0)");
    
    println!("\nBug fixes (backwards compatible):");
    println!("  → Bump PATCH version (1.5.3 → 1.5.4)");
    
    println!("\nMSRV increase:");
    println!("  → Consider MINOR bump (depends on policy)");
    
    println!("\n=== Real-World Versioning Workflow ===\n");
    
    println!("1. After releasing 2.0.3:");
    println!("   git tag v2.0.3");
    println!("   cargo publish");
    
    println!("\n2. Immediately bump version:");
    println!("   version = \"2.0.4-alpha.1\" in Cargo.toml");
    println!("   git commit -m \"chore: bump version to 2.0.4-alpha.1\"");
    
    println!("\n3. Continue development:");
    println!("   Add features, fix bugs");
    
    println!("\n4. Before next release:");
    println!("   Update CHANGELOG.md");
    println!("   version = \"2.0.4\" in Cargo.toml");
    println!("   git tag v2.0.4");
    println!("   cargo publish");
    
    println!("\n📝 Testing your versioning:");
    println!("  cargo +nightly update -Zminimal-versions");
    println!("  cargo msrv  (requires cargo-msrv)");
    println!("  cargo semver-checks  (requires cargo-semver-checks)");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_semver_comparisons() {
        let v1 = SemVer::new(1, 0, 0);
        let v2 = SemVer::new(2, 0, 0);
        
        assert!(v2.is_breaking_change_from(&v1));
        assert!(!v1.is_breaking_change_from(&v2));
    }
    
    #[test]
    fn test_version_display() {
        let v = SemVer::new(1, 2, 3);
        assert_eq!(v.to_string(), "1.2.3");
        
        let pre = v.with_pre_release("alpha.1");
        assert_eq!(pre.to_string(), "1.2.3-alpha.1");
    }
}
