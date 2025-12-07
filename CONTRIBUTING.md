# Contributing to Traefik Log Parser

Thank you for your interest in contributing to the Traefik Log Parser project!

## Development Setup

### Prerequisites

- Rust toolchain (stable)
- Windows environment for testing Windows-specific features
- Git

### Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/TraefikLogParser.git
   cd TraefikLogParser
   ```

3. Build the project:
   ```bash
   cargo build
   ```

4. Run tests:
   ```bash
   cargo test
   ```

## Development Workflow

### Making Changes

1. Create a new branch for your feature or bug fix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes and ensure:
   - Code compiles without warnings
   - All tests pass
   - Code is properly formatted
   - New tests are added for new functionality

3. Run the test suite:
   ```bash
   cargo test --verbose
   ```

4. Check code formatting:
   ```bash
   cargo fmt -- --check
   ```

5. Run clippy for linting:
   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   ```

### Submitting a Pull Request

1. Push your branch to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

2. Open a Pull Request against the `main` branch

3. Wait for CI checks to pass:
   - **PR Check Workflow** will automatically run when you create or update a PR
   - It performs:
     - Code formatting check (`cargo fmt`)
     - Linting with clippy (`cargo clippy`)
     - Debug build
     - All tests
     - Release build
     - Documentation check
     - Security audit

4. Address any review feedback

## Continuous Integration

### PR Check Workflow

**Trigger**: Automatically runs on all pull requests to `main`

**Purpose**: Ensures that new changes don't break the build or tests

**Steps**:
- Format check
- Lint check (clippy)
- Build (debug and release)
- Run all tests
- Documentation check
- Security audit

**Requirements**: All checks must pass before the PR can be merged

### Release Workflow

**Trigger**: Automatically runs when code is merged to `main`

**Purpose**: Creates a GitHub release with Windows build artifacts

**Steps**:
1. Build release binary for Windows (x64)
2. Run tests on release build
3. Generate version tag based on Cargo.toml version + build number
4. Package artifacts (executable, README, sample log)
5. Create GitHub release with:
   - Windows x64 executable
   - Zipped artifact package
   - Release notes
6. Upload artifacts to release

**Output**:
- GitHub release with tag `v{VERSION}-build.{COUNT}`
- Downloadable `traefik_log_parser-windows-x64.zip`
- Individual executable and documentation files

## Code Style

- Follow standard Rust formatting (enforced by `cargo fmt`)
- Use descriptive variable and function names
- Add comments for complex logic
- Write documentation for public APIs
- Keep functions focused and single-purpose

## Testing

- Write unit tests for new functionality
- Ensure existing tests pass
- Test edge cases and error conditions
- Use descriptive test names

Example:
```rust
#[test]
fn test_parse_valid_traefik_entry() {
    // Test implementation
}
```

## Commit Messages

- Use clear, descriptive commit messages
- Start with a verb in present tense (e.g., "Add", "Fix", "Update")
- Reference issue numbers when applicable

Examples:
- `Add support for IPv6 addresses`
- `Fix parsing error for missing RequestPath`
- `Update documentation for Windows setup`

## Questions or Issues?

- Open an issue for bugs or feature requests
- Start a discussion for questions or ideas
- Review existing issues before creating new ones

Thank you for contributing!
