//! GHOST Token — Account validation structs.
//!
//! Re-exports all `#[derive(Accounts)]` context structs used by the
//! GHOST program's instruction handlers. Each struct enforces the
//! account constraints for one instruction.

mod authorities;
mod burn;
mod initialize;
mod metadata;
mod mint;
mod transfer;

pub use authorities::*;
pub use burn::*;
pub use initialize::*;
pub use metadata::*;
pub use mint::*;
pub use transfer::*;
