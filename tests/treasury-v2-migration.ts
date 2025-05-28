import { Program } from "@coral-xyz/anchor";
import { describe } from "mocha";
import { SolStrike } from "../target/types/sol_strike";
import * as anchor from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";

describe("Migrations", () => {
    it("should run migrations without errors", async () => {
        const provider = anchor.AnchorProvider.env();
        anchor.setProvider(provider);
        const program = anchor.workspace.SolStrike as Program<SolStrike>;
        const connection = anchor.getProvider().connection;

        const [treasuryPDA] = PublicKey.findProgramAddressSync(
            [Buffer.from("TREASURY")],
            program.programId
        );

        const treasuryAccountInfoBefore = await connection.getAccountInfo(treasuryPDA);
        console.log("Treasury PDA account info before migration:", treasuryAccountInfoBefore);


        let tx = await program.methods.migrateTreasuryToV2()
            .accounts({
                signer: provider.wallet.publicKey,
                // treasury: treasuryPDA,
                // systemProgram: SystemProgram.programId,
            })
            .rpc();

        console.log("Migration transaction signature:", tx);

        // Verify the migration was successful by checking the state of the treasury PDA
        const treasuryAccount = await program.account.treasury.fetch(treasuryPDA);
        console.log("Treasury PDA after migration:", treasuryAccount);

        // Verify the migration was successful by checking the state of the treasury PDA
        const treasuryAccountInfoAfter = await connection.getAccountInfo(treasuryPDA);
        console.log("Treasury PDA account info before migration:", treasuryAccountInfoAfter);
    });
}
);