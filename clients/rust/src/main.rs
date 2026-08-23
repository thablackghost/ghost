use clap::{Parser, Subcommand};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::read_keypair_file;
use solana_sdk::transaction::Transaction;

use ghost_sdk::{
    client::GhostClient,
    instructions::{
        build_burn_ix, build_initialize_ix, build_mint_ix, build_pause_ix, build_transfer_ix,
        build_unpause_ix, build_update_metadata_ix,
    },
    types::{Authorities, GlobalConfig},
    utils::human_to_raw,
};

const DEFAULT_RPC: &str = "https://api.devnet.solana.com";

/// GHOST Token CLI — manage your GHOST tokens on Solana
#[derive(Parser)]
#[command(name = "ghost", version = "0.1.0", about = "GHOST Token CLI")]
struct Cli {
    /// RPC URL for Solana
    #[arg(short, long, default_value = DEFAULT_RPC)]
    rpc: String,

    /// Path to keypair file
    #[arg(short, long, default_value = "~/.config/solana/id.json")]
    keypair: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the GHOST token program
    Init {
        /// The mint pubkey for the GHOST token
        #[arg(long)]
        mint: String,
    },

    /// Mint GHOST tokens to a recipient
    Mint {
        /// Recipient wallet address
        #[arg(long)]
        recipient: String,

        /// Amount in human-readable form (e.g. "1000.5")
        #[arg(long)]
        amount: f64,
    },

    /// Burn GHOST tokens from your token account
    Burn {
        /// Your token account address
        #[arg(long)]
        token_account: String,

        /// Amount in human-readable form (e.g. "1000.5")
        #[arg(long)]
        amount: f64,
    },

    /// Transfer GHOST tokens to another wallet
    Transfer {
        /// Sender's token account
        #[arg(long)]
        from: String,

        /// Recipient's token account
        #[arg(long)]
        to: String,

        /// Amount in human-readable form (e.g. "1000.5")
        #[arg(long)]
        amount: f64,
    },

    /// Show GHOST token info
    Info,

    /// Pause all GHOST token operations
    Pause,

    /// Unpause GHOST token operations
    Unpause,

    /// Show current authorities
    Authorities,

    /// Show token metadata
    Metadata,

    /// Update on-chain metadata
    UpdateMetadata {
        /// New token name
        #[arg(long)]
        name: String,

        /// New token symbol
        #[arg(long)]
        symbol: String,

        /// New metadata URI
        #[arg(long)]
        uri: String,
    },
}

