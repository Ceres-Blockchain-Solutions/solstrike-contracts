import { Program, AnchorProvider, setProvider } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { clusterApiUrl, Connection, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import type { SolStrike } from "../target/types/sol_strike";
import idl from "../target/idl/sol_strike.json";
import { getAssociatedTokenAddress, TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";


anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.SolStrike as Program<SolStrike>;

(async () => {
  const [chipMintPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("CHIP_MINT")],
    program.programId
  );

  const userChipTokenAccountAddress = await getAssociatedTokenAddress(chipMintPDA, program.provider.publicKey, false, TOKEN_2022_PROGRAM_ID);
  
  console.log(program.provider.publicKey)
  
  // const tx1 = await program.methods.buyChipWithSol(new anchor.BN(LAMPORTS_PER_SOL))
  // .accountsPartial({
  //   buyer: program.provider.publicKey,
  //   chipMint: chipMintPDA,
  //   buyerChipAccount: userChipTokenAccountAddress,
  //   tokenProgram: TOKEN_2022_PROGRAM_ID,
  // })
  // .rpc();   
  
  // console.log("buyChipWithSol tx:", tx1);

  // const tx2 = await program.methods.reserveChips(new anchor.BN(LAMPORTS_PER_SOL))
  //   .accountsPartial({
  //       signer: program.provider.publicKey,
  //       chipMint: chipMintPDA,
  //       userChipAccount: userChipTokenAccountAddress,
  //       tokenProgram: TOKEN_2022_PROGRAM_ID
  //   })
  //   .rpc();

  //   console.log("reserveChips tx:", tx2);

  // console.log(program.programId)

  const pk1 = new PublicKey("EXdpyTLKmHe2ottQVWwEYRJRiiMdKJePZRCH75AWmoxk")
  const pk2 = new PublicKey("9AW1CHYkrg5mnNg17ExUfJT1zqVYvcH6RUtE6dLLR1jq")
  const pk3 = new PublicKey("CpPgwFxzWrGvmAzrQRshkDmEP6yv9WB2UQcXXBVATxXe")

  const [pk1Pda] = PublicKey.findProgramAddressSync(
    [pk1.toBuffer()],
    program.programId
  );

  const [pk2Pda] = PublicKey.findProgramAddressSync(
    [pk2.toBuffer()],
    program.programId
  );

  const [pk3Pda] = PublicKey.findProgramAddressSync(
    [pk3.toBuffer()],
    program.programId
  );

  const claimableRewards1 = await program.account.claimableRewards.fetch(
    pk1Pda
  );

  const claimableRewards2 = await program.account.claimableRewards.fetch(
    pk2Pda
  );

  const claimableRewards3 = await program.account.claimableRewards.fetch(
    pk3Pda
  );

  console.log(claimableRewards1.amount.toNumber())
  console.log(claimableRewards2.amount.toNumber())
  console.log(claimableRewards3.amount.toNumber())

    let programData = await program.provider.connection.getAccountInfo(new PublicKey("F7Dr4bH5knKjzBj8fuRJT9QGtHLyQSWTnWxYetHDnWHA"))
    let programDataAccount = new PublicKey(programData.data.subarray(programData.data.length - 32));

    const firstPlacePK = new PublicKey("9AW1CHYkrg5mnNg17ExUfJT1zqVYvcH6RUtE6dLLR1jq")

    const secondPlacePK = new PublicKey("CpPgwFxzWrGvmAzrQRshkDmEP6yv9WB2UQcXXBVATxXe")
    const thirdPlacePK = new PublicKey("EXdpyTLKmHe2ottQVWwEYRJRiiMdKJePZRCH75AWmoxk")
  
    const [firstPlaceClaimableRewardsPda] = PublicKey.findProgramAddressSync(
      [firstPlacePK.toBuffer()],
      program.programId
    );

    const [secondPlaceClaimableRewardsPda] = PublicKey.findProgramAddressSync(
      [secondPlacePK.toBuffer()],
      program.programId
    );

    const [thirdPlaceClaimableRewardsPda] = PublicKey.findProgramAddressSync(
      [thirdPlacePK.toBuffer()],
      program.programId
    );

    // const tx3 = await program.methods.setClaimableRewards()
    // .accountsPartial({
    //     signer: program.provider.publicKey,
    //     program: program.programId,
    //     programData: programDataAccount,
    //     firstPlaceClaimableRewardsAccount: firstPlaceClaimableRewardsPda,
    //     firstPlaceAuthority: firstPlacePK,
    //     secondPlaceClaimableRewardsAccount: secondPlaceClaimableRewardsPda,
    //     secondPlaceAuthority: secondPlacePK,
    //     thirdPlaceClaimableRewardsAccount: thirdPlaceClaimableRewardsPda,
    //     thirdPlaceAuthority: thirdPlacePK,
    //     systemProgram: SYSTEM_PROGRAM_ID
    //   })
    // .rpc();

    // console.log("Set claimable: ", tx3);
})();

