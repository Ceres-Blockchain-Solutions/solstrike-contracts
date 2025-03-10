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

describe("sol-strike", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolStrike as Program<SolStrike>;
  const signer = anchor.web3.Keypair.generate()
  console.log("Signer:", signer.publicKey.toBase58())

  let tokenMint: anchor.web3.PublicKey
  let usersMintsAndTokenAccounts: { users: Keypair[]; mints: Keypair[]; tokenAccounts: PublicKey[][]; };
  let user: anchor.web3.Signer;
  let userChipTokenAccountAddress: PublicKey;

  const [chipMintPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("CHIP_MINT")],
    program.programId
  );

  before(async () => {
    await airdropLamports(signer.publicKey, 1000 * LAMPORTS_PER_SOL);

    usersMintsAndTokenAccounts = await createAccountsMintsAndTokenAccounts(
      [
        [1_000_000_000], 
      ],
      1 * LAMPORTS_PER_SOL,
      provider.connection,
      signer,
    );

    user = usersMintsAndTokenAccounts.users[0]
    userChipTokenAccountAddress = await getAssociatedTokenAddress(chipMintPDA, user.publicKey, false, TOKEN_2022_PROGRAM_ID);
  });

  it("Initialize", async () => {
    const [treasuryPDA] = PublicKey.findProgramAddressSync(
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

  it("Init global config", async () => {
    const [globalConfigPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("GLOBAL_CONFIG")],
      program.programId
    );

    await program.methods
      .initGlobalConfig(new anchor.BN(0.01 * LAMPORTS_PER_SOL))
      .accountsStrict({
        globalConfig: globalConfigPDA,
        signer: signer.publicKey,
        systemProgram: SYSTEM_PROGRAM_ID,
      })
      .signers([signer])
      .rpc();

    const globalConfigAccount = await program.account.globalConfig.fetch(globalConfigPDA);
    console.log("Global config PDA:", globalConfigPDA.toBase58());
    console.log("Global config:", globalConfigAccount);
  })

  it("Buy Chips with SOL", async () => {
    const userAccountBalanceBefore = await provider.connection.getBalance(user.publicKey)
    console.log("User balance before: ", userAccountBalanceBefore)

    await program.methods.buyChipWithSol(new BN(7))
    .accountsPartial({
      buyer: user.publicKey,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    })
    .signers([user])
    .rpc()

    const userAccountBalanceAfter = await provider.connection.getBalance(user.publicKey)
    console.log("User balance after: ", userAccountBalanceAfter)
    
    const userChipTokenAccountAfter = await getAccount(provider.connection, userChipTokenAccountAddress, 'processed', TOKEN_2022_PROGRAM_ID);
    console.log("User chip balance after: ", userChipTokenAccountAfter.amount)

  })

  it("Add token", async () => {
    const [treasuryPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("TREASURY")],
      program.programId
    );

    tokenMint = await createMint(
      provider.connection,
      signer,
      signer.publicKey,
      null,
      9,
      Keypair.generate(),
      undefined,
      TOKEN_2022_PROGRAM_ID
    )

    const [chipTokenPriceStatePDA] = PublicKey.findProgramAddressSync(
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

  it("Update token price", async () => {    
    const [chipTokenPriceStatePDA] = PublicKey.findProgramAddressSync(
      [tokenMint.toBuffer()],
      program.programId
    )

    const chipTokenPriceState = await program.account.chipTokenPriceState.fetch(
      chipTokenPriceStatePDA
    );
    expect(chipTokenPriceState.tokenPrice.toNumber()).to.equal(123);

    await program.methods
      .updateChipTokenPrice(new BN(200))
      .accountsStrict({
        chipTokenPriceState: chipTokenPriceStatePDA,
        signer: signer.publicKey,
        tokenMint: tokenMint,
      })
      .signers([signer])
      .rpc()

    const chipTokenUpdatedPriceState = await program.account.chipTokenPriceState.fetch(
      chipTokenPriceStatePDA
    );
    expect(chipTokenUpdatedPriceState.tokenPrice.toNumber()).to.equal(200);
  })

  it("Buy chip", async () => {
    const paymentTokeMint = usersMintsAndTokenAccounts.mints[0]
    const userPaymentTokenAccount = usersMintsAndTokenAccounts.tokenAccounts[0][0]


    const [treasuryPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("TREASURY")],
      program.programId
    );

    const [chipTokenPriceStatePDA] = PublicKey.findProgramAddressSync(
      [paymentTokeMint.publicKey.toBuffer()],
      program.programId
    );

    const treasuryAta = await getAssociatedTokenAddress(
      paymentTokeMint.publicKey,
      treasuryPDA,
      true,
      TOKEN_2022_PROGRAM_ID
    )

    await program.methods
      .addToken(new BN(500))
      .accountsStrict({
        chipTokenPriceState: chipTokenPriceStatePDA,
        treasury: treasuryPDA,
        treasuryTokenAccount: treasuryAta,
        signer: user.publicKey,
        paymentTokenMint: paymentTokeMint.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SYSTEM_PROGRAM_ID,
      })
      .signers([user])
      .rpc()

    const [chipMintPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("CHIP_MINT")],
      program.programId
    );

    const buyerChipAccount = await getAssociatedTokenAddress(
      chipMintPDA,
      user.publicKey,
      false,
      TOKEN_2022_PROGRAM_ID
    )
    
    await program.methods
      .buyChip(new BN(100))
      .accountsStrict({
        chipTokenPriceState: chipTokenPriceStatePDA,
        treasury: treasuryPDA,
        treasuryTokenAccount: treasuryAta,
        buyer: user.publicKey,
        chipMint: chipMintPDA,
        buyerChipAccount: buyerChipAccount,
        buyerTokenAccount: userPaymentTokenAccount,
        paymentTokenMint: paymentTokeMint.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SYSTEM_PROGRAM_ID,
      })
      .signers([user])
      .rpc()

    const userAccountbalance = await provider.connection.getTokenAccountBalance(userPaymentTokenAccount)
    const treasuryAccountBalance = await provider.connection.getTokenAccountBalance(treasuryAta)
    const buyerChipAccountBalance = await provider.connection.getTokenAccountBalance(buyerChipAccount)


    console.log("User balance: " + userAccountbalance.value.amount)
    console.log("Treasury balance: " + treasuryAccountBalance.value.amount)
    console.log("User chip balance: " + buyerChipAccountBalance.value.amount)
  })

  async function airdropLamports(address: PublicKey, amount: number) {
    const signature = await program.provider.connection.requestAirdrop(address, amount);

    const latestBlockHash = await program.provider.connection.getLatestBlockhash();

    await program.provider.connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: signature,
    })
  }
});
