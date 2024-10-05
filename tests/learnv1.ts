import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Learnv1 } from "../target/types/learnv1";

// Maca unit test
describe("learnv1", () => {
  // Configure the client to use the local cluster or environment
  anchor.setProvider(anchor.AnchorProvider.env());

  // Creates a reference to the program we just built
  const program = anchor.workspace.Learnv1 as Program<Learnv1>;

  it("Is initialized!", async () => {
    // Add your test here.
    // Call the 'initialize' method, that is defined in our program
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
