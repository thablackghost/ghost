#[cfg(test)]
mod transfer_tests {
    use anchor_lang::solana_program::system_instruction;
    use solana_program_test::*;
    use solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        transaction::Transaction,
    };

    const PROGRAM_ID: Pubkey = solana_sdk::pubkey!("Gh0st111111111111111111111111111111111111111");
    const INITIAL_SUPPLY: u64 = 10_000_000_000_000_000_000;
    const DECIMALS: u8 = 9;

    mod ghost_program {
        use super::*;

        pub const TRANSFER_DISCRIMINATOR: [u8; 8] = [163, 52, 200, 231, 140, 3, 69, 186];

        pub fn transfer_ix(
            owner: &Pubkey,
            from_token_account: &Pubkey,
            to_token_account: &Pubkey,
            mint: &Pubkey,
            config: &Pubkey,
            token_program: &Pubkey,
            amount: u64,
        ) -> solana_sdk::instruction::Instruction {
            let mut data = Vec::new();
            data.extend_from_slice(&TRANSFER_DISCRIMINATOR);
            data.extend_from_slice(&amount.to_le_bytes());

            solana_sdk::instruction::Instruction {
                program_id: PROGRAM_ID,
                accounts: vec![
                    solana_sdk::instruction::AccountMeta::new_readonly(*owner, true),
                    solana_sdk::instruction::AccountMeta::new(*from_token_account, false),
                    solana_sdk::instruction::AccountMeta::new(*to_token_account, false),
                    solana_sdk::instruction::AccountMeta::new_readonly(*mint, false),
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
        ) -> solana_sdk::instruction::Instruction {
            let mut data = Vec::new();
            data.extend_from_slice(&[175, 175, 109, 31, 13, 152, 155, 237]);
            data.push(DECIMALS);
            data.extend_from_slice(&INITIAL_SUPPLY.to_le_bytes());
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

        pub const PAUSE_DISCRIMINATOR: [u8; 8] = [211, 11, 44, 80, 20, 7, 236, 189];

        pub fn pause_ix(
            authority: &Pubkey,
            config: &Pubkey,
        ) -> solana_sdk::instruction::Instruction {
            let mut data = Vec::new();
            data.extend_from_slice(&PAUSE_DISCRIMINATOR);
            solana_sdk::instruction::Instruction {
                program_id: PROGRAM_ID,
                accounts: vec![
                    solana_sdk::instruction::AccountMeta::new_readonly(*authority, true),
                    solana_sdk::instruction::AccountMeta::new(*config, false),
                ],
                data,
            }
        }
    }

    fn build_test() -> ProgramTest {
        ProgramTest::new("ghost_token", PROGRAM_ID, processor!(|_| Ok(())))
    }

    struct TransferContext {
        authority: Keypair,
        mint: Keypair,
        config: Keypair,
        from_token: Keypair,
        to_token: Keypair,
    }

    async fn setup() -> (ProgramTestContext, TransferContext) {
        let mut ctx = build_test().start_with_context().await;
        let tc = TransferContext {
            authority: Keypair::new(),
            mint: Keypair::new(),
            config: Keypair::new(),
            from_token: Keypair::new(),
            to_token: Keypair::new(),
        };

        let tx = Transaction::new_signed_with_payer(
            &[system_instruction::transfer(
                &ctx.payer.pubkey(),
                &tc.authority.pubkey(),
                1_000_000_000,
            )],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &tc.authority],
            ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(tx).await.unwrap();

        let ix = ghost_program::init_ix(
            &tc.authority.pubkey(),
            &tc.mint.pubkey(),
            &tc.config.pubkey(),
        );
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&tc.authority.pubkey()),
            &[&tc.authority, &tc.mint, &tc.config],
            ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(tx).await.unwrap();

        (ctx, tc)
    }

    #[tokio::test]
    async fn test_transfer_success() {
        let (mut ctx, tc) = setup().await;

        let transfer_amount: u64 = 100_000_000_000; // 100 GHOST
        let ix = ghost_program::transfer_ix(
            &tc.authority.pubkey(),
            &tc.from_token.pubkey(),
            &tc.to_token.pubkey(),
            &tc.mint.pubkey(),
            &tc.config.pubkey(),
            &spl_token::ID,
            transfer_amount,
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&tc.authority.pubkey()),
            &[&tc.authority, &tc.from_token, &tc.to_token],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(
            result.is_ok(),
            "Transfer should succeed: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    async fn test_transfer_zero_fails() {
        let (mut ctx, tc) = setup().await;

        let ix = ghost_program::transfer_ix(
            &tc.authority.pubkey(),
            &tc.from_token.pubkey(),
            &tc.to_token.pubkey(),
            &tc.mint.pubkey(),
            &tc.config.pubkey(),
            &spl_token::ID,
            0,
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&tc.authority.pubkey()),
            &[&tc.authority, &tc.from_token, &tc.to_token],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(
            result.is_err(),
            "Transferring 0 should fail with ZeroAmount"
        );
    }

    #[tokio::test]
    async fn test_transfer_insufficient_balance() {
        let (mut ctx, tc) = setup().await;

        // Try to transfer more than the total supply
        let excessive_amount: u64 = INITIAL_SUPPLY + 1;
        let ix = ghost_program::transfer_ix(
            &tc.authority.pubkey(),
            &tc.from_token.pubkey(),
            &tc.to_token.pubkey(),
            &tc.mint.pubkey(),
            &tc.config.pubkey(),
            &spl_token::ID,
            excessive_amount,
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&tc.authority.pubkey()),
            &[&tc.authority, &tc.from_token, &tc.to_token],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(
            result.is_err(),
            "Transferring more than balance should fail with InsufficientBalance"
        );
    }

    #[tokio::test]
    async fn test_transfer_wrong_owner_fails() {
        let (mut ctx, tc) = setup().await;
        let wrong_owner = Keypair::new();

        let tx = Transaction::new_signed_with_payer(
            &[system_instruction::transfer(
                &ctx.payer.pubkey(),
                &wrong_owner.pubkey(),
                1_000_000_000,
            )],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &wrong_owner],
            ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(tx).await.unwrap();

        let transfer_amount: u64 = 100_000_000_000;
        let ix = ghost_program::transfer_ix(
            &wrong_owner.pubkey(), // Wrong owner
            &tc.from_token.pubkey(),
            &tc.to_token.pubkey(),
            &tc.mint.pubkey(),
            &tc.config.pubkey(),
            &spl_token::ID,
            transfer_amount,
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&wrong_owner.pubkey()),
            &[&wrong_owner, &tc.from_token, &tc.to_token],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(
            result.is_err(),
            "Transfer by wrong owner should fail with Unauthorized"
        );
    }

    #[tokio::test]
    async fn test_transfer_paused_fails() {
        let (mut ctx, tc) = setup().await;

        // Pause the contract first
        let pause_ix = ghost_program::pause_ix(&tc.authority.pubkey(), &tc.config.pubkey());
        let pause_tx = Transaction::new_signed_with_payer(
            &[pause_ix],
            Some(&tc.authority.pubkey()),
            &[&tc.authority],
            ctx.last_blockhash,
        );
        ctx.banks_client
            .process_transaction(pause_tx)
            .await
            .unwrap();

        // Now try to transfer while paused
        let transfer_amount: u64 = 100_000_000_000;
        let ix = ghost_program::transfer_ix(
            &tc.authority.pubkey(),
            &tc.from_token.pubkey(),
            &tc.to_token.pubkey(),
            &tc.mint.pubkey(),
            &tc.config.pubkey(),
            &spl_token::ID,
            transfer_amount,
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&tc.authority.pubkey()),
            &[&tc.authority, &tc.from_token, &tc.to_token],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(
            result.is_err(),
            "Transfer during pause should fail with Paused"
        );
    }
}
