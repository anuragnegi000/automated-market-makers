import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AutomatedMarketMakers } from "../target/types/automated_market_makers";
import { BN } from "bn.js";
import {createAccount, createAssociatedTokenAccount, createMint, getAssociatedTokenAddress, mintTo} from "@solana/spl-token"
import { config } from "chai";

describe("automated-market-makers", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  
  const program = anchor.workspace.automatedMarketMakers as Program<AutomatedMarketMakers>;

  const signer=anchor.web3.Keypair.generate();
  let mintA:anchor.web3.PublicKey;
  let mintB:anchor.web3.PublicKey;
  let vaultA:anchor.web3.PublicKey;
  let vaultB:anchor.web3.PublicKey;
  let fee=new BN(1000); // 0.1% fee`

  it("Is initialized!", async () => {

    console.log("Reached here=========")
    // const configPda=await anchor.web3.PublicKey.findProgramAddressSync(
    //   [Buffer.from("config"),mintA.publicKey.toBuffer(),mintB.publicKey.toBuffer()],program.programId
    // )
    // const mint_lp=await anchor.web3.PublicKey.findProgramAddressSync(
    //   [Buffer.from("mint_lp"),configPda[0].toBuffer()],program.programId
    // )
    // console.log("Config PDA:", configPda[0].toBase58());
    // Add your test here.

    const airdrop=await program.provider.connection.requestAirdrop(
      signer.publicKey,
      anchor.web3.LAMPORTS_PER_SOL*4
    )
    await program.provider.connection.confirmTransaction(
      airdrop,
      "confirmed"
    )

    console.log("Airdrop successfull for",signer.publicKey.toBase58())
    
    mintA=await createMint(
      program.provider.connection,
      signer,
      signer.publicKey,
      null,
      6
    )
    console.log("Mint is minted==========")
    mintB=await createMint(
      program.provider.connection,
      signer,
      signer.publicKey,
      null,
      6
    )

    const configPda=await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("config"),mintA.toBuffer(),mintB.toBuffer()],program.programId
    )
    console.log("Config PDA:", configPda[0].toString());
   
    const mint_lp=await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("lp"),configPda[0].toBuffer()],program.programId
    )
    console.log("MInt LP",mint_lp[0].toBase58());
    vaultA=await getAssociatedTokenAddress(
      mintA,
      configPda[0],
      true
    )
    console.log("Vault A:", vaultA.toBase58());
  
    vaultB=await getAssociatedTokenAddress(
      mintB,
      configPda[0],
      true
    )
    console.log("Vault B:",vaultB.toBase58())

    // const Mint=await mintTo(
    //   program.provider.connection,
    //   signer,
    //   mintA,
    //   vaultA,
    // )

    const tx = await program.methods
      .initialize(fee, null)
      .accounts({
        owner: signer.publicKey,
        mintA: mintA,
        mintB: mintB,
        vaultA: vaultA,
        vaultB: vaultB,
        config: configPda[0],
        mintLp: mint_lp[0],
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      }).signers([signer])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
