//! # 03 - I/O Patterns: Complete I/O Methods Reference
//!
//! This example demonstrates all I/O methods available in scripty:
//! - input_reader() - Use Reader as input
//! - stream_to() - Stream output to Writer
//! - run_with_io() - Connect Reader and Writer (blocking)
//! - spawn_with_io() - Full I/O control (non-blocking)
//! - spawn_with_stdin() - Control stdin only
//! - spawn_with_stdout() - Control stdout only
//! - spawn_with_stderr() - Control stderr only
//! - spawn_with_both() - Control stdout and stderr
//!
//! Estimated time: ~5 minutes
//! Prerequisites: Complete 02_pipe_modes.rs
//! Final example: All I/O methods mastered!

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

fn spawn_with_io_examples() -> Result<()> {
    println!("🧵 4. spawn_with_io() - Full I/O Control (Non-blocking)");
    println!("========================================================\n");

    println!("🔧 Manual I/O control:");
    std::fs::write("data.txt", "item1\nitem2\nitem3\nspecial_item\nitem5")?;

    let spawn = cmd!("grep", "item").spawn_with_io()?;

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

fn spawn_with_stdin_examples() -> Result<()> {
    println!("📥 5. spawn_with_stdin() - Control Stdin Only");
    println!("=============================================\n");

    println!("⌨️ Interactive input control:");
    let (handle, stdin) = cmd!("wc", "-l").spawn_with_stdin()?;

    if let Some(mut stdin) = stdin {
        thread::spawn(move || {
            stdin.write_all(b"line1\nline2\nline3\n").ok();
        });
    }

    handle.wait()?;
    println!("   Line counting completed successfully");
    println!();

    Ok(())
}

fn spawn_with_stdout_examples() -> Result<()> {
    println!("📤 6. spawn_with_stdout() - Control Stdout Only");
    println!("===============================================\n");

    println!("📊 Output capture:");
    let (handle, stdout) = cmd!("seq", "1", "3").spawn_with_stdout()?;

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

fn spawn_with_stderr_examples() -> Result<()> {
    println!("⚠️ 7. spawn_with_stderr() - Control Stderr Only");
    println!("===============================================\n");

    println!("🚨 Error stream capture:");
    let (handle, stderr) =
        cmd!("sh", "-c", "echo 'normal'; echo 'error' >&2").spawn_with_stderr()?;

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

fn spawn_with_both_examples() -> Result<()> {
    println!("🔀 8. spawn_with_both() - Control Stdout and Stderr");
    println!("===================================================\n");

    println!("📊 Dual stream capture:");
    let (handle, stdout, stderr) =
        cmd!("sh", "-c", "echo 'success'; echo 'warning' >&2").spawn_with_both()?;

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
