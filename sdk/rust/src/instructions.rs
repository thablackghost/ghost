use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::system_program;

use crate::types::{Authorities, GlobalConfig};

/// Builds the initialize instruction for the GHOST token program.
pub fn build_initialize_ix(
    payer: &Pubkey,
    mint: &Pubkey,
    authorities: &Authorities,
    config: &GlobalConfig,
) -> Instruction {
    let (config_pda, _) = Pubkey::find_program_address(
        &[b"config", mint.as_ref()],
        &Pubkey::from_str_const("Gh0st111111111111111111111111111111111111111"),
    );

    let (metadata_pda, _) = Pubkey::find_program_address(
        &[b"metadata", mint.as_ref()],
        &Pubkey::from_str_const("Gh0st111111111111111111111111111111111111111"),
    );

    let mut data = Vec::new();
    // 8-byte anchor discriminator for "initialize"
    data.extend_from_slice(&anchor_discriminator("initialize"));
    // Serialize authorities
    data.extend_from_slice(&authorities.mint_authority.to_bytes());
    data.extend_from_slice(&authorities.freeze_authority.to_bytes());
    data.extend_from_slice(&authorities.admin.to_bytes());
    data.extend_from_slice(&config.paused.to_le_bytes());
    // Serialize initial supply
    data.extend_from_slice(&10_000_000_000u64.to_le_bytes());

    Instruction {
        program_id: Pubkey::from_str_const("Gh0st111111111111111111111111111111111111111"),
        accounts: vec![
            AccountMeta::new(*payer, true),
            AccountMeta::new(*mint, false),
            AccountMeta::new(config_pda, false),
            AccountMeta::new(metadata_pda, false),
            AccountMeta::new_readonly(spl_token_2022::id(), false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
        ],
        data,
    }
}

/// Builds a mint instruction for the GHOST token program.
pub fn build_mint_ix(
    mint_authority: &Pubkey,
    mint: &Pubkey,
    recipient: &Pubkey,
    amount: u64,
) -> Instruction {
    let ata = crate::utils::get_ata_address(recipient, mint);

    let mut data = Vec::new();
    data.extend_from_slice(&anchor_discriminator("mint"));
    data.extend_from_slice(&amount.to_le_bytes());

    Instruction {
        program_id: Pubkey::from_str_const("Gh0st111111111111111111111111111111111111111"),
        accounts: vec![
            AccountMeta::new_readonly(*mint_authority, true),
            AccountMeta::new(*mint, false),
            AccountMeta::new(ata, false),
            AccountMeta::new_readonly(*recipient, false),
            AccountMeta::new_readonly(spl_token_2022::id(), false),
            AccountMeta::new_readonly(spl_associated_token_account::id(), false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
        ],
        data,
    }
}

/// Builds a burn instruction for the GHOST token program.
pub fn build_burn_ix(
    owner: &Pubkey,
    mint: &Pubkey,
    token_account: &Pubkey,
    amount: u64,
) -> Instruction {
    let mut data = Vec::new();
    data.extend_from_slice(&anchor_discriminator("burn"));
    data.extend_from_slice(&amount.to_le_bytes());

    Instruction {
        program_id: Pubkey::from_str_const("Gh0st111111111111111111111111111111111111111"),
        accounts: vec![
            AccountMeta::new_readonly(*owner, true),
            AccountMeta::new(*token_account, false),
            AccountMeta::new(*mint, false),
            AccountMeta::new_readonly(spl_token_2022::id(), false),
        ],
        data,
    }
}

/// Builds a transfer instruction for the GHOST token program.
pub fn build_transfer_ix(
    owner: &Pubkey,
    from: &Pubkey,
    to: &Pubkey,
    mint: &Pubkey,
    amount: u64,
) -> Instruction {
    let mut data = Vec::new();
    data.extend_from_slice(&anchor_discriminator("transfer"));
    data.extend_from_slice(&amount.to_le_bytes());

    Instruction {
        program_id: Pubkey::from_str_const("Gh0st111111111111111111111111111111111111111"),
        accounts: vec![
            AccountMeta::new_readonly(*owner, true),
            AccountMeta::new(*from, false),
            AccountMeta::new(*to, false),
            AccountMeta::new_readonly(*mint, false),
            AccountMeta::new_readonly(spl_token_2022::id(), false),
        ],
        data,
    }
}

/// Builds an update_metadata instruction for the GHOST token program.
pub fn build_update_metadata_ix(
    authority: &Pubkey,
    mint: &Pubkey,
    name: &str,
    symbol: &str,
    uri: &str,
) -> Instruction {
    let (metadata_pda, _) = Pubkey::find_program_address(
        &[b"metadata", mint.as_ref()],
        &Pubkey::from_str_const("Gh0st111111111111111111111111111111111111111"),
    );

    let mut data = Vec::new();
    data.extend_from_slice(&anchor_discriminator("update_metadata"));

    // Serialize name (32 bytes padded)
    let mut name_bytes = [0u8; 32];
    let name_src = name.as_bytes();
    let copy_len = name_src.len().min(32);
    name_bytes[..copy_len].copy_from_slice(&name_src[..copy_len]);
    data.extend_from_slice(&name_bytes);

    // Serialize symbol (10 bytes padded)
    let mut symbol_bytes = [0u8; 10];
    let symbol_src = symbol.as_bytes();
    let copy_len = symbol_src.len().min(10);
    symbol_bytes[..copy_len].copy_from_slice(&symbol_src[..copy_len]);
    data.extend_from_slice(&symbol_bytes);

    // Serialize URI (200 bytes padded)
    let mut uri_bytes = [0u8; 200];
    let uri_src = uri.as_bytes();
    let copy_len = uri_src.len().min(200);
    uri_bytes[..copy_len].copy_from_slice(&uri_src[..copy_len]);
    data.extend_from_slice(&uri_bytes);

    Instruction {
        program_id: Pubkey::from_str_const("Gh0st111111111111111111111111111111111111111"),
        accounts: vec![
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new(metadata_pda, false),
            AccountMeta::new_readonly(*mint, false),
        ],
        data,
    }
}

/// Builds a pause instruction for the GHOST token program.
pub fn build_pause_ix(admin: &Pubkey) -> Instruction {
    let data = anchor_discriminator("pause");

    Instruction {
        program_id: Pubkey::from_str_const("Gh0st111111111111111111111111111111111111111"),
        accounts: vec![AccountMeta::new_readonly(*admin, true)],
        data: data.to_vec(),
    }
}

/// Builds an unpause instruction for the GHOST token program.
pub fn build_unpause_ix(admin: &Pubkey) -> Instruction {
    let data = anchor_discriminator("unpause");

    Instruction {
        program_id: Pubkey::from_str_const("Gh0st111111111111111111111111111111111111111"),
        accounts: vec![AccountMeta::new_readonly(*admin, true)],
        data: data.to_vec(),
    }
}

/// Computes the 8-byte Anchor discriminator from an instruction name.
fn anchor_discriminator(name: &str) -> [u8; 8] {
    let preimage = format!("global:{}", name);
    let hash = solana_sdk::hash::hash(preimage.as_bytes());
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&hash.to_bytes()[..8]);
    discriminator
}
