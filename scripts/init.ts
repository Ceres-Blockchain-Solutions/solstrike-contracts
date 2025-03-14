import { Program, AnchorProvider, setProvider } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { clusterApiUrl, Connection, LAMPORTS_PER_SOL } from "@solana/web3.js";
import type { SolStrike } from "../target/types/sol_strike";
import idl from "../target/idl/sol_strike.json";
import { TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";


anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.SolStrike as Program<SolStrike>;

(async () => {
  const tx1 = await program.methods.initGlobalConfig(new anchor.BN(0.01 * LAMPORTS_PER_SOL))
  .accounts({
    signer: program.provider.publicKey,
  })
  .rpc();

  console.log("InitGlobalConfig tx:", tx1);

  const tx2 = await program.methods.initialize()
    .accounts({
        signer: program.provider.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID
    })
    .rpc();

    console.log("Initialize:", tx2);
})();

