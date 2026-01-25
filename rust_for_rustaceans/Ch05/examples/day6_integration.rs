//! Day 6: Integration - Combining All Ch5 Concepts
//! 
//! This example brings together:
//! - Features for optional functionality
//! - Conditional compilation for platforms
//! - Version management concepts
//! - Best practices from the chapter

/// A sample library that demonstrates Ch5 concepts
pub struct ProjectConfig {
    name: String,
    version: String,
    #[cfg(feature = "advanced")]
    advanced_settings: AdvancedSettings,
}

#[cfg(feature = "advanced")]
pub struct AdvancedSettings {
    optimization_level: u8,
    enable_caching: bool,
}

impl ProjectConfig {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            #[cfg(feature = "advanced")]
            advanced_settings: AdvancedSettings::default(),
        }
    }
    
    #[cfg(feature = "advanced")]
    pub fn with_advanced_settings(mut self, optimization: u8, caching: bool) -> Self {
        self.advanced_settings = AdvancedSettings {
            optimization_level: optimization,
            enable_caching: caching,
        };
        self
    }
    
    pub fn display_info(&self) {
        println!("=== Project Configuration ===");
        println!("Name: {}", self.name);
        println!("Version: {}", self.version);
        
        #[cfg(feature = "advanced")]
        {
            println!("Optimization: {}", self.advanced_settings.optimization_level);
            println!("Caching: {}", self.advanced_settings.enable_caching);
        }
        
        #[cfg(not(feature = "advanced"))]
        {
            println!("Advanced settings: Disabled");
        }
    }
}

#[cfg(feature = "advanced")]
impl Default for AdvancedSettings {
    fn default() -> Self {
        Self {
            optimization_level: 2,
            enable_caching: true,
        }
    }
}

