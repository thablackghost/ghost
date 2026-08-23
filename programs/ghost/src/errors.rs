//! GHOST Token — Centralized error codes.
//! Every error the program can surface is enumerated here.

use anchor_lang::prelude::*;

#[error_code]
pub enum GhostError {
    // ── 0xxx : Authority ──────────────────────────────────────────────────────
    #[msg("Unauthorized: caller lacks the required authority")]
    Unauthorized,
    #[msg("Invalid authority: pubkey does not match any stored authority")]
    InvalidAuthority,
    #[msg("Invalid signer: expected a different signer")]
    InvalidSigner,
    #[msg("Invalid token account owner")]
    InvalidOwner,
    #[msg("Invalid emergency authority")]
    InvalidEmergencyAuthority,
    #[msg("Unauthorized mint authority")]
    UnauthorizedMintAuthority,
    #[msg("Unauthorized admin authority")]
    UnauthorizedAdmin,
    #[msg("Unauthorized metadata authority")]
    UnauthorizedMetadataAuthority,
    #[msg("Unauthorized emergency authority")]
    UnauthorizedEmergencyAuthority,
    #[msg("Unauthorized: not admin or emergency authority")]
    UnauthorizedEmergencyOrAdmin,
    #[msg("Authority cannot be zero pubkey")]
    AuthorityCannotBeZero,
    #[msg("Authority rotation: no change proposed")]
    AuthorityRotationNoChange,
    #[msg("Authority changes are locked")]
    AuthorityChangesLocked,
    #[msg("A rotation is already proposed")]
    RotationAlreadyProposed,
    #[msg("No rotation is currently proposed")]
    NoRotationProposed,
    #[msg("Rotation timelock has not elapsed")]
    RotationTimelockNotElapsed,

    // ── 1xxx : Token / Account ────────────────────────────────────────────────
    #[msg("Invalid mint: expected the canonical GHOST mint")]
    InvalidMint,
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    #[msg("Invalid token program: expected SPL Token-2022")]
    InvalidTokenProgram,
    #[msg("Token program mismatch between accounts")]
    TokenProgramMismatch,
    #[msg("Mint mismatch: token account belongs to a different mint")]
    MintMismatch,
    #[msg("Invalid decimals")]
    InvalidDecimals,
    #[msg("Invalid account owner")]
    InvalidAccountOwner,
    #[msg("Account mismatch")]
    AccountMismatch,
    #[msg("Invalid token account owner")]
    InvalidTokenAccountOwner,
    #[msg("Invalid token account data")]
    InvalidTokenAccountData,
    #[msg("Token account not initialized")]
    TokenAccountNotInitialized,
    #[msg("Token account mint mismatch")]
    TokenAccountMintMismatch,
    #[msg("Token account owner mismatch")]
    TokenAccountOwnerMismatch,
    #[msg("Invalid ATA address")]
    InvalidAtaAddress,
    #[msg("Invalid mint owner")]
    InvalidMintOwner,
    #[msg("Invalid mint data")]
    InvalidMintData,

    // ── 2xxx : Metadata ───────────────────────────────────────────────────────
    #[msg("Invalid metadata")]
    InvalidMetadata,
    #[msg("Metadata field too long")]
    MetadataTooLong,
    #[msg("Invalid URI format")]
    InvalidUri,
    #[msg("Metadata is frozen and cannot be modified")]
    MetadataFrozen,

    // ── 3xxx : Configuration ──────────────────────────────────────────────────
    #[msg("Program not initialized")]
    NotInitialized,
    #[msg("Program already initialized")]
    AlreadyInitialized,
    #[msg("Duplicate initialization detected")]
    DuplicateInitialization,
    #[msg("Invalid configuration")]
    InvalidConfiguration,
    #[msg("Invalid PDA: derived address mismatch")]
    InvalidPda,
    #[msg("Invalid bump seed")]
    InvalidBump,
    #[msg("PDA mismatch: on-chain address does not match derived address")]
    PdaMismatch,
    #[msg("Invalid state for this operation")]
    InvalidState,

    // ── 4xxx : Security / Pause ───────────────────────────────────────────────
    #[msg("Program is paused")]
    ProgramPaused,
    #[msg("Program is emergency-paused")]
    EmergencyPaused,
    #[msg("Program is emergency paused")]
    ProgramEmergencyPaused,
    #[msg("Missing required signature")]
    MissingSignature,
    #[msg("Signer mismatch")]
    SignerMismatch,
    #[msg("Invalid program ID")]
    InvalidProgramId,
    #[msg("Account is not system-owned")]
    AccountNotSystemOwned,
    #[msg("Account is not token-program-owned")]
    AccountNotTokenProgramOwned,
    #[msg("Invalid slot ordering")]
    InvalidSlotOrdering,
    #[msg("Slot is too stale")]
    SlotTooStale,
    #[msg("Invalid timestamp order")]
    InvalidTimestampOrder,
    #[msg("Invalid timestamp")]
    InvalidTimestamp,

    // ── 5xxx : Amount / Arithmetic ────────────────────────────────────────────
    #[msg("Zero amount is not permitted")]
    ZeroAmount,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Amount overflow")]
    AmountOverflow,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Arithmetic error")]
    ArithmeticError,
    #[msg("Batch size exceeded")]
    BatchSizeExceeded,
    #[msg("Transfer hook not allowed")]
    TransferHookNotAllowed,

    // ── 6xxx : Authority Rotation ─────────────────────────────────────────────
    #[msg("Authority rotation is pending")]
    AuthorityRotationPending,
    #[msg("Authority rotation has expired")]
    AuthorityRotationExpired,

    // ── 7xxx : Migration ──────────────────────────────────────────────────────
    #[msg("Invalid migration")]
    InvalidMigration,
    #[msg("Unsupported program version")]
    UnsupportedVersion,

    // ── 8xxx : Invariants ─────────────────────────────────────────────────────
    #[msg("Invariant violation detected")]
    InvariantViolation,
    #[msg("Supply invariant violated")]
    SupplyInvariantViolated,
    #[msg("Mint invariant violation")]
    MintInvariantViolation,
    #[msg("Burn invariant violation")]
    BurnInvariantViolation,

    // ── 9xxx : CPI ────────────────────────────────────────────────────────────
    #[msg("CPI failure")]
    CpiFailure,
    #[msg("CPI instruction build failed")]
    CpiInstructionBuildFailed,
    #[msg("Invalid CPI program")]
    InvalidCpiProgram,
    #[msg("CPI mint mismatch")]
    CpiMintMismatch,
    #[msg("CPI token account mint mismatch")]
    CpiTokenAccountMintMismatch,
    #[msg("CPI token account owner mismatch")]
    CpiTokenAccountOwnerMismatch,
    #[msg("CPI authority mismatch")]
    CpiAuthorityMismatch,
    #[msg("Mint failed")]
    MintFailed,
    #[msg("Transfer failed")]
    TransferFailed,
    #[msg("Burn failed")]
    BurnFailed,
}
