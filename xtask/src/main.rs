use clap::{Parser, Subcommand};
use scripty::*;
use std::path::PathBuf;

mod readme;

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
    readme::generate_readme_with_options(force)
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
