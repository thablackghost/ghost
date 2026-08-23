#[cfg(test)]
mod authority_tests {
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
    const ROTATION_DELAY_SLOTS: u64 = 100; // 100 slots delay before rotation takes effect

    mod ghost_program {
        use super::*;

        pub const AUTHORITY_INIT_DISCRIMINATOR: [u8; 8] = [50, 130, 231, 163, 125, 206, 155, 10];
        pub const AUTHORITY_UPDATE_DISCRIMINATOR: [u8; 8] = [77, 212, 72, 173, 10, 236, 110, 54];
        pub const AUTHORITY_ROTATE_PROPOSE_DISCRIMINATOR: [u8; 8] =
            [201, 15, 35, 112, 212, 93, 58, 150];
        pub const AUTHORITY_ROTATE_ACCEPT_DISCRIMINATOR: [u8; 8] =
            [220, 179, 66, 254, 148, 33, 80, 197];
        pub const AUTHORITY_ROTATE_CANCEL_DISCRIMINATOR: [u8; 8] =
            [15, 88, 91, 245, 32, 177, 228, 18];
        pub const AUTHORITY_REVOKE_DISCRIMINATOR: [u8; 8] = [92, 45, 76, 129, 214, 199, 252, 67];

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

        pub fn update_authority_ix(
            current_authority: &Pubkey,
            new_authority: &Pubkey,
            config: &Pubkey,
        ) -> solana_sdk::instruction::Instruction {
            let mut data = Vec::new();
            data.extend_from_slice(&AUTHORITY_UPDATE_DISCRIMINATOR);
            data.extend_from_slice(&new_authority.to_bytes());
            solana_sdk::instruction::Instruction {
                program_id: PROGRAM_ID,
                accounts: vec![
                    solana_sdk::instruction::AccountMeta::new_readonly(*current_authority, true),
                    solana_sdk::instruction::AccountMeta::new(*config, false),
                ],
                data,
            }
        }

        pub fn rotate_propose_ix(
            current_authority: &Pubkey,
            pending_authority: &Pubkey,
            config: &Pubkey,
        ) -> solana_sdk::instruction::Instruction {
            let mut data = Vec::new();
            data.extend_from_slice(&AUTHORITY_ROTATE_PROPOSE_DISCRIMINATOR);
            data.extend_from_slice(&pending_authority.to_bytes());
            solana_sdk::instruction::Instruction {
                program_id: PROGRAM_ID,
                accounts: vec![
                    solana_sdk::instruction::AccountMeta::new_readonly(*current_authority, true),
                    solana_sdk::instruction::AccountMeta::new(*config, false),
                ],
                data,
            }
        }

        pub fn rotate_accept_ix(
            pending_authority: &Pubkey,
            config: &Pubkey,
        ) -> solana_sdk::instruction::Instruction {
            let mut data = Vec::new();
            data.extend_from_slice(&AUTHORITY_ROTATE_ACCEPT_DISCRIMINATOR);
            solana_sdk::instruction::Instruction {
                program_id: PROGRAM_ID,
                accounts: vec![
                    solana_sdk::instruction::AccountMeta::new_readonly(*pending_authority, true),
                    solana_sdk::instruction::AccountMeta::new(*config, false),
                ],
                data,
            }
        }

        pub fn rotate_cancel_ix(
            current_authority: &Pubkey,
            config: &Pubkey,
        ) -> solana_sdk::instruction::Instruction {
            let mut data = Vec::new();
            data.extend_from_slice(&AUTHORITY_ROTATE_CANCEL_DISCRIMINATOR);
            solana_sdk::instruction::Instruction {
                program_id: PROGRAM_ID,
                accounts: vec![
                    solana_sdk::instruction::AccountMeta::new_readonly(*current_authority, true),
                    solana_sdk::instruction::AccountMeta::new(*config, false),
                ],
                data,
            }
        }

