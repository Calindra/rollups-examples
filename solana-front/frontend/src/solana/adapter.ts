import { clusterApiUrl, Connection, PublicKey } from "@solana/web3.js";
import * as anchor from "@project-serum/anchor";
import idl from './models/solzen.json';
import { AnchorProvider, Program } from "@project-serum/anchor";
import { Solzen } from "./models/solzen";
import * as secp from '@noble/secp256k1';
import { Wallet } from "@project-serum/anchor/dist/cjs/provider";

const programID = new PublicKey(idl.metadata.address);
const encoder = new TextEncoder()

export async function findDaoAddress(daoSlug: string) {
    return await PublicKey.findProgramAddress([
        encoder.encode('dao'),
        Buffer.from(daoSlug.slice(0, 32)),
    ], programID)
}

export async function findValidationAddress(daoPubkey: PublicKey) {
    return await PublicKey.findProgramAddress([
        anchor.utils.bytes.utf8.encode('child'),
        // wallet.publicKey.toBuffer(),
        daoPubkey.toBuffer(),
    ], programID);
}

export async function testSecp() {
    // keys, messages & other inputs can be Uint8Arrays or hex strings
    // Uint8Array.from([0xde, 0xad, 0xbe, 0xef]) === 'deadbeef'
    const message = anchor.utils.bytes.utf8.encode('hello world');
    const privKey = secp.utils.randomPrivateKey();
    const pubKey = secp.getPublicKey(privKey);
    const msgHash = await secp.utils.sha256(message);
    const [signature, recovery] = await secp.sign(msgHash, privKey, { recovered: true });
    const isValid = secp.verify(signature, msgHash, pubKey);

    const recoveredPubkey = secp.recoverPublicKey(msgHash, signature, recovery);
    console.log({ recoveredPubkey, pubKey });

    // Schnorr signatures
    const rpub = secp.schnorr.getPublicKey(privKey);
    const rsignature = await secp.schnorr.sign(message, privKey);
    const risValid = await secp.schnorr.verify(rsignature, message, rpub);
    console.log({ isValid, risValid })
}

class AdaptedWallet implements Wallet {
    signTransaction(tx: anchor.web3.Transaction): Promise<anchor.web3.Transaction> {
        throw new Error("Method not implemented.");
    }
    signAllTransactions(txs: anchor.web3.Transaction[]): Promise<anchor.web3.Transaction[]> {
        throw new Error("Method not implemented.");
    }
    get publicKey(): PublicKey {
        throw new Error("Method not implemented.");
    }
}

export function getProvider() {
    const commitment = 'processed';
    const network = clusterApiUrl('devnet')
    const connection = new Connection(network, commitment)
    const wallet = new AdaptedWallet();
    const provider = new AnchorProvider(connection, wallet, { commitment });
    return provider;
}

export async function getProgram() {
    const provider = getProvider();
    return new anchor.Program(idl as any, programID, provider) as Program<Solzen>;
}
