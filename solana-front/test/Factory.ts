/* eslint-disable lines-between-class-members */
/* eslint-disable no-unused-vars */
/* eslint-disable prettier/prettier */
import * as anchor from "@project-serum/anchor";
import { AnchorProvider, Program } from "@project-serum/anchor";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { createMint } from "@solana/spl-token";
import { clusterApiUrl, LAMPORTS_PER_SOL, SystemProgram, PublicKey, Connection, Keypair } from "@solana/web3.js";
import { Wallet } from "@project-serum/anchor/dist/cjs/provider";

class AdaptedWallet implements Wallet {
    private _publicKey: PublicKey = Keypair.generate().publicKey;

    signTransaction(tx: anchor.web3.Transaction): Promise<anchor.web3.Transaction> {
        throw new Error("Method not implemented.");
    }

    signAllTransactions(txs: anchor.web3.Transaction[]): Promise<anchor.web3.Transaction[]> {
        throw new Error("Method not implemented.");
    }

    get publicKey(): PublicKey {
        return this._publicKey;
    }

    set publicKey(pk) {
        this._publicKey = pk
    }
}

export default class Factory {

    static async getProvider() {
        const commitment = 'processed';
        const network = clusterApiUrl('devnet');
        const connection = new Connection(network, commitment)
        const wallet = new AdaptedWallet();
        const provider = new AnchorProvider(connection, wallet, { commitment });
        return { provider, wallet };
    }

    static async createMint() {
        const { provider, wallet } = await Factory.getProvider();
        const connection = provider.connection;
        const Keypair = anchor.web3.Keypair;
        const payer = Keypair.generate();
        wallet.publicKey = payer.publicKey;
        const mintAuthority = Keypair.generate();
        const freezeAuthority = Keypair.generate();

        const airdropSignature = await connection.requestAirdrop(
            payer.publicKey,
            LAMPORTS_PER_SOL,
        );

        await connection.confirmTransaction(airdropSignature, "confirmed");
        const mint = await createMint(
            connection,
            payer,
            mintAuthority.publicKey,
            freezeAuthority.publicKey,
            9 // We are using 9 to match the CLI decimal default exactly
        );
        return { mint, payer, mintAuthority, connection }
    }

}