//! # 03 - I/O Patterns: Complete 8-Pattern Coverage
//!
//! This example demonstrates ALL 8 possible I/O control patterns in scripty:
//!
//! ## Mathematical Coverage: 2³ = 8 Patterns
//!
//! | Pattern | stdin | stdout | stderr | Method | Use Case |
//! |---------|-------|--------|--------|--------|----------|
//! | `000` | - | - | - | `.run()/.output()` | Basic execution |
//! | `100` | ✓ | - | - | `spawn_io_in()` | Data input |
//! | `010` | - | ✓ | - | `spawn_io_out()` | Output capture |
//! | `001` | - | - | ✓ | `spawn_io_err()` | Error monitoring |
//! | `110` | ✓ | ✓ | - | `spawn_io_in_out()` ⭐ | **Interactive processing** |
//! | `101` | ✓ | - | ✓ | `spawn_io_in_err()` ⭐ | **Debug scenarios** |
//! | `011` | - | ✓ | ✓ | `spawn_io_out_err()` | Output separation |
//! | `111` | ✓ | ✓ | ✓ | `spawn_io_all()` | Complete control |
//!
//! Core Methods:
//! - input_reader() - Use Reader as input (classic pattern)
//! - stream_to() - Stream output to Writer (classic pattern)  
//! - run_with_io() - Connect Reader and Writer (blocking)
//!
//! Complete spawn_io_* Pattern Coverage:
//! - spawn_io_in() - stdin only (Pattern 100)
//! - spawn_io_out() - stdout only (Pattern 010)
//! - spawn_io_err() - stderr only (Pattern 001)
//! - spawn_io_in_out() - stdin + stdout (Pattern 110) ⭐ Most Common Interactive
//! - spawn_io_in_err() - stdin + stderr (Pattern 101) ⭐ Debug Scenarios
//! - spawn_io_out_err() - stdout + stderr (Pattern 011)
//! - spawn_io_all() - complete control (Pattern 111)
//!
//! Estimated time: ~8 minutes
//! Prerequisites: Complete 02_pipe_modes.rs
//! Final example: Complete I/O pattern mastery!

use scripty::*;
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Write, copy};
use std::thread;

fn main() -> Result<()> {
    println!("📚 Complete I/O Methods Reference");
    println!("================================\n");

    // Method 1: input_reader()
    input_reader_examples()?;

    // Method 2: stream_to()
    stream_to_examples()?;

    // Method 3: run_with_io()
    run_with_io_examples()?;

    // Method 4: spawn_io_in() - stdin only
    spawn_io_in_examples()?;

    // Method 5: spawn_io_out() - stdout only
    spawn_io_out_examples()?;

    // Method 6: spawn_io_err() - stderr only
    spawn_io_err_examples()?;

    // Method 7: spawn_io_in_out() - stdin + stdout (⭐ Most Important!)
    spawn_io_in_out_examples()?;

    // Method 8: spawn_io_in_err() - stdin + stderr (⭐ Debug Pattern!)
    spawn_io_in_err_examples()?;

    // Method 9: spawn_io_out_err() - stdout + stderr
    spawn_io_out_err_examples()?;

    // Method 10: spawn_io_all() - complete control
    spawn_io_all_examples()?;

    println!("\n🎉 All 8 I/O patterns completed!");
    println!("✅ Complete mathematical coverage: 2³ = 8 patterns mastered!");
    println!("🏆 You now understand every possible I/O control scenario!");

    Ok(())
}

fn input_reader_examples() -> Result<()> {
    println!("📥 1. input_reader() - Use Reader as Input");
    println!("==========================================\n");

    // Example 1: Cursor (in-memory) as input
    println!("💾 Using Cursor:");
    let data = "apple\nbanana\ncherry\ndate\nfig";
    let cursor = Cursor::new(data.as_bytes());

    let output = cmd!("grep", "a").input_reader(cursor).output()?;

    println!("   Items containing 'a': {}", output.trim());
    println!();

    // Example 2: File as input
    println!("📁 Using File:");
    std::fs::write("input.txt", "line1\nspecial_line\nline3")?;
    let file = File::open("input.txt")?;

    let output = cmd!("grep", "special").input_reader(file).output()?;

    println!("   Special lines: {}", output.trim());
    std::fs::remove_file("input.txt").ok();
    println!();

    Ok(())
}

