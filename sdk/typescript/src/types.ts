import { PublicKey } from '@solana/web3.js';

/** Authority key holders for the GHOST token program. */
export interface Authorities {
  mintAuthority: PublicKey;
  freezeAuthority: PublicKey;
  admin: PublicKey;
}

/** Global on-chain configuration for the GHOST token. */
export interface GlobalConfig {
  authorities: Authorities;
  decimals: number;
  totalSupply: number;
  paused: boolean;
  bump: number;
}

/** On-chain metadata for the GHOST token (Token-2022 extension). */
export interface Metadata {
  name: string;
  symbol: string;
  uri: string;
}