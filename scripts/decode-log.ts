import * as anchor from "@coral-xyz/anchor";
import {BorshCoder, EventParser, Program, web3} from "@coral-xyz/anchor";
import {SolStrike} from "../target/types/sol_strike";


anchor.setProvider(anchor.AnchorProvider.env());
const program = anchor.workspace.SolStrike as Program<SolStrike>;

(async () => {
let signature = '2NPD4o35cMa6x6RLhTbzHSAokxVMevkdJgpRprJd7bVtJcBx5YNh6kE1ZYwKashSnWPnJQpbs7ApLqL3WjeXT6QE';

// Get transaction from its signature
const tx = await anchor.getProvider().connection.getTransaction(signature, {
    commitment: "confirmed",
});

console.log("logs:", tx.meta.logMessages);

const eventParser = new EventParser(program.programId, new BorshCoder(program.idl));
const events = eventParser.parseLogs(tx.meta.logMessages);
console.log("Parsed events:", events);
for (let event of events) {
    console.log(event);
}
})();