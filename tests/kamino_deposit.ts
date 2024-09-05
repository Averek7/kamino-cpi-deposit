import { assert } from "chai";
import { Keypair, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import {
  KaminoMarket,
  KaminoAction,
  VanillaObligation,
  PROGRAM_ID,
  getAssociatedTokenAddress,
} from "@kamino-finance/klend-sdk";
import { BN } from "@coral-xyz/anchor";
import { KaminoDeposit } from "../target/types/kamino_deposit";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  Token,
} from "@solana/spl-token";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const MAINNET_LENDING_MARKET = new PublicKey(
  "7u3HeHxYDLhnCoErrtycNokbQYbWGzLs6JSDqGAv5PfF"
);

describe("Kamino Operations", () => {
  const depositAmount = new BN(1 * 10 ** 9);
  let program: anchor.Program<KaminoDeposit>;
  let payer: Keypair;
  let obligation: Keypair;
  let reserve: Keypair;
  let reserveFarmState: Keypair;
  let obligationFarm: Keypair;
  let ownerUserMetadata: Keypair;
  let instrutionSysvar: Keypair;
  let obligationFarmUserState: Keypair;
  let reserveLiquidityMint: Keypair;
  let reserveCollateralMint: Keypair;
  let reserveLiquiditySupply: PublicKey;
  let reserveDestinationDepositCollateral: PublicKey;
  let userSourceLiquidity: PublicKey;
  let placeholderUserDestinationCollateral: PublicKey;

  before(async () => {
    // Initialize variables before running tests
    program = anchor.workspace.KaminoDeposit as anchor.Program<KaminoDeposit>;
    payer = Keypair.generate();
    obligation = Keypair.generate();
    reserve = Keypair.generate();
    reserveFarmState = Keypair.generate();
    obligationFarm = Keypair.generate();
    ownerUserMetadata = Keypair.generate();
    instrutionSysvar = Keypair.generate();
    obligationFarmUserState = Keypair.generate();
    reserveLiquidityMint = Keypair.generate();
    reserveCollateralMint = Keypair.generate();

    // Set up token accounts
    reserveLiquiditySupply = await createTokenAccount(
      provider.connection,
      payer,
      reserveLiquidityMint.publicKey,
      payer.publicKey
    );

    reserveDestinationDepositCollateral = await createTokenAccount(
      provider.connection,
      payer,
      reserveCollateralMint.publicKey,
      payer.publicKey
    );

    userSourceLiquidity = await createTokenAccount(
      provider.connection,
      payer,
      reserveLiquidityMint.publicKey,
      provider.wallet.publicKey
    );

    placeholderUserDestinationCollateral = await createTokenAccount(
      provider.connection,
      payer,
      reserveCollateralMint.publicKey,
      provider.wallet.publicKey
    );
  });

  it("should execute Kamino operations", async () => {
    const kaminoMarket = await KaminoMarket.load(
      provider.connection,
      MAINNET_LENDING_MARKET,
      10000
    );

    assert.isNotNull(kaminoMarket, "Kamino Market not found!");

    const kaminoAction = await KaminoAction.buildDepositTxns(
      kaminoMarket,
      depositAmount,
      new PublicKey("So11111111111111111111111111111111111111112"),
      provider.wallet.publicKey,
      new VanillaObligation(PROGRAM_ID)
    );

    const allInstructions = [
      ...kaminoAction.setupIxs,
      ...kaminoAction.lendingIxs,
      ...kaminoAction.cleanupIxs,
    ];

    const tx = await program.methods
      .executeKaminoOperations(depositAmount)
      .accounts({
        obligationOwner: provider.wallet.publicKey,
        feePayer: payer.publicKey,
        obligation: obligation.publicKey,
        lendingMarket: MAINNET_LENDING_MARKET,
        lendingMarketAuthority: new PublicKey(
          "9DrvZvyWh1HuAoZxvYWMvkf2XCzryCpGgHqrMjyDWpmo"
        ),
        ownerUserMetadata: ownerUserMetadata.publicKey,
        reserve: reserve.publicKey,
        reserveFarmState: reserveFarmState.publicKey,
        obligationFarm: obligationFarm.publicKey,
        farmsProgram: new PublicKey(
          "FarmsPZpWu9i7Kky8tPN37rs2TpmMrAZrC7S7vJa91Hr"
        ),
        reserveLiquidityMint: reserveLiquidityMint.publicKey,
        reserveLiquiditySupply: reserveLiquiditySupply,
        reserveCollateralMint: reserveCollateralMint.publicKey,
        reserveDestinationDepositCollateral:
          reserveDestinationDepositCollateral,
        userSourceLiquidity: userSourceLiquidity,
        placeholderUserDestinationCollateral:
          placeholderUserDestinationCollateral,
        collateralTokenProgram: new PublicKey(
          "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        ),
        liquidityTokenProgram: new PublicKey(
          "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        ),
        instructionSysvar: instrutionSysvar.publicKey,
        kaminoProgram: new PublicKey(
          "KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD"
        ),
        crank: provider.wallet.publicKey,
        obligationFarmUserState: obligationFarmUserState.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .remainingAccounts(
        allInstructions.map((ix) => ({
          pubkey: ix.programId,
          isSigner: false,
          isWritable: false,
        }))
      )
      .signers([payer])
      .rpc();

    assert.isNotNull(tx, "Transaction failed");
    console.log("Transaction Signature:", tx);
  });
});

async function createTokenAccount(
  connection: anchor.web3.Connection,
  payer: Keypair,
  mint: PublicKey,
  owner: PublicKey
): Promise<PublicKey> {
  const associatedTokenAccount = await getAssociatedTokenAddress(
    mint,
    owner,
    true,
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID
  );

  // Check if the account already exists
  const accountInfo = await connection.getAccountInfo(associatedTokenAccount);
  if (!accountInfo) {
    // Create the associated token account instruction
    const transaction = new Transaction().add(
      Token.createAssociatedTokenAccountInstruction(
        payer.publicKey,
        associatedTokenAccount,
        owner,
        mint,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
      )
    );

    // Send the transaction to the blockchain and confirm it
    await connection.sendTransaction(transaction, [payer], {
      skipPreflight: false,
      preflightCommitment: "singleGossip",
    });
  }

  return associatedTokenAccount;
}
