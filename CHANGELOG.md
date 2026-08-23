# Changelog

All notable changes to the GHOST token project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-01-01

### Added
- Initial GHOST token implementation
- SPL Token-2022 support with metadata extension
- Authority-controlled unlimited minting
- Permissionless burn functionality
- Token metadata (name, symbol, URI) via Token-2022
- Emergency pause and resume mechanism
- Two-step authority rotation with timelock
- Multi-authority model (mint, admin, emergency)
- Config PDA for on-chain state management
- Comprehensive test suite
  - Unit tests
  - Integration tests
  - Security/negative tests
  - Property-based tests
  - Fuzz targets (10 targets)
- Rust SDK for program interaction
- TypeScript SDK with IDL types
- Vanity address generation utility
- CLI tools for common operations
- Documentation
  - Architecture overview and program design
  - Threat model and security model
  - Authority and CPI security docs
  - Deployment guides (devnet, mainnet)
  - Operations runbook and emergency procedures
  - Tokenomics documentation
  - Metadata specification
  - Testing strategy
- CI/CD pipeline
  - Format, lint, and test workflows
  - Security scanning (cargo audit, deny, gitleaks)
  - Build verification
  - Coverage reporting (tarpaulin + codecov)
  - Release automation
- Devnet deployment configs and fixtures
- Migration framework (v1 initial state)
- Security checklist (50 items)
- STRIDE threat model