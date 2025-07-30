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
  let configPda:anchor.web3.PublicKey;
  let mint_lp:anchor.web3.PublicKey;
  let usertokenAccountA:anchor.web3.PublicKey;
  let usertokenAccountB:anchor.web3.PublicKey;
  let usertokenAccountLp:anchor.web3.PublicKey;

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

     configPda = (await anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("config"), mintA.toBuffer(), mintB.toBuffer()], 
  program.programId
))[0];
    console.log("Config PDA:", configPda.toString());

    mint_lp=(await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("lp"),configPda.toBuffer()],program.programId
    ))[0];
    
    console.log("MInt LP",mint_lp.toBase58());
    vaultA=await getAssociatedTokenAddress(
      mintA,
      configPda,
      true
    )
    console.log("Vault A:", vaultA.toBase58());
  
    vaultB=await getAssociatedTokenAddress(
      mintB,
      configPda,
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
  it("Deposit Initial Liquidity", async () => {
    usertokenAccountA=await createAssociatedTokenAccount(
      program.provider.connection,
      signer,
      mintA,
      signer.publicKey,
    )
    usertokenAccountB=await createAssociatedTokenAccount(
      program.provider.connection,
      signer,
      mintB,
      signer.publicKey,
    )
    usertokenAccountLp=await createAssociatedTokenAccount(
      program.provider.connection,
      signer,
      mint_lp,
      signer.publicKey
    )
    let minted_to_a=await mintTo(
      program.provider.connection,
      signer,
      mintA,
      usertokenAccountA,
      signer.publicKey,
      10000000000
    )
    console.log("Minted to A:", minted_to_a);
    let minted_to_b=await mintTo(
      program.provider.connection,
      signer,
      mintB,
      usertokenAccountB,
      signer.publicKey,
      10000000000
    )
    console.log("Minted to B:", minted_to_b);
    console.log("User Token Account A:", usertokenAccountA.toBase58());
    console.log("User Token Account B:", usertokenAccountB.toBase58());
    const tx=await program.methods.deposit(new BN(1000000),new BN(10000000),new BN(40000000)).accounts({
      user: signer.publicKey,
      mintA: mintA,
      mintB:mintB,
      config:configPda,
      vaultA:vaultA,
      vaultB:vaultB,
      usertokenAccountA:usertokenAccountA,
      usertokenAccountB:usertokenAccountB,
      usertokenAccountLp:usertokenAccountLp
    }).signers([signer]).rpc();
  })
});
