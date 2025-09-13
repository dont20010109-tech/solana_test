// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { Demo } from "../target/types/demo";

// describe("demo", () => {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());

//   const program = anchor.workspace.demo as Program<Demo>;

//   it("Is initialized!", async () => {
//     // Add your test here.
//     const tx = await program.methods.initialize().rpc();
//     console.log("Your transaction signature", tx);
//   });
// });

import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { HelloAnchor } from "../target/types/hello_anchor";
import { Keypair } from "@solana/web3.js";
import assert from "assert";
 
describe("hello_anchor", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.HelloAnchor as Program<HelloAnchor>;
 
  it("initialize", async () => {
    // Generate keypair for the new account
    const newAccountKp = new Keypair();
 
    // Send transaction
    const data = new BN(42);


    const transactionSignature = await program.methods
      .initialize(data)
      .accounts({
        newAccount: newAccountKp.publicKey,
        signer: wallet.publicKey,
      })
      .signers([newAccountKp])
      .rpc();
 
    // Fetch the created account
    const newAccount = await program.account.newAccount.fetch(
      newAccountKp.publicKey,
    );
 
    console.log("Transaction signature: ", transactionSignature);
    console.log("On-chain data is:", newAccount.data.toString());
    assert(data.eq(newAccount.data));
  });
});
