use clap::{Parser, Subcommand};
use scripty::*;
use std::collections::HashMap;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Development task runner for scripty
#[derive(Parser)]
#[command(
    name = "xtask",
    about = "Development task runner for scripty",
    long_about = "⚠️  IMPORTANT: README.md is auto-generated from src/lib.rs\n   To update README.md: edit src/lib.rs and run 'cargo xtask readme'\n\nBefore committing: cargo xtask ci",
    version
)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Suppress output (overrides verbose)
    #[arg(short, long, global = true)]
    quiet: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate README.md from lib.rs documentation
    Readme {
        /// Force regeneration even if README.md is newer than lib.rs
        #[arg(short, long)]
        force: bool,
    },
    /// Run pre-commit checks (test + clippy + fmt)
    Precommit,
    /// Run all CI tasks
    Ci,
}

fn get_project_root() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    let current_name = current_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    if current_name == "xtask" {
        Ok(current_dir.parent().unwrap().to_path_buf())
    } else {
        Ok(current_dir)
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set global verbosity
    let verbose = cli.verbose && !cli.quiet;
    let _quiet = cli.quiet;

    match cli.command {
        Commands::Readme { force } => generate_readme(force)?,
        Commands::Precommit => run_precommit(verbose)?,
        Commands::Ci => run_ci(verbose)?,
    }

    Ok(())
}

fn generate_readme(force: bool) -> Result<()> {
    generate_readme_with_options(force)
}

fn run_precommit(verbose: bool) -> Result<()> {
    if !verbose {
        println!("🔍 Running pre-commit checks...");
    }
    let project_root = get_project_root()?;

    // Generate README first to ensure documentation is up to date
    if !verbose {
        println!("📝 Generating README.md...");
    }
    generate_readme(false)?;
    if !verbose {
        println!("✅ README.md updated!");
    }

    // Run tests
    if !verbose {
        println!("🧪 Running tests...");
    }
    cmd!("cargo", "test").current_dir(&project_root).run()?;
    if !verbose {
        println!("✅ Tests passed!");
    }

    // Run comprehensive clippy
    if !verbose {
        println!("📎 Running comprehensive clippy checks...");
    }
    cmd!(
        "cargo",
        "clippy",
        "--all-targets",
        "--all-features",
        "--",
        "-D",
        "warnings"
    )
    .current_dir(&project_root)
    .run()?;
    if !verbose {
        println!("✅ Clippy checks passed!");
    }

    // Format code
    if !verbose {
        println!("🎨 Formatting code...");
    }
    cmd!("cargo", "fmt").current_dir(&project_root).run()?;
    if !verbose {
        println!("✅ Code formatted!");
    }

    if !verbose {
        println!("🎉 Pre-commit checks completed successfully!");
        println!("✅ Ready to commit!");
    }

    Ok(())
}

fn run_ci(verbose: bool) -> Result<()> {
    if !verbose {
        println!("🚀 Running full CI pipeline...");
    }
    let project_root = get_project_root()?;

    // Format code first
    if !verbose {
        println!("🎨 Formatting code...");
    }
    cmd!("cargo", "fmt").current_dir(&project_root).run()?;
    if !verbose {
        println!("✅ Code formatted!");
    }

    // Run static analysis
    if !verbose {
        println!("📎 Running clippy lints...");
    }
    cmd!(
        "cargo",
        "clippy",
        "--all-targets",
        "--all-features",
        "--",
        "-D",
        "warnings"
    )
    .current_dir(&project_root)
    .run()?;
    if !verbose {
        println!("✅ Clippy checks passed!");
    }

    // Check compilation
    if !verbose {
        println!("🔍 Running cargo check...");
    }
    cmd!("cargo", "check", "--all-targets")
        .current_dir(&project_root)
        .run()?;
    if !verbose {
        println!("✅ Check passed!");
    }

    // Run tests
    if !verbose {
        println!("🧪 Running tests...");
    }
    cmd!("cargo", "test").current_dir(&project_root).run()?;
    if !verbose {
        println!("✅ Tests passed!");
    }

    // Generate documentation
    generate_readme(false)?;

    if !verbose {
        println!("🎉 All CI tasks completed successfully!");
        println!("🔍 Summary:");
        println!("  ✅ Code formatting");
        println!("  ✅ Clippy lints");
        println!("  ✅ Compilation check");
        println!("  ✅ Test suite");
        println!("  ✅ README generation");
    }

    Ok(())
}

