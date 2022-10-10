/* eslint-disable @typescript-eslint/no-unused-vars */
import { AccountInfo, clusterApiUrl, Commitment, ConfirmOptions, Connection, GetAccountInfoConfig, Keypair, PublicKey, SerializeConfig, Signer, Transaction, TransactionSignature } from "@solana/web3.js";
import { Buffer } from 'buffer';
import { ethers } from 'ethers';

import * as anchor from "@project-serum/anchor";
import idl from './models/solzen.json';
import { AnchorProvider, Program } from "@project-serum/anchor";
import { Solzen } from "./models/solzen";
import { Wallet } from "@project-serum/anchor/dist/cjs/provider";
import { cartesiRollups } from "../utils/cartesi";

export const programID = new PublicKey(idl.metadata.address);
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
        const msg = tx.compileMessage()
        console.log(msg.accountKeys.map(k => k.toBase58()))

        // just fill the signature bytes
        const signature = Buffer.alloc(64);

        tx.addSignature(this._publicKey, signature);

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

class AnchorProviderAdapter extends AnchorProvider {
    public etherSigner: ethers.Signer | undefined

    async sendAndConfirm(tx: Transaction,
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
        const payload = toBuffer(rawTx).toString('base64');
        console.log('Cartesi Rollups payload', payload);
        const inputBytes = ethers.utils.toUtf8Bytes(payload);
        if (this.etherSigner) {
            const { inputContract } = await cartesiRollups(this.etherSigner);

            // send transaction
            const txEth = await inputContract.addInput(inputBytes);
            console.log(`transaction: ${txEth.hash}`);
            console.log("waiting for confirmation...");
            const receipt = await txEth.wait(1);
            console.log(`receipt: ${JSON.stringify(receipt, null, 4)}`);
        }
        return { ok: 1 } as any
    }

}

class ConnectionAdapter extends Connection {
    async getAccountInfo(
        publicKey: PublicKey,
        _commitmentOrConfig?: Commitment | GetAccountInfoConfig,
    ): Promise<AccountInfo<Buffer> | null> {
        let host = window.location.host;
        const protocol = window.location.protocol;
        if (/^[0-9]{4}/.test(host)) {
            // gitpod host
            host = host.replace(/^[0-9]*/, '5005');
        } else {
            // localhost like
            host = host.replace(/:[0-9]+$/,':5005')
        }
        const url = `${protocol}//${host}/inspect/${publicKey.toBase58()}`;
        console.log('Cartesi inspect url', url);
        const resp = await fetch(url.toString());
        const cartesiResponse = await resp.json();
        if (!cartesiResponse.reports || !cartesiResponse.reports.length) {
            return null;
        }
        const jsonString = ethers.utils.toUtf8String(cartesiResponse.reports[0].payload);
        const infoData = JSON.parse(jsonString);
        console.log({ [publicKey.toBase58()]: infoData })
        return {
            owner: new PublicKey(infoData.owner),
            data: Buffer.from(infoData.data, 'base64'),
            executable: false, // pode ser que seja executavel
            lamports: +infoData.lamports,
        };
    }
}

export function convertSolanaAddress2Eth(pubkey: PublicKey) {
    const buffer = pubkey.toBuffer();
    const eth20bytes: number[] = [];
    for (let i = buffer.length - 1; i > 11; i--) {
        eth20bytes.push(buffer[i]);
    }
    const recoveredAddress = ethers.utils.hexValue(eth20bytes);
    return recoveredAddress;
}

export function convertEthAddress2Solana(ethereumAddress: string) {
    const bytes = Buffer.from(ethereumAddress.substring(2), 'hex');
    const sol32bytes: number[] = [];
    for (let i = 0; i < 32; i++) {
        sol32bytes.push(bytes[i] || 0)
    }
    // existe espaco para colocar o byte para recuperar a chave publica original
    const pubkey = PublicKey.decode(Buffer.from(sol32bytes));
    return pubkey;
}

export function getProvider(signer?: ethers.Signer) {
    const commitment = 'processed';
    const network = clusterApiUrl('devnet');
    const connection = new ConnectionAdapter(network, commitment);
    const wallet = new AdaptedWallet();
    const provider = new AnchorProviderAdapter(connection, wallet, { commitment });
    provider.etherSigner = signer;
    return { provider, wallet };
}

export async function getProgram(signer?: ethers.Signer) {
    const { provider, wallet } = getProvider(signer);
    const program = new anchor.Program(idl as any, programID, provider) as Program<Solzen>;
    if (signer) {
        const ethAddress = await signer.getAddress();
        wallet.publicKey = convertEthAddress2Solana(ethAddress);
        console.log('wallet publicKey changed')
    }
    return { program, provider, wallet }
}
