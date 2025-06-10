# AI Agent Instructions

## 🔴 CRITICAL: Confirmation Principle

**ALWAYS explain your implementation plan first and get confirmation before making any changes.**

- Present a clear overview of what you intend to do
- List the files that will be affected
- Describe the approach you will take
- Wait for user approval before proceeding with implementation

## 🔴 CRITICAL: Implementation Principle

**Keep implementations minimal and focused on the specific request.**

- Avoid excessive implementation beyond what is necessary
- Limit changes to what is sufficient to solve the problem
- Do not add features or improvements not explicitly requested

## Testing Principles

- **Test Coverage**: When fixing bugs, add tests that would have caught the issue
- **Test Documentation**: Document unusual test approaches within the test file itself
- **Regression Prevention**: For bugs related to output behavior, consider tests that verify actual
  stdout/stderr output, not just captured strings

## Project Information

### Overview

See `Cargo.toml` for official project metadata (name, description, version, etc.)

### Technology Stack

- **Language**: Rust 2024 Edition
- **Target Platform**: Unix-like systems only
- **Dependencies**: Minimal (see Cargo.toml)
- **Testing**: Unit tests + integration tests + doc tests

### Project Structure and Documentation

- See `CONTRIBUTING.md` for detailed project structure and development guidelines
- **AI Responsibility**: When making changes that affect project structure, file organization, or
  development procedures, proactively update `CONTRIBUTING.md` to reflect these changes
- **Documentation Maintenance**: Always verify that documentation matches current project state
  before completing tasks

### Documentation Update Triggers

AI agents should update `CONTRIBUTING.md` when:

- Adding, removing, or moving **source modules or directories** (not individual test files)
- Changing development workflows or build processes
- Modifying testing strategies or CI/CD procedures
- Adding new development tools or dependencies
- Restructuring the codebase organization
- Adding new **categories** of files (e.g., a new test directory, new module type)

### Documentation Best Practices

- **Implementation Details**: Document special implementation approaches, unusual patterns, or
  complex logic within the source files themselves using comments, not in CONTRIBUTING.md
- **Test Documentation**: Special test approaches (e.g., subprocess testing, integration test
  patterns) should be documented in the test files with detailed comments explaining the rationale
- **CONTRIBUTING.md Scope**: Keep CONTRIBUTING.md focused on project structure overview, development
  workflows, and contribution guidelines. Avoid implementation-specific details
