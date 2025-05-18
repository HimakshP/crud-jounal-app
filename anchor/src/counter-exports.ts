import { AnchorProvider, Program } from "@coral-xyz/anchor";
import { Cluster, PublicKey } from "@solana/web3.js";
import JournalIDL from "../target/idl/crud.json";
import type { Crud } from "../target/types/crud";

// Re-export the generated IDL and type
export { Crud, JournalIDL };

// After updating your program ID (e.g. after running `anchor keys sync`) update the value below.
export const JOURNAL_PROGRAM_ID = new PublicKey(
  "A9apbLdvLHj322EZxpShv77qgybArWJ7MRgKjDirCKKf");

// This is a helper function to get the Counter Anchor program.
export function getJournalProgram(provider: AnchorProvider) {
  return new Program(JournalIDL as Crud, provider);
}

// This is a helper function to get the program ID for the Journal program depending on the cluster.
export function getJournalProgramId(cluster: Cluster) {
  switch (cluster) {
    case "devnet":
    case "testnet":
    case "mainnet-beta":
    default:
      return JOURNAL_PROGRAM_ID;
  }
}
