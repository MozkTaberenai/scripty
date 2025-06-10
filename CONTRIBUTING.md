# Contributing to scripty

Thank you for your interest in contributing to scripty! This document provides essential guidelines
for development.

## ⚠️ CRITICAL: Mandatory Pre-Commit Process

**BEFORE EVERY COMMIT, YOU MUST RUN:**

```bash
cargo xtask precommit
```

This command performs the following actions:

1. **Version synchronization** - Updates version in src/lib.rs documentation from Cargo.toml
2. **README generation** - Regenerates README.md from src/lib.rs using `cargo readme`
3. **Example verification** - Runs all examples to ensure they work correctly
4. **Test execution** - Runs all tests
5. **Clippy checks** - Ensures no warnings with strict settings
6. **Format checking** - Verifies code formatting with rustfmt

**ALL checks must pass** before committing. The command will modify files (README.md, src/lib.rs) if
needed.

## Quick Start

1. **Fork and clone** the repository
2. **Install Rust** (latest stable version)
3. **Verify setup**:
   ```bash
   cargo test            # Run tests
   cargo xtask precommit # Pre-commit checks
   ```

## Project Structure

```
scripty/
├── src/
│   ├── lib.rs              # Main library entry point & README source
│   ├── cmd/                # Command execution core
│   │   ├── mod.rs          # Module definitions
│   │   ├── command.rs      # Cmd struct implementation
│   │   ├── pipeline.rs     # Pipeline execution logic
│   │   ├── types.rs        # Type definitions (Cmd, Pipeline, etc.)
│   │   ├── error.rs        # Error types and handling
│   │   ├── macros.rs       # cmd! macro definition
│   │   └── tests/          # Comprehensive test suite
│   │       ├── basic.rs              # Basic command execution tests
│   │       ├── environment.rs        # Environment variable & working directory tests
│   │       ├── error_handling.rs     # Error scenarios and edge cases
│   │       ├── io_patterns.rs        # I/O control patterns and spawn methods
│   │       ├── no_echo.rs            # Echo suppression functionality
│   │       ├── pipeline.rs           # Pipeline operations and pipe modes
│   │       ├── quoting.rs            # Argument quoting for display
│   │       ├── run_output_verification.rs  # Special tests for stdout/stderr inheritance
│   │       └── write_methods.rs      # write_to, write_err_to, write_both_to tests
│   ├── output.rs           # Command echo formatting and control
│   ├── fs.rs               # File system utilities (read_to_string, etc.)
│   ├── io_ext.rs           # I/O extension traits (ReadExt)
│   ├── style.rs            # ANSI color and styling support
│   └── color.rs            # Public color API
├── examples/               # Usage examples demonstrating features
│   ├── 00_hello_world.rs   # Comprehensive introduction to scripty
│   ├── 01_simple_pipes.rs  # Basic piping patterns
│   ├── 02_pipe_modes.rs    # Advanced pipe modes (stdout/stderr/both)
│   ├── 03_read_ext.rs      # ReadExt trait for fluent reader piping
│   ├── 04_run_with_io.rs   # Blocking reader-writer I/O operations
│   ├── 05_spawn_io.rs      # Non-blocking I/O control patterns
│   ├── error_handling.rs   # Error handling best practices
│   └── fs_operations.rs    # File system operations with echo
├── tests/                  # Integration tests
└── xtask/                  # Development automation
    └── src/main.rs         # Tasks: precommit, ci, readme generation
```

## MANDATORY: Branch-Based Development Workflow

### Absolute Rules (ZERO TOLERANCE)

- **NEVER WORK DIRECTLY ON MAIN** - All changes must go through feature branches
- **NEVER COMMIT** with ANY clippy warnings or test failures
- **NEVER PUSH TO MAIN** - Always use feature branches and Pull Requests
- **ALWAYS** commit formatting changes separately before work commits

### Starting New Work

1. **Update main branch first:**
   ```bash
   git checkout main
   git pull origin main
   ```

2. **Create feature branch:**
   ```bash
   # Use descriptive branch names with prefixes
   git checkout -b feature/your-feature-name
   git checkout -b fix/bug-description
   git checkout -b docs/documentation-update
   git checkout -b refactor/code-improvement
   git checkout -b test/add-test-coverage
   git checkout -b chore/dependency-update
   ```

