//! Input/Output processing tests.
//!
//! Tests for scripty-specific I/O functionality including readers, writers,
//! and binary data handling methods.

use crate::cmd;

/// Tests binary input/output methods
#[test]
fn test_binary_input_output_methods() {
    // Test with pure binary data (including null bytes and non-UTF8 sequences)
    let binary_data: Vec<u8> = vec![
        0x00, 0x01, 0x02, 0x03, 0xFF, 0xFE, 0xFD, 0xFC, 0x80, 0x81, 0x82,
        0x83, // Invalid UTF-8 sequences
        b'H', b'e', b'l', b'l', b'o', // Valid ASCII
    ];

    // Test input_bytes and output_bytes
    let output_bytes = cmd!("cat")
        .input_bytes(&binary_data)
        .no_echo()
        .output_bytes()
        .unwrap();

    assert_eq!(output_bytes, binary_data);

    // Test that regular output() method handles binary data with lossy conversion
    let output_string = cmd!("cat")
        .input_bytes(&binary_data)
        .no_echo()
        .output()
        .unwrap();

    // Should contain the "Hello" part at minimum
    assert!(output_string.contains("Hello"));
    // Should handle invalid UTF-8 gracefully (with replacement characters)
    assert!(!output_string.is_empty());
}

/// Tests mixed text and binary API usage
#[test]
fn test_mixed_text_binary_api() {
    let text_data = "Hello, World!";

    // Test input() with output_bytes()
    let bytes_output = cmd!("cat")
        .input(text_data)
        .no_echo()
        .output_bytes()
        .unwrap();

    assert_eq!(bytes_output, text_data.as_bytes());

    // Test input_bytes() with output()
    let string_output = cmd!("cat")
        .input_bytes(text_data.as_bytes())
        .no_echo()
        .output()
        .unwrap();

    assert_eq!(string_output.trim(), text_data);
}

/// Tests zero-copy optimization with input_bytes_owned
#[test]
fn test_zero_copy_input_bytes_owned() {
    // Test Cmd::input_bytes_owned
    let binary_data = vec![
        b'H', b'e', b'l', b'l', b'o', b' ', b'W', b'o', b'r', b'l', b'd',
    ];
    let output = cmd!("cat")
        .input_bytes_owned(binary_data) // Takes ownership, should not copy
        .no_echo()
        .output()
        .unwrap();

    assert_eq!(output.trim(), "Hello World");

    // Test Pipeline::input_bytes_owned
    let large_data = vec![b'X'; 1024]; // 1KB of 'X' characters
    let result = cmd!("wc", "-c")
        .input_bytes_owned(large_data) // Zero-copy ownership transfer
        .no_echo()
        .output()
        .unwrap();

    let count: usize = result.trim().parse().unwrap();
    assert_eq!(count, 1024);
}

/// Tests performance comparison between regular and owned input methods
#[test]
fn test_input_method_equivalence() {
    let test_data = b"Performance test data with various bytes: \x00\x01\x02\xFF";

    // Test that both methods produce identical results
    let output1 = cmd!("cat")
        .input_bytes(test_data)
        .no_echo()
        .output_bytes()
        .unwrap();

    let output2 = cmd!("cat")
        .input_bytes_owned(test_data.to_vec())
        .no_echo()
        .output_bytes()
        .unwrap();

    assert_eq!(output1, output2);
    assert_eq!(output1, test_data);
}

/// Tests Reader-based input methods
#[test]
fn test_input_reader() {
    use std::io::Cursor;

    // Test with Cursor (in-memory reader)
    let test_data = "Hello from cursor";
    let cursor = Cursor::new(test_data);

    let output = cmd!("cat").input_reader(cursor).no_echo().output().unwrap();

    assert_eq!(output.trim(), test_data);

    // Test with byte data through reader
    let binary_data = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]; // "Hello"
    let cursor = Cursor::new(binary_data.clone());

    let output_bytes = cmd!("cat")
        .input_reader(cursor)
        .no_echo()
        .output_bytes()
        .unwrap();

    assert_eq!(output_bytes, binary_data);
}

