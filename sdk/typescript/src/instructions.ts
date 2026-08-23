import { PublicKey, TransactionInstruction, SystemProgram, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { Authorities, GlobalConfig } from './types';
import { getATA } from './utils';

const GHOST_PROGRAM_ID = new PublicKey('Gh0st111111111111111111111111111111111111111');

/**
 * Computes the 8-byte Anchor discriminator from an instruction name.
 */
function anchorDiscriminator(name: string): Buffer {
  const preimage = `global:${name}`;
  const { createHash } = require('crypto');
  const hash = createHash('sha256').update(Buffer.from(preimage)).digest();
  return hash.subarray(0, 8);
}

/**
 * Creates an initialize instruction for the GHOST token program.
 */
export function createInitializeInstruction(
  payer: PublicKey,
  mint: PublicKey,
  authorities: Authorities,
  config: GlobalConfig,
): TransactionInstruction {
  const [configPda] = PublicKey.findProgramAddressSync(
    [Buffer.from('config'), mint.toBuffer()],
    GHOST_PROGRAM_ID,
  );
  const [metadataPda] = PublicKey.findProgramAddressSync(
    [Buffer.from('metadata'), mint.toBuffer()],
    GHOST_PROGRAM_ID,
  );

  const data = Buffer.concat([
    anchorDiscriminator('initialize'),
    authorities.mintAuthority.toBuffer(),
    authorities.freezeAuthority.toBuffer(),
    authorities.admin.toBuffer(),
    Buffer.from([config.paused ? 1 : 0]),
    Buffer.from(new BigUint64Array([BigInt(10_000_000_000)]).buffer), // initial supply
  ]);

  return new TransactionInstruction({
    programId: GHOST_PROGRAM_ID,
    keys: [
      { pubkey: payer, isSigner: true, isWritable: true },
      { pubkey: mint, isSigner: false, isWritable: true },
      { pubkey: configPda, isSigner: false, isWritable: true },
      { pubkey: metadataPda, isSigner: false, isWritable: true },
      { pubkey: TOKEN_2022_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
    ],
    data,
  });
}

/**
 * Creates a mint instruction for the GHOST token program.
 */
export function createMintInstruction(
  mintAuthority: PublicKey,
  mint: PublicKey,
  recipient: PublicKey,
  amount: bigint,
): TransactionInstruction {
  const ata = getATA(mint, recipient);

  const data = Buffer.concat([
    anchorDiscriminator('mint'),
    Buffer.from(new BigUint64Array([amount]).buffer),
  ]);

  return new TransactionInstruction({
    programId: GHOST_PROGRAM_ID,
    keys: [
      { pubkey: mintAuthority, isSigner: true, isWritable: false },
      { pubkey: mint, isSigner: false, isWritable: true },
      { pubkey: ata, isSigner: false, isWritable: true },
      { pubkey: recipient, isSigner: false, isWritable: false },
      { pubkey: TOKEN_2022_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
    ],
    data,
  });
}

/**
 * Creates a burn instruction for the GHOST token program.
 */
export function createBurnInstruction(
  owner: PublicKey,
  mint: PublicKey,
  tokenAccount: PublicKey,
  amount: bigint,
): TransactionInstruction {
  const data = Buffer.concat([
    anchorDiscriminator('burn'),
    Buffer.from(new BigUint64Array([amount]).buffer),
  ]);

  return new TransactionInstruction({
    programId: GHOST_PROGRAM_ID,
    keys: [
      { pubkey: owner, isSigner: true, isWritable: false },
      { pubkey: tokenAccount, isSigner: false, isWritable: true },
      { pubkey: mint, isSigner: false, isWritable: true },
      { pubkey: TOKEN_2022_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    data,
  });
}

/**
 * Creates a transfer instruction for the GHOST token program.
 */
export function createTransferInstruction(
  owner: PublicKey,
  from: PublicKey,
  to: PublicKey,
  mint: PublicKey,
  amount: bigint,
): TransactionInstruction {
  const data = Buffer.concat([
    anchorDiscriminator('transfer'),
    Buffer.from(new BigUint64Array([amount]).buffer),
  ]);

  return new TransactionInstruction({
    programId: GHOST_PROGRAM_ID,
    keys: [
      { pubkey: owner, isSigner: true, isWritable: false },
      { pubkey: from, isSigner: false, isWritable: true },
      { pubkey: to, isSigner: false, isWritable: true },
      { pubkey: mint, isSigner: false, isWritable: false },
      { pubkey: TOKEN_2022_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    data,
  });
}

/**
 * Creates an update_metadata instruction for the GHOST token program.
 */
export function createUpdateMetadataInstruction(
  authority: PublicKey,
  mint: PublicKey,
  name: string,
  symbol: string,
  uri: string,
): TransactionInstruction {
  const [metadataPda] = PublicKey.findProgramAddressSync(
    [Buffer.from('metadata'), mint.toBuffer()],
    GHOST_PROGRAM_ID,
  );

  // Pad name to 32 bytes
  const nameBuffer = Buffer.alloc(32);
  Buffer.from(name, 'utf8').copy(nameBuffer);

  // Pad symbol to 10 bytes
  const symbolBuffer = Buffer.alloc(10);
  Buffer.from(symbol, 'utf8').copy(symbolBuffer);

  // Pad URI to 200 bytes
  const uriBuffer = Buffer.alloc(200);
  Buffer.from(uri, 'utf8').copy(uriBuffer);

  const data = Buffer.concat([
    anchorDiscriminator('update_metadata'),
    nameBuffer,
    symbolBuffer,
    uriBuffer,
  ]);

  return new TransactionInstruction({
    programId: GHOST_PROGRAM_ID,
    keys: [
      { pubkey: authority, isSigner: true, isWritable: false },
      { pubkey: metadataPda, isSigner: false, isWritable: true },
      { pubkey: mint, isSigner: false, isWritable: false },
    ],
    data,
  });
}

/**
 * Creates a pause instruction for the GHOST token program.
 */
export function createPauseInstruction(admin: PublicKey): TransactionInstruction {
  return new TransactionInstruction({
    programId: GHOST_PROGRAM_ID,
    keys: [{ pubkey: admin, isSigner: true, isWritable: false }],
    data: anchorDiscriminator('pause'),
  });
}

/**
 * Creates an unpause instruction for the GHOST token program.
 */
export function createUnpauseInstruction(admin: PublicKey): TransactionInstruction {
  return new TransactionInstruction({
    programId: GHOST_PROGRAM_ID,
    keys: [{ pubkey: admin, isSigner: true, isWritable: false }],
    data: anchorDiscriminator('unpause'),
  });
}