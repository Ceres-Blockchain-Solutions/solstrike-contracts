import { Program, AnchorProvider, setProvider } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { clusterApiUrl, Connection, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import type { SolStrike } from "../target/types/sol_strike";
import idl from "../target/idl/sol_strike.json";
import { TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";


anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.SolStrike as Program<SolStrike>;

(async () => {
  let programData = await program.provider.connection.getAccountInfo(program.programId)
  let programDataAccount = new PublicKey(programData.data.subarray(programData.data.length - 32));

  const tx1 = await program.methods.initialize(new anchor.BN(0.01 * LAMPORTS_PER_SOL))
  .accountsPartial({
    signer: program.provider.publicKey,
    program: program.programId,
    programData: programDataAccount,
    tokenProgram: TOKEN_2022_PROGRAM_ID  })
  .rpc();

  console.log("Initialize tx:", tx1);

})();

