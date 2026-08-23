//! GHOST Token — State module.
//!
//! Re-exports all on-chain account state structs used by the GHOST program.
//! Each state struct is defined in its own submodule for clarity and
//! auditability.

mod authority_state;
mod global_config;
mod metadata_state;
mod migration_state;
mod mint_config;
mod pause_state;
mod version_state;

pub use authority_state::*;
pub use global_config::*;
pub use metadata_state::*;
pub use migration_state::*;
pub use mint_config::*;
pub use pause_state::*;
pub use version_state::*;

/// Represents the type of supply operation being performed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupplyOperation {
    Mint,
    Burn,
    Transfer,
}
