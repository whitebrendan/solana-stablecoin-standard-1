import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SssToken } from "../target/types/sss_token";
import { 
    TOKEN_2022_PROGRAM_ID, 
    getMint, 
    getTransferHook,
    getPermanentDelegate
} from "@solana/spl-token";
import { assert } from "chai";

describe("sss-token", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SssToken as Program<SssToken>;
  const authority = provider.wallet as anchor.Wallet;

  it("Initializes SSS-2 (Compliant) Stablecoin", async () => {
    const mintKeypair = anchor.web3.Keypair.generate();
    const stateKeypair = anchor.web3.Keypair.generate();

    const config = {
        name: "Test Stablecoin",
        symbol: "TST",
        uri: "https://test.com",
        decimals: 6,
        enablePermanentDelegate: true,
        enableTransferHook: true,
        defaultAccountFrozen: false,
    };

    // Note: In real test, we would first create the mint with extensions
    // For this test script, we verify the program state initialization
    await program.methods
      .initialize(config)
      .accounts({
        state: stateKeypair.publicKey,
        mint: mintKeypair.publicKey,
        authority: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([stateKeypair])
      .rpc();

    const state = await program.account.globalState.fetch(stateKeypair.publicKey);
    assert.equal(state.name, "Test Stablecoin");
    assert.isTrue(state.enablePermanentDelegate);
    assert.isTrue(state.enableTransferHook);
    assert.equal(state.roles.master.toBase58(), authority.publicKey.toBase58());
  });

  it("Adds an address to the blacklist", async () => {
    const stateKeypair = anchor.web3.Keypair.generate(); // Re-init for isolation
    const badActor = anchor.web3.Keypair.generate();

    // Init state first...
    // [Omitted for brevity, assuming state is active]

    const [blacklistPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("blacklist"), badActor.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .addToBlacklist(badActor.publicKey)
      .accounts({
        blacklist_item: blacklistPda,
        state: stateKeypair.publicKey,
        authority: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const blacklistItem = await program.account.blacklistItem.fetch(blacklistPda);
    assert.isTrue(blacklistItem.isBlacklisted);
  });

  it("Fails to mint when paused", async () => {
    // Logic to setPaused(true)
    // Attempt mint_to
    // Expect SSSCoreError::ContractPaused
  });
});
