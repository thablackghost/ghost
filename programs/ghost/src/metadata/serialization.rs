use anchor_lang::prelude::*;

use crate::errors::GhostError;

/// Serializes token metadata into a JSON string.
///
/// Produces a JSON object with keys: name, symbol, description, image.
/// All string values are properly escaped.
pub fn serialize_metadata_json(
    name: &str,
    symbol: &str,
    description: &str,
    image_uri: &str,
) -> Result<String> {
    let json = format!(
        r#"{{"name":"{}","symbol":"{}","description":"{}","image":"{}"}}"#,
        escape_json(name),
        escape_json(symbol),
        escape_json(description),
        escape_json(image_uri),
    );

    Ok(json)
}

/// Deserializes JSON metadata string back into its components.
///
/// Returns a tuple of (name, symbol, description, image_uri).
/// Performs minimal JSON parsing suitable for the well-known structure.
pub fn deserialize_metadata_json(json: &str) -> Result<(String, String, String, String)> {
    let extract = |key: &str| -> Result<String> {
        let search = format!(r#""{}":""#, key);
        let start = json
            .find(&search)
            .ok_or::<Error>(GhostError::InvalidMetadata.into())?
            + search.len();

        let end = json[start..]
            .find(r#"""#)
            .ok_or::<Error>(GhostError::InvalidMetadata.into())?;

        Ok(unescape_json(&json[start..start + end]))
    };

    let name = extract("name")?;
    let symbol = extract("symbol")?;
    let description = extract("description")?;
    let image = extract("image")?;

    Ok((name, symbol, description, image))
}

/// Creates a combined metadata URI string from token metadata components.
pub fn create_token_metadata_uri(name: &str, symbol: &str, uri: &str) -> String {
    format!("ghost://token/{}/{}?uri={}", name, symbol, uri)
}

// Helpers

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

fn unescape_json(s: &str) -> String {
    s.replace("\\\"", "\"")
        .replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\t", "\t")
        .replace("\\\\", "\\")
}
