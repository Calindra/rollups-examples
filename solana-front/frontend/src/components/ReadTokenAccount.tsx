import { ReactElement, useEffect, useState } from "react";
import { useWeb3React } from "@web3-react/core";
import { ethers } from "ethers";
import * as anchor from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { useCartesi } from "../solana/adapter";
import { getAccount } from "@solana/spl-token";
import { Provider } from '../utils/provider';
import { StyledButton } from "./comps";

export function ReadTokenAccount(): ReactElement {
    const { library } = useWeb3React<Provider>();
    const [signer, setSigner] = useState<ethers.Signer>();
    const { wallet, connection, program, provider } = useCartesi(signer);

    useEffect((): void => {
        if (!library) {
            setSigner(undefined);
            return;
        }
        setSigner(library.getSigner());
    }, [library]);

    const [tokenAccount, setTokenAccount] = useState<string>('');

    async function readTokenAccount() {
        if (!wallet?.publicKey || !provider) {
            return;
        }
        try {
            const mint = new PublicKey("4xRtyUw1QSVZSGi1BUb7nbYBk8TC9P1K1AE2xtxwaZmV");
            const [escrowWallet, _bump] = await PublicKey.findProgramAddress(
                [
                    Buffer.from(anchor.utils.bytes.utf8.encode("wallet")),
                    mint.toBuffer(),
                ],
                program.programId
            );
            const accountInfo = await connection.getAccountInfo(escrowWallet);
            const tokenAccount = await getAccount(connection, escrowWallet);

            setTokenAccount(JSON.stringify({
                programIdOwner: accountInfo?.owner.toBase58(),
                owner: tokenAccount.owner,
                amount: tokenAccount.amount.toLocaleString(),
                isNative: tokenAccount.isNative,
                mint: tokenAccount.mint.toBase58(),
            }, null, 4));
        } catch (e) {
            console.error(`Read TokenAccount error:`, e);
            if (e instanceof Error) {
                alert(`Error: ${e.message}`);
            }
        }
    }

    return (<>
        <StyledButton
            onClick={readTokenAccount}
        >
            Read TokenAccount
        </StyledButton>
        <pre>{tokenAccount}</pre>
    </>)
}
