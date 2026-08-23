import { Connection, PublicKey } from '@solana/web3.js';
import { GlobalConfig, Authorities, Metadata } from './types';

const GHOST_PROGRAM_ID = new PublicKey('Gh0st111111111111111111111111111111111111111');

export class GhostClient {
  private connection: Connection;
  private programId: PublicKey;
  private mintPubkey: PublicKey;

  constructor(connection: Connection, programId: PublicKey, mintPubkey: PublicKey) {
    this.connection = connection;
    this.programId = programId;
    this.mintPubkey = mintPubkey;
  }

  /** Retrieves the global configuration PDA from the program. */
  async getConfig(): Promise<GlobalConfig> {
    const [configPda] = PublicKey.findProgramAddressSync(
      [Buffer.from('config'), this.mintPubkey.toBuffer()],
      this.programId,
    );

    const accountInfo = await this.connection.getAccountInfo(configPda);
    if (!accountInfo) {
      throw new Error(`Config account not found at ${configPda.toBase58()}`);
    }

    const data = accountInfo.data;
    // Skip 8-byte anchor discriminator
    let offset = 8;

    // Read authorities (3 x 32 byte pubkeys)
    const mintAuthority = new PublicKey(data.slice(offset, offset + 32));
    offset += 32;
    const freezeAuthority = new PublicKey(data.slice(offset, offset + 32));
    offset += 32;
    const admin = new PublicKey(data.slice(offset, offset + 32));
    offset += 32;

    // Read decimals (1 byte)
    const decimals = data[offset];
    offset += 1;

    // Read total_supply (8 bytes, little-endian)
    const totalSupply = Number(data.readBigUInt64LE(offset));
    offset += 8;

    // Read paused (1 byte)
    const paused = data[offset] !== 0;
    offset += 1;

    // Read bump (1 byte)
    const bump = data[offset];
    offset += 1;

    // Skip reserved (64 bytes)
    offset += 64;

    return {
      authorities: { mintAuthority, freezeAuthority, admin },
      decimals,
      totalSupply,
      paused,
      bump,
    };
  }

  /** Retrieves the current total supply of GHOST tokens from the SPL mint. */
  async getTotalSupply(): Promise<bigint> {
    const accountInfo = await this.connection.getAccountInfo(this.mintPubkey);
    if (!accountInfo) {
      throw new Error(`Mint account not found at ${this.mintPubkey.toBase58()}`);
    }
    // SPL Token 2022 Mint layout: offset 36 = supply (8 bytes, little-endian)
    const supply = accountInfo.data.readBigUInt64LE(36);
    return supply;
  }

  /** Retrieves authority information from the config PDA. */
  async getAuthorities(): Promise<Authorities> {
    const config = await this.getConfig();
    return config.authorities;
  }

  /** Retrieves token metadata from on-chain storage. */
  async getMetadata(): Promise<Metadata> {
    const [metadataPda] = PublicKey.findProgramAddressSync(
      [Buffer.from('metadata'), this.mintPubkey.toBuffer()],
      this.programId,
    );

    const accountInfo = await this.connection.getAccountInfo(metadataPda);
    if (!accountInfo) {
      throw new Error(`Metadata account not found at ${metadataPda.toBase58()}`);
    }

    const data = accountInfo.data;
    // Skip 8-byte anchor discriminator
    let offset = 8;

    // Read name (32 bytes)
    const nameBytes = data.slice(offset, offset + 32);
    const name = Buffer.from(nameBytes).toString('utf8').replace(/\0/g, '');
    offset += 32;

    // Read symbol (10 bytes)
    const symbolBytes = data.slice(offset, offset + 10);
    const symbol = Buffer.from(symbolBytes).toString('utf8').replace(/\0/g, '');
    offset += 10;

    // Read URI (200 bytes)
    const uriBytes = data.slice(offset, offset + 200);
    const uri = Buffer.from(uriBytes).toString('utf8').replace(/\0/g, '');

    return { name, symbol, uri };
  }

  /** Checks if the protocol is currently paused. */
  async isPaused(): Promise<boolean> {
    const config = await this.getConfig();
    return config.paused;
  }
}