const PROGRAM_ID: &str = "Gh0st111111111111111111111111111111111111111";
const MINT_PUBKEY: &str = "Gh0stMint11111111111111111111111111111111111";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    let program_id = Pubkey::from_str_const(PROGRAM_ID);
    let mint_pubkey = Pubkey::from_str_const(MINT_PUBKEY);
    let payer = read_keypair_file(&cli.keypair)?;

    let client = GhostClient::new(&cli.rpc, program_id, mint_pubkey);

    match cli.command {
        Commands::Init { mint } => {
            let mint = mint.parse::<Pubkey>()?;
            let authorities = Authorities {
                mint_authority: payer.pubkey(),
                freeze_authority: payer.pubkey(),
                admin: payer.pubkey(),
            };
            let config = GlobalConfig {
                authorities: authorities.clone(),
                decimals: 9,
                total_supply: 0,
                paused: false,
                bump: 0,
                _reserved: [0u8; 64],
            };

            let ix = build_initialize_ix(&payer.pubkey(), &mint, &authorities, &config);
            let blockhash = client.get_recent_blockhash()?;
            let tx = Transaction::new_signed_with_payer(
                &[ix],
                Some(&payer.pubkey()),
                &[&payer],
                blockhash,
            );
            let sig = client.send_transaction(&tx)?;
            println!("✅ Initialized! TX: {}", sig);
        }

        Commands::Mint { recipient, amount } => {
            let recipient = recipient.parse::<Pubkey>()?;
            let raw = human_to_raw(amount, 9);
            let ix = build_mint_ix(&payer.pubkey(), &mint_pubkey, &recipient, raw);
            let blockhash = client.get_recent_blockhash()?;
            let tx = Transaction::new_signed_with_payer(
                &[ix],
                Some(&payer.pubkey()),
                &[&payer],
                blockhash,
            );
            let sig = client.send_transaction(&tx)?;
            println!("✅ Minted {} GHOST to {} — TX: {}", amount, recipient, sig);
        }

        Commands::Burn {
            token_account,
            amount,
        } => {
            let token_account = token_account.parse::<Pubkey>()?;
            let raw = human_to_raw(amount, 9);
            let ix = build_burn_ix(&payer.pubkey(), &mint_pubkey, &token_account, raw);
            let blockhash = client.get_recent_blockhash()?;
            let tx = Transaction::new_signed_with_payer(
                &[ix],
                Some(&payer.pubkey()),
                &[&payer],
                blockhash,
            );
            let sig = client.send_transaction(&tx)?;
            println!("✅ Burned {} GHOST — TX: {}", amount, sig);
        }

        Commands::Transfer { from, to, amount } => {
            let from = from.parse::<Pubkey>()?;
            let to = to.parse::<Pubkey>()?;
            let raw = human_to_raw(amount, 9);
            let ix = build_transfer_ix(&payer.pubkey(), &from, &to, &mint_pubkey, raw);
            let blockhash = client.get_recent_blockhash()?;
            let tx = Transaction::new_signed_with_payer(
                &[ix],
                Some(&payer.pubkey()),
                &[&payer],
                blockhash,
            );
            let sig = client.send_transaction(&tx)?;
            println!(
                "✅ Transferred {} GHOST from {} to {} — TX: {}",
                amount, from, to, sig
            );
        }

        Commands::Info => {
            let config = client.get_config()?;
            let supply = client.get_total_supply()?;
            println!("=== GHOST Token Info ===");
            println!("Program ID:  {}", PROGRAM_ID);
            println!("Mint:        {}", MINT_PUBKEY);
            println!("Decimals:    {}", config.decimals);
            println!("Total Supply: {}", supply);
            println!("Paused:      {}", config.paused);
        }

        Commands::Pause => {
            let ix = build_pause_ix(&payer.pubkey());
            let blockhash = client.get_recent_blockhash()?;
            let tx = Transaction::new_signed_with_payer(
                &[ix],
                Some(&payer.pubkey()),
                &[&payer],
                blockhash,
            );
            let sig = client.send_transaction(&tx)?;
            println!("✅ Paused — TX: {}", sig);
        }

        Commands::Unpause => {
            let ix = build_unpause_ix(&payer.pubkey());
            let blockhash = client.get_recent_blockhash()?;
            let tx = Transaction::new_signed_with_payer(
                &[ix],
                Some(&payer.pubkey()),
                &[&payer],
                blockhash,
            );
            let sig = client.send_transaction(&tx)?;
            println!("✅ Unpaused — TX: {}", sig);
        }

        Commands::Authorities => {
            let authorities = client.get_authorities()?;
            println!("=== Authorities ===");
            println!("Mint Authority:   {}", authorities.mint_authority);
            println!("Freeze Authority: {}", authorities.freeze_authority);
            println!("Admin:            {}", authorities.admin);
        }

        Commands::Metadata => {
            let metadata = client.get_metadata()?;
            println!("=== Metadata ===");
            println!("Name:   {}", metadata.name);
            println!("Symbol: {}", metadata.symbol);
            println!("URI:    {}", metadata.uri);
        }

        Commands::UpdateMetadata { name, symbol, uri } => {
            let ix = build_update_metadata_ix(&payer.pubkey(), &mint_pubkey, &name, &symbol, &uri);
            let blockhash = client.get_recent_blockhash()?;
            let tx = Transaction::new_signed_with_payer(
                &[ix],
                Some(&payer.pubkey()),
                &[&payer],
                blockhash,
            );
            let sig = client.send_transaction(&tx)?;
            println!("✅ Metadata updated — TX: {}", sig);
        }
    }

    Ok(())
}
