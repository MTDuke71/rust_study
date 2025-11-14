# Custom Cargo Command Example - Chapter 14.5

This example demonstrates how to create and use custom Cargo commands, as covered in Rust Book Chapter 14.5.

## Overview

Custom Cargo commands extend Cargo's functionality by following a simple naming convention:
- Binary name: `cargo-<command-name>`
- Usage: `cargo <command-name>`

## Development and Testing

### During Development
While developing the custom command, you can test it directly:

```bash
# Run the binary directly
cargo run -p cargo-example --bin cargo-example

# Test with arguments
cargo run -p cargo-example --bin cargo-example -- --help
cargo run -p cargo-example --bin cargo-example -- --verbose
cargo run -p cargo-example --bin cargo-example -- demo --count 3
cargo run -p cargo-example --bin cargo-example -- version
```

### Installation for Use as Cargo Subcommand

To actually use it as `cargo example`, you need to install it:

```bash
# Navigate to the custom command directory
cd rust_book/Ch14/custom_command

# Install locally (installs to ~/.cargo/bin/)
cargo install --path .

# Now you can use it as a cargo subcommand
cargo example
cargo example --help
cargo example --verbose
cargo example demo --count 3
cargo example version
```

### Uninstallation

```bash
cargo uninstall cargo-example
```

### Complete Lifecycle Demo Scripts

The project includes demo scripts that show the complete lifecycle:

**Installation Demo:**
```bash
# PowerShell (Windows)
.\install_demo.ps1

# Bash (Linux/Mac)
./install_demo.sh
```

**Uninstallation Demo:**
```bash
# PowerShell (Windows)
.\uninstall_demo.ps1

# Bash (Linux/Mac)  
./uninstall_demo.sh
```

These scripts demonstrate:
- Development testing with `cargo run`
- Failed attempt to use `cargo example` before installation
- Installation process with `cargo install --path .`
- Successful usage as `cargo example` after installation
- Complete uninstallation process
- Verification that development mode still works after uninstall

## How It Works

1. **Naming Convention**: The binary is named `cargo-example`
2. **PATH Integration**: When installed via `cargo install`, it goes to `~/.cargo/bin/`
3. **Cargo Discovery**: Cargo automatically finds binaries in PATH that start with `cargo-`
4. **Argument Handling**: The command receives "example" as the first argument when called via `cargo example`

## Smart Detection

The example code detects how it's being called:
- When called directly (`cargo-example`): Shows development mode messages
- When called via cargo (`cargo example`): Shows success messages

This helps demonstrate the difference between development testing and actual usage.

## Popular Real-World Examples

Many popular Rust tools follow this pattern:
- `cargo-watch` → `cargo watch`
- `cargo-audit` → `cargo audit` 
- `cargo-expand` → `cargo expand`
- `cargo-flamegraph` → `cargo flamegraph`

## Key Learning Points

1. **Binary Naming**: Must follow `cargo-<name>` convention
2. **Installation Required**: `cargo <name>` only works after `cargo install`
3. **Argument Parsing**: Handle the extra "name" argument when called via cargo
4. **Development Testing**: Use `cargo run` during development
5. **Global Availability**: Once installed, available from any directory

## File Structure

```
custom_command/
├── Cargo.toml          # Package metadata with [[bin]] section
├── src/
│   └── main.rs         # Command implementation with clap
└── README.md           # This documentation
```

This example demonstrates the complete lifecycle of creating, testing, installing, and using custom Cargo commands as described in Chapter 14.5 of The Rust Programming Language book.

---
*Links: [[rust_book/Ch14]] [[rust_book/Ch14/custom_command]] [[rust-book-ch14-command-line-arguments]]*