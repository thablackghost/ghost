import { Command } from 'commander';
import { Connection, PublicKey, Transaction, sendAndConfirmTransaction } from '@solana/web3.js';
import { GhostClient } from '../../sdk/typescript/src/client';
import {
  createInitializeInstruction,
  createMintInstruction,
  createBurnInstruction,
  createTransferInstruction,
  createUpdateMetadataInstruction,
  createPauseInstruction,
  createUnpauseInstruction,
} from '../../sdk/typescript/src/instructions';
import { Authorities, GlobalConfig } from '../../sdk/typescript/src/types';
import { humanToRaw } from '../../sdk/typescript/src/utils';
import * as fs from 'fs';
import * as path from 'path';
import { fileURLToPath } from 'url';

// We inline keypair loading for the CLI since it reads from filesystem
function loadKeypair(filepath: string): any {
  const resolved = filepath.startsWith('~')
    ? path.join(process.env.HOME || '/root', filepath.slice(1))
    : filepath;
  const raw = JSON.parse(fs.readFileSync(resolved, 'utf-8'));
  const { Keypair } = require('@solana/web3.js');
  return Keypair.fromSecretKey(Uint8Array.from(raw));
}

const GHOST_PROGRAM_ID = new PublicKey('Gh0st111111111111111111111111111111111111111');
const GHOST_MINT = new PublicKey('Gh0stMint11111111111111111111111111111111111');

type KeypairType = InstanceType<typeof import('@solana/web3.js').Keypair>;

const program = new Command();

program
  .name('ghost')
  .version('0.1.0')
  .description('GHOST Token CLI (TypeScript)')
  .option('-r, --rpc <url>', 'RPC URL', 'https://api.devnet.solana.com')
  .option('-k, --keypair <path>', 'Keypair path', '~/.config/solana/id.json');

program
  .command('init')
  .description('Initialize the GHOST token program')
  .requiredOption('--mint <pubkey>', 'The mint pubkey')
  .action(async (opts: { mint: string }) => {
    const connection = new Connection(program.opts().rpc, 'confirmed');
    const payer = loadKeypair(program.opts().keypair);
    const mint = new PublicKey(opts.mint);

    const authorities: Authorities = {
      mintAuthority: payer.publicKey,
      freezeAuthority: payer.publicKey,
      admin: payer.publicKey,
    };

    const config: GlobalConfig = {
      authorities,
      decimals: 9,
      totalSupply: 0,
      paused: false,
      bump: 0,
    };

    const ix = createInitializeInstruction(payer.publicKey, mint, authorities, config);
    const tx = new Transaction().add(ix);
    const sig = await sendAndConfirmTransaction(connection, tx, [payer]);
    console.log(`✅ Initialized! TX: ${sig}`);
  });

program
  .command('mint')
  .description('Mint GHOST tokens')
  .requiredOption('--recipient <pubkey>', 'Recipient address')
  .requiredOption('--amount <number>', 'Amount in human-readable form')
  .action(async (opts: { recipient: string; amount: string }) => {
    const connection = new Connection(program.opts().rpc, 'confirmed');
    const payer = loadKeypair(program.opts().keypair);
    const recipient = new PublicKey(opts.recipient);
    const amount = parseFloat(opts.amount);
    const raw = humanToRaw(amount, 9);

    const ix = createMintInstruction(payer.publicKey, GHOST_MINT, recipient, raw);
    const tx = new Transaction().add(ix);
    const sig = await sendAndConfirmTransaction(connection, tx, [payer]);
    console.log(`✅ Minted ${amount} GHOST to ${recipient.toBase58()} — TX: ${sig}`);
  });

program
  .command('burn')
  .description('Burn GHOST tokens')
  .requiredOption('--token-account <pubkey>', 'Your token account')
  .requiredOption('--amount <number>', 'Amount in human-readable form')
  .action(async (opts: { tokenAccount: string; amount: string }) => {
    const connection = new Connection(program.opts().rpc, 'confirmed');
    const payer = loadKeypair(program.opts().keypair);
    const tokenAccount = new PublicKey(opts.tokenAccount);
    const amount = parseFloat(opts.amount);
    const raw = humanToRaw(amount, 9);

    const ix = createBurnInstruction(payer.publicKey, GHOST_MINT, tokenAccount, raw);
    const tx = new Transaction().add(ix);
    const sig = await sendAndConfirmTransaction(connection, tx, [payer]);
    console.log(`✅ Burned ${amount} GHOST — TX: ${sig}`);
  });

