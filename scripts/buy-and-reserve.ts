import { Program, AnchorProvider, setProvider } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { clusterApiUrl, Connection, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import type { SolStrike } from "../target/types/sol_strike";
import idl from "../target/idl/sol_strike.json";
import { getAssociatedTokenAddress, TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";


anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.SolStrike as Program<SolStrike>;

(async () => {
  const [chipMintPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("CHIP_MINT")],
    program.programId
  );

  const userChipTokenAccountAddress = await getAssociatedTokenAddress(chipMintPDA, program.provider.publicKey, false, TOKEN_2022_PROGRAM_ID);
  

  // const tx1 = await program.methods.buyChipWithSol(new anchor.BN(LAMPORTS_PER_SOL))
  // .accountsPartial({
  //   buyer: program.provider.publicKey,
  //   chipMint: chipMintPDA,
  //   buyerChipAccount: userChipTokenAccountAddress,
  //   tokenProgram: TOKEN_2022_PROGRAM_ID,
  // })
  // .rpc();   

  // console.log("InitGlobalConfig tx:", tx1);

  const tx2 = await program.methods.reserveChips(new anchor.BN(LAMPORTS_PER_SOL))
    .accountsPartial({
        signer: program.provider.publicKey,
        chipMint: chipMintPDA,
        userChipAccount: userChipTokenAccountAddress,
        tokenProgram: TOKEN_2022_PROGRAM_ID
    })
    .rpc();

    console.log("Initialize:", tx2);
})();

