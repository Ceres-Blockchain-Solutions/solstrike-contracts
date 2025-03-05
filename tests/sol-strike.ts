import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolStrike } from "../target/types/sol_strike";
import { assert, expect } from "chai";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { PublicKey, LAMPORTS_PER_SOL, Keypair } from "@solana/web3.js";
import {
  createMint,
  createAccount,
  getAccount,
  getOrCreateAssociatedTokenAccount,
  transfer,
  mintTo,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";

import {
  createAccountsMintsAndTokenAccounts
} from "@solana-developers/helpers"
import { BN } from "bn.js";
import { token } from "@coral-xyz/anchor/dist/cjs/utils";

describe("sol-strike", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();

  const program = anchor.workspace.SolStrike as Program<SolStrike>;
  const signer = anchor.web3.Keypair.generate()

  async function airdropLamports(address: PublicKey, amount: number) {
    const signature = await program.provider.connection.requestAirdrop(address, amount);

    const latestBlockHash = await program.provider.connection.getLatestBlockhash();

    await program.provider.connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: signature,
    })
  }

  before(async () => {
    await airdropLamports(signer.publicKey, 1000 * LAMPORTS_PER_SOL);
  });

  it("Initialize", async () => {
    const [chipMintPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("CHIP_MINT")],
      program.programId
    );
    
    const [treasuryPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("TREASURY")],
      program.programId
    );

    await program.methods
      .initialize()
      .accountsStrict({
        chipMint: chipMintPDA,
        treasury: treasuryPDA,
        signer: signer.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: SYSTEM_PROGRAM_ID,
      })
      .signers([signer])
      .rpc();

    const chipMintAccount = await provider.connection.getAccountInfo(chipMintPDA);
    const treasuryAccount = await provider.connection.getAccountInfo(treasuryPDA);

    console.log("Chip Mint PDA:", chipMintPDA.toBase58());
    console.log("Treasury PDA:", treasuryPDA.toBase58());

    assert(chipMintAccount !== null, "chip_mint PDA was not created!");
    assert(treasuryAccount !== null, "treasury PDA was not created!");
  });

  it("Add token", async () => {
    const [treasuryPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("TREASURY")],
      program.programId
    );

    const tokenMint = await createMint(
      provider.connection,
      signer,
      signer.publicKey,
      null,
      9,
      Keypair.generate(),
      undefined,
      TOKEN_2022_PROGRAM_ID
    )

    const [chipTokenPriceStatePDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [tokenMint.toBuffer()],
      program.programId
    );

    const treasuryAta = await getAssociatedTokenAddress(
      tokenMint,
      treasuryPDA,
      true,
      TOKEN_2022_PROGRAM_ID
    )

    await program.methods
      .addToken(new BN(123))
      .accountsStrict({
        chipTokenPriceState: chipTokenPriceStatePDA,
        treasury: treasuryPDA,
        treasuryTokenAccount: treasuryAta,
        signer: signer.publicKey,
        paymentTokenMint: tokenMint,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SYSTEM_PROGRAM_ID,
      })
      .signers([signer])
      .rpc()

    const chipTokenPriceState = await program.account.chipTokenPriceState.fetch(
      chipTokenPriceStatePDA
    );
    expect(chipTokenPriceState.tokenPrice.toNumber()).to.equal(123);

    const treasuryAtaInfo = await getAccount(provider.connection, treasuryAta, 'processed', TOKEN_2022_PROGRAM_ID);
    console.log("Treasury token balance:", treasuryAtaInfo.amount);
    expect(Number(treasuryAtaInfo.amount)).to.be.equal(0);
  })
});
