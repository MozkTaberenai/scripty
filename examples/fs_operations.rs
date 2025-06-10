//! # File System Operations: Working with Files and Directories
//!
//! This example demonstrates scripty's file system operations with automatic logging:
//! - Creating and removing files/directories
//! - Reading and writing files
//! - Copying and renaming
//! - Working with metadata
//! - Directory traversal
//! - Permissions (Unix-specific)
//!
//! All operations automatically echo what they're doing for visibility.
//!
//! Estimated time: ~5 minutes
//! Prerequisites: Basic file system knowledge
//! Related examples: 00_hello_world.rs

use scripty::*;
use std::path::Path;

fn main() -> Result<()> {
    println!("📁 File System Operations with scripty");
    println!("=====================================\n");

    // Create a temporary workspace
    let workspace = std::env::temp_dir().join("scripty_fs_demo");

    // Ensure workspace exists
    fs::create_dir_all(&workspace)?;
    println!("Created workspace directory");
    println!();

    // 1. Directory operations
    println!("1️⃣ Directory operations:");
    directory_operations(&workspace)?;
    println!();

    // 2. File operations
    println!("2️⃣ File operations:");
    file_operations(&workspace)?;
    println!();

    // 3. Copy and rename
    println!("3️⃣ Copy and rename operations:");
    copy_rename_operations(&workspace)?;
    println!();

    // 4. Reading files
    println!("4️⃣ Reading files:");
    reading_operations(&workspace)?;
    println!();

    // 5. Directory traversal
    println!("5️⃣ Directory traversal:");
    directory_traversal(&workspace)?;
    println!();

    // 6. Metadata operations
    println!("6️⃣ Working with metadata:");
    metadata_operations(&workspace)?;
    println!();

    // 7. Advanced operations (Unix-specific)
    #[cfg(unix)]
    {
        println!("7️⃣ Advanced operations (Unix):");
        advanced_operations(&workspace)?;
        println!();
    }

    // Cleanup
    println!("8️⃣ Cleanup:");
    cleanup(&workspace)?;

    println!("\n🎉 File system operations completed!");
    println!("💡 Tip: All operations are automatically logged with colorful output");
    println!("💡 Set NO_ECHO=1 to disable operation logging");

    Ok(())
}

fn directory_operations(workspace: &Path) -> Result<()> {
    // Create single directory
    let simple_dir = workspace.join("simple_dir");
    fs::create_dir(&simple_dir)?;
    println!("   ✅ Created single directory");

    // Create nested directories
    let nested_dir = workspace.join("parent/child/grandchild");
    fs::create_dir_all(&nested_dir)?;
    println!("   ✅ Created nested directories");

    // Try to create existing directory (will fail)
    match fs::create_dir(&simple_dir) {
        Ok(_) => println!("   Unexpected: created existing directory"),
        Err(e) => println!("   ✅ Expected error for existing directory: {}", e),
    }

    Ok(())
}

fn file_operations(workspace: &Path) -> Result<()> {
    let file_path = workspace.join("test_file.txt");

    // Write text to file
    fs::write(
        &file_path,
        "Hello from scripty!\nFile operations are easy.\n",
    )?;
    println!("   ✅ Written text to file");

    // Write binary data
    let binary_file = workspace.join("binary_data.bin");
    let binary_data: Vec<u8> = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]; // "Hello" in hex
    fs::write(&binary_file, &binary_data)?;
    println!("   ✅ Written binary data");

    // Append to file (using write with read-modify-write pattern)
    let existing = fs::read_to_string(&file_path)?;
    fs::write(&file_path, format!("{}\nAppended line!", existing))?;
    println!("   ✅ Appended to file");

    Ok(())
}

fn copy_rename_operations(workspace: &Path) -> Result<()> {
    let source = workspace.join("source.txt");
    let destination = workspace.join("destination.txt");
    let renamed = workspace.join("renamed.txt");

    // Create source file
    fs::write(&source, "Content to be copied")?;

    // Copy file
    let bytes_copied = fs::copy(&source, &destination)?;
    println!("   ✅ Copied {} bytes", bytes_copied);

    // Rename file
    fs::rename(&destination, &renamed)?;
    println!("   ✅ Renamed file");

    // Move file to subdirectory
    let subdir = workspace.join("simple_dir");
    let moved = subdir.join("moved.txt");
    fs::rename(&renamed, &moved)?;
    println!("   ✅ Moved file to subdirectory");

    Ok(())
}

