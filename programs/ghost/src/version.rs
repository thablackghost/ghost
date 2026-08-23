//! GHOST Token — Program versioning.
//! Tracks the current program version for migration support.

/// Current program version. Increment on breaking changes.
pub const PROGRAM_VERSION: u8 = 1;

/// Check if a version is compatible with the current program.
pub fn is_compatible(version: u8) -> bool {
    version == PROGRAM_VERSION
}

/// Check if a version is newer than the current program.
pub fn is_newer(version: u8) -> bool {
    version > PROGRAM_VERSION
}
