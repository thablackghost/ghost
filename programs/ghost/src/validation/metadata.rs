use anchor_lang::prelude::*;

use crate::errors::GhostError;
use crate::metadata::constants::{MAX_NAME_LEN, MAX_SYMBOL_LEN, MAX_URI_LEN};

/// Validates metadata name: not empty, not too long.
pub fn validate_metadata_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(GhostError::InvalidMetadata.into());
    }
    if name.len() > MAX_NAME_LEN {
        return Err(GhostError::MetadataTooLong.into());
    }
    Ok(())
}

/// Validates metadata symbol: not empty, not too long, must be uppercase ASCII alphabetic.
pub fn validate_metadata_symbol(symbol: &str) -> Result<()> {
    if symbol.is_empty() {
        return Err(GhostError::InvalidMetadata.into());
    }
    if symbol.len() > MAX_SYMBOL_LEN {
        return Err(GhostError::MetadataTooLong.into());
    }
    // All characters must be uppercase ASCII letters
    for ch in symbol.chars() {
        if !ch.is_ascii_uppercase() {
            return Err(GhostError::InvalidMetadata.into());
        }
    }
    Ok(())
}

/// Validates metadata URI: not empty, not too long, must start with http(s).
pub fn validate_metadata_uri(uri: &str) -> Result<()> {
    if uri.is_empty() {
        return Err(GhostError::InvalidUri.into());
    }
    if uri.len() > MAX_URI_LEN {
        return Err(GhostError::MetadataTooLong.into());
    }
    if !uri.starts_with("http://") && !uri.starts_with("https://") {
        return Err(GhostError::InvalidUri.into());
    }
    Ok(())
}

/// Validates that metadata is not frozen before an update operation.
pub fn validate_metadata_update_not_frozen(frozen: bool) -> Result<()> {
    if frozen {
        return Err(GhostError::MetadataFrozen.into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_metadata_name() {
        assert!(validate_metadata_name("GHOST").is_ok());
        assert!(validate_metadata_name("").is_err());
        assert!(validate_metadata_name(&"A".repeat(33)).is_err());
    }

    #[test]
    fn test_validate_metadata_symbol() {
        assert!(validate_metadata_symbol("GHOST").is_ok());
        assert!(validate_metadata_symbol("").is_err());
        assert!(validate_metadata_symbol("ghost").is_err());
        assert!(validate_metadata_symbol("GHOST123456").is_err());
    }

    #[test]
    fn test_validate_metadata_uri() {
        assert!(validate_metadata_uri("https://ghost.io/meta.json").is_ok());
        assert!(validate_metadata_uri("http://ghost.io/meta.json").is_ok());
        assert!(validate_metadata_uri("").is_err());
        assert!(validate_metadata_uri("ftp://ghost.io").is_err());
    }

    #[test]
    fn test_validate_frozen() {
        assert!(validate_metadata_update_not_frozen(false).is_ok());
        assert!(validate_metadata_update_not_frozen(true).is_err());
    }
}
