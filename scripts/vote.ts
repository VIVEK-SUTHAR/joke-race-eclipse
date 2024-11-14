import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { JokeRaceEclipse } from "../target/types/joke_race_eclipse";

const BN = anchor.BN;
type BN = anchor.BN;

async function main() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.JokeRaceEclipse as Program<JokeRaceEclipse>;

  const CONTEST_ID = new BN(0);
  const MIN_CONTESTANT = 1;
  const MAX_CONTESTANT = 29;
  const VOTE_INTERVAL = 500;
  const VOTE_AMOUNT = new BN(0.005 * anchor.web3.LAMPORTS_PER_SOL);
  const MAX_VOTES = 2000;
  let voteCount = 0;

  console.log("PID", program.programId.toString());

  function getProposalCounterAddress() {
    return anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("contest_counter")],
      program.programId
    )[0];
  }

  function getVaultAddress() {
    return anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault")],
      program.programId
    )[0];
  }

  function getContestPDA(contestId: BN) {
    return anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("contest"),
        provider.wallet.publicKey.toBuffer(),
        contestId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    )[0];
  }

  function getVoterRecordPDA(
    voter: anchor.web3.PublicKey,
    contestId: BN,
    contestantId: BN
  ) {
    return anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("vote_account"),
        voter.toBuffer(),
        contestId.toArrayLike(Buffer, "le", 8),
        contestantId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    )[0];
  }

  function getRandomContestantId(): BN {
    return new BN(
      Math.floor(
        Math.random() * (MAX_CONTESTANT - MIN_CONTESTANT + 1) + MIN_CONTESTANT
      )
    );
  }

  async function initializeContestCounter() {
    try {
      await program.methods
        .initContestCounter()
        .accounts({
          owner: provider.wallet.publicKey,
          contestCounter: getProposalCounterAddress(),
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      console.log("Contest counter initialized");
    } catch (error) {
      console.log("Contest counter might already exist:", error.message);
    }
  }

  async function createContest() {
    try {
      const contestPDA = getContestPDA(CONTEST_ID);
      // await program.methods
      //   .createContest(
      //     "https://example.com/contest-metadata",
      //     new BN(Date.now())
      //   )
      //   .accounts({
      //     contest: contestPDA,
      //     author: provider.wallet.publicKey,
      //     systemProgram: anchor.web3.SystemProgram.programId,
      //     contestCounter: getProposalCounterAddress(),
      //   })
      //   .rpc();
      console.log("Contest created successfully!");
      return contestPDA;
    } catch (error) {
      console.error("Error creating contest:", error);
      throw error;
    }
  }

  async function simulateVote(contestPDA: anchor.web3.PublicKey) {
    try {
      const contestantId = getRandomContestantId();
      const voterRecordPDA = getVoterRecordPDA(
        provider.wallet.publicKey,
        CONTEST_ID,
        contestantId
      );

      console.log(`Voting for contestant ${contestantId.toString()}...`);

      const tx = await program.methods
        .vote(VOTE_AMOUNT, contestantId)
        .accounts({
          contest: contestPDA,
          voter: provider.wallet.publicKey,
          vault: getVaultAddress(),
          voterRecord: voterRecordPDA,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      voteCount++;

      console.log(`Vote successful! Transaction: ${tx}`);
      console.log(`Vote count: ${voteCount}`);
      console.log(`Timestamp: ${new Date().toISOString()}`);
      console.log("----------------------------------------");

      if (voteCount >= MAX_VOTES) {
        console.log("Reached maximum vote count, stopping simulation.");
        clearInterval(voteInterval);
      }
    } catch (error) {
      voteCount--;
      console.error("Error while voting:", error);
    }
  }

  console.log("Starting vote simulator...");

  const contestPDA = new anchor.web3.PublicKey(
    "HjFvBz1GhpFtjEaAi4zNZULdf8R6WdU94v9avKqVNDbD"
  );

  console.log(`Contest created at address: ${contestPDA.toString()}`);
  console.log(`Contest ID: ${CONTEST_ID.toString()}`);
  console.log(`Voting interval: ${VOTE_INTERVAL / 4000} seconds`);
  console.log(`Contestant range: ${MIN_CONTESTANT} to ${MAX_CONTESTANT}`);
  console.log("----------------------------------------");

  await simulateVote(contestPDA);

  const voteInterval = setInterval(
    () => simulateVote(contestPDA),
    VOTE_INTERVAL
  );
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
