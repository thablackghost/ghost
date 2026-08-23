use anchor_lang::prelude::*;

/// Logs a general security event for auditing and monitoring purposes.
/// The event is emitted via `msg!` so it appears in transaction logs.
///
/// # Arguments
/// * `event_type` - Category of the security event (e.g., "MINT", "BURN", "AUTH_FAILURE").
/// * `details` - Human-readable description of the event.
pub fn log_security_event(event_type: &str, details: &str) {
    msg!(
        "[GHOST_SECURITY] event_type=\"{}\" details=\"{}\"",
        event_type,
        details
    );
}

/// Logs an authorization failure event.
/// Records which authority attempted an operation and what operation was denied.
/// This is critical for detecting potential attacks or misconfigurations.
///
/// # Arguments
/// * `authority` - The pubkey of the account that failed authorization.
/// * `operation` - The operation that was attempted (e.g., "mint", "admin_update").
pub fn log_authorization_failure(authority: &Pubkey, operation: &str) {
    msg!(
        "[GHOST_AUTH_FAILURE] authority=\"{}\" operation=\"{}\"",
        authority,
        operation
    );
}

/// Logs a CPI (Cross-Program Invocation) failure event.
/// Records which program's CPI call failed and the reason for the failure.
/// Helps diagnose issues with external program interactions.
///
/// # Arguments
/// * `program` - The pubkey of the program that was being called via CPI.
/// * `reason` - The reason the CPI call failed.
pub fn log_cpi_failure(program: &Pubkey, reason: &str) {
    msg!(
        "[GHOST_CPI_FAILURE] program=\"{}\" reason=\"{}\"",
        program,
        reason
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::solana_program::pubkey::Pubkey;

    #[test]
    fn test_log_security_event_does_not_panic() {
        // These functions should never panic; they only emit logs.
        log_security_event("TEST", "test event");
        log_security_event("MINT", "minted 1000 tokens");
    }

    #[test]
    fn test_log_authorization_failure_does_not_panic() {
        let key = Pubkey::new_unique();
        log_authorization_failure(&key, "mint");
        log_authorization_failure(&Pubkey::default(), "admin_update");
    }

    #[test]
    fn test_log_cpi_failure_does_not_panic() {
        let program_id = Pubkey::new_unique();
        log_cpi_failure(&program_id, "token account mismatch");
    }
}

