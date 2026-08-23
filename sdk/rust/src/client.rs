use solana_client::rpc_client::RpcClient;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::Transaction;

use crate::types::{Authorities, GlobalConfig, Metadata};

pub struct GhostClient {
    pub rpc: RpcClient,
    pub program_id: Pubkey,
    pub mint_pubkey: Pubkey,
}

impl GhostClient {
    pub fn new(rpc_url: &str, program_id: Pubkey, mint_pubkey: Pubkey) -> Self {
        Self {
            rpc: RpcClient::new(rpc_url.to_string()),
            program_id,
            mint_pubkey,
        }
    }

    /// Get a recent blockhash.
    pub fn get_recent_blockhash(&self) -> Result<Hash, anyhow::Error> {
        Ok(self.rpc.get_latest_blockhash()?)
    }

    /// Send and confirm a transaction.
    pub fn send_transaction(&self, tx: &Transaction) -> Result<Signature, anyhow::Error> {
        Ok(self.rpc.send_and_confirm_transaction(tx)?)
    }

    /// Retrieves the global configuration PDA from the program.
    pub fn get_config(&self) -> Result<GlobalConfig, anyhow::Error> {
        let (config_pda, _bump) =
            Pubkey::find_program_address(&[b"config", self.mint_pubkey.as_ref()], &self.program_id);
        let account_data = self.rpc.get_account_data(&config_pda)?;
        GlobalConfig::try_from(account_data)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize config: {}", e))
    }

    /// Retrieves the current total supply of GHOST tokens.
    pub fn get_total_supply(&self) -> Result<u64, anyhow::Error> {
        let mint_account = self.rpc.get_account_data(&self.mint_pubkey)?;
        let mint: spl_token_2022::state::Mint = spl_token_2022::state::Mint::unpack(&mint_account)?;
        Ok(mint.supply)
    }

    /// Retrieves authority information from the config PDA.
    pub fn get_authorities(&self) -> Result<Authorities, anyhow::Error> {
        let config = self.get_config()?;
        Ok(config.authorities)
    }

    /// Retrieves token metadata from on-chain storage.
    pub fn get_metadata(&self) -> Result<Metadata, anyhow::Error> {
        let (metadata_pda, _bump) = Pubkey::find_program_address(
            &[b"metadata", self.mint_pubkey.as_ref()],
            &self.program_id,
        );
        let account_data = self.rpc.get_account_data(&metadata_pda)?;

        // Deserialize metadata: first 4 bytes discriminator, then name (32 bytes),
        // symbol (10 bytes), uri (200 bytes)
        if account_data.len() < 4 + 32 + 10 + 200 {
            return Err(anyhow::anyhow!("Metadata account too small"));
        }

        let offset = 8; // skip anchor discriminator
        let name_bytes = &account_data[offset..offset + 32];
        let name = String::from_utf8_lossy(
            &name_bytes[..name_bytes
                .iter()
                .position(|&b| b == 0)
                .unwrap_or(name_bytes.len())],
        )
        .to_string();

        let offset = offset + 32;
        let symbol_bytes = &account_data[offset..offset + 10];
        let symbol = String::from_utf8_lossy(
            &symbol_bytes[..symbol_bytes
                .iter()
                .position(|&b| b == 0)
                .unwrap_or(symbol_bytes.len())],
        )
        .to_string();

        let offset = offset + 10;
        let uri_bytes = &account_data[offset..offset + 200];
        let uri = String::from_utf8_lossy(
            &uri_bytes[..uri_bytes
                .iter()
                .position(|&b| b == 0)
                .unwrap_or(uri_bytes.len())],
        )
        .to_string();

        Ok(Metadata { name, symbol, uri })
    }

    /// Checks if the protocol is currently paused.
    pub fn is_paused(&self) -> Result<bool, anyhow::Error> {
        let config = self.get_config()?;
        Ok(config.paused)
    }
}
