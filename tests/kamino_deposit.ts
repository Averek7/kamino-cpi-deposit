import * as anchor from '@coral-xyz/anchor';
import { SystemProgram, Keypair, PublicKey } from '@solana/web3.js';
import { assert } from 'chai';

anchor.setProvider(anchor.AnchorProvider.env());

const idl = require('../target/idl/kamino_deposit.json');
const programId = new PublicKey('EQ4ZfkAaGPuuVGkaBUhdSi4QQ6RdkxNWQ8eCU96x2dQU'); 
const program = new anchor.Program(idl, programId);

// Helper function to generate a keypair and fund it
async function createAccount(connection: anchor.web3.Connection, payer: anchor.web3.Keypair) {
    const account = anchor.web3.Keypair.generate();
    const tx = await connection.requestAirdrop(account.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await connection.confirmTransaction(tx);
    return account;
}

describe('kamino_deposit', () => {
    let provider: anchor.AnchorProvider;
    let payer: anchor.web3.Keypair;
    let obligation: anchor.web3.Keypair;
    let lendingMarket: anchor.web3.Keypair;
    let reserve: anchor.web3.Keypair;

    before(async () => {
        provider = anchor.AnchorProvider.env();
        payer = new Keypair();

        // Initialize accounts
        obligation = await createAccount(provider.connection, payer);
        lendingMarket = await createAccount(provider.connection, payer);
        reserve = await createAccount(provider.connection, payer);

    });

    it('should execute Kamino operations correctly', async () => {
        const amount = new anchor.BN(5000000); 

        try {
            // Execute the Kamino operations
            await program.rpc.executeKaminoOperations(amount, {
                accounts: {
                    obligationOwner: payer.publicKey,
                    feePayer: payer.publicKey,
                    obligation: obligation.publicKey,
                    lendingMarket: lendingMarket.publicKey,
                    reserve: reserve.publicKey,
                    // Add other accounts here as per your contract requirements
                },
                signers: [payer],
            });
            console.log('Transaction successful!');
        } catch (err) {
            console.error('Error executing Kamino operations:', err);
            assert.fail('Transaction failed');
        }

        const obligationAccount = await program.account.obligation.fetch(obligation.publicKey);
        console.log('Obligation Account State:', obligationAccount);

        assert.isOk(obligationAccount, 'Obligation account should be initialized');
    });

});
