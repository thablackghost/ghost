use anchor_lang::prelude::*;

use super::constants::{MAX_DESCRIPTION_LEN, MAX_NAME_LEN, MAX_SYMBOL_LEN, MAX_URI_LEN};
use crate::errors::GhostError;

/// Validates that the name is not empty, fits within the max length, and is valid UTF-8.
pub fn validate_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(GhostError::InvalidMetadata.into());
    }
    if name.len() > MAX_NAME_LEN {
        return Err(GhostError::MetadataTooLong.into());
    }
    // Rust strings are always valid UTF-8; this is a compile-time guarantee.
    // Explicit check included for clarity.
    if std::str::from_utf8(name.as_bytes()).is_err() {
        return Err(GhostError::InvalidMetadata.into());
    }
    Ok(())
}

/// Validates that the symbol is not empty, fits within the max length, and is uppercase.
pub fn validate_symbol(symbol: &str) -> Result<()> {
    if symbol.is_empty() {
        return Err(GhostError::InvalidMetadata.into());
    }
    if symbol.len() > MAX_SYMBOL_LEN {
        return Err(GhostError::MetadataTooLong.into());
    }
    if symbol
        .chars()
        .any(|c| c.is_alphabetic() && !c.is_uppercase())
    {
        return Err(GhostError::InvalidMetadata.into());
    }
    Ok(())
}

/// Validates that the URI is not empty, fits within the max length, and starts with http or https.
pub fn validate_uri(uri: &str) -> Result<()> {
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

/// Validates that the description fits within the max length.
pub fn validate_description(desc: &str) -> Result<()> {
    if desc.len() > MAX_DESCRIPTION_LEN {
        return Err(GhostError::MetadataTooLong.into());
    }
    Ok(())
}

/// Validates all metadata fields together.
pub fn validate_metadata(name: &str, symbol: &str, uri: &str) -> Result<()> {
    validate_name(name)?;
    validate_symbol(symbol)?;
    validate_uri(uri)?;
    Ok(())
}