### Branch Naming Convention

- `feature/` - New functionality (e.g., `feature/stderr-piping`)
- `fix/` - Bug fixes (e.g., `fix/clippy-warnings`)
- `docs/` - Documentation updates (e.g., `docs/update-readme`)
- `refactor/` - Code refactoring (e.g., `refactor/pipeline-structure`)
- `test/` - Test additions/fixes (e.g., `test/pipe-mode-coverage`)
- `chore/` - Build process, dependencies (e.g., `chore/update-deps`)

## Documentation Management

### Important: README.md is Generated

**⚠️ DO NOT edit README.md directly!**

- README.md is automatically generated from `src/lib.rs` docstrings
- To update README.md: edit `src/lib.rs` and run `cargo readme`

## Development Commands

```bash
# Essential commands
cargo test                                              # Run tests
cargo clippy --all-targets --all-features -- -D warnings  # Lint code
cargo fmt                                               # Format code

# Project-specific xtask commands
cargo readme          # Generate README.md from src/lib.rs
cargo xtask precommit # Version sync + README gen + examples + test + clippy + fmt
cargo xtask ci        # Full CI pipeline (same as precommit)
```

## Code Quality (MANDATORY)

### Pre-Commit Process (CRITICAL)

**MANDATORY before EVERY commit:**

```bash
# Step 1: Ensure tests pass
cargo test

# Step 2: Run all pre-commit checks (MANDATORY)
cargo xtask precommit  # Syncs version, generates README, runs examples, tests, clippy, fmt

# Step 3: CRITICAL - Handle formatting changes
git status  # Check for changes made by rustfmt
# If any files are modified by formatting, MUST commit them:
git add .
git commit -m "fix: apply rustfmt formatting changes"

# Step 4: Verify formatting is clean
cargo fmt -- --check  # MUST show no errors before proceeding

# Step 5: Update README if needed
cargo readme    # If src/lib.rs docs were changed

# Step 6: Commit your actual changes
git add .
git commit -m "feat: descriptive commit message"
git push origin feature/branch-name
```

### Code Quality Requirements

- Fix ALL clippy warnings before committing
- Use `cargo fmt` for consistent formatting
- All tests must pass

## Pull Request Workflow

### Before Submitting a PR

1. **During development** (run frequently):
   ```bash
   cargo xtask precommit  # Runs test + clippy + fmt
   ```

2. **Before final commit**:
   ```bash
   cargo xtask ci  # Same as precommit, ensures everything is ready
   ```

3. **Important**: If `cargo fmt` makes changes, commit them separately:
   ```bash
   git status              # Check for formatting changes
   git add . && git commit -m "fix: apply rustfmt formatting"
   ```

4. **Commit both source and generated files**

### Pull Request Process

1. **Create PR** via GitHub CLI or web interface
2. **Use conventional commit format** (feat:, fix:, docs:, etc.)
3. **Include description** of changes and testing instructions
4. **Ensure all checks pass** before requesting review
5. **Use "Squash and merge"** to maintain clean history

## Platform Support

- **Primary platforms**: Linux and macOS
- **CI**: Only runs on Linux and macOS

When adding examples, use commands available on Unix systems.

## Code Standards

- Follow Rust conventions and use `cargo fmt`
- Document public APIs with examples
- Write tests for new functionality
- Examples must work on Unix systems (Linux/macOS)

## Commit Guidelines

- **BRANCH FIRST**: Never commit directly to main - always use feature branches
- Use conventional commit format (feat:, fix:, docs:, refactor:, test:, chore:)
- Test locally with full workflow before committing
- Update documentation in `src/lib.rs` when adding features
- Separate logical changes into different commits
- **MANDATORY**: Run clippy checks before EVERY commit
- **PULL REQUEST REQUIRED**: All changes must go through PR review process

## Getting Help

- Review existing code and examples for patterns
- Ask questions in GitHub issues or discussions
- Check CHANGELOG.md for recent changes

---

Be respectful and constructive in all interactions.
