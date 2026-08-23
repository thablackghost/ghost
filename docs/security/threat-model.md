# Threat Model

## 1. Threat Actors

| Actor | Motivation | Capability | Impact |
|-------|-----------|------------|--------|
| **External Attacker** | Financial gain, disruption | Can submit transactions, interact with program via CPI | High — can attempt all attack vectors |
| **Malicious Mint Authority** | Rug-pull, infinite inflation | Holds mint authority keypair | Critical — can mint unlimited tokens |
| **Compromised Admin** | Takeover via authority rotation | Holds admin keypair | High — can rotate all authorities |
| **Compromised Emergency Authority** | Service disruption | Holds emergency keypair | Medium — can pause operations |
| **Front-end Attacker** | Phishing, transaction manipulation | Controls web interface | Medium — can mislead users |
| **MEV / Sandwich Bot** | Extract value from trades | Can reorder transactions | Low-Medium — economic, not program-level |
| **Insider (developer)** | Backdoor in program code | Can propose/deploy upgrades | Critical — full program control |
| **Protocol Dependents** | Malicious CPI calls | Programs integrating ghost-token via CPI | Medium — can trigger edge cases |

## 2. Attack Vectors & Mitigations

### 2.1 Unauthorized Minting

**Vector:** An attacker calls `MintTokens` without holding the mint authority keypair.

**Severity:** Critical

**Mitigation:**
- Signer check: `require!(authority.key == config.mint_authority, UnauthorizedMinter)`
- Signer must be present: account marked `signer` in instruction
- PDA comparison prevents key substitution attacks

### 2.2 Authority Compromise

**Vector:** Private key of mint, admin, or emergency authority is exfiltrated.

**Severity:** Critical (mint), High (admin), Medium (emergency)

**Mitigation:**
- Two-step authority rotation with timelock prevents instant takeover
- Emergency pause stops minting even if mint authority is compromised
- Admin can rotate emergency authority without timelock (immediate)
- Mint authority rotation requires admin signature + timelock
- Recommended: store authorities in multisig or hardware wallets

### 2.3 PDA Collision

**Vector:** An attacker creates a Config-like account at a different address, hoping the program reads it instead of the real Config PDA.

**Severity:** High

**Mitigation:**
- Config is always derived via `Pubkey::find_program_address(&[b"config"], &program_id)`
- `bump` seed is stored and verified on every access
- Cannot collide: PDA derivation includes program ID as signer

### 2.4 Account Substitution

**Vector:** An attacker supplies a different mint account than the one in Config, minting tokens on a fake mint.

**Severity:** High

**Mitigation:**
- Mint instruction verifies `mint.key == config.mint` before CPI
- Config PDA derivation ensures authenticity
- Token-2022 program ID is hardcoded in allowed CPI targets

### 2.5 Malicious CPI

**Vector:** Another program calls ghost-token via CPI with crafted accounts.

**Severity:** Medium-High

**Mitigation:**
- Program validates ALL accounts regardless of caller
- No reliance on `tx.instructions` or transaction-level data
- Config PDA check ensures only this program's config is used
- Program ID check for self-CPI prevention

### 2.6 Arithmetic Overflow

**Vector:** Supplying extreme `amount` values causes integer overflow/underflow.

**Severity:** Low (Solana-native u64, checked math in Rust)

**Mitigation:**
- Rust's `checked_add`, `checked_mul` used for all arithmetic
- `#[deny(arithmetic_overflow)]` compiler flag
- Supply tracking uses `u64` with overflow checks

### 2.7 Replay Attack

**Vector:** An attacker captures and replays a valid signed instruction.

**Severity:** Low

**Mitigation:**
- Solana transactions include `recent_blockhash` — replay impossible after blockhash expires (~2 min)
- PDA state changes make replays idempotent or cause errors
- Nonce accounts not used (no need)

### 2.8 Stale Config

**Vector:** Due to Solana's account loading model, an instruction might read stale Config data.

**Severity:** Low

**Mitigation:**
- Config is marked `writable` in all instructions that check it
- Solana runtime ensures the latest committed state
- No caching across transactions

### 2.9 Timelock Bypass

**Vector:** Attacker attempts to confirm authority rotation before timelock expires.

**Severity:** High

**Mitigation:**
- `Clock::get()?.unix_timestamp` used for timelock enforcement
- `require!(current_time >= pending.effective_at, TimelockNotExpired)`
- Clock sysvar is read from runtime (cannot be spoofed by user accounts)

### 2.10 Program Upgrade Backdoor

**Vector:** Upgrade authority deploys malicious new version of the program.

**Severity:** Critical

**Mitigation:**
- Upgrade authority key should be held in multisig
- Eventual path to immutability (set upgrade authority to `None`)
- Code is open-source and verifiable on-chain
- Upgrade events emitted for monitoring

## 3. Risk Matrix

| Risk | Likelihood | Impact | Risk Level |
|------|-----------|--------|------------|
| Unauthorized minting | Low | Critical | High |
| Mint authority compromise | Medium | Critical | Critical |
| Admin authority compromise | Low | High | Medium-High |
| Emergency authority compromise | Low | Medium | Low-Medium |
| PDA collision | Very Low | High | Low |
| Account substitution | Low | High | Low-Medium |
| Malicious CPI | Medium | Medium | Medium |
| Arithmetic overflow | Very Low | Medium | Very Low |
| Replay attack | Very Low | Low | Very Low |
| Timelock bypass | Very Low | High | Low |
| Upgrade backdoor | Very Low | Critical | Medium |

## 4. Assumptions

1. Solana runtime (Sealevel) operates correctly and processes transactions atomically
2. Ed25519 signature scheme is secure
3. SHA-256 is collision-resistant (PDA derivation)
4. SPL Token-2022 program is secure and correctly implements the token standard
5. Authority keypairs are generated with sufficient entropy
6. Clock sysvar provides accurate timestamps within Solana's tolerance
7. Transaction blockhash validity period (~2 minutes) is sufficient for anti-replay