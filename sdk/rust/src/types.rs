use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

/// Global on-chain configuration for the GHOST token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub authorities: Authorities,
    pub decimals: u8,
    pub total_supply: u64,
    pub paused: bool,
    pub bump: u8,
    pub _reserved: [u8; 64],
}

impl TryFrom<Vec<u8>> for GlobalConfig {
    type Error = anyhow::Error;

    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        if data.len() < 8 + 32 + 32 + 32 + 1 + 8 + 1 + 1 + 64 {
            return Err(anyhow::anyhow!("Invalid GlobalConfig data length"));
        }

        // Skip 8-byte anchor discriminant
        let offset = 8;

        // Read authorities (3 x 32 byte pubkeys)
        let mint_authority = Pubkey::new_from_array(data[offset..offset + 32].try_into().unwrap());
        let freeze_authority =
            Pubkey::new_from_array(data[offset + 32..offset + 64].try_into().unwrap());
        let admin = Pubkey::new_from_array(data[offset + 64..offset + 96].try_into().unwrap());

        let offset = offset + 96;
        let decimals = data[offset];
        let offset = offset + 1;

        let total_supply = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        let offset = offset + 8;

        let paused = data[offset] != 0;
        let offset = offset + 1;

        let bump = data[offset];
        let offset = offset + 1;

        let mut _reserved = [0u8; 64];
        _reserved.copy_from_slice(&data[offset..offset + 64]);

        Ok(GlobalConfig {
            authorities: Authorities {
                mint_authority,
                freeze_authority,
                admin,
            },
            decimals,
            total_supply,
            paused,
            bump,
            _reserved,
        })
    }
}

/// Authority key holders for the GHOST token program.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Authorities {
    pub mint_authority: Pubkey,
    pub freeze_authority: Pubkey,
    pub admin: Pubkey,
}

/// On-chain metadata for the GHOST token (Token-2022 extension).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_config_try_from() {
        let mut data = vec![0u8; 8 + 96 + 1 + 8 + 1 + 1 + 64]; // 8 + 96 + 1 + 8 + 1 + 1 + 64 = 179

        // Some well-known pubkey
        let pk = Pubkey::from_str_const("Gh0st111111111111111111111111111111111111111");
        let pk_bytes = pk.to_bytes();

        // Set authorities
        data[8..8 + 32].copy_from_slice(&pk_bytes);
        data[8 + 32..8 + 64].copy_from_slice(&pk_bytes);
        data[8 + 64..8 + 96].copy_from_slice(&pk_bytes);

        // decimals = 9
        data[8 + 96] = 9;

        // total_supply = 10_000_000_000
        data[8 + 97..8 + 97 + 8].copy_from_slice(&10_000_000_000u64.to_le_bytes());

        // paused = false
        data[8 + 97 + 8] = 0;

        // bump = 255
        data[8 + 97 + 8 + 1] = 255;

        let config = GlobalConfig::try_from(data).unwrap();
        assert_eq!(config.decimals, 9);
        assert_eq!(config.total_supply, 10_000_000_000);
        assert!(!config.paused);
        assert_eq!(config.bump, 255);
        assert_eq!(config.authorities.mint_authority, pk);
    }
}