fn stream_to_examples() -> Result<()> {
    println!("📤 2. stream_to() - Stream Output to Writer");
    println!("===========================================\n");

    // Example 1: Stream to Vec<u8> (in-memory)
    println!("💾 Stream to Vec<u8>:");
    let mut buffer = Vec::new();

    cmd!("echo", "hello\nworld")
        .pipe(cmd!("sort", "-r"))
        .stream_to(&mut buffer)?;

    let result = String::from_utf8_lossy(&buffer);
    println!("   Reverse sorted: {}", result.trim());
    println!();

    // Example 2: Stream to File
    println!("📁 Stream to File:");
    let mut output_file = File::create("output.txt")?;

    cmd!("seq", "1", "5").stream_to(&mut output_file)?;

    let content = std::fs::read_to_string("output.txt")?;
    println!("   Numbers 1-5: {}", content.trim().replace('\n', ", "));
    std::fs::remove_file("output.txt").ok();
    println!();

    Ok(())
}

fn run_with_io_examples() -> Result<()> {
    println!("🔄 3. run_with_io() - Connect Reader and Writer (Blocking)");
    println!("==========================================================\n");

    // Example: File to File processing
    println!("📁 File to File processing:");
    std::fs::write("input.txt", "zebra\napple\nbanana\ncherry")?;

    let input_file = File::open("input.txt")?;
    let output_file = File::create("sorted.txt")?;

    cmd!("sort").run_with_io(input_file, output_file)?;

    let sorted = std::fs::read_to_string("sorted.txt")?;
    println!("   Sorted result: {}", sorted.trim().replace('\n', ", "));

    std::fs::remove_file("input.txt").ok();
    std::fs::remove_file("sorted.txt").ok();
    println!();

    Ok(())
}

fn spawn_io_in_examples() -> Result<()> {
    println!("📥 4. spawn_io_in() - Control Stdin Only (Pattern 100)");
    println!("========================================\n");

    println!("⌨️ Interactive input control:");
    let (handle, stdin) = cmd!("wc", "-l").spawn_io_in()?;

    let input_handle = stdin.map(|mut stdin| {
        thread::spawn(move || {
            stdin.write_all(b"line1\nline2\nline3\n").unwrap();
            // stdin is automatically closed when it goes out of scope
        })
    });

    // Wait for input thread to complete first
    if let Some(h) = input_handle {
        h.join().unwrap();
    }

    handle.wait()?;
    println!("   Line counting completed successfully");
    println!();

    Ok(())
}

fn spawn_io_out_examples() -> Result<()> {
    println!("📤 5. spawn_io_out() - Control Stdout Only (Pattern 010)");
    println!("==========================================\n");

    println!("📊 Output capture:");
    let (handle, stdout) = cmd!("seq", "1", "3").spawn_io_out()?;

    let output_handle = stdout.map(|stdout| {
        thread::spawn(move || {
            let mut buffer = Vec::new();
            let mut reader = BufReader::new(stdout);
            reader.read_to_end(&mut buffer).ok();
            String::from_utf8_lossy(&buffer).trim().to_string()
        })
    });

    handle.wait()?;

    if let Some(h) = output_handle {
        let result = h.join().unwrap();
        println!("   Captured output: {}", result.replace('\n', ", "));
    }
    println!();

    Ok(())
}

