# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

HpCli is a multi-purpose command-line application written in Rust that provides:
- File encryption/decryption using cryptographic algorithms
- Hashing functions (SHA256, SHA512, MD5)
- String manipulation and search utilities
- File zipping/unzipping capabilities
- Random data generation
- String inspection tools

The application is built using the Clap library for CLI argument parsing and supports various cryptographic operations through the Orion library.

## Common Commands

### Build and Development
```bash
# Build the project
cargo build

# Build for release
cargo build --release

# Run the application
cargo run

# Run with specific command (examples)
cargo run -- hashing --type string --algorithm sha256 --input "test"
cargo run -- encrypting --type file --algorithm default --file-path input.txt --output encrypted.txt --password mypassword
cargo run -- decrypting --type file --algorithm default --file-path encrypted.txt --output decrypted.txt --password mypassword

# Check code without building
cargo check

# Run tests (if any exist)
cargo test

# Generate documentation
cargo doc --open

# Clean build artifacts
cargo clean
```

### Application Usage Examples
```bash
# Hash a string using SHA256
./target/release/HpCli hashing -t string -a sha256 -i "hello world"

# Encrypt a file
./target/release/HpCli encrypting -t file -a default -f input.txt -o encrypted.txt -p password123

# Decrypt a file
./target/release/HpCli decrypting -t file -a default -f encrypted.txt -o decrypted.txt -p password123

# Inspect a string
./target/release/HpCli inspect "hello123" --digits

# String search functionality
./target/release/HpCli string-search "search text" "text"
```

## Code Architecture

### Module Structure
- **main.rs**: Entry point, command parsing and routing
- **cliapp.rs**: CLI application structure using Clap
- **commands.rs**: Command definitions and argument structures
- **cmd_handler.rs**: Command processing logic and routing
- **cryptography/**: Cryptographic operations module
  - **hash_algorithm.rs**: Hashing algorithms implementation
  - **file_encryption.rs**: File encryption functionality
  - **file_decryption.rs**: File decryption functionality
  - **common.rs**: Shared cryptographic utilities
- **files/**: File handling utilities
  - **file_helper.rs**: File system operations
  - **zipping.rs**: Compression/decompression
- **strings/**: String manipulation utilities
  - **string_handler.rs**: String processing functions
- **credentials.rs**: API credentials management
- **rand_generator.rs**: Random data generation

### Command Flow
1. CLI arguments are parsed in `main.rs` using the `CliApp` struct
2. Commands are matched against the `Commands` enum in `commands.rs`
3. Each command is routed to appropriate handlers in `cmd_handler.rs`
4. Handlers call specific functionality from respective modules

### Key Dependencies
- **clap**: Command-line argument parsing with derive macros
- **anyhow**: Error handling
- **orion**: Cryptographic operations
- **sha2**: SHA hashing algorithms
- **md5**: MD5 hashing
- **rand**: Random number generation
- **serde/serde_json**: Serialization support
- **tempfile**: Temporary file handling

### Error Handling
The application uses `anyhow::Result` for error handling throughout the codebase. Most functions return `Result<T, Error>` and errors are propagated up the call stack.

### Cryptographic Implementation
- Uses Orion library for secure encryption/decryption operations
- Supports multiple hashing algorithms (SHA256, SHA512, MD5)
- File encryption uses password-based encryption with the "default" algorithm
- Hash operations support both string and file inputs

## Development Notes

- The project follows standard Rust project structure with `src/` containing all source code
- Uses Cargo for dependency management and build automation
- No custom test runners or lint configurations are present - use standard Cargo commands
- The application prints current directory and debug information on startup
- Some command handlers (like Random, StringSearch) have incomplete implementations
- The project appears to be in active development with some TODO comments and unused code paths