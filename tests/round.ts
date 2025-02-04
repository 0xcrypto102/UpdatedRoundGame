import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Round } from "../target/types/round";
import { SystemProgram, Keypair, PublicKey, Transaction, SYSVAR_RENT_PUBKEY, SYSVAR_CLOCK_PUBKEY } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createAccount, createAssociatedTokenAccount, getAssociatedTokenAddress, ASSOCIATED_TOKEN_PROGRAM_ID, createMint, mintTo, mintToChecked, getAccount, getMint, getAssociatedTokenAddressSync } from "@solana/spl-token";
import * as bs58 from "bs58";
describe("round", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Round as Program<Round>;
  
  const owner = Keypair.fromSecretKey(
    bs58.default.decode("3qibRLkRmec7vrM1r2yaEjSYuRubghamp633kJ7CRqFBKGnd2zGUSDpF87xRqgeXCAdfFvCNVVnTxNp1z9jLTntd")
  );
  
  let user = Keypair.fromSecretKey(
    bs58.default.decode("4coaGcDANdLr6CAAXsCgLSyxL9NNs7p5E65teAmDGr3BQxc2UQHwsg2U3pDdmeeAT39eFQDi8BhuDTs9z1i5nyQK")
  );

  let user1 = Keypair.fromSecretKey(
    bs58.default.decode("4ZNguEsNiVjpZhbmoLNT5JxFoNV66frJRw2NBNsKacL1Bw8maajju1AXhVGrEQEYJqp9PNqu6Hf1dmynEwQkL63k")
  );
  let reference = new PublicKey("Hm2JsB9ftHyCiwCnb9oiP64zePdnCcZvhGBQViwfdFMU");
  const GLOBAL_STATE_SEED = "GLOBAL-STATE-SEED";
  const VAULT_SEED = "VAULT-SEED";
  const ROUND_SEED = "ROUND-SEED";
  const ROUN_USER_INFO_SEED = "ROUND-USER-INFO-SEED";

  let globalState, vault: PublicKey;
  let globalStateBump, vaultBump: number;
  let roundIndex = 1;


  it("GET PDA", async () => {
    [globalState, globalStateBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(GLOBAL_STATE_SEED)
      ],
      program.programId
    );
    console.log("globalState->", globalState.toString());

    [vault, vaultBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(VAULT_SEED)
      ],
      program.programId
    );
    console.log("vault->", vault.toString());

  });
  it("Is initialized!", async () => {
    // Add your test here.
    const slotTokenPrice = 10000000; // 0.01SOL
    const fee = 25;// (2.5%)
    const tx = await program.rpc.initialize(
      new anchor.BN(slotTokenPrice),
      new anchor.BN(fee),
      {
        accounts: {
          owner: owner.publicKey,
          globalState,
          vault,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId
        },
        signers: [owner]
      }
    );
    const globalStateData = await program.account.globalState.fetch(globalState);
    console.log(globalStateData);
  });
  /*
  it("dd", async() => {
    try {
      const [round, bump] = await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from(ROUND_SEED),
        ],
        program.programId
      );
      
      const roundData = await program.account.roundState.fetch(round);
      console.log("roundData->", roundData);
      const userInfo = await program.account.userInfo.all();
      console.log("users->", userInfo);
    } catch (error) {
      console.log(error);
    }
  })
    */
  /*
  it("create round 1", async () => {
    // Round 1
    const [round, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(ROUND_SEED),
      ],
      program.programId
    );
    const tx = await program.rpc.createRound(
      roundIndex,
      {
        accounts: {
          owner: owner.publicKey,
          globalState,
          round,
          systemProgram: SystemProgram.programId
        },
        signers: [owner]
      }
    );
    const roundData = await program.account.roundState.fetch(round);
    console.log("roundData->", roundData);
  }); 
  it("buy 1 slot in round 1", async () => {
    roundIndex = 1;

    const [round, bump1] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(ROUND_SEED),
      ],
      program.programId
    );

    const [userInfo, bump4] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(ROUN_USER_INFO_SEED),
        user.publicKey.toBuffer()
      ],
      program.programId
    );

    const amount = 1;
    const globalStateData = await program.account.globalState.fetch(globalState);
    console.log(globalStateData);

    try {
      // false: no chad mod
      // true: with chad mod
      const tx = await program.rpc.buySlot(
        roundIndex,
        new anchor.BN(amount),
        false,
        {
          accounts: {
            user: user.publicKey,
            owner: new PublicKey(globalStateData.owner),
            globalState,
            round,
            vault,
            userInfo,
            reference,
            systemProgram: SystemProgram.programId,
          },
          signers: [user]
        }
      );
      const roundData = await program.account.roundState.fetch(round);
      console.log("roundData->", roundData);
    } catch (error) {
      console.log(error);
    }
  });

  it("create round 2", async () => {
    // Round 2
    roundIndex = 2;
    const [round, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(ROUND_SEED),
      ],
      program.programId
    );
    const tx = await program.rpc.createRound(
      roundIndex,
      {
        accounts: {
          owner: owner.publicKey,
          globalState,
          round,
          systemProgram: SystemProgram.programId
        },
        signers: [owner]
      }
    );
    const roundData = await program.account.roundState.fetch(round);
    console.log("roundData->", roundData);
    console.log("tx->", tx);
  }); 

  it("buy 2 slot in round 2", async () => {
    roundIndex = 2;

    const [round, bump1] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(ROUND_SEED),
      ],
      program.programId
    );

    const [userInfo, bump4] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(ROUN_USER_INFO_SEED),
        user.publicKey.toBuffer()
      ],
      program.programId
    );

    const amount = 2;
    const globalStateData = await program.account.globalState.fetch(globalState);

    try {
      const tx = await program.rpc.buySlot(
        roundIndex,
        new anchor.BN(amount),
        false,
        {
          accounts: {
            user: user.publicKey,
            owner: new PublicKey(globalStateData.owner),
            globalState,
            round,
            vault,
            userInfo,
            reference,
            systemProgram: SystemProgram.programId,
          },
          signers: [user]
        }
      );
      const roundData = await program.account.roundState.fetch(round);
      console.log("roundData->", roundData);
      console.log("tx->", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("create round 3", async () => {
    // Round 2
    roundIndex = 3;
    const [round, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(ROUND_SEED),
      ],
      program.programId
    );
    const tx = await program.rpc.createRound(
      roundIndex,
      {
        accounts: {
          owner: owner.publicKey,
          globalState,
          round,
          systemProgram: SystemProgram.programId
        },
        signers: [owner]
      }
    );
    const roundData = await program.account.roundState.fetch(round);
    console.log("roundData->", roundData);
    console.log("tx->", tx);
  }); 

  it("buy 4 with user slot with chadmod in round 3", async () => {
    roundIndex = 3;

    const [round, bump1] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(ROUND_SEED),
      ],
      program.programId
    );

    const [userInfo, bump4] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(ROUN_USER_INFO_SEED),
        user.publicKey.toBuffer()
      ],
      program.programId
    );

    const amount = 4;
    const globalStateData = await program.account.globalState.fetch(globalState);

    try {
      const tx = await program.rpc.buySlot(
        roundIndex,
        new anchor.BN(amount),
        true,
        {
          accounts: {
            user: user.publicKey,
            owner: new PublicKey(globalStateData.owner),
            globalState,
            round,
            vault,
            userInfo,
            reference,
            systemProgram: SystemProgram.programId,
          },
          signers: [user]
        }
      );
      const roundData = await program.account.roundState.fetch(round);
      console.log("roundData->", roundData);
      console.log("tx->", tx);
      const userInfoData = await program.account.userInfo.fetch(userInfo);
      console.log("userInfoData->", userInfoData);

    } catch (error) {
      console.log(error);
    }
  });

  it("de-active chad mod", async() => {
    try {
      const [round, bump] = await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from(ROUND_SEED),
        ],
        program.programId
      );

      const [userInfo, bump4] = await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from(ROUN_USER_INFO_SEED),
          user.publicKey.toBuffer()
        ],
        program.programId
      );

      const tx = await program.rpc.deactiveChadMod(
        {
          accounts: {
            user: user.publicKey,
            userInfo,
            round
          },
          signers: [user]
        }
      );
      console.log("tx->", tx);
    } catch (error) {
      console.log(error);
    }
  });
 

  it("claim slot in round 3", async () => {
    roundIndex = 3;

    const [round, bump1] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(ROUND_SEED),
      ],
      program.programId
    );

    const [userInfo, bump4] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(ROUN_USER_INFO_SEED),
        user.publicKey.toBuffer()
      ],
      program.programId
    );

    try {
      const balance = await program.provider.connection.getBalance(vault);
      const lamportBalance = (balance / 1000000000);
  
      const userInfoData = await program.account.userInfo.fetch(userInfo);
      console.log("userInfoData->", userInfoData);
      const globalStateData = await program.account.globalState.fetch(globalState);
      console.log("globalStateData->", globalStateData);

      const tx = await program.rpc.claimSlot(
        {
          accounts: {
            user: user.publicKey,
            globalState,
            owner: globalStateData.owner,
            vault,
            reference,
            userInfo,
            round,
            systemProgram: SystemProgram.programId
          },
          signers: [user]
        }
      );
      const roundData = await program.account.roundState.fetch(round);
      console.log("roundData->", roundData);
      console.log("tx->", tx);
    } catch (error) {
      console.log(error);
    }

  });
  it("withdraw sol", async () => {
    let balance = await program.provider.connection.getBalance(vault);
    let lamportBalance = (balance / 1000000000);
    console.log("lamportBalance before withdraw->", lamportBalance);

    const tx = await program.rpc.withdrawSol(
      new anchor.BN(balance),
      {
        accounts: {
          owner: owner.publicKey,
          globalState,
          vault,
          systemProgram: SystemProgram.programId
        },
        signers: [owner]
      }
    );
    balance = await program.provider.connection.getBalance(vault);
    lamportBalance = (balance / 1000000000);
    console.log("lamportBalance after withdraw->", lamportBalance);
  });
  */
});
