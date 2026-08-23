//! GHOST Token — Authority and configuration account contexts.
//!
//! Defines the `#[derive(Accounts)]` structs for authority management,
//! pause/resume, configuration, migration, and close-config instructions.

use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::{
    GlobalConfig, AuthorityState, PauseState, MigrationState, VersionState,
};

// ── UpdateAuthority ────────────────────────────────────────────────────────────

/// Accounts for immediately updating an authority (admin-only, no timelock).
///
/// Used by the admin to directly set admin, metadata, or emergency
/// authority pubkeys. For timelocked rotation, use `RotateAuthority`.
#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    /// The admin authority. Must sign.
    pub admin: Signer<'info>,

    /// The global config PDA (mutable — authority fields updated).
    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,
}

// ── RotateAuthority ────────────────────────────────────────────────────────────

/// Accounts for proposing or accepting a timelocked authority rotation.
///
/// Rotation lifecycle: propose → wait (timelock) → accept (or cancel).
/// Only one rotation can be pending at a time.
#[derive(Accounts)]
pub struct RotateAuthority<'info> {
    /// The current authority for the role being rotated. Must sign.
    pub current_authority: Signer<'info>,

    /// The authority state PDA (mutable — rotation proposal tracked here).
    #[account(
        mut,
        seeds = [AUTHORITY_SEED],
        bump = authority_state.bump,
    )]
    pub authority_state: Account<'info, AuthorityState>,

    /// The global config PDA (read-only for authority validation).
    #[account(
        seeds = [CONFIG_SEED],
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,
}

// ── PauseContext ───────────────────────────────────────────────────────────────

/// Accounts for pausing or unpausing the program (admin-only).
///
/// Used by both `pause` and `unpause` instructions. The handler
/// determines which operation to perform.
#[derive(Accounts)]
pub struct PauseContext<'info> {
    /// The admin authority. Must sign.
    pub admin: Signer<'info>,

    /// The global config PDA (mutable — pause flag updated).
    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    /// The pause state PDA (mutable).
    #[account(
        mut,
        seeds = [PAUSE_SEED],
        bump = pause_state.bump,
    )]
    pub pause_state: Account<'info, PauseState>,
}

// ── EmergencyPauseContext ──────────────────────────────────────────────────────

/// Accounts for emergency-pausing or emergency-resuming the program.
///
/// The emergency authority can bypass all other gates to freeze
/// operations instantly.
#[derive(Accounts)]
pub struct EmergencyPauseContext<'info> {
    /// The emergency authority. Must sign.
    pub emergency_authority: Signer<'info>,

    /// The global config PDA (mutable — emergency pause flag updated).
    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    /// The pause state PDA (mutable).
    #[account(
        mut,
        seeds = [PAUSE_SEED],
        bump = pause_state.bump,
    )]
    pub pause_state: Account<'info, PauseState>,
}

// ── Configure ──────────────────────────────────────────────────────────────────

/// Accounts for updating program configuration parameters.
///
/// Allows the admin to change max name/symbol/URI lengths and
/// other tunable parameters without a full program upgrade.
#[derive(Accounts)]
pub struct Configure<'info> {
    /// The admin authority. Must sign.
    pub admin: Signer<'info>,

    /// The global config PDA (mutable — configuration fields updated).
    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,
}

// ── Migrate ────────────────────────────────────────────────────────────────────

/// Accounts for migration operations.
///
/// Transitions the program state from one version to another,
/// applying any necessary data transformations.
#[derive(Accounts)]
pub struct Migrate<'info> {
    /// The admin authority. Must sign.
    pub admin: Signer<'info>,

    /// The global config PDA (read-only for validation).
    #[account(
        seeds = [CONFIG_SEED],
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    /// The migration state PDA (mutable).
    #[account(
        mut,
        seeds = [MIGRATION_SEED],
        bump = migration_state.bump,
    )]
    pub migration_state: Account<'info, MigrationState>,

    /// The version state PDA (mutable — version updated on completion).
    #[account(
        mut,
        seeds = [VERSION_SEED],
        bump = version_state.bump,
    )]
    pub version_state: Account<'info, VersionState>,

    /// System program (for potential account resizing during migration).
    pub system_program: Program<'info, System>,
}

// ── CloseConfig ────────────────────────────────────────────────────────────────

/// Accounts for closing the config account and recovering rent.
///
/// This is a nuclear option that should only be used during
/// emergency decommissioning or a full program redeployment.
#[derive(Accounts)]
pub struct CloseConfig<'info> {
    /// The admin authority. Must sign.
    pub admin: Signer<'info>,

    /// The global config PDA (to be closed, rent returned to admin).
    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump = global_config.bump,
        close = admin,
    )]
    pub global_config: Account<'info, GlobalConfig>,
}