/// Platform-specific resource management
pub struct ResourceManager {
    platform: &'static str,
    arch: &'static str,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            platform: Self::detect_platform(),
            arch: Self::detect_arch(),
        }
    }
    
    fn detect_platform() -> &'static str {
        if cfg!(target_os = "windows") {
            "Windows"
        } else if cfg!(target_os = "linux") {
            "Linux"
        } else if cfg!(target_os = "macos") {
            "macOS"
        } else {
            "Other"
        }
    }
    
    fn detect_arch() -> &'static str {
        if cfg!(target_arch = "x86_64") {
            "x86_64"
        } else if cfg!(target_arch = "aarch64") {
            "ARM64"
        } else {
            "Other"
        }
    }
    
    pub fn display_info(&self) {
        println!("\n=== Resource Manager ===");
        println!("Platform: {}", self.platform);
        println!("Architecture: {}", self.arch);
        
        #[cfg(all(unix, target_pointer_width = "64"))]
        println!("Optimizations: Unix 64-bit specific");
        
        #[cfg(windows)]
        println!("Windows API: Available");
        
        if cfg!(debug_assertions) {
            println!("Build mode: Debug");
        } else {
            println!("Build mode: Release");
        }
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Feature detection utility
pub struct FeatureDetector;

impl FeatureDetector {
    pub fn detect_all() {
        println!("\n=== Feature Detection ===");
        
        Self::check_feature("std", cfg!(feature = "std"));
        Self::check_feature("advanced", cfg!(feature = "advanced"));
        Self::check_feature("networking", cfg!(feature = "networking"));
        Self::check_feature("compression", cfg!(feature = "compression"));
        Self::check_feature("experimental", cfg!(feature = "experimental"));
        Self::check_feature("serde", cfg!(feature = "serde"));
    }
    
    fn check_feature(name: &str, enabled: bool) {
        let status = if enabled { "✓" } else { "✗" };
        println!("  {} {}", status, name);
    }
}

/// Workspace simulation
pub mod workspace_demo {
    pub fn show_structure() {
        println!("\n=== Workspace Structure Example ===\n");
        println!("rust_study/");
        println!("├── Cargo.toml                    (workspace root)");
        println!("├── Cargo.lock                    (shared, 80+ crates)");
        println!("├── target/                       (shared builds)");
        println!("├── rust_for_rustaceans/");
        println!("│   ├── Ch01/");
        println!("│   ├── Ch04/");
        println!("│   └── Ch05/                     (this crate)");
        println!("├── missions/");
        println!("│   ├── Mission1/ ... Mission12/");
        println!("├── tutorials/");
        println!("│   └── Mission*_tut/");
        println!("├── advent_of_code/");
        println!("│   ├── aoc2015/ ... aoc2025/");
        println!("└── project_euler/");
        
        println!("\nBenefits:");
        println!("  ✓ Single Cargo.lock");
        println!("  ✓ Shared target/ directory");
        println!("  ✓ Compile shared deps once");
        println!("  ✓ Easy cross-crate testing");
    }
}

/// Versioning best practices reminder
pub mod versioning_guide {
    pub fn show_checklist() {
        println!("\n=== Versioning Checklist ===\n");
        
        println!("Before releasing:");
        println!("  [ ] Update CHANGELOG.md");
        println!("  [ ] Bump version in Cargo.toml");
        println!("  [ ] Run cargo test --all");
        println!("  [ ] Check MSRV compatibility");
        println!("  [ ] Test with minimal versions");
        
        println!("\nAfter releasing:");
        println!("  [ ] git tag vX.Y.Z");
        println!("  [ ] cargo publish");
        println!("  [ ] Bump to X.Y.(Z+1)-alpha.1");
        println!("  [ ] Update documentation");
    }
}

fn main() {
    println!("🎯 Day 6: Integration Example\n");
    println!("Combining all Chapter 5 concepts:\n");
    
    // 1. Project configuration with features
    let config = ProjectConfig::new("rust-study", "0.1.0");
    
    #[cfg(feature = "advanced")]
    let config = config.with_advanced_settings(3, true);
    
    config.display_info();
    
    // 2. Platform detection and conditional compilation
    let resources = ResourceManager::new();
    resources.display_info();
    
    // 3. Feature detection
    FeatureDetector::detect_all();
    
    // 4. Workspace structure
    workspace_demo::show_structure();
    
    // 5. Versioning guide
    versioning_guide::show_checklist();
    
    // Summary
    println!("\n=== Chapter 5 Summary ===\n");
    
    println!("✓ Features: Optional functionality via compile-time flags");
    println!("✓ Workspaces: Multi-crate project organization");
    println!("✓ Configuration: Profiles, metadata, patching");
    println!("✓ Conditional Compilation: Platform-specific code");
    println!("✓ Versioning: SemVer, MSRV, dependency management");
    
    println!("\n=== Key Takeaways ===\n");
    
    println!("1. Features should be additive");
    println!("2. Use workspaces for large projects");
    println!("3. Tune profiles for dev vs release");
    println!("4. Conditional compilation avoids runtime overhead");
    println!("5. Document and test your MSRV");
    println!("6. Keep changelogs for users");
    println!("7. Use pre-release versions during development");
    
    println!("\n=== Next Steps ===\n");
    
    println!("Week 6: Chapter 6 - Testing");
    println!("  • Unit tests");
    println!("  • Integration tests");
    println!("  • Doctests");
    println!("  • Benchmarks");
    
    println!("\n📝 Practice:");
    println!("  1. Analyze rust_study workspace structure");
    println!("  2. Add feature flags to Project Euler solutions");
    println!("  3. Create platform-specific optimizations");
    println!("  4. Set up MSRV testing in CI");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_project_config() {
        let config = ProjectConfig::new("test", "1.0.0");
        assert_eq!(config.name, "test");
        assert_eq!(config.version, "1.0.0");
    }
    
    #[test]
    fn test_resource_manager() {
        let rm = ResourceManager::new();
        // Platform and arch will vary based on build target
        assert!(!rm.platform.is_empty());
        assert!(!rm.arch.is_empty());
    }
    
    #[test]
    #[cfg(feature = "advanced")]
    fn test_advanced_features() {
        // This test only runs when advanced feature is enabled
        let config = ProjectConfig::new("test", "1.0.0")
            .with_advanced_settings(3, false);
        assert_eq!(config.advanced_settings.optimization_level, 3);
        assert!(!config.advanced_settings.enable_caching);
    }
}
