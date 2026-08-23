use anchor_lang::prelude::*;

/// Logs an instruction entry with its name and number of accounts.
///
/// Format: "Instruction: {name} | accounts: {count}"
pub fn log_instruction(name: &str, accounts: usize) {
    msg!("Instruction: {} | accounts: {}", name, accounts);
}

/// Logs an authority check with the authority's pubkey and role.
///
/// Format: "Authority check: {pubkey} | role: {role}"
pub fn log_authority_check(authority: &Pubkey, role: &str) {
    msg!("Authority check: {} | role: {}", authority, role);
}

/// Logs a CPI (Cross-Program Invocation) call.
///
/// Format: "CPI call: {program} -> {instruction}"
pub fn log_cpi_call(program: &str, instruction: &str) {
    msg!("CPI call: {} -> {}", program, instruction);
}

/// Logs a state change with the field name, old value, and new value.
///
/// Format: "State change: {field}: {old_value} -> {new_value}"
pub fn log_state_change(field: &str, old_value: &str, new_value: &str) {
    msg!("State change: {}: {} -> {}", field, old_value, new_value);
}

/// Logs a numeric state change using u64 values.
pub fn log_state_change_u64(field: &str, old_value: u64, new_value: u64) {
    msg!("State change: {}: {} -> {}", field, old_value, new_value);
}

/// Logs a boolean state change.
pub fn log_state_change_bool(field: &str, old_value: bool, new_value: bool) {
    msg!("State change: {}: {} -> {}", field, old_value, new_value);
}

/// Logs a metadata operation.
pub fn log_metadata_operation(operation: &str, name: &str, symbol: &str) {
    msg!("Metadata {}: name={}, symbol={}", operation, name, symbol);
}

/// Logs a supply-related event.
pub fn log_supply_event(operation: &str, amount: u64, pre_supply: u64, post_supply: u64) {
    msg!(
        "Supply {}: amount={}, supply: {} -> {}",
        operation,
        amount,
        pre_supply,
        post_supply
    );
}