/// Tests buffered Reader input
#[test]
fn test_input_buffered() {
    use std::io::{BufReader, Cursor};

    let test_data = "Line 1\nLine 2\nLine 3\n";
    let cursor = Cursor::new(test_data);
    let buffered_cursor = BufReader::new(cursor);

    let output = cmd!("wc", "-l")
        .input_reader(buffered_cursor)
        .output()
        .expect("Failed to execute command");

    let count = output.trim().parse::<i32>().expect("Failed to parse count");
    assert_eq!(count, 3);
}

/// Tests Writer-based output streaming
#[test]
fn test_stream_to_writer() {
    // Test streaming to Vec<u8> (implements Write)
    let mut output_buffer = Vec::new();

    cmd!("echo", "-n", "Hello, Writer!")
        .no_echo()
        .stream_to(&mut output_buffer)
        .unwrap();

    let result = String::from_utf8(output_buffer).unwrap();
    assert_eq!(result, "Hello, Writer!");

    // Test streaming binary data
    let binary_input = vec![0x01, 0x02, 0x03, 0xFF];
    let mut binary_output = Vec::new();

    cmd!("cat")
        .input_bytes(&binary_input)
        .no_echo()
        .stream_to(&mut binary_output)
        .unwrap();

    assert_eq!(binary_output, binary_input);
}

/// Tests combined Reader + Writer usage
#[test]
fn test_run_with_io() {
    use std::io::Cursor;

    let input_data = "apple\nbanana\ncherry\napricot\n";
    let input_reader = Cursor::new(input_data);
    let mut output_buffer = Vec::new();

    cmd!("grep", "ap")
        .no_echo()
        .run_with_io(input_reader, &mut output_buffer)
        .unwrap();

    let result = String::from_utf8(output_buffer).unwrap();
    let lines: Vec<&str> = result.trim().split('\n').collect();
    assert_eq!(lines.len(), 2);
    assert!(lines.contains(&"apple"));
    assert!(lines.contains(&"apricot"));
}

/// Tests Pipeline with Reader/Writer using new spawn API
#[test]
fn test_pipeline_with_io() {
    use std::io::Cursor;
    use std::thread;

    // Test pipeline with reader input using spawn API
    let input_data = "zebra\napple\nbanana\ncherry\n";
    let cursor = Cursor::new(input_data);

    let spawn = cmd!("sort")
        .pipe(cmd!("head", "-2"))
        .no_echo()
        .spawn_with_io()
        .unwrap();

    // Handle input in separate thread
    if let Some(mut stdin) = spawn.stdin {
        let mut reader = cursor;
        thread::spawn(move || {
            use std::io::copy;
            let _ = copy(&mut reader, &mut stdin);
        });
    }

    // Collect output
    let output = if let Some(stdout) = spawn.stdout {
        let mut result = Vec::new();
        use std::io::Read;
        let mut reader = std::io::BufReader::new(stdout);
        reader.read_to_end(&mut result).unwrap();
        String::from_utf8(result).unwrap()
    } else {
        String::new()
    };

    spawn.handle.wait().unwrap();

    let lines: Vec<&str> = output.trim().split('\n').collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "apple");
    assert_eq!(lines[1], "banana");

    // Test pipeline stream_to using run_with_io (which now uses spawn internally)
    let input_data2 = "third\nfirst\nsecond\n";
    let cursor2 = Cursor::new(input_data2);
    let mut output_buffer = Vec::new();

    cmd!("sort")
        .pipe(cmd!("head", "-1"))
        .no_echo()
        .run_with_io(cursor2, &mut output_buffer)
        .unwrap();

    let result = String::from_utf8(output_buffer).unwrap();
    assert_eq!(result.trim(), "first");
}

/// Tests error handling in Reader/Writer operations
#[test]
fn test_reader_writer_error_handling() {
    use std::io::Cursor;

    // Test with command that fails
    let cursor = Cursor::new("test data");
    let result = cmd!("nonexistent-command-67890")
        .input_reader(cursor)
        .no_echo()
        .run();

    assert!(result.is_err());

    // Test pipeline error handling
    let cursor2 = Cursor::new("more test data");
    let result2 = cmd!("cat")
        .pipe(cmd!("nonexistent-command-67890"))
        .input_reader(cursor2)
        .no_echo()
        .run();

    assert!(result2.is_err());
}
