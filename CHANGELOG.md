# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Nothing yet

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
