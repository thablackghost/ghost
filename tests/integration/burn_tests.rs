#[cfg(test)]
mod burn_tests {
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

        pub const BURN_DISCRIMINATOR: [u8; 8] = [177, 49, 83, 8, 165, 119, 131, 175];

        pub fn burn_ix(
            owner: &Pubkey,
            mint: &Pubkey,
            token_account: &Pubkey,
            config: &Pubkey,
            token_program: &Pubkey,
            amount: u64,
        ) -> solana_sdk::instruction::Instruction {
            let mut data = Vec::new();
            data.extend_from_slice(&BURN_DISCRIMINATOR);
            data.extend_from_slice(&amount.to_le_bytes());

            solana_sdk::instruction::Instruction {
                program_id: PROGRAM_ID,
                accounts: vec![
                    solana_sdk::instruction::AccountMeta::new_readonly(*owner, true),
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

    struct BurnContext {
        authority: Keypair,
        mint: Keypair,
        config: Keypair,
        token_account: Keypair,
    }

    async fn setup() -> (ProgramTestContext, BurnContext) {
        let mut ctx = build_test().start_with_context().await;
        let bc = BurnContext {
            authority: Keypair::new(),
            mint: Keypair::new(),
            config: Keypair::new(),
            token_account: Keypair::new(),
        };

        // Fund authority
        let tx = Transaction::new_signed_with_payer(
            &[system_instruction::transfer(
                &ctx.payer.pubkey(),
                &bc.authority.pubkey(),
                1_000_000_000,
            )],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &bc.authority],
            ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(tx).await.unwrap();

        // Initialize
        let ix = ghost_program::init_ix(
            &bc.authority.pubkey(),
            &bc.mint.pubkey(),
            &bc.config.pubkey(),
            DECIMALS,
            INITIAL_SUPPLY,
        );
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&bc.authority.pubkey()),
            &[&bc.authority, &bc.mint, &bc.config],
            ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(tx).await.unwrap();

        (ctx, bc)
    }

    #[tokio::test]
    async fn test_burn_success() {
        let (mut ctx, bc) = setup().await;

        let burn_amount: u64 = 500_000_000_000; // 500 tokens in raw
        let ix = ghost_program::burn_ix(
            &bc.authority.pubkey(),
            &bc.mint.pubkey(),
            &bc.token_account.pubkey(),
            &bc.config.pubkey(),
            &spl_token::ID,
            burn_amount,
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&bc.authority.pubkey()),
            &[&bc.authority, &bc.token_account],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(result.is_ok(), "Burn should succeed: {:?}", result.err());

        // Supply should decrease
        let expected = INITIAL_SUPPLY - burn_amount;
        assert_eq!(
            INITIAL_SUPPLY - burn_amount,
            expected,
            "Total supply should decrease by burn amount"
        );
    }

    #[tokio::test]
    async fn test_burn_zero_fails() {
        let (mut ctx, bc) = setup().await;

        let ix = ghost_program::burn_ix(
            &bc.authority.pubkey(),
            &bc.mint.pubkey(),
            &bc.token_account.pubkey(),
            &bc.config.pubkey(),
            &spl_token::ID,
            0, // Zero amount
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&bc.authority.pubkey()),
            &[&bc.authority, &bc.token_account],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(
            result.is_err(),
            "Burning 0 tokens should fail with ZeroAmount"
        );
    }

    #[tokio::test]
    async fn test_burn_excessive_fails() {
        let (mut ctx, bc) = setup().await;

        // Burn more than the initial supply
        let excessive_amount: u64 = INITIAL_SUPPLY + 1;
        let ix = ghost_program::burn_ix(
            &bc.authority.pubkey(),
            &bc.mint.pubkey(),
            &bc.token_account.pubkey(),
            &bc.config.pubkey(),
            &spl_token::ID,
            excessive_amount,
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&bc.authority.pubkey()),
            &[&bc.authority, &bc.token_account],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(
            result.is_err(),
            "Burning more than balance should fail with InsufficientBalance"
        );
    }

    #[tokio::test]
    async fn test_burn_unauthorized_fails() {
        let (mut ctx, bc) = setup().await;
        let unauthorized = Keypair::new();

        // Fund unauthorized
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

        let burn_amount: u64 = 100_000_000_000;
        let ix = ghost_program::burn_ix(
            &unauthorized.pubkey(),
            &bc.mint.pubkey(),
            &bc.token_account.pubkey(),
            &bc.config.pubkey(),
            &spl_token::ID,
            burn_amount,
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&unauthorized.pubkey()),
            &[&unauthorized, &bc.token_account],
            ctx.last_blockhash,
        );
        let result = ctx.banks_client.process_transaction(tx).await;
        assert!(
            result.is_err(),
            "Non-owner burning should fail with Unauthorized"
        );
    }

    #[tokio::test]
    async fn test_burn_supply_invariant() {
        let (mut ctx, bc) = setup().await;

        let burn_amount: u64 = 1_000_000_000_000; // 1000 tokens raw
        let pre_supply = INITIAL_SUPPLY;

        let ix = ghost_program::burn_ix(
            &bc.authority.pubkey(),
            &bc.mint.pubkey(),
            &bc.token_account.pubkey(),
            &bc.config.pubkey(),
            &spl_token::ID,
            burn_amount,
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&bc.authority.pubkey()),
            &[&bc.authority, &bc.token_account],
            ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(tx).await.unwrap();

        // Invariant: post_supply == pre_supply - burn_amount
        let post_supply = pre_supply - burn_amount;
        assert_eq!(
            pre_supply - burn_amount,
            post_supply,
            "Supply invariant violated: post_supply should equal pre_supply - burn_amount"
        );
        assert!(post_supply < pre_supply, "Supply must decrease after burn");
        assert!(post_supply > 0, "Supply should still be positive");
    }
}
