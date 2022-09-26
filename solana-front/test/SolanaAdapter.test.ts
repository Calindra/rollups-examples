/* eslint-disable no-unused-vars */
/* eslint-disable prettier/prettier */
import * as anchor from "@project-serum/anchor";
import { getProgram } from '../frontend/src/solana/adapter';
import { PublicKey } from "@solana/web3.js";

import Factory from './Factory';

describe.only('SolanaAdapter', () => {

    it('should run', async () => {
        const daoSlug = 'the-dao'
        const { mint, payer } = await Factory.createMint();
        const program = await getProgram()
        const [daoPubkey, _bump] = await PublicKey.findProgramAddress([
            anchor.utils.bytes.utf8.encode('dao'),
            Buffer.from(daoSlug.slice(0, 32)),
        ], program.programId);

        const [userAccount, _bump2] = await PublicKey.findProgramAddress([
            anchor.utils.bytes.utf8.encode('child'),
            payer.publicKey.toBuffer(),
            daoPubkey.toBuffer(),
        ], program.programId);
        console.log(`dao = ${daoPubkey.toBase58()}`)
        console.log(`validation = ${userAccount.toBase58()}`)
        console.log(`programId = ${program.programId.toBase58()}`)
        console.log(mint.toBase58())
        console.log(payer.publicKey.toBase58())
        await program.methods.initialize(mint, new anchor.BN(1000), daoSlug)
            .accounts({
                zendao: daoPubkey,
                validation: userAccount,
            })
            .rpc()
    })

})
