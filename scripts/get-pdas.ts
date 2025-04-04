import { Program, AnchorProvider, setProvider } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { clusterApiUrl, Connection, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import type { SolStrike } from "../target/types/sol_strike";
import idl from "../target/idl/sol_strike.json";
import { getMint, TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";


anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.SolStrike as Program<SolStrike>;
const connection = anchor.getProvider().connection;

(async () => {
    const [chipMintPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from("CHIP_MINT")],
        program.programId
    );
    const chipMintData = await getMint(connection, chipMintPDA, 'confirmed', TOKEN_2022_PROGRAM_ID);
    console.log("Chip mint data:", JSON.stringify(chipMintData, (key, value) =>
        typeof value === 'bigint'
            ? value.toString()
            : value // return everything else unchanged
        , 2));

    // const allChipTokenPriceState = await program.account.chipTokenPriceState.all();
    // console.log("\nAllChipTokenPriceState:", allChipTokenPriceState);

    const globalConfig = await program.account.globalConfig.all();
    console.log("\nGlobal config", globalConfig.map(gc => ({
        publicKey: gc.publicKey.toString(),
        solChipPrice: Number(gc.account.solChipPrice.toString()) / LAMPORTS_PER_SOL,
        bump: gc.account.bump,
    })));

    const treasury = await program.account.treasury.all();
    const treasuryPubkeys = treasury.map(tr => tr.publicKey);

    // Batch fetch balances
    const balances = await connection.getMultipleAccountsInfo(treasuryPubkeys);

    console.log("\nTreasury:", treasury.map((tr, i) => ({
        publicKey: tr.publicKey.toString(),
        balance: balances[i] ? balances[i].lamports / LAMPORTS_PER_SOL : null,
        bump: tr.account.bump,
    })));
})();