fn reading_operations(workspace: &Path) -> Result<()> {
    let text_file = workspace.join("test_file.txt");
    let binary_file = workspace.join("binary_data.bin");

    // Read as string
    let content = fs::read_to_string(&text_file)?;
    println!("   ✅ Read {} characters from text file", content.len());
    println!("   First line: {}", content.lines().next().unwrap_or(""));

    // Read as bytes
    let bytes = fs::read(&binary_file)?;
    println!("   ✅ Read {} bytes from binary file", bytes.len());
    println!("   First 5 bytes: {:?}", &bytes[..5.min(bytes.len())]);

    // Read non-existent file
    match fs::read_to_string(workspace.join("nonexistent.txt")) {
        Ok(_) => println!("   Unexpected: read non-existent file"),
        Err(e) => println!("   ✅ Expected error for missing file: {}", e),
    }

    Ok(())
}

fn directory_traversal(workspace: &Path) -> Result<()> {
    // Create some files for listing
    let list_dir = workspace.join("listing_demo");
    fs::create_dir(&list_dir)?;
    fs::write(list_dir.join("file1.txt"), "content1")?;
    fs::write(list_dir.join("file2.txt"), "content2")?;
    fs::create_dir(list_dir.join("subdir"))?;

    // List directory contents
    let entries = fs::read_dir(&list_dir)?;
    let mut count = 0;

    println!("   Directory contents:");
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let file_type = if path.is_dir() { "DIR " } else { "FILE" };
        println!(
            "     {} {}",
            file_type,
            path.file_name().unwrap().to_string_lossy()
        );
        count += 1;
    }
    println!("   ✅ Found {} entries", count);

    Ok(())
}

fn metadata_operations(workspace: &Path) -> Result<()> {
    let file_path = workspace.join("test_file.txt");

    // Get file metadata
    let metadata = fs::metadata(&file_path)?;
    println!("   File size: {} bytes", metadata.len());
    println!("   Is file: {}", metadata.is_file());
    println!("   Is directory: {}", metadata.is_dir());

    // Get symlink metadata (works on regular files too)
    let _symlink_meta = fs::symlink_metadata(&file_path)?;
    println!("   ✅ Retrieved symlink metadata");

    // Directory metadata
    let dir_meta = fs::metadata(workspace)?;
    println!("   Directory metadata - is_dir: {}", dir_meta.is_dir());

    Ok(())
}

#[cfg(unix)]
fn advanced_operations(workspace: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let file_path = workspace.join("permissions_test.txt");
    fs::write(&file_path, "Test file for permissions")?;

    // Set permissions (Unix only)
    let metadata = fs::metadata(&file_path)?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o644); // rw-r--r--
    fs::set_permissions(&file_path, permissions)?;
    println!("   ✅ Set file permissions to 0644");

    // Try hard link (may fail on some filesystems)
    let hard_link_path = workspace.join("hard_link.txt");
    match fs::hard_link(&file_path, &hard_link_path) {
        Ok(_) => {
            println!("   ✅ Created hard link");
            // Verify both files have same content
            let original = fs::read_to_string(&file_path)?;
            let linked = fs::read_to_string(&hard_link_path)?;
            assert_eq!(original, linked);
            println!("   ✅ Verified hard link content matches");
        }
        Err(e) => {
            println!("   ⚠️ Hard link not supported: {}", e);
        }
    }

    Ok(())
}

fn cleanup(workspace: &Path) -> Result<()> {
    // Remove individual files
    if workspace.join("source.txt").exists() {
        fs::remove_file(workspace.join("source.txt"))?;
        println!("   ✅ Removed individual file");
    }

    // Remove empty directory
    let empty_dir = workspace.join("empty_to_remove");
    fs::create_dir(&empty_dir)?;
    fs::remove_dir(&empty_dir)?;
    println!("   ✅ Removed empty directory");

    // Remove directory with contents
    fs::remove_dir_all(workspace)?;
    println!("   ✅ Removed entire workspace directory");

    Ok(())
}

// Type alias for convenience
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