fn spawn_io_err_examples() -> Result<()> {
    println!("⚠️ 6. spawn_io_err() - Control Stderr Only (Pattern 001)");
    println!("==========================================\n");

    println!("🚨 Error stream capture:");
    let (handle, stderr) = cmd!("sh", "-c", "echo 'normal'; echo 'error' >&2").spawn_io_err()?;

    let error_handle = stderr.map(|stderr| {
        thread::spawn(move || {
            let mut buffer = Vec::new();
            let mut reader = BufReader::new(stderr);
            reader.read_to_end(&mut buffer).ok();
            String::from_utf8_lossy(&buffer).trim().to_string()
        })
    });

    handle.wait()?;

    if let Some(h) = error_handle {
        let error_output = h.join().unwrap();
        println!("   Captured stderr: '{}'", error_output);
    }
    println!();

    Ok(())
}

fn spawn_io_in_out_examples() -> Result<()> {
    println!("🔄 7. spawn_io_in_out() - Stdin + Stdout Control (Pattern 110) ⭐ MOST IMPORTANT!");
    println!("=================================================================================\n");

    println!("🧮 Interactive calculator session:");
    let (handle, stdin, stdout) = cmd!("bc", "-l").spawn_io_in_out()?;

    // Send mathematical expressions
    let input_handle = stdin.map(|mut stdin| {
        thread::spawn(move || {
            stdin.write_all(b"scale=2\n").unwrap();
            stdin.write_all(b"22/7\n").unwrap();
            stdin.write_all(b"sqrt(2)\n").unwrap();
            stdin.write_all(b"quit\n").unwrap();
        })
    });

    // Capture calculation results
    let output_handle = stdout.map(|stdout| {
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            let mut results = Vec::new();
            #[allow(clippy::manual_flatten)]
            for line in reader.lines() {
                if let Ok(line) = line {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() && !trimmed.contains("(standard_in)") {
                        results.push(trimmed.to_string());
                    }
                }
            }
            results
        })
    });

    if let Some(h) = input_handle {
        h.join().unwrap();
    }

    handle.wait()?;

    if let Some(h) = output_handle {
        let results = h.join().unwrap();
        println!(
            "   📊 Results: Pi ≈ {}, √2 ≈ {}",
            results.first().unwrap_or(&"?".to_string()),
            results.get(1).unwrap_or(&"?".to_string())
        );
    }

    println!("\n🔄 Data transformation pipeline:");
    let (handle, stdin, stdout) = cmd!("tr", "a-z", "A-Z").spawn_io_in_out()?;

    let input_handle = stdin.map(|mut stdin| {
        thread::spawn(move || {
            stdin.write_all(b"hello interactive world").unwrap();
        })
    });

    let output_handle = stdout.map(|stdout| {
        thread::spawn(move || {
            let mut buffer = Vec::new();
            let mut reader = BufReader::new(stdout);
            reader.read_to_end(&mut buffer).ok();
            String::from_utf8_lossy(&buffer).trim().to_string()
        })
    });

    if let Some(h) = input_handle {
        h.join().unwrap();
    }

    handle.wait()?;

    if let Some(h) = output_handle {
        let result = h.join().unwrap();
        println!("   🔠 Transformed: {}", result);
    }

    println!("   ✅ Interactive processing - perfect for data transformation!\n");
    Ok(())
}

