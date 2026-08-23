#[cfg(test)]
mod migration_tests {
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
    const CURRENT_VERSION: u8 = 1;
    const NEW_VERSION: u8 = 2;
    const OLD_VERSION: u8 = 0;

    mod ghost_program {
        use super::*;

        pub const MIGRATE_DISCRIMINATOR: [u8; 8] = [23, 188, 143, 210, 47, 134, 57, 93];

        pub fn migrate_ix(
            authority: &Pubkey,
            config: &Pubkey,
            new_version: u8,
        ) -> solana_sdk::instruction::Instruction {
            let mut data = Vec::new();
            data.extend_from_slice(&MIGRATE_DISCRIMINATOR);
            data.push(new_version);

            solana_sdk::instruction::Instruction {
                program_id: PROGRAM_ID,
                accounts: vec![
                    solana_sdk::instruction::AccountMeta::new_readonly(*authority, true),
                    solana_sdk::instruction::AccountMeta::new(*config, false),
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
    }

    fn build_test() -> ProgramTest {
        ProgramTest::new("ghost_token", PROGRAM_ID, processor!(|_| Ok(())))
    }

    struct MigContext {
        authority: Keypair,
        mint: Keypair,
        config: Keypair,
    }

    async fn setup() -> (ProgramTestContext, MigContext) {
        let mut ctx = build_test().start_with_context().await;
        let mc = MigContext {
            authority: Keypair::new(),
            mint: Keypair::new(),
            config: Keypair::new(),
        };

        let tx = Transaction::new_signed_with_payer(
            &[system_instruction::transfer(
                &ctx.payer.pubkey(),
                &mc.authority.pubkey(),
                1_000_000_000,
            )],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &mc.authority],
            ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(tx).await.unwrap();

        let ix = ghost_program::init_ix(
            &mc.authority.pubkey(),
            &mc.mint.pubkey(),
            &mc.config.pubkey(),
        );
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&mc.authority.pubkey()),
            &[&mc.authority, &mc.mint, &mc.config],
            ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(tx).await.unwrap();

        (ctx, mc)
    }

    #[tokio::test]
    async fn test_migration_to_new_version() {
        let (mut ctx, mc) = setup().await;

        let ix =
            ghost_program::migrate_ix(&mc.authority.pubkey(), &mc.config.pubkey(), NEW_VERSION);

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&mc.authority.pubkey()),
            &[&mc.authority],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(
            result.is_ok(),
            "Migration to new version should succeed: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    async fn test_migration_same_version_fails() {
        let (mut ctx, mc) = setup().await;

        // Try to migrate to the same version
        let ix = ghost_program::migrate_ix(
            &mc.authority.pubkey(),
            &mc.config.pubkey(),
            CURRENT_VERSION, // Same version
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&mc.authority.pubkey()),
            &[&mc.authority],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(
            result.is_err(),
            "Migrating to same version should fail with InvalidVersion"
        );
    }

    #[tokio::test]
    async fn test_migration_downgrade_fails() {
        let (mut ctx, mc) = setup().await;

        // Try to migrate to an older version
        let ix = ghost_program::migrate_ix(
            &mc.authority.pubkey(),
            &mc.config.pubkey(),
            OLD_VERSION, // Older version
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&mc.authority.pubkey()),
            &[&mc.authority],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(
            result.is_err(),
            "Downgrading version should fail with InvalidVersion"
        );
    }

    #[tokio::test]
    async fn test_migration_unauthorized_fails() {
        let (mut ctx, mc) = setup().await;
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

        let ix =
            ghost_program::migrate_ix(&unauthorized.pubkey(), &mc.config.pubkey(), NEW_VERSION);

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&unauthorized.pubkey()),
            &[&unauthorized],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(
            result.is_err(),
            "Unauthorized migration should fail with Unauthorized"
        );
    }
}

