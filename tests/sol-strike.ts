import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolStrike } from "../target/types/sol_strike";
import { assert } from "chai";

describe("sol-strike", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();

  const program = anchor.workspace.SolStrike as Program<SolStrike>;

  it("Is initialized!", async () => {
    const [chipMintPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("CHIP_MINT")],
      program.programId
    );
    
    const [treasuryPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("TREASURY")],
      program.programId
    );

    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature:", tx);

    const chipMintAccount = await provider.connection.getAccountInfo(chipMintPDA);
    const treasuryAccount = await provider.connection.getAccountInfo(treasuryPDA);

    console.log("Chip Mint PDA:", chipMintPDA.toBase58());
    console.log("Treasury PDA:", treasuryPDA.toBase58());

    assert(chipMintAccount !== null, "chip_mint PDA was not created!");
    assert(treasuryAccount !== null, "treasury PDA was not created!");
  });
});
