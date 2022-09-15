import { Keypair, PublicKey, Connection } from '@solana/web3.js'
import * as anchor from '@project-serum/anchor';
import { IDL } from "../target/types/solkitties";
import { getKeypairFromString, PROGRAM_ID } from './utils';
import { initialize } from './actions/initialize';
import { BN } from 'bn.js';
import { updateState } from './actions/updateState';

async function main() {
  const connection = new Connection("https://api.devnet.solana.com");

  const authority = getKeypairFromString("");

  const provider = new anchor.AnchorProvider(connection, new anchor.Wallet(authority), anchor.AnchorProvider.defaultOptions());
  const program = new anchor.Program(IDL, PROGRAM_ID, provider);

  /*
  await initialize(program, authority,
    new BN(5_000_000), // Ticket price
    new BN(100), // Total supply
    new BN(new Date().getTime() / 1000), // End date
  )
  */

  await updateState(program, authority,
    new BN(10_000_000), // Ticket price
    new BN(100), // Total supply
    new BN(new Date().getTime() / 1000 + 60 * 60), // End date
  )
}

main().then(
  () => process.exit(),
  (err) => {
    console.error(err);
    process.exit(-1);
  }
);
