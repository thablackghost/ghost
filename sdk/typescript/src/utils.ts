import { PublicKey } from '@solana/web3.js';
import { getAssociatedTokenAddressSync } from '@solana/spl-token';
import { TOKEN_2022_PROGRAM_ID } from '@solana/spl-token';

/**
 * Returns the Associated Token Account (ATA) address for a mint and owner
 * using the Token-2022 program.
 */
export function getATA(mint: PublicKey, owner: PublicKey): PublicKey {
  return getAssociatedTokenAddressSync(
    mint,
    owner,
    false,
    TOKEN_2022_PROGRAM_ID,
  );
}

/**
 * Converts a human-readable amount (e.g. 100.5) to raw token units
 * using bigint for unlimited precision.
 */
export function humanToRaw(amount: number, decimals: number): bigint {
  const multiplier = 10 ** decimals;
  return BigInt(Math.round(amount * multiplier));
}

/**
 * Converts raw token units to a human-readable number.
 */
export function rawToHuman(raw: bigint, decimals: number): number {
  const divisor = 10 ** decimals;
  return Number(raw) / divisor;
}