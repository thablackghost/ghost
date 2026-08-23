#[cfg(test)]
mod initialize_tests {
    use anchor_lang::prelude::*;
    use anchor_lang::solana_program::system_instruction;
    use solana_program_test::*;
    use solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        transaction::Transaction,
    };

    // Program ID for the GHOST token program
    const PROGRAM_ID: Pubkey = solana_sdk::pubkey!("Gh0st111111111111111111111111111111111111111");

    // These would be generated from your Anchor program's IDL.
    // We simulate the instruction data manually for testing.
    mod ghost_program {
        use super::*;

        pub const INITIALIZE_DISCRIMINATOR: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];

        pub fn initialize_ix(
            authority: &Pubkey,
            mint: &Pubkey,
            config: &Pubkey,
            system_program: &Pubkey,
            token_program: &Pubkey,
            decimals: u8,
            initial_supply: u64,
        ) -> solana_sdk::instruction::Instruction {
            let mut data = Vec::new();
            data.extend_from_slice(&INITIALIZE_DISCRIMINATOR);
            data.push(decimals);
            data.extend_from_slice(&initial_supply.to_le_bytes());

            solana_sdk::instruction::Instruction {
                program_id: PROGRAM_ID,
                accounts: vec![
                    solana_sdk::instruction::AccountMeta::new(*authority, true),
                    solana_sdk::instruction::AccountMeta::new(*mint, false),
                    solana_sdk::instruction::AccountMeta::new(*config, false),
                    solana_sdk::instruction::AccountMeta::new_readonly(*system_program, false),
                    solana_sdk::instruction::AccountMeta::new_readonly(*token_program, false),
                    solana_sdk::instruction::AccountMeta::new_readonly(
                        solana_sdk::sysvar::rent::ID,
                        false,
                    ),
                ],
                data,
            }
        }
    }

    fn build_ghost_test() -> ProgramTest {
        let mut program_test = ProgramTest::new(
            "ghost_token",
            PROGRAM_ID,
            processor!(|_| {
                // In a real test, this would be the entrypoint.
                // For program_test, we register the .so
                Ok(())
            }),
        );
        // Set the compute max to something generous
        program_test.set_compute_max_units(1_400_000);
        program_test
    }

    #[tokio::test]
    async fn test_initialize_success() {
        let mut ctx = build_ghost_test().start_with_context().await;
        let authority = Keypair::new();
        let mint_keypair = Keypair::new();
        let config_keypair = Keypair::new();

        // Fund the authority
        {
            let tx = Transaction::new_signed_with_payer(
                &[system_instruction::transfer(
                    &ctx.payer.pubkey(),
                    &authority.pubkey(),
                    1_000_000_000,
                )],
                Some(&ctx.payer.pubkey()),
                &[&ctx.payer, &ctx.authority],
                ctx.last_blockhash,
            );
            ctx.banks_client.process_transaction(tx).await.unwrap();
        }

        let decimals: u8 = 9;
        let initial_supply: u64 = 10_000_000_000_000_000_000; // 10B tokens * 10^9 decimals

        let ix = ghost_program::initialize_ix(
            &authority.pubkey(),
            &mint_keypair.pubkey(),
            &config_keypair.pubkey(),
            &solana_sdk::system_program::ID,
            &spl_token::ID,
            decimals,
            initial_supply,
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&authority.pubkey()),
            &[&authority, &mint_keypair, &config_keypair],
            ctx.last_blockhash,
        );

        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(
            result.is_ok(),
            "Initialize should succeed: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    async fn test_initialize_duplicate_fails() {
        let mut ctx = build_ghost_test().start_with_context().await;
        let authority = Keypair::new();
        let mint_keypair = Keypair::new();
        let config_keypair = Keypair::new();

        // Fund authority
        {
            let tx = Transaction::new_signed_with_payer(
                &[system_instruction::transfer(
                    &ctx.payer.pubkey(),
                    &authority.pubkey(),
                    1_000_000_000,
                )],
                Some(&ctx.payer.pubkey()),
                &[&ctx.payer, &ctx.authority],
                ctx.last_blockhash,
            );
            ctx.banks_client.process_transaction(tx).await.unwrap();
        }

        let decimals: u8 = 9;
        let initial_supply: u64 = 10_000_000_000_000_000_000;

        // First initialization
        let ix = ghost_program::initialize_ix(
            &authority.pubkey(),
            &mint_keypair.pubkey(),
            &config_keypair.pubkey(),
            &solana_sdk::system_program::ID,
            &spl_token::ID,
            decimals,
            initial_supply,
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&authority.pubkey()),
            &[&authority, &mint_keypair, &config_keypair],
            ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(tx).await.unwrap();

        // Second initialization on the same config PDA should fail
        let ix2 = ghost_program::initialize_ix(
            &authority.pubkey(),
            &mint_keypair.pubkey(),
            &config_keypair.pubkey(),
            &solana_sdk::system_program::ID,
            &spl_token::ID,
            decimals,
            initial_supply,
        );

        let tx2 = Transaction::new_signed_with_payer(
            &[ix2],
            Some(&authority.pubkey()),
            &[&authority, &mint_keypair, &config_keypair],
            ctx.last_blockhash,
        );

        let result = ctx.banks_client.process_transaction(tx2).await;
        assert!(
            result.is_err(),
            "Duplicate initialization should fail with AlreadyInitialized"
        );
    }

    #[tokio::test]
    async fn test_initialize_invalid_decimals_fails() {
        let mut ctx = build_ghost_test().start_with_context().await;
        let authority = Keypair::new();
        let mint_keypair = Keypair::new();
        let config_keypair = Keypair::new();

        // Fund authority
        {
            let tx = Transaction::new_signed_with_payer(
                &[system_instruction::transfer(
                    &ctx.payer.pubkey(),
                    &authority.pubkey(),
                    1_000_000_000,
                )],
                Some(&ctx.payer.pubkey()),
                &[&ctx.payer, &ctx.authority],
                ctx.last_blockhash,
            );
            ctx.banks_client.process_transaction(tx).await.unwrap();
        }

        // Invalid decimals: SPL tokens support 0-9 decimals
        let invalid_decimals: u8 = 19;
        let initial_supply: u64 = 10_000_000_000_000_000_000;

        let ix = ghost_program::initialize_ix(
            &authority.pubkey(),
            &mint_keypair.pubkey(),
            &config_keypair.pubkey(),
            &solana_sdk::system_program::ID,
            &spl_token::ID,
            invalid_decimals,
            initial_supply,
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&authority.pubkey()),
            &[&authority, &mint_keypair, &config_keypair],
            ctx.last_blockhash,
        );

        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(
            result.is_err(),
            "Initialize with 19 decimals should fail with InvalidDecimals"
        );
    }

    #[tokio::test]
    async fn test_initialize_initial_supply_correct() {
        let mut ctx = build_ghost_test().start_with_context().await;
        let authority = Keypair::new();
        let mint_keypair = Keypair::new();
        let config_keypair = Keypair::new();

        // Fund authority
        {
            let tx = Transaction::new_signed_with_payer(
                &[system_instruction::transfer(
                    &ctx.payer.pubkey(),
                    &authority.pubkey(),
                    1_000_000_000,
                )],
                Some(&ctx.payer.pubkey()),
                &[&ctx.payer, &ctx.authority],
                ctx.last_blockhash,
            );
            ctx.banks_client.process_transaction(tx).await.unwrap();
        }

        let decimals: u8 = 9;
        // 10 billion tokens * 10^9 decimals = 10_000_000_000_000_000_000 raw units
        let expected_raw_supply: u64 = 10_000_000_000_000_000_000;

        let ix = ghost_program::initialize_ix(
            &authority.pubkey(),
            &mint_keypair.pubkey(),
            &config_keypair.pubkey(),
            &solana_sdk::system_program::ID,
            &spl_token::ID,
            decimals,
            expected_raw_supply,
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&authority.pubkey()),
            &[&authority, &mint_keypair, &config_keypair],
            ctx.last_blockhash,
        );

        ctx.banks_client.process_transaction(tx).await.unwrap();

        // After initialization, we expect 10B tokens * 10^9 = 10,000,000,000,000,000,000 raw units
        // Verify that the calculated human-readable amount matches
        let human_readable = expected_raw_supply as f64 / 10f64.powi(decimals as i32);
        assert!(
            (human_readable - 10_000_000_000.0).abs() < 1.0,
            "Initial supply should be ~10 billion GHOST tokens, got: {}",
            human_readable
        );
        assert_eq!(decimals, 9, "Decimals must be 9");
        assert_eq!(
            expected_raw_supply, 10_000_000_000_000_000_000,
            "Raw supply must be 10B * 10^9"
        );
    }
}
