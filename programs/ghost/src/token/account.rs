use anchor_lang::prelude::*;
use crate::errors::GhostError;

/// Retrieves the balance of a SPL Token-2022 account.
///
/// Deserializes the token account data and returns the `amount` field,
/// which represents the current balance in base units.
///
/// # Arguments
/// * `account` - The token account to query.
///
/// # Returns
/// The balance as `u64`, or an error if the account data cannot be deserialized.
pub fn get_token_account_balance(account: &AccountInfo) -> Result<u64> {
    require!(
        account.owner == &spl_token_2022::ID,
        GhostError::InvalidTokenAccountOwner
    );

    let data = account.data.borrow();
    let token_account = spl_token_2022::extension::StateWithExtensions::<
        spl_token_2022::state::Account,
    >::unpack(&data)
        .map_err(|_| GhostError::InvalidTokenAccountData)?;

    Ok(token_account.base.amount)
}

/// Validates that a token account exists and is properly initialized.
///
/// Checks that:
/// 1. The account is owned by SPL Token-2022.
/// 2. The account data can be deserialized (meaning it's initialized).
/// 3. The account's mint field is set (not the default/zero pubkey).
pub fn validate_token_account_exists(account: &AccountInfo) -> Result<()> {
    require!(
        account.owner == &spl_token_2022::ID,
        GhostError::InvalidTokenAccountOwner
    );

    let data = account.data.borrow();
    let token_account = spl_token_2022::extension::StateWithExtensions::<
        spl_token_2022::state::Account,
    >::unpack(&data)
        .map_err(|_| GhostError::InvalidTokenAccountData)?;

    // A valid token account must have a non-zero mint.
    require!(
        token_account.base.mint != Pubkey::default(),
        GhostError::TokenAccountNotInitialized
    );

    Ok(())
}

/// Creates an Associated Token Account (ATA) if one does not already exist.
///
/// This is a template function that demonstrates the pattern for creating an ATA.
/// In a real program, this would be called via CPI to the Associated Token Account
/// program.
///
/// # Arguments
/// * `payer` - The account paying for the ATA creation (lamports for rent).
/// * `owner` - The owner of the new token account.
/// * `mint` - The mint of the token account.
/// * `ata` - The ATA account to create (must be a PDA derived from owner + mint).
/// * `token_program` - The SPL Token-2022 program.
/// * `associated_token_program` - The ATA program.
/// * `system_program` - The System program.
/// * `seeds` - Optional seeds if the payer is a PDA.
pub fn create_ata_if_needed<'info>(
    payer: &AccountInfo<'info>,
    owner: &AccountInfo<'info>,
    mint: &AccountInfo<'info>,
    ata: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    associated_token_program: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    seeds: Option<&[&[&[u8]]]>,
) -> Result<()> {
    // Check if the ATA already exists.
    if ata.owner == &spl_token_2022::ID {
        // Account already exists, verify it's the correct token account.
        let data = ata.data.borrow();
        if let Ok(token_account) =
            spl_token_2022::extension::StateWithExtensions::<
                spl_token_2022::state::Account,
            >::unpack(&data)
        {
            if token_account.base.mint == mint.key()
                && token_account.base.owner == owner.key()
            {
                msg!("ATA already exists: {}", ata.key());
                return Ok(());
            }
        }
    }

    // Determine the ATA address for SPL Token-2022.
    let expected_ata = spl_associated_token_account::get_associated_token_address_with_program_id(
        &owner.key(),
        &mint.key(),
        &spl_token_2022::ID,
    );

    require!(
        ata.key() == expected_ata,
        GhostError::InvalidAtaAddress
    );

    // Build CPI to create the ATA.
    let ix = spl_associated_token_account::instruction::create_associated_token_account_idempotent(
        &payer.key(),
        &owner.key(),
        &mint.key(),
        &spl_token_2022::ID,
    );

    let account_infos = vec![
        payer.clone(),
        ata.clone(),
        owner.clone(),
        mint.clone(),
        system_program.clone(),
        token_program.clone(),
        associated_token_program.clone(),
    ];

    match seeds {
        Some(signer_seeds) => {
            solana_program::program::invoke_signed(&ix, &account_infos, signer_seeds)?;
        }
        None => {
            solana_program::program::invoke(&ix, &account_infos)?;
        }
    }

    msg!("ATA created: {} for owner: {}", ata.key(), owner.key());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::solana_program::pubkey::Pubkey;

    #[test]
    fn test_get_token_account_balance_wrong_owner() {
        let key = Pubkey::new_unique();
        let mut lamports = 0u64;
        let mut data: Vec<u8> = vec![0; 165];
        let owner = Pubkey::new_unique(); // Not Token-2022

        let account_info = AccountInfo::new(
            &key,
            false,
            false,
            &mut lamports,
            &mut data,
            &owner,
            false,
            0,
        );

        assert!(get_token_account_balance(&account_info).is_err());
    }

    #[test]
    fn test_validate_token_account_exists_invalid_data() {
        let key = Pubkey::new_unique();
        let mut lamports = 0u64;
        let mut data: Vec<u8> = vec![0; 8]; // Too small to deserialize
        let owner = spl_token_2022::ID;

        let account_info = AccountInfo::new(
            &key,
            false,
            false,
            &mut lamports,
            &mut data,
            &owner,
            false,
            0,
        );

        assert!(validate_token_account_exists(&account_info).is_err());
    }
}