fn spawn_io_in_err_examples() -> Result<()> {
    println!("🐛 8. spawn_io_in_err() - Stdin + Stderr Control (Pattern 101) ⭐ DEBUG PATTERN!");
    println!("================================================================================\n");

    println!("🛠️ Compilation error monitoring:");
    let (handle, stdin, stderr) = cmd!("rustc", "-").spawn_io_in_err()?;

    // Send invalid Rust code
    let input_handle = stdin.map(|mut stdin| {
        thread::spawn(move || {
            stdin
                .write_all(b"fn main() { let x: i32 = \"invalid\"; }")
                .unwrap();
        })
    });

    // Monitor compilation errors
    let error_handle = stderr.map(|stderr| {
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            let mut error_count = 0;
            #[allow(clippy::manual_flatten)]
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.contains("error") {
                        error_count += 1;
                    }
                }
            }
            error_count
        })
    });

    if let Some(h) = input_handle {
        h.join().unwrap();
    }

    let _ = handle.wait(); // Expected to fail

    if let Some(h) = error_handle {
        let errors = h.join().unwrap();
        println!("   🔍 Compilation errors detected: {}", errors);
    }

    println!("\n🔍 JSON validation with error monitoring:");
    let (handle, stdin, stderr) = cmd!("jq", ".").spawn_io_in_err()?;

    let input_handle = stdin.map(|mut stdin| {
        thread::spawn(move || {
            // Send invalid JSON
            stdin.write_all(b"{\"name\": \"test\", \"age\": }").unwrap();
        })
    });

    let error_handle = stderr.map(|stderr| {
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            let mut has_parse_error = false;
            #[allow(clippy::manual_flatten)]
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.contains("parse error") || line.contains("Invalid") {
                        has_parse_error = true;
                    }
                }
            }
            has_parse_error
        })
    });

    if let Some(h) = input_handle {
        h.join().unwrap();
    }

    let _ = handle.wait(); // Expected to fail due to invalid JSON

    if let Some(h) = error_handle {
        let has_error = h.join().unwrap();
        println!(
            "   ✓ JSON validation error properly captured: {}",
            has_error
        );
    }

    println!("   ✅ Debug pattern - perfect for development tools!\n");
    Ok(())
}

fn spawn_io_out_err_examples() -> Result<()> {
    println!("🔀 9. spawn_io_out_err() - Stdout + Stderr Control (Pattern 011)");
    println!("=================================================================\n");

    println!("📊 Dual stream capture:");
    let (handle, stdout, stderr) =
        cmd!("sh", "-c", "echo 'success'; echo 'warning' >&2").spawn_io_out_err()?;

    let stdout_handle = stdout.map(|stdout| {
        thread::spawn(move || {
            let mut buffer = Vec::new();
            let mut reader = BufReader::new(stdout);
            reader.read_to_end(&mut buffer).ok();
            String::from_utf8_lossy(&buffer).trim().to_string()
        })
    });

    let stderr_handle = stderr.map(|stderr| {
        thread::spawn(move || {
            let mut buffer = Vec::new();
            let mut reader = BufReader::new(stderr);
            reader.read_to_end(&mut buffer).ok();
            String::from_utf8_lossy(&buffer).trim().to_string()
        })
    });

    handle.wait()?;

    if let Some(h) = stdout_handle {
        let stdout_result = h.join().unwrap();
        println!("   Stdout: '{}'", stdout_result);
    }

    if let Some(h) = stderr_handle {
        let stderr_result = h.join().unwrap();
        println!("   Stderr: '{}'", stderr_result);
    }
    println!();

    Ok(())
}

fn spawn_io_all_examples() -> Result<()> {
    println!("🎛️ 10. spawn_io_all() - Complete I/O Control (Pattern 111)");
    println!("===========================================================\n");

    println!("🔧 Manual I/O control:");
    std::fs::write("data.txt", "item1\nitem2\nitem3\nspecial_item\nitem5")?;

    let spawn = cmd!("grep", "item").spawn_io_all()?;

    // Handle input in background thread
    let input_handle = spawn.stdin.map(|mut stdin| {
        thread::spawn(move || {
            let mut input_file = File::open("data.txt").unwrap();
            copy(&mut input_file, &mut stdin).ok();
        })
    });

    // Handle output in background thread
    let output_handle = spawn.stdout.map(|mut stdout| {
        thread::spawn(move || {
            let mut output_file = File::create("filtered.txt").unwrap();
            copy(&mut stdout, &mut output_file).ok();
        })
    });

    println!("   Command running in background...");

    // Wait for completion
    spawn.handle.wait()?;
    if let Some(h) = input_handle {
        h.join().unwrap();
    }
    if let Some(h) = output_handle {
        h.join().unwrap();
    }

    let result = std::fs::read_to_string("filtered.txt")?;
    println!("   Filtered items: {}", result.trim().replace('\n', ", "));

    std::fs::remove_file("data.txt").ok();
    std::fs::remove_file("filtered.txt").ok();
    println!();

    Ok(())
}
