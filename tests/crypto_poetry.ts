import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CryptoPoetry } from "../target/types/crypto_poetry";
const { Keypair } = anchor.web3;

describe("crypto_poetry", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CryptoPoetry as Program<CryptoPoetry>;

  it("Is initialized!", async () => {
    // Add your test here.

    // Create a new account for the poem
    const poemAccount = Keypair.generate();

    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const tx = await program.methods.initialize().accounts({
      poetryAccount: poemAccount.publicKey,
      user: provider.wallet.publicKey,
    }).signers([poemAccount]).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Generates poetry!", async () => {
    const poemAccount = Keypair.generate();

    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    // Initialize account
    await program.methods.initialize().accounts({
      poetryAccount: poemAccount.publicKey,
      user: provider.wallet.publicKey,
    }).signers([poemAccount]).rpc();
  
    // Generate poetry
    const tx = await program.methods.generatePoetry().accounts({
      poetryAccount: poemAccount.publicKey,
    }).rpc();
  
    console.log("Your transaction signature", tx);
  
    // Fetch and check the poetry account state
    const account = await program.account.poetryAccount.fetch(poemAccount.publicKey);
    console.log("Generated Poem:", account.poem);
  });

  it ("Can close poetry account", async () => {
    const poemAccount = Keypair.generate();

    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    // Initialize account
    await program.methods.initialize().accounts({
      poetryAccount: poemAccount.publicKey,
      user: provider.wallet.publicKey,
    }).signers([poemAccount]).rpc();

    // Close account
    const tx = await program.methods.closePoetryAccount().accounts({
      poetryAccount: poemAccount.publicKey,
      owner: provider.wallet.publicKey,
    }).signers([]).rpc();

    console.log("Your transaction signature", tx);

    // Check if account is closed
    try {
      await program.account.poetryAccount.fetch(poemAccount.publicKey);
    } catch (err) {
      console.log("Account closed successfully!");
    }
  });
});
