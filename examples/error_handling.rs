//! # Error Handling: Best Practices for Robust Scripts
//!
//! This example demonstrates comprehensive error handling strategies with scripty:
//! - Different types of errors and how to handle them
//! - Checking exit codes and command success
//! - Pipeline error propagation
//! - Graceful error recovery
//! - Custom error handling patterns
//! - Debugging failed commands
//!
//! Estimated time: ~5 minutes
//! Prerequisites: Basic familiarity with Rust error handling
//! Related examples: 00_hello_world.rs, 01_simple_pipes.rs

use scripty::*;
use std::fs;

fn main() {
    println!("🛡️ Error Handling Best Practices with scripty");
    println!("===========================================\n");

    // 1. Basic error checking
    println!("1️⃣ Basic error checking:");
    basic_error_handling();
    println!();

    // 2. Checking specific error conditions
    println!("2️⃣ Checking specific error conditions:");
    check_error_types();
    println!();

    // 3. Pipeline error propagation
    println!("3️⃣ Pipeline error propagation:");
    pipeline_errors();
    println!();

    // 4. Graceful fallbacks
    println!("4️⃣ Implementing graceful fallbacks:");
    graceful_fallbacks();
    println!();

    // 5. Collecting multiple errors
    println!("5️⃣ Handling multiple operations:");
    handle_multiple_operations();
    println!();

    // 6. Custom error context
    println!("6️⃣ Adding custom error context:");
    custom_error_context();
    println!();

    // 7. Exit code handling
    println!("7️⃣ Working with exit codes:");
    exit_code_handling();
    println!();

    // 8. Debugging helpers
    println!("8️⃣ Debugging failed commands:");
    debugging_helpers();

    println!("\n🎉 Error handling examples completed!");
    println!("💡 Remember: Good error handling makes scripts more reliable and easier to debug!");
}

fn basic_error_handling() {
    // Using match for explicit handling
    match cmd!("ls", "/nonexistent").no_echo().run() {
        Ok(_) => println!("   ✅ Command succeeded"),
        Err(e) => println!("   ❌ Command failed: {}", e),
    }

    // Using if-let for success-only handling
    if let Ok(output) = cmd!("echo", "Success").no_echo().output() {
        println!("   ✅ Got output: {}", output.trim());
    }

    // Using unwrap_or for default values
    let output = cmd!("cat", "/nonexistent")
        .no_echo()
        .output()
        .unwrap_or_else(|_| "File not found".to_string());
    println!("   Output with fallback: {}", output.trim());
}

fn check_error_types() {
    // Command not found
    let result = cmd!("definitely_not_a_real_command").no_echo().run();
    if let Err(e) = result {
        println!("   Command not found error: {}", e);
    }

    // Permission denied (trying to write to root)
    #[cfg(unix)]
    {
        let result = cmd!("touch", "/root/test.txt").no_echo().run();
        if let Err(e) = result {
            println!("   Permission error: {}", e);
        }
    }

    // Invalid arguments
    let result = cmd!("ls", "--invalid-flag").no_echo().run();
    if let Err(e) = result {
        println!("   Invalid argument error: {}", e);
    }
}

fn pipeline_errors() {
    // Error in first command
    let result = cmd!("cat", "/nonexistent")
        .pipe(cmd!("grep", "pattern"))
        .no_echo()
        .run();

    match result {
        Ok(_) => println!("   Pipeline succeeded"),
        Err(e) => println!("   Pipeline failed at first command: {}", e),
    }

    // Error in middle of pipeline
    let result = cmd!("echo", "test")
        .pipe(cmd!("nonexistent_filter"))
        .pipe(cmd!("wc", "-l"))
        .no_echo()
        .run();

    if let Err(e) = result {
        println!("   Pipeline failed in middle: {}", e);
    }

    // Successful pipeline for comparison
    let result = cmd!("echo", "success")
        .pipe(cmd!("grep", "success"))
        .no_echo()
        .output();

    match result {
        Ok(output) => println!("   ✓ Successful pipeline output: {}", output.trim()),
        Err(e) => println!("   ✗ Pipeline error: {}", e),
    }
}

fn graceful_fallbacks() {
    // Try multiple commands until one succeeds
    let editors = ["nano", "vim", "vi", "emacs"];
    let mut found = false;

    for editor in &editors {
        if cmd!("which", editor).no_echo().run().is_ok() {
            println!("   ✅ Found editor: {}", editor);
            found = true;
            break;
        }
    }

    if !found {
        println!("   ❌ No text editor found");
    }

    // Fallback with different approaches
    let content = cmd!("cat", "/etc/os-release")
        .no_echo()
        .output()
        .or_else(|_| cmd!("cat", "/etc/system-release").no_echo().output())
        .or_else(|_| cmd!("uname", "-a").no_echo().output())
        .unwrap_or_else(|_| "Unknown system".to_string());

    println!(
        "   System info: {}",
        content.lines().next().unwrap_or("Unknown")
    );
}

