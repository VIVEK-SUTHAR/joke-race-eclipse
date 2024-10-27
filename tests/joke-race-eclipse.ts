import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { JokeRaceEclipse } from "../target/types/joke_race_eclipse";
import { expect } from "chai";
import { BN } from "bn.js";

describe("JokeRace Tests On Eclipse", async () => {
  const valutKeyPair = anchor.web3.Keypair.generate();

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.JokeRaceEclipse as Program<JokeRaceEclipse>;

  function getProposalCounterAddress() {
    const pdaAddress = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("contest_counter")],
      program.programId
    )[0];
    return pdaAddress;
  }

  function getVaultAddress() {
    const pdaAddress = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault")],
      program.programId
    )[0];
    return pdaAddress;
  }

  let proposalCounterPDA;
  let proposalAddress;

  it("Initializes the vault account", async () => {
    await provider.connection.requestAirdrop(
      valutKeyPair.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 5
    );
    const vaultPubkey = getVaultAddress();
    await program.methods
      .initialize()
      .accounts({
        vault: vaultPubkey,
        owner: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    const vaultAccount = await program.account.vault.fetch(vaultPubkey);
    expect(vaultAccount.authority.toString()).to.equal(
      provider.wallet.publicKey.toString()
    );
  });

  it("Initializes the Proposal Counter Account", async () => {
    await program.methods
      .initContestCounter()
      .accounts({
        owner: provider.wallet.publicKey,
        contestCounter: getProposalCounterAddress(),
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    const proposalCounterPDA = await program.account.contestCounter.fetch(
      getProposalCounterAddress()
    );
    expect(proposalCounterPDA.count.toString()).to.equals("0");
  });

  it("Creates a contest", async () => {
    const proposalPDA = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("contest"),
        provider.wallet.publicKey.toBuffer(),
        new BN(0).toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    )[0];
    await program.methods
      .createContest("https://sine", new BN(Date.now()))
      .accounts({
        contest: proposalPDA,
        author: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        contestCounter: getProposalCounterAddress(),
      })
      .rpc();

    const proposalAccount = await program.account.contest.fetch(
      proposalPDA.toString()
    );
    proposalAddress = proposalPDA.toString();
    expect(proposalAccount.metadataUri).to.equal("https://sine");
    expect(proposalAccount.upvotes.toNumber()).to.equal(0);
    expect(proposalAccount.author.toString()).to.equal(
      provider.wallet.publicKey.toString()
    );
  });

  it("Upvotes on a contestant", async () => {
    const contestAccount = await program.account.contest.fetch(proposalAddress);
    const contestantId = new BN(1);
    const voterRecordPDA = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("vote_account"),
        provider.wallet.publicKey.toBuffer(),
        contestAccount.id.toArrayLike(Buffer, "le", 8),
        contestantId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    )[0];
    //0.20 Cents depending upon ETH Price From Coingeckk
    const tx = await program.methods
      .vote(new BN(0.1 * anchor.web3.LAMPORTS_PER_SOL), contestantId)
      .accounts({
        contest: proposalAddress,
        voter: provider.wallet.publicKey,
        vault: getVaultAddress(),
        voterRecord: voterRecordPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const updatedContestAccount = await program.account.contest.fetch(
      proposalAddress
    );
    expect(updatedContestAccount.upvotes.toNumber()).to.equal(1);
  });

  it("Shoule be able to Vote on Same contestant for second time", async () => {
    const contestAccount = await program.account.contest.fetch(proposalAddress);
    const contestantId = new BN(1);
    const voterRecordPDA = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("vote_account"),
        provider.wallet.publicKey.toBuffer(),
        contestAccount.id.toArrayLike(Buffer, "le", 8),
        contestantId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    )[0];

    const tx = await program.methods
      .vote(new BN(0.1 * anchor.web3.LAMPORTS_PER_SOL), contestantId)
      .accounts({
        contest: proposalAddress,
        voter: provider.wallet.publicKey,
        vault: getVaultAddress(),
        voterRecord: voterRecordPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const updatedContestAccount = await program.account.contest.fetch(
      proposalAddress
    );
    expect(updatedContestAccount.upvotes.toNumber()).to.equal(2);
  });

  it("Distributes the funds to the given address", async () => {
    const recipient = anchor.web3.Keypair.generate();
    const vaultPubkey = getVaultAddress();

    const initialVaultBalance = await provider.connection.getBalance(
      vaultPubkey
    );

    const distributeAmount = new BN(0.0005 * anchor.web3.LAMPORTS_PER_SOL);

    const rentExemptionAmount =
      await provider.connection.getMinimumBalanceForRentExemption(0);

    const tx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: provider.wallet.publicKey,
        newAccountPubkey: recipient.publicKey,
        lamports: rentExemptionAmount,
        space: 0,
        programId: anchor.web3.SystemProgram.programId,
      })
    );
    await provider.sendAndConfirm(tx, [recipient]);

    await program.methods
      .distribute(distributeAmount)
      .accounts({
        vault: vaultPubkey,
        authority: provider.wallet.publicKey,
        recipient: recipient.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const finalVaultBalance = await provider.connection.getBalance(vaultPubkey);
    const recipientBalance = await provider.connection.getBalance(
      recipient.publicKey
    );

    expect(finalVaultBalance).to.equal(
      initialVaultBalance - distributeAmount.toNumber()
    );

    expect(recipientBalance).to.equal(
      rentExemptionAmount + distributeAmount.toNumber()
    );
  });

  it("Fails to distribute funds with an incorrect authority", async () => {
    const recipient = anchor.web3.Keypair.generate();
    const vaultPubkey = getVaultAddress();

    const initialVaultBalance = await provider.connection.getBalance(
      vaultPubkey
    );
    console.log(
      "Initial Vault Balance",
      initialVaultBalance / anchor.web3.LAMPORTS_PER_SOL
    );

    const distributeAmount = new BN(0.05 * anchor.web3.LAMPORTS_PER_SOL);

    const rentExemptionAmount =
      await provider.connection.getMinimumBalanceForRentExemption(0);

    const tx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: provider.wallet.publicKey,
        newAccountPubkey: recipient.publicKey,
        lamports: rentExemptionAmount,
        space: 0,
        programId: anchor.web3.SystemProgram.programId,
      })
    );
    await provider.sendAndConfirm(tx, [recipient]);

    const incorrectAuthority = anchor.web3.Keypair.generate();

    try {
      await program.methods
        .distribute(distributeAmount)
        .accounts({
          vault: vaultPubkey,
          authority: incorrectAuthority.publicKey,
          recipient: recipient.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([incorrectAuthority])
        .rpc();

      throw new Error("failed due to unauthorized authority");
    } catch (err) {
      expect(err.error.errorMessage).to.include(
        "has one constraint was violated"
      );
    }

    const finalVaultBalance = await provider.connection.getBalance(vaultPubkey);
    const recipientBalance = await provider.connection.getBalance(
      recipient.publicKey
    );

    expect(finalVaultBalance).to.equal(initialVaultBalance);
    expect(recipientBalance).to.equal(rentExemptionAmount);
  });
});
