# Security Policy

## Supported Versions

| Version | Supported          |
|---------|--------------------|
| 0.1.x   | ✅ Active support  |

Only the latest released version receives security updates. Older versions are not supported.

## Reporting a Vulnerability

### DO NOT CREATE A PUBLIC GITHUB ISSUE

Please report security vulnerabilities privately to:

📧 **security@ghost-token.io**

### What to Include

- Detailed description of the vulnerability
- Steps to reproduce
- Potential impact assessment
- Suggested fix (if available)
- Your contact information

### Response Timeline

| Stage | Timeframe |
|-------|-----------|
| Acknowledgment | Within 24 hours |
| Initial assessment | Within 48 hours |
| Fix development | 7-14 days (depending on severity) |
| Public disclosure | Coordinated with reporter |

### Bug Bounty

GHOST token participates in bug bounty programs. Rewards are based on severity:

| Severity | Reward Range |
|----------|-------------|
| Critical | Up to $50,000 |
| High | Up to $10,000 |
| Medium | Up to $2,000 |
| Low | Up to $500 |

## Security Model Overview

The GHOST token program employs a defense-in-depth security model with multiple validation layers:

1. **Solana Runtime (Sealevel)**: Transaction atomicity, signature verification, BPF sandbox
2. **SPL Token-2022 Program**: Mint authority enforcement, supply tracking
3. **Ghost Program Account Validation**: Signer checks, PDA verification, owner validation
4. **Business Logic**: Authority matching, pause gating, timelock enforcement
5. **Operational Security**: Key management, multisig, monitoring

### Key Security Features

- **Two-step authority rotation** with mandatory timelock
- **Emergency pause** to halt minting and rotations
- **PDA-based config** preventing account spoofing
- **Compile-time program IDs** for CPI targets
- **Checked arithmetic** preventing overflow
- **Comprehensive input validation** on all accounts

### Known Limitations

- Unlimited minting is by design — trust is placed in the mint authority
- Program is upgradable — trust is placed in the upgrade authority
- No on-chain supply cap enforcement

## Disclosure Process

1. Reporter submits vulnerability to security@ghost-token.io
2. Team acknowledges receipt within 24 hours
3. Team validates and assesses severity
4. Fix is developed and tested
5. Fix is deployed to mainnet
6. Public advisory is published
7. CVE is requested (if applicable)

### Embargo Period

During the fix development period, the vulnerability details must remain confidential. The reporter and team agree on a disclosure date.

## Security Audits

| Audit | Date | Auditor | Report |
|-------|------|---------|--------|
| Internal Security Review | 2024-01 | Core Team | `audits/reports/placeholder.md` |
| External Audit | TBD | TBD | Pending |

## Security Contacts

| Role | Contact |
|------|---------|
| Security Lead | security@ghost-token.io |
| Protocol Lead | protocol@ghost-token.io |
| GPG Key | [Available on request] |

## Acknowledgments

We thank the following individuals and organizations for responsibly disclosing security issues:

- (None yet — be the first!)