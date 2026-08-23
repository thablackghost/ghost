# Devnet Deployment Checklist

## Pre-Deployment

- [ ] Solana CLI installed (v1.18.0+)
- [ ] Anchor CLI installed (v0.30.0+)
- [ ] Devnet RPC accessible: `solana cluster-version --url devnet`
- [ ] Deploy keypair generated
- [ ] Deploy account funded (minimum 5 SOL)
- [ ] Devnet airdrop completed: `solana airdrop 5`
- [ ] All tests passing: `anchor test`
- [ ] Build successful: `anchor build`
- [ ] Program binary exists: `target/deploy/ghost_token.so`
- [ ] Vanity keypair present: `target/deploy/ghost_token-keypair.json`

## Deployment

- [ ] Program deployed: `solana program deploy target/deploy/ghost_token.so --program-id target/deploy/ghost_token-keypair.json`
- [ ] Program ID verified: `Gh0st111111111111111111111111111111111111111`
- [ ] Program executable: `solana program show Gh0st111111111111111111111111111111111111111`
- [ ] Program owner is BPF Loader
- [ ] Deploy transaction signature recorded

## Token Initialization

- [ ] Mint authority keypair generated
- [ ] Admin authority keypair generated
- [ ] Emergency authority keypair generated
- [ ] Token account for initial supply created
- [ ] Initialize instruction executed
- [ ] Mint address recorded
- [ ] Config PDA derived and recorded

## Verification

- [ ] Mint decimals = 9
- [ ] Initial supply = 10,000,000,000 GHOST
- [ ] Mint authority matches expected
- [ ] Admin authority matches expected
- [ ] Emergency authority matches expected
- [ ] Config PDA paused = false
- [ ] Metadata name = "GHOST"
- [ ] Metadata symbol = "GHOST"
- [ ] All on Explorer: `https://explorer.solana.com?cluster=devnet`

## Functional Tests

- [ ] Test mint (small amount): `mint 1000 GHOST`
- [ ] Test burn (small amount): `burn 500 GHOST`
- [ ] Verify supply increased correctly after mint
- [ ] Verify supply decreased correctly after burn
- [ ] Test pause: protocol enters paused state
- [ ] Test mint rejected while paused
- [ ] Test resume: protocol returns to active state
- [ ] Test authority rotation (initiate + confirm)
- [ ] Test rotation cancellation
- [ ] Test rotation timelock enforcement

## Cleanup

- [ ] All test transaction signatures recorded
- [ ] Configuration values updated in `devnet-config.json`
- [ ] Deployment log written
- [ ] Test artifacts archived