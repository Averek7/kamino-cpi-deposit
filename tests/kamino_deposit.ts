import {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import {
  KaminoMarket,
  KaminoAction,
  VanillaObligation,
  PROGRAM_ID,
  DEFAULT_RECENT_SLOT_DURATION_MS,
} from "@kamino-finance/klend-sdk";
import fs from "fs";
import BN from "bn.js";
import { KaminoDeposit } from "../target/types/kamino_deposit";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const MAINNET_LENDING_MARKET = new PublicKey(
  "7u3HeHxYDLhnCoErrtycNokbQYbWGzLs6JSDqGAv5PfF"
);

const FARM = new PublicKey("FarmsPZpWu9i7Kky8tPN37rs2TpmMrAZrC7S7vJa91Hr");
const KAMINO_PROGRAM = new PublicKey(
  "KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD"
);

const connection = new Connection(
  "https://mainnet.helius-rpc.com/?api-key=91acf6dc-f1f0-4db8-9763-aff8b775fa6a"
);

const program = anchor.workspace.KaminoDeposit as anchor.Program<KaminoDeposit>;
describe("Exec Kamino", () => {
  let payer = Keypair.generate();
  const depositAmount = new BN(1 * 10 ** 9);

  it("deposit", async () => {
    try {
      const kaminoMarket = await KaminoMarket.load(
        connection,
        MAINNET_LENDING_MARKET,
        DEFAULT_RECENT_SLOT_DURATION_MS,
        PROGRAM_ID,
        true
      );

      const kaminoAction = await KaminoAction.buildDepositTxns(
        kaminoMarket,
        depositAmount,
        new PublicKey("So11111111111111111111111111111111111111112"),
        payer.publicKey,
        new VanillaObligation(PROGRAM_ID),
        1_000_000,
        true
      );

      console.log("Setup Instructions:", kaminoAction.setupIxs);
      //   console.log("Lending Instructions:", kaminoAction.lendingIxs);
      //   console.log("Cleanup Instructions:", kaminoAction.cleanupIxs);

      const allInstructions = [
        ...kaminoAction.setupIxs,
        ...kaminoAction.lendingIxs,
        ...kaminoAction.cleanupIxs,
      ];

      const kaminoIxs = allInstructions.filter((ix) =>
        ix.programId.equals(KAMINO_PROGRAM)
      );

      const allAccountMetas = kaminoIxs.flatMap((ix) => ix.keys);

      console.log(allAccountMetas.length);

      const ixDatas: Buffer[] = kaminoIxs.map((ix) => ix.data);

      console.log(ixDatas.length);

      const txn = await program.methods
        .executeKaminoOperations(ixDatas)
        .accounts({
          kaminoProgram: KAMINO_PROGRAM,
        })
        .remainingAccounts(allAccountMetas)
        .signers([payer])
        .rpc();

      console.log("Transaction signature:", txn);
    } catch (error) {
      console.error("Error:", error);
    }
  });
});