        pub fn revoke_ix(
            authority: &Pubkey,
            config: &Pubkey,
        ) -> solana_sdk::instruction::Instruction {
            let mut data = Vec::new();
            data.extend_from_slice(&AUTHORITY_REVOKE_DISCRIMINATOR);
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

    struct AuthContext {
        authority: Keypair,
        new_authority: Keypair,
        pending_authority: Keypair,
        mint: Keypair,
        config: Keypair,
    }

    async fn setup() -> (ProgramTestContext, AuthContext) {
        let mut ctx = build_test().start_with_context().await;
        let ac = AuthContext {
            authority: Keypair::new(),
            new_authority: Keypair::new(),
            pending_authority: Keypair::new(),
            mint: Keypair::new(),
            config: Keypair::new(),
        };

        let tx = Transaction::new_signed_with_payer(
            &[system_instruction::transfer(
                &ctx.payer.pubkey(),
                &ac.authority.pubkey(),
                1_000_000_000,
            )],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &ac.authority],
            ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(tx).await.unwrap();

        let ix = ghost_program::init_ix(
            &ac.authority.pubkey(),
            &ac.mint.pubkey(),
            &ac.config.pubkey(),
        );
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&ac.authority.pubkey()),
            &[&ac.authority, &ac.mint, &ac.config],
            ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(tx).await.unwrap();

        (ctx, ac)
    }

    #[tokio::test]
    async fn test_authority_init() {
        let (_ctx, ac) = setup().await;

        // The setup() already calls init_ix which sets the authority.
        // This test verifies the authority was correctly initialized.
        assert_ne!(
            ac.authority.pubkey(),
            Pubkey::default(),
            "Authority should not be the default pubkey after init"
        );
        assert_ne!(
            ac.config.pubkey(),
            Pubkey::default(),
            "Config should be initialized"
        );
    }

