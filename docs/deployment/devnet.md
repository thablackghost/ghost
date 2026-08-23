# Devnet Deployment Guide

## Prerequisites

### System Requirements
- **Operating System:** Ubuntu 22.04+ / macOS 14+ / Windows (WSL2)
- **RAM:** 16GB+ recommended
- **Disk:** 50GB+ free space
- **Network:** Stable internet connection

### Software Dependencies
| Tool | Version | Installation |
|------|---------|-------------|
| Rust | 1.75+ | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| Solana CLI | 1.18+ | `sh -c "$(curl -sSfL https://release.solana.com/v1.18.0/install)"` |
| Anchor CLI | 0.30+ | `cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked` |
| Node.js | 18+ | `https://nodejs.org/` or `nvm` |
| Yarn | 1.22+ | `npm install -g yarn` |

### Verify Installations
```bash
rustc --version          # rustc 1.75.0+
solana --version         # solana-cli 1.18.0+
anchor --version         # anchor-cli 0.30.0+
node --version           # v18.0.0+
yarn --version           # 1.22.0+
```

## Step 1: Configure Solana for Devnet

```bash
# Set cluster to devnet
solana config set --url https://api.devnet.solana.com

# Verify RPC connection
solana cluster-version

# Generate a deploy keypair if you don't have one
solana-keygen new --outfile ~/.config/solana/ghost-deploy.json

# Set as default keypair
solana config set --keypair ~/.config/solana/ghost-deploy.json

# Check balance (you will need ~4 SOL for deployment)
solana balance

# Airdrop if needed (devnet faucet)
solana airdrop 5
```

## Step 2: Build the Program

```bash
# Clone repository
git clone https://github.com/ghost-token/ghost-token.git
cd ghost-token

# Install dependencies
yarn install

# Build program
anchor build

# Verify build output
ls -la target/deploy/ghost_token.so
```

The program binary is at `target/deploy/ghost_token.so`.

Expected output:
```
BPF SDK: /home/user/.local/share/solana/install/active_releases/.../sdk/bpf
cargo-build-bpf child: rustup toolchain list
cargo-build-bpf child: cargo +bpf build ...
To deploy this program:
  $ solana program deploy target/deploy/ghost_token.so
```

## Step 3: Deploy to Devnet

```bash
# Deploy program
solana program deploy \
  --program-id target/deploy/ghost_token-keypair.json \
  target/deploy/ghost_token.so

# Output shows:
# Program Id: Gh0st111111111111111111111111111111111111111
```

**IMPORTANT:** The program ID is `Gh0st111111111111111111111111111111111111111`. This is a vanity address. The keypair file containing the private key was generated through a vanity address search (see `docs/vanity/vanity-generation.md`). Without this specific keypair, you CANNOT deploy to this program ID.

## Step 4: Initialize the Token

```bash
# Generate mint authority keypair
solana-keygen new --outfile mint-authority.json --no-bip39-passphrase

# Generate admin authority keypair
solana-keygen new --outfile admin-authority.json --no-bip39-passphrase

# Generate emergency authority keypair
solana-keygen new --outfile emergency-authority.json --no-bip39-passphrase

# Generate token account for initial supply
solana-keygen new --outfile token-account.json --no-bip39-passphrase

# Run initialize transaction
anchor run initialize-devnet
```

Or manually:
```bash
# Create the Initialize instruction using the SDK
node scripts/initialize.js \
  --mint-authority $(solana address -k mint-authority.json) \
  --admin-authority $(solana address -k admin-authority.json) \
  --emergency-authority $(solana address -k emergency-authority.json) \
  --token-account $(solana address -k token-account.json)
```

## Step 5: Verify Deployment

### Verify Program
```bash
# Check program is deployed
solana program show Gh0st111111111111111111111111111111111111111

# Expected output:
# Program Id: Gh0st111111111111111111111111111111111111111
# Owner: BPFLoaderUpgradeab1e11111111111111111111111
# ProgramData Address: ...
```

### Verify Mint
```bash
# Get mint address (from initialize output)
MINT_ADDRESS="<your-mint-address>"

# Check mint info
spl-token display $MINT_ADDRESS --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

# Expected: 9 decimals, initial supply 10,000,000,000
```

### Verify Config PDA
```bash
# Derive the config PDA
solana find-program-derived-address Gh0st111111111111111111111111111111111111111 "config"

# Read config data
solana account <config-pda-address>
```

## Step 6: Mint Verification

```bash
# Mint some additional tokens (devnet test)
node scripts/mint.js \
  --amount 1000000 \
  --mint $MINT_ADDRESS \
  --recipient $(solana address -k token-account.json) \
  --authority $(solana address -k mint-authority.json)

# Verify supply increased
spl-token display $MINT_ADDRESS --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb
```

## Step 7: Metadata Check

```bash
# Check Token-2022 metadata
spl-token display $MINT_ADDRESS --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb

# Expected fields:
# - name: GHOST
# - symbol: GHOST
# - uri: https://arweave.net/placeholder-ghost-logo
```

## Step 8: View on Explorer

```bash
# Program on Solana Explorer (devnet)
echo "https://explorer.solana.com/address/Gh0st111111111111111111111111111111111111111?cluster=devnet"

# Token on Solana Explorer
echo "https://explorer.solana.com/address/$MINT_ADDRESS?cluster=devnet"
```

## Troubleshooting

### "Insufficient funds"
```bash
solana airdrop 5
```

### "Account already in use"
```bash
# Check if program is already deployed
solana program show Gh0st111111111111111111111111111111111111111
```

### "Build failed"
```bash
# Ensure BPF target is installed
rustup target add bpfel-unknown-unknown
# or
cargo build-bpf --version
```

### "RPC timeout"
```bash
# Use a different RPC endpoint
solana config set --url https://api.devnet.solana.com
# Or try a public RPC provider
solana config set --url https://solana-devnet-rpc.publicnode.com
```

## Devnet Configuration Summary

| Item | Value |
|------|-------|
| Cluster | Devnet |
| RPC URL | `https://api.devnet.solana.com` |
| Program ID | `Gh0st111111111111111111111111111111111111111` |
| Token Decimals | 9 |
| Initial Supply | 10,000,000,000 GHOST |