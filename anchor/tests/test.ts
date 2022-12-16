import * as anchor from "@project-serum/anchor";

describe("hello-anchor", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const idl = require("../idl.json");
    const program = new anchor.Program(
        idl,
        new anchor.web3.PublicKey("CFzoprd9wxj64pZKCMQaRtpZWeJPgZ5WFEzhewDLjZH4"),
        provider
    )
    const payer = provider.wallet as anchor.Wallet;

    it("Say hello!", async () => {
        await program.methods.hello()
            .accounts({
                payer: payer.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([payer.payer])
            .rpc();
    });
});