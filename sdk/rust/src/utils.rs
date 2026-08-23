use solana_sdk::pubkey::Pubkey;

/// Returns the Associated Token Account (ATA) address for a wallet and mint.
pub fn get_ata_address(wallet: &Pubkey, mint: &Pubkey) -> Pubkey {
    spl_associated_token_account::get_associated_token_address_with_program_id(
        wallet,
        mint,
        &spl_token_2022::id(),
    )
}

/// Converts a human-readable amount (e.g. 100.5) to raw token units (e.g. 100_500_000_000 for 9 decimals).
pub fn human_to_raw(amount_human: f64, decimals: u8) -> u64 {
    let multiplier = 10u64.pow(decimals as u32) as f64;
    (amount_human * multiplier) as u64
}

/// Converts raw token units to a human-readable f64.
pub fn raw_to_human(raw: u64, decimals: u8) -> f64 {
    let divisor = 10u64.pow(decimals as u32) as f64;
    raw as f64 / divisor
}

/// Formats a raw amount into a human-readable string with proper decimal places.
pub fn format_amount(raw: u64, decimals: u8) -> String {
    let divisor = 10u64.pow(decimals as u32);
    let integer_part = raw / divisor;
    let fractional_part = raw % divisor;

    if decimals == 0 {
        return integer_part.to_string();
    }

    let frac_str = format!("{:0width$}", fractional_part, width = decimals as usize);
    // Trim trailing zeros
    let trimmed_frac = frac_str.trim_end_matches('0');
    if trimmed_frac.is_empty() {
        integer_part.to_string()
    } else {
        format!("{}.{}", integer_part, trimmed_frac)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ata_address() {
        let wallet = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let ata = get_ata_address(&wallet, &mint);
        assert_ne!(ata, wallet);
        assert_ne!(ata, mint);
    }

    #[test]
    fn test_human_to_raw() {
        assert_eq!(human_to_raw(1.0, 9), 1_000_000_000);
        assert_eq!(human_to_raw(0.5, 9), 500_000_000);
        assert_eq!(human_to_raw(100.0, 6), 100_000_000);
    }

    #[test]
    fn test_raw_to_human() {
        assert!((raw_to_human(1_000_000_000, 9) - 1.0).abs() < 0.00001);
        assert!((raw_to_human(500_000_000, 9) - 0.5).abs() < 0.00001);
    }

    #[test]
    fn test_format_amount() {
        assert_eq!(format_amount(1_000_000_000, 9), "1");
        assert_eq!(format_amount(1_500_000_000, 9), "1.5");
        assert_eq!(format_amount(100_000_000, 6), "100");
        assert_eq!(format_amount(0, 9), "0");
    }
}
