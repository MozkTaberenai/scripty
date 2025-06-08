# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Nothing yet

## [0.2.0] - 2025-01-01

### Added
- Complete I/O pattern analysis and mathematical coverage (2³ = 8 patterns)
- New standardized `spawn_io_*` methods for all I/O control scenarios:
  - `spawn_io_in()` - stdin only (Pattern 100)
  - `spawn_io_out()` - stdout only (Pattern 010)  
  - `spawn_io_err()` - stderr only (Pattern 001)
  - `spawn_io_in_out()` - stdin + stdout (Pattern 110) - Interactive processing
  - `spawn_io_in_err()` - stdin + stderr (Pattern 101) - Debug scenarios
  - `spawn_io_out_err()` - stdout + stderr (Pattern 011)
  - `spawn_io_all()` - complete control (Pattern 111)

### Changed
- Standardized pipe operation naming for consistency:
  - Simplified `pipe_stderr()` → `pipe_err()` 
  - Simplified `pipe_both()` → `pipe_out_err()`
- Updated example `03_io_patterns.rs` to demonstrate all 8 I/O patterns with practical use cases
- Renamed test file from `input_output.rs` to `io_patterns.rs` for clarity
- Updated test function names to match new API naming conventions
- Removed version-specific references from documentation (Rust 1.87.0+ notes) since it's now the MSRV

### Fixed
- Resolved all clippy warnings with appropriate safety considerations
- Improved code quality and consistency across examples and tests

### Deprecated
- `pipe_stderr()` method (use `pipe_err()` instead)
- `pipe_both()` method (use `pipe_out_err()` instead)

## [0.1.0] - 2025-06-08

### Added
- Initial public release of scripty
- `cmd!` macro for intuitive command execution
- Command piping with `.pipe()`, `.pipe_stderr()`, and `.pipe_both()` methods
- Builder pattern API for flexible command configuration
- Environment variable setting with `.env()` method
- Working directory changes with `.cwd()` method
- Input/output handling with `.input()` and `.output()` methods
- Quiet mode with `.quiet()` method and `NO_ECHO` environment variable
- File system operations module (`fs`) with automatic logging
- Colorful command echoing using `anstyle` for better visibility
- Comprehensive error handling with Rust's `Result` type
- Cross-platform support for Unix-like systems (Linux, macOS)
- Zero runtime dependencies except `anstyle` for colors
- Command argument quoting for improved readability
- Native pipeline performance using `std::io::pipe` (Rust 1.87.0+)
- Extensive example collection organized by difficulty level:
  - Basic examples (hello_world, simple_commands, simple_pipes, simple_fs)
  - Intermediate examples (environment, error_handling, pipe_modes, mixed_pipe_modes, reader_writer_demo)
  - Advanced examples (complex_pipes, command_quoting, control_char_demo, streaming_io)
- Comprehensive test suite with 118+ unit tests and integration tests
- Complete documentation with usage examples and tutorials
