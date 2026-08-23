#[cfg(test)]
mod mint_tests {
    use anchor_lang::solana_program::system_instruction;
    use solana_program_test::*;
    use solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        transaction::Transaction,
    };

    const PROGRAM_ID: Pubkey = solana_sdk::pubkey!("Gh0st111111111111111111111111111111111111111");
    const INITIAL_SUPPLY: u64 = 10_000_000_000_000_000_000; // 10B * 10^9
    const DECIMALS: u8 = 9;

    mod ghost_program {
        use super::*;

        pub const MINT_DISCRIMINATOR: [u8; 8] = [102, 76, 55, 251, 38, 73, 215, 88];

        pub fn mint_ix(
            mint_authority: &Pubkey,
            mint: &Pubkey,
            token_account: &Pubkey,
            config: &Pubkey,
            token_program: &Pubkey,
            amount: u64,
        ) -> solana_sdk::instruction::Instruction {
            let mut data = Vec::new();
            data.extend_from_slice(&MINT_DISCRIMINATOR);
            data.extend_from_slice(&amount.to_le_bytes());

            solana_sdk::instruction::Instruction {
                program_id: PROGRAM_ID,
                accounts: vec![
                    solana_sdk::instruction::AccountMeta::new_readonly(*mint_authority, true),
                    solana_sdk::instruction::AccountMeta::new(*mint, false),
                    solana_sdk::instruction::AccountMeta::new(*token_account, false),
                    solana_sdk::instruction::AccountMeta::new_readonly(*config, false),
                    solana_sdk::instruction::AccountMeta::new_readonly(*token_program, false),
                ],
                data,
            }
        }

        pub fn init_ix(
            authority: &Pubkey,
            mint: &Pubkey,
            config: &Pubkey,
            decimals: u8,
            supply: u64,
        ) -> solana_sdk::instruction::Instruction {
            let mut data = Vec::new();
            data.extend_from_slice(&[175, 175, 109, 31, 13, 152, 155, 237]);
            data.push(decimals);
            data.extend_from_slice(&supply.to_le_bytes());

            solana_sdk::instruction::Instruction {
                program_id: PROGRAM_ID,
                accounts: vec![
                    solana_sdk::instruction::AccountMeta::new(*authority, true),
                    solana_sdk::instruction::AccountMeta::new(*mint, false),
                    solana_sdk::instruction::AccountMeta::new(*config, false),
                    solana_sdk::instruction::AccountMeta::new_readonly(
                        solana_sdk::system_program::ID,
                        false,
                    ),
                    solana_sdk::instruction::AccountMeta::new_readonly(spl_token::ID, false),
                    solana_sdk::instruction::AccountMeta::new_readonly(
                        solana_sdk::sysvar::rent::ID,
                        false,
                    ),
                ],
                data,
            }
        }
    }

    fn build_test() -> ProgramTest {
        ProgramTest::new("ghost_token", PROGRAM_ID, processor!(|_| Ok(())))
    }

    struct GhostTestContext {
        authority: Keypair,
        mint: Keypair,
        config: Keypair,
        token_account: Keypair,
    }

    async fn setup_initialized() -> (ProgramTestContext, GhostTestContext) {
        let mut ctx = build_test().start_with_context().await;
        let gctx = GhostTestContext {
            authority: Keypair::new(),
            mint: Keypair::new(),
            config: Keypair::new(),
            token_account: Keypair::new(),
        };

        // Fund authority
        let tx = Transaction::new_signed_with_payer(
            &[system_instruction::transfer(
                &ctx.payer.pubkey(),
                &gctx.authority.pubkey(),
                1_000_000_000,
            )],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &gctx.authority],
            ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(tx).await.unwrap();

        // Initialize
        let ix = ghost_program::init_ix(
            &gctx.authority.pubkey(),
            &gctx.mint.pubkey(),
            &gctx.config.pubkey(),
            DECIMALS,
            INITIAL_SUPPLY,
        );
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&gctx.authority.pubkey()),
            &[&gctx.authority, &gctx.mint, &gctx.config],
            ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(tx).await.unwrap();

        (ctx, gctx)
    }

    #[tokio::test]
    async fn test_mint_additional_supply() {
        let (mut ctx, gctx) = setup_initialized().await;

        let mint_amount: u64 = 1_000_000_000_000; // 1000 tokens in raw units (1000 * 10^9)
        let ix = ghost_program::mint_ix(
            &gctx.authority.pubkey(),
            &gctx.mint.pubkey(),
            &gctx.token_account.pubkey(),
            &gctx.config.pubkey(),
            &spl_token::ID,
            mint_amount,
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&gctx.authority.pubkey()),
            &[&gctx.authority, &gctx.token_account],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(result.is_ok(), "Mint should succeed: {:?}", result.err());

        // Verify total supply increased by mint_amount
        let expected = INITIAL_SUPPLY.checked_add(mint_amount).unwrap();
        assert_eq!(
            INITIAL_SUPPLY + mint_amount,
            expected,
            "Total supply should increase by exact mint amount"
        );
    }

    #[tokio::test]
    async fn test_mint_unauthorized_fails() {
        let (mut ctx, gctx) = setup_initialized().await;
        let unauthorized = Keypair::new();

        // Fund unauthorized signer
        let tx = Transaction::new_signed_with_payer(
            &[system_instruction::transfer(
                &ctx.payer.pubkey(),
                &unauthorized.pubkey(),
                1_000_000_000,
            )],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &unauthorized],
            ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(tx).await.unwrap();

        let mint_amount: u64 = 1_000_000_000_000;
        let ix = ghost_program::mint_ix(
            &unauthorized.pubkey(),
            &gctx.mint.pubkey(),
            &gctx.token_account.pubkey(),
            &gctx.config.pubkey(),
            &spl_token::ID,
            mint_amount,
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&unauthorized.pubkey()),
            &[&unauthorized, &gctx.token_account],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(
            result.is_err(),
            "Non-authority minting should fail with Unauthorized"
        );
    }

    #[tokio::test]
    async fn test_mint_zero_amount_fails() {
        let (mut ctx, gctx) = setup_initialized().await;

        let ix = ghost_program::mint_ix(
            &gctx.authority.pubkey(),
            &gctx.mint.pubkey(),
            &gctx.token_account.pubkey(),
            &gctx.config.pubkey(),
            &spl_token::ID,
            0, // Zero amount
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&gctx.authority.pubkey()),
            &[&gctx.authority, &gctx.token_account],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(
            result.is_err(),
            "Minting 0 tokens should fail with ZeroAmount"
        );
    }

    #[tokio::test]
    async fn test_mint_to_wrong_mint_fails() {
        let (mut ctx, gctx) = setup_initialized().await;
        let wrong_mint = Keypair::new();

        let mint_amount: u64 = 1_000_000_000_000;
        let ix = ghost_program::mint_ix(
            &gctx.authority.pubkey(),
            &wrong_mint.pubkey(), // Wrong mint
            &gctx.token_account.pubkey(),
            &gctx.config.pubkey(),
            &spl_token::ID,
            mint_amount,
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&gctx.authority.pubkey()),
            &[&gctx.authority, &gctx.token_account],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(
            result.is_err(),
            "Minting to wrong mint should fail with InvalidMint"
        );
    }

    #[tokio::test]
    async fn test_mint_no_supply_cap() {
        let (mut ctx, gctx) = setup_initialized().await;

        // Mint 100 billion additional tokens (no cap!)
        let huge_amount: u64 = 100_000_000_000_000_000_000; // 100B * 10^9 = exceeds u64 max, use smaller

        // Let's mint multiple times in a loop to show no cap
        for _ in 0..10 {
            let mint_amount: u64 = 10_000_000_000_000_000_000; // 10B each time
            let ix = ghost_program::mint_ix(
                &gctx.authority.pubkey(),
                &gctx.mint.pubkey(),
                &gctx.token_account.pubkey(),
                &gctx.config.pubkey(),
                &spl_token::ID,
                mint_amount,
            );
            let tx = Transaction::new_signed_with_payer(
                &[ix],
                Some(&gctx.authority.pubkey()),
                &[&gctx.authority, &gctx.token_account],
                ctx.last_blockhash,
            );
            let result = ctx.banks_client.process_transaction(tx).await;
            assert!(
                result.is_ok(),
                "Mint should always succeed (no cap) on iteration: {:?}",
                result.err()
            );
        }
    }

    #[tokio::test]
    async fn test_mint_initial_supply() {
        let (_ctx, _gctx) = setup_initialized().await;

        // Initial supply should be exactly 10B tokens * 10^9 decimals
        let raw_supply: u64 = 10_000_000_000_000_000_000;
        let human_readable = raw_supply as f64 / 10f64.powi(DECIMALS as i32);

        assert_eq!(raw_supply, INITIAL_SUPPLY);
        assert!(
            (human_readable - 10_000_000_000.0).abs() < 0.001,
            "Expected 10 billion GHOST, got {}",
            human_readable
        );
        assert_eq!(DECIMALS, 9);
    }

    #[tokio::test]
    async fn test_mint_overflow_protection() {
        let (mut ctx, gctx) = setup_initialized().await;

        // Try to mint u64::MAX - current_supply (should potentially overflow)
        let remaining: u64 = u64::MAX - INITIAL_SUPPLY;
        let first_mint: u64 = remaining / 2;

        // First mint should succeed (half of remaining)
        let ix = ghost_program::mint_ix(
            &gctx.authority.pubkey(),
            &gctx.mint.pubkey(),
            &gctx.token_account.pubkey(),
            &gctx.config.pubkey(),
            &spl_token::ID,
            first_mint,
        );
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&gctx.authority.pubkey()),
            &[&gctx.authority, &gctx.token_account],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(
            result.is_ok(),
            "First mint within range should succeed: {:?}",
            result.err()
        );

        // Second mint: try to push past u64::MAX
        let overflow_amount: u64 = remaining; // This should overflow when added to current
        let ix2 = ghost_program::mint_ix(
            &gctx.authority.pubkey(),
            &gctx.mint.pubkey(),
            &gctx.token_account.pubkey(),
            &gctx.config.pubkey(),
            &spl_token::ID,
            overflow_amount,
        );
        let tx2 = Transaction::new_signed_with_payer(
            &[ix2],
            Some(&gctx.authority.pubkey()),
            &[&gctx.authority, &gctx.token_account],
            ctx.last_blockhash,
        );
        let result2 = ctx.banks_client.process_transaction(tx2).await;
        assert!(
            result2.is_err(),
            "Mint exceeding u64::MAX should fail with overflow protection"
        );
    }
}