pub fn generate_readme_with_options(force: bool) -> Result<()> {
    println!("🔧 Generating README.md...");

    let project_root = get_project_root()?;
    let lib_rs_path = project_root.join("src/lib.rs");
    let readme_path = project_root.join("README.md");

    // Check if regeneration is needed (unless forced)
    if !force && readme_path.exists() {
        if let (Ok(lib_meta), Ok(readme_meta)) = (
            std::fs::metadata(&lib_rs_path),
            std::fs::metadata(&readme_path),
        ) {
            if let (Ok(lib_modified), Ok(readme_modified)) =
                (lib_meta.modified(), readme_meta.modified())
            {
                if readme_modified > lib_modified {
                    println!("✅ README.md is up to date (use --force to regenerate anyway)");
                    return Ok(());
                }
            }
        }
    }

    // Read the lib.rs file to extract documentation and examples
    let _lib_content = fs::read_to_string(&lib_rs_path)?;

    // Extract examples from the examples directory
    let examples = extract_examples(&project_root)?;

    // Generate README using cargo-readme as base
    let base_readme = cmd!("cargo", "readme", "--no-title", "--no-badges")
        .current_dir(&project_root)
        .output()?;

    // Create enhanced README content
    let readme_content = build_enhanced_readme(&base_readme, &examples)?;

    // Write to README.md
    fs::write(&readme_path, &readme_content)?;

    println!("✅ README.md generated successfully!");
    println!("📊 Generated with {} examples", examples.len());

    Ok(())
}

fn extract_examples(project_root: &Path) -> Result<HashMap<String, String>> {
    use std::fs;

    let mut examples = HashMap::new();

    // Check if examples directory exists
    let examples_dir = project_root.join("examples");
    if !examples_dir.exists() {
        return Ok(examples);
    }

    // Read all example files
    if let Ok(entries) = fs::read_dir(&examples_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Ok(content) = fs::read_to_string(&path) {
                    let filename = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown");

                    // Extract the main content, skipping use statements and comments
                    let cleaned_content = clean_example_content(&content);
                    examples.insert(filename.to_string(), cleaned_content);
                }
            }
        }
    }

    Ok(examples)
}

fn clean_example_content(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut in_main = false;
    let mut brace_count = 0;
    let mut found_main = false;

    for line in lines {
        let trimmed = line.trim();

        // Skip initial comments and use statements before main
        if !found_main && (trimmed.starts_with("//") || trimmed.starts_with("use ")) {
            continue;
        }

        // Start capturing from main function
        if trimmed.starts_with("fn main()") {
            found_main = true;
            in_main = true;
            // Count the opening brace on this line or next
            brace_count += line.chars().filter(|&c| c == '{').count() as i32;
            if brace_count == 0 {
                continue; // Will get opening brace on next line
            }
            continue;
        }

        if in_main {
            // Count braces to know when main function ends
            brace_count += line.chars().filter(|&c| c == '{').count() as i32;
            brace_count -= line.chars().filter(|&c| c == '}').count() as i32;

            // Skip the opening brace line if it's standalone
            if brace_count >= 1 && trimmed == "{" {
                continue;
            }

            // Stop at the closing brace of main
            if brace_count == 0 && trimmed == "}" {
                break;
            }

            // Remove one level of indentation and add to result
            if let Some(cleaned_line) = line.strip_prefix("    ") {
                if !cleaned_line.trim().is_empty() {
                    result.push(cleaned_line);
                }
            } else if !line.trim().is_empty() {
                result.push(line);
            }
        }
    }

    result.join("\n")
}

fn build_enhanced_readme(base_content: &str, examples: &HashMap<String, String>) -> Result<String> {
    let header = r#"# scripty

[![Crates.io](https://img.shields.io/crates/v/scripty.svg)](https://crates.io/crates/scripty)
[![Documentation](https://docs.rs/scripty/badge.svg)](https://docs.rs/scripty)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/MozkTaberenai/scripty/workflows/CI/badge.svg)](https://github.com/MozkTaberenai/scripty/actions)

"#;

    // Start with header and base content
    let mut content = format!("{}{}", header, base_content);

    // Add examples section if we have examples
    if !examples.is_empty() {
        content.push_str("\n\n## Examples\n\n");
        content.push_str("The following examples are available in the `examples/` directory:\n\n");

        for (name, example_content) in examples {
            content.push_str(&format!("### {}\n\n", name));
            content.push_str(&format!("```rust\n{}\n```\n\n", example_content));
            content.push_str(&format!("Run with: `cargo run --example {}`\n\n", name));
        }
    }

    // Add development section
    content.push_str(
        r#"
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

"#,
    );

    Ok(content)
}
