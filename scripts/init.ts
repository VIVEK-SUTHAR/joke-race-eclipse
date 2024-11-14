import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import { JokeRaceEclipse } from "../target/types/joke_race_eclipse";

const BN = anchor.BN;
type BN = anchor.BN;

async function main() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.JokeRaceEclipse as Program<JokeRaceEclipse>;

  const CONTEST_ID = new BN(0);
  const MIN_CONTESTANT = 1;
  const MAX_CONTESTANT = 19;
  const VOTE_INTERVAL = 100;
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
      const proposalPDA = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("contest"),
          provider.wallet.publicKey.toBuffer(),
          new BN(0).toArrayLike(Buffer, "le", 8),
        ],
        program.programId
      )[0];

      const currentTime = new Date();
      const startTime = new BN(
        Math.floor(currentTime.getTime() / 1000) + 30 * 60
      );

      const endDate = new Date("2024-11-15T18:00:00Z");
      const endTime = new BN(Math.floor(endDate.getTime() / 1000));

      await program.methods
        .createContest("https://sine", startTime, endTime)
        .accounts({
          contest: proposalPDA,
          author: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          contestCounter: getProposalCounterAddress(),
        })
        .rpc();
      console.log("Contest created successfully!");
    } catch (error) {
      console.error("Error creating contest:", error);
      throw error;
    }
  }
  const contestPDA = getContestPDA(new BN(0));
  console.log(`Contest created at address: ${contestPDA.toString()}`);
  console.log(`Contest ID: ${CONTEST_ID.toString()}`);
  console.log("----------------------------------------");
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
