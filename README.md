# scripty

[![Crates.io](https://img.shields.io/crates/v/scripty.svg)](https://crates.io/crates/scripty)
[![Documentation](https://docs.rs/scripty/badge.svg)](https://docs.rs/scripty)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/MozkTaberenai/scripty/workflows/CI/badge.svg)](https://github.com/MozkTaberenai/scripty/actions)

## scripty

**Scripty** - A simple and intuitive library that makes running shell commands and file operations easy and visible.

### Why scripty?

When you need to write system administration scripts, build tools, or automation in Rust,
you often find yourself wrestling with `std::process::Command` and `std::fs`. scripty
provides a clean, shell-script-like interface while keeping all the benefits of Rust's
type safety and error handling.

#### Key Features

- **🎨 Colorful output**: See exactly what commands are being executed
- **🔗 Easy piping**: Chain commands together naturally with stdout, stderr, or both
- **📁 File operations**: Wrapper around `std::fs` with automatic logging
- **🔧 Builder pattern**: Fluent API for command construction
- **⚡ Minimal dependencies**: Only uses `anstyle` for colors
- **🛡️ Type safe**: All the safety of Rust with the convenience of shell scripts
- **🚰 Streaming I/O**: Efficient handling of large data with readers and writers

### Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
scripty = "0.1.0"
```

### Platform Support

Currently supported platforms:
- **Linux** ✅ Full support with native pipe optimization
- **macOS** ✅ Full support with native pipe optimization

Scripty is designed for Unix-like systems and uses Unix shell commands and utilities.

### Requirements

- **Rust 1.87.0 or later** - Required for native pipeline performance with `std::io::pipe`

### Basic Usage

#### Command Execution

```rust
use scripty::*;

// Simple command execution
cmd!("echo", "Hello, World!").run()?;

// Get command output
let output = cmd!("date").output()?;
println!("Current date: {}", output.trim());

// Command with multiple arguments
cmd!("ls", "-la", "/tmp").run()?;

// Using the builder pattern
cmd!("grep", "error")
    .arg("logfile.txt")
    .current_dir("/var/log")
    .env("LANG", "C")
    .run()?;
```

#### Command Piping

Chain commands together just like in shell scripts. **New in Rust 1.87.0**: scripty now uses native `std::io::pipe` for enhanced performance and memory efficiency!

```rust
use scripty::*;

// Simple pipe (stdout)
cmd!("echo", "hello world")
    .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
    .run()?;

// Pipe stderr
cmd!("some-command")
    .pipe_stderr(cmd!("grep", "ERROR"))
    .run()?;

// Pipe both stdout and stderr
cmd!("some-command")
    .pipe_both(cmd!("sort"))
    .run()?;

// Multiple pipes - now using efficient native pipes!
cmd!("cat", "/etc/passwd")
    .pipe(cmd!("grep", "bash"))
    .pipe(cmd!("wc", "-l"))
    .run()?;

// Get piped output with streaming processing
let result = cmd!("ps", "aux")
    .pipe(cmd!("grep", "rust"))
    .pipe(cmd!("wc", "-l"))
    .output()?;
println!("Rust processes: {}", result.trim());
```

##### Pipeline Performance Improvements (Rust 1.87.0+)

- **Memory efficient**: Uses streaming instead of buffering all data
- **Better performance**: Native pipes reduce process overhead
- **Platform independent**: No shell dependency for multi-command pipes
- **Native implementation**: Uses `std::io::pipe` for optimal performance

```rust
use scripty::*;

// Large data processing with efficient streaming
let large_data = "..."; // Megabytes of data
let result = cmd!("grep", "pattern")
    .pipe(cmd!("sort"))
    .pipe(cmd!("uniq", "-c"))
    .input(large_data)
    .output()?; // Processes without loading all data into memory
```

#### Core API Reference

##### The `cmd!` Macro

The heart of scripty is the `cmd!` macro for creating commands:

```rust
use scripty::*;

// Basic command
cmd!("ls").run()?;

// Command with arguments
cmd!("ls", "-la").run()?;

// Multiple arguments
cmd!("echo", "Hello", "World").run()?;
```

##### Command Builder Methods

Commands support a fluent builder pattern:

```rust
use scripty::*;

cmd!("grep", "error")
    .arg("logfile.txt")                    // Add single argument
    .args(["--color", "always"])           // Add multiple arguments
    .current_dir("/var/log")               // Set working directory
    .env("LANG", "C")                      // Set environment variable
    .no_echo()                             // Suppress command echoing
    .run()?;
```

##### Execution Methods

Different ways to execute commands:

```rust
use scripty::*;

// Execute and check exit status
cmd!("echo", "hello").run()?;

// Capture text output
let output = cmd!("date").output()?;
println!("Current date: {}", output.trim());

// Capture binary output
let bytes = cmd!("cat", "binary-file").output_bytes()?;

// Stream to a writer
use std::fs::File;
let file = File::create("output.txt")?;
cmd!("echo", "hello").stream_to(file)?;
```

##### Input Methods

Provide input to commands in various ways:

```rust
use scripty::*;
use std::io::Cursor;

// Text input
let result = cmd!("sort")
    .input("banana\napple\ncherry\n")
    .output()?;
println!("Sorted fruits: {}", result.trim());

// Binary input
let bytes = cmd!("cat")
    .input_bytes(b"binary data")
    .output_bytes()?;

// Stream from reader
use std::fs::File;
let file = File::open("data.txt")?;
cmd!("sort").input_reader(file).run()?;

// Buffered reading for large files
use std::io::BufReader;
let large_file = File::open("large.txt")?;
cmd!("grep", "pattern").input_reader(BufReader::new(large_file)).run()?;
```

##### Advanced I/O Control

For complex I/O scenarios, use the spawn methods:

```rust
use scripty::*;
use std::io::{BufRead, BufReader, Write};
use std::thread;

// Full I/O control
let spawn = cmd!("sort").spawn_with_io()?;

// Handle input in separate thread
if let Some(mut stdin) = spawn.stdin {
    thread::spawn(move || {
        writeln!(stdin, "zebra").unwrap();
        writeln!(stdin, "apple").unwrap();
        writeln!(stdin, "banana").unwrap();
    });
}

// Read output
if let Some(stdout) = spawn.stdout {
    let reader = BufReader::new(stdout);
    for line in reader.lines() {
        println!("Line: {}", line?);
    }
}

spawn.handle.wait()?;
```

##### File System Operations

All file operations are automatically logged:

```rust
use scripty::*;

// Basic file operations
fs::write("config.txt", "debug=true\nport=8080")?;
let content = fs::read_to_string("config.txt")?;
println!("Config: {}", content);

// Directory operations
fs::create_dir_all("project/src")?;
fs::copy("config.txt", "project/config.txt")?;

// Directory traversal
for entry in fs::read_dir("project")? {
    let entry = entry?;
    println!("Path: {}", entry.path().display());
}

// Cleanup
fs::remove_file("config.txt")?;
fs::remove_dir_all("project")?;
```

##### Error Handling

Use standard Rust error handling patterns:

```rust
use scripty::*;

// Handle command failures gracefully
match cmd!("nonexistent-command").run() {
    Ok(_) => println!("Command succeeded"),
    Err(e) => println!("Command failed: {}", e),
}

// Check command availability
if cmd!("which", "git").no_echo().run().is_ok() {
    println!("Git is available");
    cmd!("git", "--version").run()?;
}

// Use the ? operator for early returns
fn deploy_app() -> Result<()> {
    cmd!("cargo", "build", "--release").run()?;
    cmd!("docker", "build", "-t", "myapp", ".").run()?;
    cmd!("docker", "push", "myapp").run()?;
    println!("Deployment complete!");
    Ok(())
}
```

##### Global Configuration

Control scripty's behavior with environment variables:

- `NO_ECHO`: Set to any value to suppress command echoing globally

```bash
NO_ECHO=1 cargo run  # Run without command echoing
```

Or use the `.no_echo()` method on individual commands.

### Examples

This crate includes focused examples showcasing scripty's core strengths: **pipeline operations** and **I/O handling**:

Examples are numbered for optimal learning progression:

1. **`01_simple_pipes.rs`** - Basic pipeline operations and command chaining
2. **`02_pipe_modes.rs`** - Complete pipeline control with stdout/stderr piping
3. **`03_io_patterns.rs`** - Complete I/O methods reference with Reader/Writer patterns

Run examples in order for the best learning experience:
```bash
cargo run --example 01_simple_pipes    # 1. Pipeline fundamentals
cargo run --example 02_pipe_modes      # 2. Advanced piping control
cargo run --example 03_io_patterns     # 3. Complete I/O methods
```

**Learning Path:** Start with `01_simple_pipes.rs` and progress through each numbered example in sequence to build your expertise with scripty's pipeline and I/O capabilities.

#### Advanced Pipeline Performance & Best Practices

##### Performance Optimization

scripty's native pipeline implementation (Rust 1.87.0+) provides significant performance benefits:

```rust
use scripty::*;

// ✅ Efficient: Native pipes with streaming
let result = cmd!("cat", "large_file.txt")
    .pipe(cmd!("grep", "pattern"))
    .pipe(cmd!("sort"))
    .pipe(cmd!("uniq", "-c"))
    .output()?; // Processes without loading all data into memory

// ✅ Memory efficient: Stream large data directly
use std::fs::File;
let large_file = File::open("multi_gb_file.txt")?;
cmd!("grep", "ERROR")
    .pipe(cmd!("wc", "-l"))
    .input_reader(large_file)
    .output()?; // Handles gigabytes efficiently
```

##### Pipeline Best Practices

**Memory Management:**
```rust
use scripty::*;

// ✅ Good: Stream processing for large data
cmd!("find", "/var/log", "-name", "*.log")
    .pipe(cmd!("xargs", "grep", "ERROR"))
    .pipe(cmd!("sort"))
    .output()?;

// ❌ Avoid: Loading large outputs into memory first
let large_output = cmd!("find", "/", "-type", "f").output()?; // Don't do this
cmd!("grep", "pattern").input(&large_output).output()?;
```

**Error-Prone Pipelines:**
```rust
use scripty::*;

// ✅ Good: Graceful error handling in pipelines
match cmd!("risky-command")
    .pipe(cmd!("sort"))
    .no_echo()
    .output()
{
    Ok(result) => println!("Success: {}", result.trim()),
    Err(_) => {
        // Fallback strategy
        println!("Using fallback approach");
        cmd!("safe-alternative").run()?;
    }
}
```

**Complex Data Processing:**
```rust
use scripty::*;

// ✅ Efficient multi-stage processing
let processed = cmd!("cat", "data.json")
    .pipe(cmd!("jq", ".items[]"))           // Extract items
    .pipe(cmd!("grep", "active"))           // Filter active
    .pipe(cmd!("jq", "-r", ".name"))        // Extract names
    .pipe(cmd!("sort"))                     // Sort results
    .output()?;
```

##### Troubleshooting Common Issues

**Large Data Processing:**
```rust
use scripty::*;
// Problem: Memory usage with large files
// Solution: Use streaming with BufReader
use std::io::BufReader;
use std::fs::File;

let large_file = File::open("huge_dataset.txt")?;
let reader = BufReader::new(large_file);

cmd!("awk", "{sum += $1} END {print sum}")
    .input_reader(reader)
    .output()?; // Processes efficiently regardless of file size
```

**Pipeline Debugging:**
```rust
use scripty::*;

// Enable command echoing for debugging (set before running your program)
// export SCRIPTY_DEBUG=1

// Commands are echoed by default unless .no_echo() is used
cmd!("complex-command")
    .pipe(cmd!("grep", "pattern"))
    .pipe(cmd!("sort"))
    .run()?; // Commands will be shown as they execute
```

**Error Isolation:**
```rust
use scripty::*;

// Test each stage of a complex pipeline individually
let stage1 = cmd!("stage1-command").output()?;
println!("Stage 1 output: {}", stage1);

let stage2 = cmd!("stage2-command").input(&stage1).output()?;
println!("Stage 2 output: {}", stage2);

// Then combine when each stage works correctly
```

### Platform Support

- **Linux** ✅ Full support with native pipe optimization
- **macOS** ✅ Full support with native pipe optimization
- **Windows** ❌ Not supported (Unix-like systems only)

### Contributing

We welcome contributions! Please see our [GitHub repository](https://github.com/MozkTaberenai/scripty) for more information.

### License

This project is licensed under the MIT License.

License: MIT


## Examples

The following examples are available in the `examples/` directory:

### 02_pipe_modes

```rust
println!("🔀 Pipeline Control with Different Pipe Modes");
println!("============================================\n");
// Section 1: Basic pipe modes
basic_pipe_modes()?;
// Section 2: Mixed mode examples
mixed_mode_examples()?;
println!("\n🎉 Pipe modes tutorial completed!");
println!("Key concepts learned:");
println!("  • pipe() - Routes stdout to next command's stdin (default)");
println!("  • pipe_stderr() - Routes stderr to next command's stdin");
println!("  • pipe_both() - Routes both stdout+stderr to next command's stdin");
println!("\n🚀 Next step:");
println!("   • Run 'cargo run --example 03_io_patterns' for I/O operations");
Ok(())
```

Run with: `cargo run --example 02_pipe_modes`

### 03_io_patterns

```rust
println!("📚 Complete I/O Methods Reference");
println!("================================\n");
// Method 1: input_reader()
input_reader_examples()?;
// Method 2: stream_to()
stream_to_examples()?;
// Method 3: run_with_io()
run_with_io_examples()?;
// Method 4: spawn_with_io()
spawn_with_io_examples()?;
// Method 5: spawn_with_stdin()
spawn_with_stdin_examples()?;
// Method 6: spawn_with_stdout()
spawn_with_stdout_examples()?;
// Method 7: spawn_with_stderr()
spawn_with_stderr_examples()?;
// Method 8: spawn_with_both()
spawn_with_both_examples()?;
println!("\n🎉 All I/O methods completed!");
println!("🏆 Congratulations! You've mastered all scripty I/O patterns!");
Ok(())
```

Run with: `cargo run --example 03_io_patterns`

### 01_simple_pipes

```rust
println!("🔗 Pipeline Fundamentals - scripty's Core Strength");
println!("=================================================\n");
// 1. Basic two-command pipes
println!("1. Basic pipelines:");
basic_pipes()?;
// 2. Multiple command chains
println!("\n2. Multi-stage command chains:");
multiple_pipes()?;
// 3. Input data processing
println!("\n3. Data processing pipelines:");
input_processing()?;
// 4. Performance and memory efficiency
println!("\n4. Performance advantages:");
performance_demo()?;
println!("\n🎉 Pipeline fundamentals completed!");
println!("🚀 Next step:");
println!("   • Run 'cargo run --example 02_pipe_modes' for stderr/stdout control");
Ok(())
```

Run with: `cargo run --example 01_simple_pipes`


## Development

This project uses `cargo xtask` for development tasks:

```bash
# Generate README.md
cargo xtask readme

# Run all tests
cargo xtask test

# Run code formatting
cargo xtask fmt

# Run clippy lints
cargo xtask clippy

# Run full CI pipeline
cargo xtask ci
```