program
  .command('transfer')
  .description('Transfer GHOST tokens')
  .requiredOption('--from <pubkey>', 'Sender token account')
  .requiredOption('--to <pubkey>', 'Recipient token account')
  .requiredOption('--amount <number>', 'Amount in human-readable form')
  .action(async (opts: { from: string; to: string; amount: string }) => {
    const connection = new Connection(program.opts().rpc, 'confirmed');
    const payer = loadKeypair(program.opts().keypair);
    const from = new PublicKey(opts.from);
    const to = new PublicKey(opts.to);
    const amount = parseFloat(opts.amount);
    const raw = humanToRaw(amount, 9);

    const ix = createTransferInstruction(payer.publicKey, from, to, GHOST_MINT, raw);
    const tx = new Transaction().add(ix);
    const sig = await sendAndConfirmTransaction(connection, tx, [payer]);
    console.log(`✅ Transferred ${amount} GHOST — TX: ${sig}`);
  });

program
  .command('info')
  .description('Show GHOST token info')
  .action(async () => {
    const connection = new Connection(program.opts().rpc, 'confirmed');
    const client = new GhostClient(connection, GHOST_PROGRAM_ID, GHOST_MINT);

    const config = await client.getConfig();
    const supply = await client.getTotalSupply();

    console.log('=== GHOST Token Info ===');
    console.log(`Program ID:   ${GHOST_PROGRAM_ID.toBase58()}`);
    console.log(`Mint:         ${GHOST_MINT.toBase58()}`);
    console.log(`Decimals:     ${config.decimals}`);
    console.log(`Total Supply: ${supply}`);
    console.log(`Paused:       ${config.paused}`);
  });

program
  .command('pause')
  .description('Pause all GHOST token operations')
  .action(async () => {
    const connection = new Connection(program.opts().rpc, 'confirmed');
    const payer = loadKeypair(program.opts().keypair);

    const ix = createPauseInstruction(payer.publicKey);
    const tx = new Transaction().add(ix);
    const sig = await sendAndConfirmTransaction(connection, tx, [payer]);
    console.log(`✅ Paused — TX: ${sig}`);
  });

program
  .command('unpause')
  .description('Unpause GHOST token operations')
  .action(async () => {
    const connection = new Connection(program.opts().rpc, 'confirmed');
    const payer = loadKeypair(program.opts().keypair);

    const ix = createUnpauseInstruction(payer.publicKey);
    const tx = new Transaction().add(ix);
    const sig = await sendAndConfirmTransaction(connection, tx, [payer]);
    console.log(`✅ Unpaused — TX: ${sig}`);
  });

program
  .command('authorities')
  .description('Show current authorities')
  .action(async () => {
    const connection = new Connection(program.opts().rpc, 'confirmed');
    const client = new GhostClient(connection, GHOST_PROGRAM_ID, GHOST_MINT);
    const authorities = await client.getAuthorities();

    console.log('=== Authorities ===');
    console.log(`Mint Authority:   ${authorities.mintAuthority.toBase58()}`);
    console.log(`Freeze Authority: ${authorities.freezeAuthority.toBase58()}`);
    console.log(`Admin:            ${authorities.admin.toBase58()}`);
  });

program
  .command('metadata')
  .description('Show token metadata')
  .action(async () => {
    const connection = new Connection(program.opts().rpc, 'confirmed');
    const client = new GhostClient(connection, GHOST_PROGRAM_ID, GHOST_MINT);
    const metadata = await client.getMetadata();

    console.log('=== Metadata ===');
    console.log(`Name:   ${metadata.name}`);
    console.log(`Symbol: ${metadata.symbol}`);
    console.log(`URI:    ${metadata.uri}`);
  });

program
  .command('update-metadata')
  .description('Update on-chain metadata')
  .requiredOption('--name <string>', 'New token name')
  .requiredOption('--symbol <string>', 'New token symbol')
  .requiredOption('--uri <string>', 'New metadata URI')
  .action(async (opts: { name: string; symbol: string; uri: string }) => {
    const connection = new Connection(program.opts().rpc, 'confirmed');
    const payer = loadKeypair(program.opts().keypair);

    const ix = createUpdateMetadataInstruction(
      payer.publicKey,
      GHOST_MINT,
      opts.name,
      opts.symbol,
      opts.uri,
    );
    const tx = new Transaction().add(ix);
    const sig = await sendAndConfirmTransaction(connection, tx, [payer]);
    console.log(`✅ Metadata updated — TX: ${sig}`);
  });

program.parseAsync().catch((err: Error) => {
  console.error('Error:', err.message);
  process.exit(1);
});