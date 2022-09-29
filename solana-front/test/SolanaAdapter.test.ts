/* eslint-disable no-unused-vars */
/* eslint-disable prettier/prettier */
import * as anchor from "@project-serum/anchor";
import { convertEthAddress2Solana, convertSolanaAddress2Eth, getProgram, programID } from '../frontend/src/solana/adapter';
import { PublicKey } from "@solana/web3.js";
import * as secp from '@noble/secp256k1';

import Factory from './Factory';
import { expect } from "chai";
import { ethers } from "hardhat";

describe('SolanaAdapter', () => {

    it('should run', async () => {
        const daoSlug = 'slug'
        const { mint, payer } = await Factory.createMint();
        const { program, wallet } = await getProgram()
        const [daoPubkey, _bump] = await PublicKey.findProgramAddress([
            anchor.utils.bytes.utf8.encode('dao'),
            Buffer.from(daoSlug.slice(0, 32)),
        ], program.programId);

        const [userAccount, _bump2] = await PublicKey.findProgramAddress([
            anchor.utils.bytes.utf8.encode('child'),
            wallet.publicKey.toBuffer(),
            daoPubkey.toBuffer(),
        ], program.programId);
        console.log(`dao = ${daoPubkey.toBase58()}`)
        console.log(`validation = ${userAccount.toBase58()}`)
        console.log(`programId = ${program.programId.toBase58()}`)
        console.log(`mint = ${mint.toBase58()}`)
        console.log(`payer = ${payer.publicKey.toBase58()}`)
        const tx = await program.methods.initialize(mint, new anchor.BN(1000), daoSlug)
            .accounts({
                zendao: daoPubkey,
                validation: userAccount,
            })
            .rpc()
        console.log({ tx })
    })

    it('should recover the public key', async () => {
        // keys, messages & other inputs can be Uint8Arrays or hex strings
        // Uint8Array.from([0xde, 0xad, 0xbe, 0xef]) === 'deadbeef'
        const ETHEREUM_PUBLIC_KEY_SIZE = 65;
        const SOLANA_PUBLIC_KEY_SIZE = 32;
        const message = anchor.utils.bytes.utf8.encode('hello world');
        const privKey = secp.utils.randomPrivateKey();
        const pubKey = secp.getPublicKey(privKey);
        const msgHash = await secp.utils.sha256(message);
        const [signature, recovery] = await secp.sign(msgHash, privKey, { recovered: true });
        const isValid = secp.verify(signature, msgHash, pubKey);

        const recoveredPubkey = secp.recoverPublicKey(msgHash, signature, recovery);
        console.log({ recoveredPubkey, pubKey });
        expect(pubKey.length).to.eq(ETHEREUM_PUBLIC_KEY_SIZE);
        expect(recoveredPubkey.length).to.eq(ETHEREUM_PUBLIC_KEY_SIZE);

        expect(programID.toBuffer().length).to.eq(SOLANA_PUBLIC_KEY_SIZE);

        // Schnorr signatures
        const rpub = secp.schnorr.getPublicKey(privKey);
        const rsignature = await secp.schnorr.sign(message, privKey);
        const risValid = await secp.schnorr.verify(rsignature, message, rpub);
        console.log({ isValid, risValid })
    })

    it.only('should convert the public key from eth to solana', async () => {
        const ethereumAddress = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266';
        const pubkey = convertEthAddress2Solana(ethereumAddress);
        expect(pubkey.toBase58()).to.eq('1111111111112RXi1yn6kTp7G8Td7o6z3Ciqw9v2');
    })

    it.only('should convert the public key from solana to eth', async () => {
        const ethereumAddress = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266';
        const pubkey = convertEthAddress2Solana(ethereumAddress);
        const recoveredAddress = convertSolanaAddress2Eth(pubkey);
        expect(recoveredAddress).to.eq(ethereumAddress.toLowerCase());
    })

})
