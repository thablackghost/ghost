use anchor_lang::prelude::*;

declare_id!("DK9MPoDFDPyCZG16ESkY6mUPrpKJaeGVpmEjwvWghost");

pub mod config;
pub mod constants;
pub mod errors;
pub mod events;
pub mod state;
pub mod version;

#[allow(ambiguous_glob_reexports)]
pub mod instructions {
    pub mod approve;
    pub mod burn;
    pub mod close_config;
    pub mod configure;
    pub mod emergency_pause;
    pub mod emergency_resume;
    pub mod initialize;
    pub mod initialize_authority;
    pub mod initialize_config;
    pub mod initialize_metadata;
    pub mod migrate;
    pub mod mint;
    pub mod mint_batch;
    pub mod pause;
    pub mod revoke;
    pub mod rotate_authority;
    pub mod transfer;
    pub mod unpause;
    pub mod update_authority;
    pub mod update_metadata;

    pub use approve::*;
    pub use burn::*;
    pub use close_config::*;
    pub use configure::*;
    pub use emergency_pause::*;
    pub use emergency_resume::*;
    pub use initialize::*;
    pub use initialize_authority::*;
    pub use initialize_config::*;
    pub use initialize_metadata::*;
    pub use migrate::*;
    pub use mint::*;
    pub use mint_batch::*;
    pub use pause::*;
    pub use revoke::*;
    pub use rotate_authority::*;
    pub use transfer::*;
    pub use unpause::*;
    pub use update_authority::*;
    pub use update_metadata::*;
}

use instructions::*;

#[program]
pub mod ghost {
    use super::*;

    pub fn initialize(ctx: Context<InitializeAccounts>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }
    pub fn initialize_authority(ctx: Context<InitializeAuthorityAccounts>) -> Result<()> {
        instructions::initialize_authority::handler(ctx)
    }
    pub fn initialize_metadata(ctx: Context<InitializeMetadataAccounts>) -> Result<()> {
        instructions::initialize_metadata::handler(ctx)
    }
    pub fn initialize_config(ctx: Context<InitializeConfigAccounts>) -> Result<()> {
        instructions::initialize_config::handler(ctx)
    }
    pub fn mint_tokens(ctx: Context<MintAccounts>, amount: u64) -> Result<()> {
        instructions::mint::handler(ctx, amount)
    }
    pub fn mint_batch(
        ctx: Context<MintBatchAccounts>,
        recipients_and_amounts: Vec<(Pubkey, u64)>,
    ) -> Result<()> {
        instructions::mint_batch::handler(ctx, recipients_and_amounts)
    }
    pub fn burn_tokens(ctx: Context<BurnAccounts>, amount: u64) -> Result<()> {
        instructions::burn::handler(ctx, amount)
    }
    pub fn transfer_tokens(ctx: Context<TransferAccounts>, amount: u64) -> Result<()> {
        instructions::transfer::handler(ctx, amount)
    }
    pub fn approve(ctx: Context<ApproveAccounts>, amount: u64) -> Result<()> {
        instructions::approve::handler(ctx, amount)
    }
    pub fn revoke(ctx: Context<RevokeAccounts>) -> Result<()> {
        instructions::revoke::handler(ctx)
    }
    pub fn update_metadata(
        ctx: Context<UpdateMetadataAccounts>,
        new_name: String,
        new_symbol: String,
        new_uri: String,
    ) -> Result<()> {
        instructions::update_metadata::handler(ctx, new_name, new_symbol, new_uri)
    }
    pub fn update_authority(
        ctx: Context<UpdateAuthorityAccounts>,
        authority_type: u8,
        new_authority: Pubkey,
    ) -> Result<()> {
        instructions::update_authority::handler(ctx, authority_type, new_authority)
    }
    pub fn rotate_authority(
        ctx: Context<RotateAuthorityAccounts>,
        action: u8,
        new_authority: Pubkey,
    ) -> Result<()> {
        instructions::rotate_authority::handler(ctx, action, new_authority)
    }
    pub fn pause(ctx: Context<PauseAccounts>) -> Result<()> {
        instructions::pause::handler(ctx)
    }
    pub fn unpause(ctx: Context<UnpauseAccounts>) -> Result<()> {
        instructions::unpause::handler(ctx)
    }
    pub fn emergency_pause(ctx: Context<EmergencyPauseAccounts>) -> Result<()> {
        instructions::emergency_pause::handler(ctx)
    }
    pub fn emergency_resume(ctx: Context<EmergencyResumeAccounts>) -> Result<()> {
        instructions::emergency_resume::handler(ctx)
    }
    pub fn configure(ctx: Context<ConfigureAccounts>, field: u8, value: Vec<u8>) -> Result<()> {
        instructions::configure::handler(ctx, field, value)
    }
    pub fn migrate(ctx: Context<MigrateAccounts>, target_version: u8) -> Result<()> {
        instructions::migrate::handler(ctx, target_version)
    }
    pub fn close_config(ctx: Context<CloseConfigAccounts>) -> Result<()> {
        instructions::close_config::handler(ctx)
    }
}
