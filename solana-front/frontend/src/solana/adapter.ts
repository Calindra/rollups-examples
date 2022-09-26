import { clusterApiUrl, ConfirmOptions, Connection, Keypair, PublicKey, SerializeConfig, Signer, Transaction, TransactionSignature } from "@solana/web3.js";
import { Buffer } from 'buffer';

import * as anchor from "@project-serum/anchor";
import idl from './models/solzen.json';
import { AnchorProvider, Program } from "@project-serum/anchor";
import { Solzen } from "./models/solzen";
import * as secp from '@noble/secp256k1';
import { Wallet } from "@project-serum/anchor/dist/cjs/provider";

const programID = new PublicKey(idl.metadata.address);
const encoder = new TextEncoder()

export const toBuffer = (arr: Buffer | Uint8Array | Array<number>): Buffer => {
    if (Buffer.isBuffer(arr)) {
        return arr;
    } else if (arr instanceof Uint8Array) {
        return Buffer.from(arr.buffer, arr.byteOffset, arr.byteLength);
    } else {
        return Buffer.from(arr);
    }
};

export async function findDaoAddress(daoSlug: string) {
    return await PublicKey.findProgramAddress([
        encoder.encode('dao'),
        Buffer.from(daoSlug.slice(0, 32)),
    ], programID)
}

export async function findValidationAddress(daoPubkey: PublicKey, walletPublicKey: PublicKey) {
    return await PublicKey.findProgramAddress([
        anchor.utils.bytes.utf8.encode('child'),
        walletPublicKey.toBuffer(),
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
    public payer = Keypair.fromSecretKey(Uint8Array.from([
        121, 122, 251, 173, 123, 1, 141, 44, 75, 160, 11,
        107, 14, 238, 24, 175, 213, 180, 116, 96, 185, 108,
        36, 202, 121, 64, 84, 243, 230, 252, 143, 86, 23,
        38, 214, 28, 85, 180, 211, 69, 250, 22, 31, 72,
        53, 69, 227, 12, 92, 172, 150, 196, 4, 59, 219,
        216, 77, 34, 176, 132, 80, 157, 198, 198
    ]))
    private _publicKey: PublicKey = this.payer.publicKey;

    async signTransaction(tx: anchor.web3.Transaction): Promise<anchor.web3.Transaction> {
        console.log('signTransaction...')
        // const buffer = tx.serialize()
        // console.log(buffer)
        const msg = tx.compileMessage()
        // // console.log(msg)
        console.log(msg.accountKeys.map(k => k.toBase58()))
        const signature = Buffer.alloc(64);
        for (let i = 0; i < signature.length; i++) {
            signature[i] = 1;
        }
        tx.addSignature(this._publicKey, signature);

        // const serializedMessage = msg.serialize();
        // console.log({ serializedMessage })
        tx.serialize = function (_config?: SerializeConfig): Buffer {
            const signData = this.serializeMessage();
            return (this as any)._serialize(signData);
        }
        return tx;
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

export function getProvider() {
    const commitment = 'processed';
    const network = clusterApiUrl('devnet');
    const connection = new Connection(network, commitment)
    const wallet = new AdaptedWallet();
    const provider = new AnchorProvider(connection, wallet, { commitment });
    provider.sendAndConfirm = async function (tx: Transaction,
        signers?: Signer[],
        opts?: ConfirmOptions
    ): Promise<TransactionSignature> {
        if (opts === undefined) {
            opts = this.opts;
        }

        tx.feePayer = this.wallet.publicKey;
        tx.recentBlockhash = (
            await this.connection.getRecentBlockhash(opts.preflightCommitment)
        ).blockhash;

        tx = await this.wallet.signTransaction(tx);
        (signers ?? []).forEach((kp) => {
            tx.partialSign(kp);
        });

        const rawTx = tx.serialize();

        console.log({ rawTx: toBuffer(rawTx).toString('base64') })
        return { ok: 1 } as any
    }
    return { provider, wallet };
}

export async function getProgram() {
    const { provider } = getProvider();
    return new anchor.Program(idl as any, programID, provider) as Program<Solzen>;
}
