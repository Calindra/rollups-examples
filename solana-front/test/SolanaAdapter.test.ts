/* eslint-disable no-unused-vars */
/* eslint-disable prettier/prettier */
import { getProgram } from '../frontend/src/solana/adapter';

describe.only('SolanaAdapter', () => {

    it('should run', async () => {
        const program = await getProgram()
    })
})
