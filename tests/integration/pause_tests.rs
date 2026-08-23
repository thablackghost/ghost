#[cfg(test)]
mod pause_tests {
    use solana_program_test::*;
    use solana_sdk::{
        signature::{Keypair, Signer},
        transaction::Transaction,
        pubkey::Pubkey,
    };
    use anchor_lang::solana_program::system_instruction;

    const PROGRAM_ID: Pubkey = solana_sdk::pubkey!("Gh0st111111111111111111111111111111111111111");
    const INITIAL_SUPPLY: u64 = 10_000_000_000_000_000_000;
    const DECIMALS: u8 = 9;

    mod ghost_program {
        use super::*;

        pub const PAUSE_DISCRIMINATOR: [u8; 8] = [211, 11, 44, 80, 20, 7, 236, 189];
        pub const UNPAUSE_DISCRIMINATOR: [u8; 8] = [88, 67, 143, 52, 254, 215, 3, 27];
        pub const EMERGENCY_PAUSE_DISCRIMINATOR: [u8; 8] = [145, 216, 92, 178, 34, 99, 249, 111];
        pub const EMERGENCY_RESUME_DISCRIMINATOR: [u8; 8] = [33, 187, 42, 200, 171, 5, 213, 254];
        pub const MINT_DISCRIMINATOR: [u8; 8] = [102, 76, 55, 251, 38, 73, 215, 88];
        pub const TRANSFER_DISCRIMINATOR: [u8; 8] = [163, 52, 200, 231, 140, 3, 69, 186];

