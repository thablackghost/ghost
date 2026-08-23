# Contributing to GHOST Token

Thank you for your interest in contributing to GHOST token! This document provides guidelines for contributing to the project.

## Code of Conduct

Please read and follow our Code of Conduct. We expect all contributors to:
- Be respectful and inclusive
- Use welcoming language
- Accept constructive feedback graciously
- Focus on what is best for the community

## Getting Started

### Prerequisites

- **Rust**: 1.75.0+
- **Solana CLI**: 1.18.0+
- **Anchor CLI**: 0.30.0+
- **Node.js**: 18+
- **Yarn**: 1.22+

### Development Setup

```bash
# Clone the repository
git clone https://github.com/ghost-token/ghost-token.git
cd ghost-token

# Install dependencies
yarn install

# Build the program
anchor build

# Run tests
anchor test

# Run specific test suite
anchor test -- --test-path-pattern security
```

### Project Structure

```
ghost-token/
├── programs/ghost-token/    # On-chain Solana program (Rust)
│   └── src/
│       ├── lib.rs           # Entry point
│       ├── state.rs         # Config struct
│       ├── instructions/    # Instruction handlers
│       ├── errors.rs        # Error types
│       └── constants.rs     # Seeds, constants
├── sdk/
│   ├── rust/                # Rust SDK
│   └── typescript/          # TypeScript SDK
├── tests/                   # Integration tests
├── docs/                    # Documentation
├── metadata/                # Token metadata JSON
└── .github/                 # CI/CD workflows
```

## Development Workflow

### 1. Create an Issue

Before starting work, create an issue describing the change:
- Bug reports: Use the bug report template
- Features: Use the feature request template
- Security: See SECURITY.md

### 2. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-fix-name
```

### 3. Make Changes

- Follow the existing code style
- Add tests for new functionality
- Update documentation as needed
- Keep changes focused and atomic

### 4. Run Tests

```bash
# Format check
cargo fmt --all -- --check

# Lint check
cargo clippy --all-targets --all-features -- -D warnings

# Unit tests
cargo test --all-features

# Integration tests
anchor test
```

### 5. Commit

```bash
git add .
git commit -m "Description of changes"
```

Commit messages should follow conventional commits:
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation
- `test:` for tests
- `chore:` for maintenance
- `refactor:` for code changes

### 6. Create a Pull Request

- Push your branch
- Create a PR against `main`
- Fill out the PR template
- Link related issues
- Request review from maintainers

## Code Style

### Rust

- Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/README.html)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Prefer descriptive variable names
- Document public APIs with doc comments
- Use `thiserror` for error types
- Use `borsh` for serialization

### Anchor/Solana Conventions

- Validate ALL accounts in instruction handlers
- Use `require!` macros for validation
- Use custom error types for all failure cases
- Prefer checked arithmetic (`checked_add`, etc.)
- Use PDA derivation for program-controlled accounts
- Never trust user-supplied program IDs

### TypeScript

- Use strict TypeScript
- Follow ESLint configuration
- Use `@solana/web3.js` and Anchor TS client
- Document public API methods

## Testing Requirements

All changes must include appropriate tests:

| Change Type | Required Tests |
|-------------|---------------|
| New instruction | Integration test + security test |
| Bug fix | Regression test |
| New validation | Negative test (invalid input) |
| State change | State transition test |
| Documentation | No tests required |

## Documentation

- Update relevant docs in `docs/` for significant changes
- Add doc comments for new public API methods
- Update CHANGELOG.md for each release
- Keep architecture docs in sync with code

## Review Process

1. Author creates PR with description and linked issues
2. CI must pass all checks (fmt, clippy, test, build)
3. At least one maintainer reviews and approves
4. Author addresses review feedback
5. Maintainer merges PR

## Release Process

1. Update version in `Cargo.toml` and `package.json`
2. Update `CHANGELOG.md`
3. Create and push a git tag: `git tag v0.1.0 && git push --tags`
4. CI automatically builds and creates a GitHub Release

## Communication

- **Issues**: Bug reports and feature requests
- **Pull Requests**: Code contributions and review
- **Discussions**: General questions and ideas
- **Discord**: Real-time chat (link TBD)

## License

By contributing, you agree that your contributions will be licensed under the project's license.

---

Thank you for contributing to GHOST token! 🚀