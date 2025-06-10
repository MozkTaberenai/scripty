//! # 00 - Hello World: Getting Started with scripty
//!
//! This example introduces the fundamental concepts of scripty:
//! - Running simple commands
//! - Capturing command output
//! - Working with arguments
//! - Basic error handling
//! - Using the builder pattern
//!
//! Estimated time: ~3 minutes
//! Prerequisites: None
//! Next example: 01_simple_pipes.rs

use scripty::*;

fn main() -> Result<()> {
    println!("Welcome to scripty - Let's start with the basics!");
    println!("==================================================\n");

    // 1. The simplest way to run a command
    println!("1. Running a simple command:");
    cmd!("echo", "Hello, World!").run()?;
    println!();

    // 2. Capturing command output
    println!("2. Capturing output from a command:");
    let output = cmd!("echo", "Hello from scripty!").output()?;
    println!("   Captured: {}", output.trim());
    println!();

    // 3. Commands with multiple arguments
    println!("3. Commands with multiple arguments:");
    cmd!("echo", "Multiple", "arguments", "work", "great!").run()?;
    println!();

    // 4. Using the builder pattern for more control
    println!("4. Builder pattern for advanced configuration:");
    let current_date = cmd!("date")
        .arg("+%Y-%m-%d") // Add argument for date format
        .output()?;
    println!("   Today's date: {}", current_date.trim());
    println!();

    // 5. Working with environment variables
    println!("5. Setting environment variables:");
    let greeting = cmd!("sh", "-c", "echo Hello, $NAME!")
        .env("NAME", "scripty user")
        .output()?;
    println!("   {}", greeting.trim());
    println!();

    // 6. Changing working directory
    println!("6. Running commands in different directories:");
    let temp_dir = std::env::temp_dir();
    let pwd_output = cmd!("pwd").current_dir(&temp_dir).output()?;
    println!("   Working in: {}", pwd_output.trim());
    println!();

    // 7. Suppressing command echo for cleaner output
    println!("7. Quiet mode (no command echo):");
    let quiet_output = cmd!("echo", "This command won't be echoed")
        .no_echo()
        .output()?;
    println!("   Output: {}", quiet_output.trim());
    println!("   (Notice: no command echo above)");
    println!();

    // 8. Error handling demonstration
    println!("8. Error handling:");
    match cmd!("this_command_does_not_exist").run() {
        Ok(_) => println!("   Unexpected success!"),
        Err(e) => println!("   Expected error: {}", e),
    }
    println!();

    // 9. Binary output handling
    println!("9. Working with binary data:");
    let byte_count = cmd!("echo", "-n", "Binary data").output_bytes()?.len();
    println!("   Output contains {} bytes", byte_count);
    println!();

    // 10. Providing input to commands
    println!("10. Sending input to commands:");
    let uppercase = cmd!("tr", "[:lower:]", "[:upper:]")
        .input("hello scripty")
        .output()?;
    println!("    Input: hello scripty");
    println!("    Output: {}", uppercase.trim());

    println!("\n✓ Congratulations! You've learned the scripty basics!");
    println!("\nReady for more? Try these next examples:");
    println!("   • cargo run --example 01_simple_pipes");
    println!("   • cargo run --example fs_operations");
    println!("\nPro tip: Set NO_ECHO=1 to disable all command echoing");

    Ok(())
}

// Type alias for convenience
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