        pub fn init_ix(authority: &Pubkey, mint: &Pubkey, config: &Pubkey) -> solana_sdk::instruction::Instruction {
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
                    solana_sdk::instruction::AccountMeta::new_readonly(solana_sdk::system_program::ID, false),
                    solana_sdk::instruction::AccountMeta::new_readonly(spl_token::ID, false),
                    solana_sdk::instruction::AccountMeta::new_readonly(solana_sdk::sysvar::rent::ID, false),
                ],
                data,
            }
        }

        pub fn pause_ix(authority: &Pubkey, config: &Pubkey) -> solana_sdk::instruction::Instruction {
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

        pub fn unpause_ix(authority: &Pubkey, config: &Pubkey) -> solana_sdk::instruction::Instruction {
            let mut data = Vec::new();
            data.extend_from_slice(&UNPAUSE_DISCRIMINATOR);
            solana_sdk::instruction::Instruction {
                program_id: PROGRAM_ID,
                accounts: vec![
                    solana_sdk::instruction::AccountMeta::new_readonly(*authority, true),
                    solana_sdk::instruction::AccountMeta::new(*config, false),
                ],
                data,
            }
        }

        pub fn emergency_pause_ix(authority: &Pubkey, config: &Pubkey) -> solana_sdk::instruction::Instruction {
            let mut data = Vec::new();
            data.extend_from_slice(&EMERGENCY_PAUSE_DISCRIMINATOR);
            solana_sdk::instruction::Instruction {
                program_id: PROGRAM_ID,
                accounts: vec![
                    solana_sdk::instruction::AccountMeta::new_readonly(*authority, true),
                    solana_sdk::instruction::AccountMeta::new(*config, false),
                ],
                data,
            }
        }

        pub fn emergency_resume_ix(authority: &Pubkey, config: &Pubkey) -> solana_sdk::instruction::Instruction {
            let mut data = Vec::new();
            data.extend_from_slice(&EMERGENCY_RESUME_DISCRIMINATOR);
            solana_sdk::instruction::Instruction {
                program_id: PROGRAM_ID,
                accounts: vec![
                    solana_sdk::instruction::AccountMeta::new_readonly(*authority, true),
                    solana_sdk::instruction::AccountMeta::new(*config, false),
                ],
                data,
            }
        }

        pub fn mint_ix(
            authority: &Pubkey, mint: &Pubkey, token_account: &Pubkey,
            config: &Pubkey, amount: u64,
        ) -> solana_sdk::instruction::Instruction {
            let mut data = Vec::new();
            data.extend_from_slice(&MINT_DISCRIMINATOR);
            data.extend_from_slice(&amount.to_le_bytes());
            solana_sdk::instruction::Instruction {
                program_id: PROGRAM_ID,
                accounts: vec![
                    solana_sdk::instruction::AccountMeta::new_readonly(*authority, true),
                    solana_sdk::instruction::AccountMeta::new(*mint, false),
                    solana_sdk::instruction::AccountMeta::new(*token_account, false),
                    solana_sdk::instruction::AccountMeta::new_readonly(*config, false),
                    solana_sdk::instruction::AccountMeta::new_readonly(spl_token::ID, false),
                ],
                data,
            }
        }

        pub fn transfer_ix(
            owner: &Pubkey, from_tk: &Pubkey, to_tk: &Pubkey,
            mint: &Pubkey, config: &Pubkey, amount: u64,
        ) -> solana_sdk::instruction::Instruction {
            let mut data = Vec::new();
            data.extend_from_slice(&TRANSFER_DISCRIMINATOR);
            data.extend_from_slice(&amount.to_le_bytes());
            solana_sdk::instruction::Instruction {
                program_id: PROGRAM_ID,
                accounts: vec![
                    solana_sdk::instruction::AccountMeta::new_readonly(*owner, true),
                    solana_sdk::instruction::AccountMeta::new(*from_tk, false),
                    solana_sdk::instruction::AccountMeta::new(*to_tk, false),
                    solana_sdk::instruction::AccountMeta::new_readonly(*mint, false),
                    solana_sdk::instruction::AccountMeta::new_readonly(*config, false),
                    solana_sdk::instruction::AccountMeta::new_readonly(spl_token::ID, false),
                ],
                data,
            }
        }
    }

    fn build_test() -> ProgramTest {
        ProgramTest::new("ghost_token", PROGRAM_ID, processor!(|_| Ok(())))
    }

    struct PauseContext {
        authority: Keypair,
        mint: Keypair,
        config: Keypair,
        token_account: Keypair,
    }

    async fn setup() -> (ProgramTestContext, PauseContext) {
        let mut ctx = build_test().start_with_context().await;
        let pc = PauseContext {
            authority: Keypair::new(),
            mint: Keypair::new(),
            config: Keypair::new(),
            token_account: Keypair::new(),
        };

        let tx = Transaction::new_signed_with_payer(
            &[system_instruction::transfer(&ctx.payer.pubkey(), &pc.authority.pubkey(), 1_000_000_000)],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &pc.authority],
            ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(tx).await.unwrap();

        let ix = ghost_program::init_ix(&pc.authority.pubkey(), &pc.mint.pubkey(), &pc.config.pubkey());
        let tx = Transaction::new_signed_with_payer(
            &[ix], Some(&pc.authority.pubkey()),
            &[&pc.authority, &pc.mint, &pc.config], ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(tx).await.unwrap();

        (ctx, pc)
    }

    #[tokio::test]
    async fn test_pause_success() {
        let (mut ctx, pc) = setup().await;

        let ix = ghost_program::pause_ix(&pc.authority.pubkey(), &pc.config.pubkey());
        let tx = Transaction::new_signed_with_payer(
            &[ix], Some(&pc.authority.pubkey()), &[&pc.authority], ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(result.is_ok(), "Pause should succeed: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_unpause_success() {
        let (mut ctx, pc) = setup().await;

        // Pause first
        let pause_ix = ghost_program::pause_ix(&pc.authority.pubkey(), &pc.config.pubkey());
        let pause_tx = Transaction::new_signed_with_payer(
            &[pause_ix], Some(&pc.authority.pubkey()), &[&pc.authority], ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(pause_tx).await.unwrap();

        // Then unpause
        let unpause_ix = ghost_program::unpause_ix(&pc.authority.pubkey(), &pc.config.pubkey());
        let tx = Transaction::new_signed_with_payer(
            &[unpause_ix], Some(&pc.authority.pubkey()), &[&pc.authority], ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(result.is_ok(), "Unpause should succeed: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_emergency_pause() {
        let (mut ctx, pc) = setup().await;

        let ix = ghost_program::emergency_pause_ix(&pc.authority.pubkey(), &pc.config.pubkey());
        let tx = Transaction::new_signed_with_payer(
            &[ix], Some(&pc.authority.pubkey()), &[&pc.authority], ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(result.is_ok(), "Emergency pause should succeed: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_emergency_resume() {
        let (mut ctx, pc) = setup().await;

        // Emergency pause first
        let e_pause_ix = ghost_program::emergency_pause_ix(&pc.authority.pubkey(), &pc.config.pubkey());
        let e_pause_tx = Transaction::new_signed_with_payer(
            &[e_pause_ix], Some(&pc.authority.pubkey()), &[&pc.authority], ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(e_pause_tx).await.unwrap();

        // Emergency resume
        let resume_ix = ghost_program::emergency_resume_ix(&pc.authority.pubkey(), &pc.config.pubkey());
        let tx = Transaction::new_signed_with_payer(
            &[resume_ix], Some(&pc.authority.pubkey()), &[&pc.authority], ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(result.is_ok(), "Emergency resume should succeed: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_mint_during_pause_fails() {
        let (mut ctx, pc) = setup().await;

        // Pause
        let pause_ix = ghost_program::pause_ix(&pc.authority.pubkey(), &pc.config.pubkey());
        let pause_tx = Transaction::new_signed_with_payer(
            &[pause_ix], Some(&pc.authority.pubkey()), &[&pc.authority], ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(pause_tx).await.unwrap();

        // Try to mint while paused
        let mint_amount: u64 = 1_000_000_000_000;
        let mint_ix = ghost_program::mint_ix(
            &pc.authority.pubkey(), &pc.mint.pubkey(),
            &pc.token_account.pubkey(), &pc.config.pubkey(), mint_amount,
        );
        let tx = Transaction::new_signed_with_payer(
            &[mint_ix], Some(&pc.authority.pubkey()),
            &[&pc.authority, &pc.token_account], ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(result.is_err(), "Minting during pause should fail with Paused");
    }

    #[tokio::test]
    async fn test_transfer_during_pause_fails() {
        let (mut ctx, pc) = setup().await;

        // Pause
        let pause_ix = ghost_program::pause_ix(&pc.authority.pubkey(), &pc.config.pubkey());
        let pause_tx = Transaction::new_signed_with_payer(
            &[pause_ix], Some(&pc.authority.pubkey()), &[&pc.authority], ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(pause_tx).await.unwrap();

        // Try to transfer while paused
        let to_account = Keypair::new();
        let transfer_amount: u64 = 100_000_000_000;
        let transfer_ix = ghost_program::transfer_ix(
            &pc.authority.pubkey(), &pc.token_account.pubkey(),
            &to_account.pubkey(), &pc.mint.pubkey(), &pc.config.pubkey(), transfer_amount,
        );
        let tx = Transaction::new_signed_with_payer(
            &[transfer_ix], Some(&pc.authority.pubkey()),
            &[&pc.authority, &pc.token_account, &to_account], ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(result.is_err(), "Transfer during pause should fail with Paused");
    }

    #[tokio::test]
    async fn test_unauthorized_pause_fails() {
        let (mut ctx, pc) = setup().await;
        let unauthorized = Keypair::new();

        let fund_tx = Transaction::new_signed_with_payer(
            &[system_instruction::transfer(&ctx.payer.pubkey(), &unauthorized.pubkey(), 1_000_000_000)],
            Some(&ctx.payer.pubkey()), &[&ctx.payer, &unauthorized], ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(fund_tx).await.unwrap();

        let ix = ghost_program::pause_ix(&unauthorized.pubkey(), &pc.config.pubkey());
        let tx = Transaction::new_signed_with_payer(
            &[ix], Some(&unauthorized.pubkey()), &[&unauthorized], ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(result.is_err(), "Unauthorized pause should fail with Unauthorized");
    }
}