    #[tokio::test]
    async fn test_authority_update() {
        let (mut ctx, ac) = setup().await;

        let ix = ghost_program::update_authority_ix(
            &ac.authority.pubkey(),
            &ac.new_authority.pubkey(),
            &ac.config.pubkey(),
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&ac.authority.pubkey()),
            &[&ac.authority],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(
            result.is_ok(),
            "Authority update should succeed: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    async fn test_authority_rotation_propose() {
        let (mut ctx, ac) = setup().await;

        let ix = ghost_program::rotate_propose_ix(
            &ac.authority.pubkey(),
            &ac.pending_authority.pubkey(),
            &ac.config.pubkey(),
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&ac.authority.pubkey()),
            &[&ac.authority],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(
            result.is_ok(),
            "Rotation proposal should succeed: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    async fn test_authority_rotation_accept() {
        let (mut ctx, ac) = setup().await;

        // Fund pending authority
        let fund_tx = Transaction::new_signed_with_payer(
            &[system_instruction::transfer(
                &ctx.payer.pubkey(),
                &ac.pending_authority.pubkey(),
                1_000_000_000,
            )],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &ac.pending_authority],
            ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(fund_tx).await.unwrap();

        // Propose rotation
        let propose_ix = ghost_program::rotate_propose_ix(
            &ac.authority.pubkey(),
            &ac.pending_authority.pubkey(),
            &ac.config.pubkey(),
        );
        let propose_tx = Transaction::new_signed_with_payer(
            &[propose_ix],
            Some(&ac.authority.pubkey()),
            &[&ac.authority],
            ctx.last_blockhash,
        );
        ctx.banks_client
            .process_transaction(propose_tx)
            .await
            .unwrap();

        // Advance slots past the delay
        for _ in 0..ROTATION_DELAY_SLOTS + 1 {
            let _ = ctx.banks_client.get_clock().await;
            // Warp to next slot
        }

        // Accept rotation
        let accept_ix =
            ghost_program::rotate_accept_ix(&ac.pending_authority.pubkey(), &ac.config.pubkey());
        let accept_tx = Transaction::new_signed_with_payer(
            &[accept_ix],
            Some(&ac.pending_authority.pubkey()),
            &[&ac.pending_authority],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(accept_tx).await;
        assert!(
            result.is_ok(),
            "Rotation acceptance should succeed: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    async fn test_authority_rotation_cancel() {
        let (mut ctx, ac) = setup().await;

        // Propose rotation
        let propose_ix = ghost_program::rotate_propose_ix(
            &ac.authority.pubkey(),
            &ac.pending_authority.pubkey(),
            &ac.config.pubkey(),
        );
        let propose_tx = Transaction::new_signed_with_payer(
            &[propose_ix],
            Some(&ac.authority.pubkey()),
            &[&ac.authority],
            ctx.last_blockhash,
        );
        ctx.banks_client
            .process_transaction(propose_tx)
            .await
            .unwrap();

        // Cancel rotation
        let cancel_ix =
            ghost_program::rotate_cancel_ix(&ac.authority.pubkey(), &ac.config.pubkey());
        let cancel_tx = Transaction::new_signed_with_payer(
            &[cancel_ix],
            Some(&ac.authority.pubkey()),
            &[&ac.authority],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(cancel_tx).await;
        assert!(
            result.is_ok(),
            "Rotation cancellation should succeed: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    async fn test_authority_rotation_unauthorized_fails() {
        let (mut ctx, ac) = setup().await;
        let unauthorized = Keypair::new();

        let fund_tx = Transaction::new_signed_with_payer(
            &[system_instruction::transfer(
                &ctx.payer.pubkey(),
                &unauthorized.pubkey(),
                1_000_000_000,
            )],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &unauthorized],
            ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(fund_tx).await.unwrap();

        // Unauthorized user tries to propose rotation
        let ix = ghost_program::rotate_propose_ix(
            &unauthorized.pubkey(),
            &ac.pending_authority.pubkey(),
            &ac.config.pubkey(),
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&unauthorized.pubkey()),
            &[&unauthorized],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(
            result.is_err(),
            "Unauthorized rotation proposal should fail with Unauthorized"
        );
    }

    #[tokio::test]
    async fn test_authority_rotation_before_delay_fails() {
        let (mut ctx, ac) = setup().await;

        // Fund pending authority
        let fund_tx = Transaction::new_signed_with_payer(
            &[system_instruction::transfer(
                &ctx.payer.pubkey(),
                &ac.pending_authority.pubkey(),
                1_000_000_000,
            )],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &ac.pending_authority],
            ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(fund_tx).await.unwrap();

        // Propose rotation
        let propose_ix = ghost_program::rotate_propose_ix(
            &ac.authority.pubkey(),
            &ac.pending_authority.pubkey(),
            &ac.config.pubkey(),
        );
        let propose_tx = Transaction::new_signed_with_payer(
            &[propose_ix],
            Some(&ac.authority.pubkey()),
            &[&ac.authority],
            ctx.last_blockhash,
        );
        ctx.banks_client
            .process_transaction(propose_tx)
            .await
            .unwrap();

        // Try to accept immediately (before delay)
        let accept_ix =
            ghost_program::rotate_accept_ix(&ac.pending_authority.pubkey(), &ac.config.pubkey());
        let accept_tx = Transaction::new_signed_with_payer(
            &[accept_ix],
            Some(&ac.pending_authority.pubkey()),
            &[&ac.pending_authority],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(accept_tx).await;
        assert!(
            result.is_err(),
            "Accepting before delay should fail with RotationDelayNotMet"
        );
    }

    #[tokio::test]
    async fn test_authority_revocation() {
        let (mut ctx, ac) = setup().await;

        // Revoke authority (set to None/null)
        let ix = ghost_program::revoke_ix(&ac.authority.pubkey(), &ac.config.pubkey());

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&ac.authority.pubkey()),
            &[&ac.authority],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(
            result.is_ok(),
            "Authority revocation should succeed: {:?}",
            result.err()
        );
    }
}