fn handle_multiple_operations() {
    let temp_dir = std::env::temp_dir().join("scripty_error_example");
    let mut errors = Vec::new();
    let mut successes = 0;

    // Create test directory
    let _ = fs::create_dir_all(&temp_dir);

    // Run multiple operations, collecting errors
    let operations = vec![
        ("Create file", cmd!("touch", temp_dir.join("test.txt"))),
        ("List directory", cmd!("ls", &temp_dir)),
        ("Invalid operation", cmd!("rm", "--invalid-flag", "/")),
        ("Show date", cmd!("date")),
    ];

    for (name, cmd) in operations {
        match cmd.no_echo().run() {
            Ok(_) => {
                successes += 1;
                println!("   ✅ {}", name);
            }
            Err(e) => {
                errors.push((name, e));
                println!("   ❌ {}", name);
            }
        }
    }

    println!(
        "   📊 Summary: {} succeeded, {} failed",
        successes,
        errors.len()
    );

    // Cleanup
    let _ = fs::remove_dir_all(&temp_dir);
}

fn custom_error_context() {
    // Helper function to add context to errors
    fn run_with_context(cmd: Cmd, context: &str) -> std::result::Result<String, String> {
        cmd.no_echo()
            .output()
            .map_err(|e| format!("{}: {}", context, e))
    }

    // Use the helper
    match run_with_context(cmd!("git", "status"), "Checking git repository") {
        Ok(output) => println!("   ✅ Git status: {}", output.lines().next().unwrap_or("")),
        Err(e) => println!("   ❌ {}", e),
    }

    // Chain operations with context
    let result = run_with_context(cmd!("which", "cargo"), "Finding cargo")
        .and_then(|_| run_with_context(cmd!("cargo", "--version"), "Checking cargo version"));

    match result {
        Ok(version) => println!("   ✅ Cargo: {}", version.trim()),
        Err(e) => println!("   ❌ {}", e),
    }
}

fn exit_code_handling() {
    // grep returns 1 when no matches found (not an error per se)
    let result = cmd!("echo", "hello")
        .pipe(cmd!("grep", "goodbye"))
        .no_echo()
        .run();

    match result {
        Ok(_) => println!("   ✅ Pattern found"),
        Err(_) => println!("   ⚠️ Pattern not found (grep exit code 1)"),
    }

    // Using output to check both success and result
    match cmd!("test", "-f", "/etc/passwd").no_echo().run() {
        Ok(_) => println!("   ✅ File exists"),
        Err(_) => println!("   ❌ File does not exist"),
    }

    // Differentiating between failure types
    let test_file = "/tmp/scripty_test_file";
    let _ = fs::write(test_file, "test");

    // This should succeed
    if cmd!("test", "-f", test_file).no_echo().run().is_ok() {
        println!("   ✅ Test file exists");
    }

    // This should fail with exit code 1 (not an execution error)
    if cmd!("test", "-d", test_file).no_echo().run().is_err() {
        println!("   ✅ Correctly identified: not a directory");
    }

    let _ = fs::remove_file(test_file);
}

fn debugging_helpers() {
    // Helper to debug command execution
    fn debug_command(cmd: Cmd) -> std::result::Result<String, Box<dyn std::error::Error>> {
        println!("   🔍 Attempting to run command...");

        match cmd.output() {
            Ok(output) => {
                println!("   ✅ Success! Output: {}", output.trim());
                Ok(output)
            }
            Err(e) => {
                println!("   ❌ Failed with error: {}", e);

                // Try to provide more context
                if let Err(_which_err) = cmd!("which", "nonexistent_command").no_echo().run() {
                    println!("   💡 Hint: Command might not be installed");
                }

                Err(e.into())
            }
        }
    }

    // Test with a command that might fail
    let _ = debug_command(cmd!("echo", "Debug test").no_echo());

    // Test with environment variables for debugging
    unsafe {
        std::env::set_var("DEBUG", "1");
    }
    let output = cmd!(
        "sh",
        "-c",
        "if [ \"$DEBUG\" = \"1\" ]; then echo '   🐛 Debug mode enabled'; fi"
    )
    .no_echo()
    .output()
    .unwrap_or_default();
    print!("{}", output);
    unsafe {
        std::env::remove_var("DEBUG");
    }
}
