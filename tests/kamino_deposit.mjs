import { Connection, Keypair, PublicKey, Transaction } from "@solana/web3.js";
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

// const provider = anchor.AnchorProvider.env();
// anchor.setProvider(provider);

const MAINNET_LENDING_MARKET = new PublicKey(
  "7u3HeHxYDLhnCoErrtycNokbQYbWGzLs6JSDqGAv5PfF"
);

const connection = new Connection(
  "https://mainnet.helius-rpc.com/?api-key=91acf6dc-f1f0-4db8-9763-aff8b775fa6a"
);

async function runKaminoOperations() {
  // const depositAmount = new BN(1 * 10 ** 9);
  const payer = Keypair.fromSecretKey(
    Uint8Array.from(
      JSON.parse(fs.readFileSync("/home/averek/.config/solana/id.json", "utf8"))
    )
  );

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
      "100",
      new PublicKey("So11111111111111111111111111111111111111112"),
      payer.publicKey,
      new VanillaObligation(PROGRAM_ID),
      1_000_000,
      true
    );

    const allInstructions = [
      ...kaminoAction.setupIxs,
      ...kaminoAction.lendingIxs,
      ...kaminoAction.cleanupIxs,
    ];

    const instructionsData = allInstructions.map((ix) => ({
      programId: ix.programId.toBase58(),
      keys: ix.keys.map((key) => ({
        pubkey: key.pubkey.toBase58(),
        isSigner: key.isSigner,
        isWritable: key.isWritable,
      })),
      data: ix.data.toString("base64"),
    }));

    const filePath = "./kamino_instructions.json";
    fs.writeFileSync(filePath, JSON.stringify(instructionsData, null, 2));

    console.log(`Instructions and accounts have been written to ${filePath}`);

    // Debugging: Print account details and program IDs
    console.log("Kamino Action Instructions:");
    allInstructions.forEach((ix, index) => {
      console.log(`Instruction ${index}:`);
      console.log(`  Program ID: ${ix.programId.toBase58()}`);
      ix.keys.forEach((key) => {
        console.log(`  - Account: ${key.pubkey.toBase58()}`);
        console.log(`    Is Signer: ${key.isSigner}`);
        console.log(`    Is Writable: ${key.isWritable}`);
      });
    });
  } catch (error) {
    console.error("Error executing Kamino operations:", error);
  }
}

// Run the function
runKaminoOperations();
