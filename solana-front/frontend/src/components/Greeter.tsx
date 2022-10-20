/* eslint-disable @typescript-eslint/no-unused-vars */
import { PublicKey } from '@solana/web3.js';
import { useWeb3React } from '@web3-react/core';
import { Contract, ethers, Signer } from 'ethers';
import { Buffer } from 'buffer';
import {
  ChangeEvent,
  MouseEvent,
  ReactElement,
  useEffect,
  useState
} from 'react';
import styled from 'styled-components';
import GreeterArtifact from '../artifacts/contracts/Greeter.sol/Greeter.json';
import { getProgram } from '../solana/adapter';
import * as anchor from "@project-serum/anchor";
import { Provider } from '../utils/provider';
import { SectionDivider } from './SectionDivider';
import { createMint, getAccount } from '@solana/spl-token';

const StyledDeployContractButton = styled.button`
  width: 180px;
  height: 2rem;
  border-radius: 1rem;
  border-color: blue;
  cursor: pointer;
  place-self: center;
`;

const StyledGreetingDiv = styled.div`
  display: grid;
  grid-template-rows: 1fr 1fr 1fr;
  grid-template-columns: 135px 2.7fr 1fr;
  grid-gap: 10px;
  place-self: center;
  align-items: center;
`;

const StyledLabel = styled.label`
  font-weight: bold;
`;

const StyledInput = styled.input`
  padding: 0.4rem 0.6rem;
  line-height: 2fr;
`;

const StyledButton = styled.button`
  width: 150px;
  height: 2rem;
  border-radius: 1rem;
  border-color: blue;
  cursor: pointer;
`;

export function Greeter(): ReactElement {
  const context = useWeb3React<Provider>();
  const { library } = context;

  const [signer, setSigner] = useState<Signer>();
  const [daoAccount, setDaoAccount] = useState<string>('');
  const [tokenAccount, setTokenAccount] = useState<string>('');

  async function readTokenAccount() {

    const mint = new PublicKey("4xRtyUw1QSVZSGi1BUb7nbYBk8TC9P1K1AE2xtxwaZmV");
    // const mint = new PublicKey("CasshNb6PacBzSwbd5gw8uqoQEjcWxaQ9u9byFApShwT");
    const { program, connection } = await getProgram(signer)
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
  }

  async function readMint() {
    const { connection } = await getProgram(signer)

    const mint = new PublicKey("4xRtyUw1QSVZSGi1BUb7nbYBk8TC9P1K1AE2xtxwaZmV");
    // const tokenProgramAddress = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    // console.log(tokenProgramAddress.toBuffer());
    const mintInfo = await connection.getAccountInfo(mint);
    console.log(JSON.stringify(mintInfo, null, 4));
  }

  async function createEscrowWalletTokenAccount() {
    const { program } = await getProgram(signer)
    const fromWallet = anchor.web3.Keypair.generate();
    // const toWallet = anchor.web3.Keypair.generate();
    // const signature = await connection.requestAirdrop(fromWallet.publicKey, 1_000_000_000);
    // await connection.confirmTransaction(signature, 'confirmed');
    // const mint = await createMint(connection, fromWallet, fromWallet.publicKey, fromWallet.publicKey, 9);
    const mint = new PublicKey("4xRtyUw1QSVZSGi1BUb7nbYBk8TC9P1K1AE2xtxwaZmV");
    // const tokenProgramAddress = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    // console.log(tokenProgramAddress.toBuffer());

    const [escrowWallet, bump] = await PublicKey.findProgramAddress(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode("wallet")),
        mint.toBuffer(),
      ],
      program.programId
    );
    console.log('Init wallet...', {
      from: fromWallet.publicKey.toBase58(),
      escrowWallet: escrowWallet.toBase58(),
      mint: mint.toBase58(),
      bump
    });
    try {
      await program.methods
      .initWallet()
      .accounts({
        escrowWallet,
        mint,
      })
      .rpc();
    } catch(e) {
      console.error('Create TokenAccount error', e);
      alert(`Create TokenAccount error: ${(e as any).message}`);
    }
  }

  async function createAccountInfoInsideCartesi() {
    if (!signer) {
      console.log('Signer is missing');
      return;
    }
    try {
      const signerAddress = await signer.getAddress();
      console.log(`using account "${signerAddress}"`);

      const daoSlug = 'slug'
      const { program, wallet } = await getProgram(signer)
      const mint = new PublicKey("CasshNb6PacBzSwbd5gw8uqoQEjcWxaQ9u9byFApShwT");

      const [daoPubkey, _bump1] = await PublicKey.findProgramAddress([
        anchor.utils.bytes.utf8.encode('dao'),
        Buffer.from(daoSlug.slice(0, 32)),
      ], program.programId);

      const [validation, _bump2] = await PublicKey.findProgramAddress([
        anchor.utils.bytes.utf8.encode('child'),
        wallet.publicKey.toBuffer(),
        daoPubkey.toBuffer(),
      ], program.programId);
      console.log(`dao = ${daoPubkey.toBase58()}`)
      console.log(`validation = ${validation.toBase58()}`)
      console.log(`programId = ${program.programId.toBase58()}`)
      console.log(`mint = ${mint.toBase58()}`)
      console.log(`payer = ${wallet.publicKey.toBase58()}`)
      const txSolana = await program.methods.initialize(mint, new anchor.BN(1000), daoSlug)
        .accounts({
          zendao: daoPubkey,
          validation: validation,
        })
        .rpc()
      console.log({ txSolana });
    } catch (e) {
      console.error('Create AccountInfo error', e);
      alert(`Create error: ${(e as any).message}`);
    }
  }

  async function readDataFromCartesiRollups() {
    if (!signer) {
      console.log('Signer is missing');
      return;
    }
    try {
      const daoSlug = 'slug'
      const { program } = await getProgram(signer)
      const [daoPubkey, _bump] = await PublicKey.findProgramAddress([
        anchor.utils.bytes.utf8.encode('dao'),
        Buffer.from(daoSlug.slice(0, 32)),
      ], program.programId)
      const daoAccount = await program.account.zendao.fetch(daoPubkey);
      setDaoAccount(JSON.stringify(daoAccount, null, 4));
    } catch (e) {
      console.log(e);
      setDaoAccount((e as any).message);
    }
  }

  async function updateDataInsideCartesiRollups() {
    if (!signer) {
      console.log('Signer is missing');
      return;
    }
    const mint = new PublicKey("So11111111111111111111111111111111111111112");
    const daoSlug = 'slug'
    const { program } = await getProgram(signer)
    const [daoPubkey, _bump] = await PublicKey.findProgramAddress([
      anchor.utils.bytes.utf8.encode('dao'),
      Buffer.from(daoSlug.slice(0, 32)),
    ], program.programId)

    try {
      const txSolana = await program.methods.update(mint, new anchor.BN(1234))
        .accounts({
          zendao: daoPubkey,
        })
        .rpc()
      console.log({ txSolana })
    } catch (e) {
      console.error(`Update error`, e);
      alert(`Update error: ${(e as any).message}`);
    }
  }

  async function deleteDataInsideCartesiRollups() {
    if (!signer) {
      console.log('Signer is missing');
      return;
    }
    const daoSlug = 'slug'
    const { program } = await getProgram(signer)
    const [daoPubkey, _bump] = await PublicKey.findProgramAddress([
      anchor.utils.bytes.utf8.encode('dao'),
      Buffer.from(daoSlug.slice(0, 32)),
    ], program.programId)

    try {
      const txSolana = await program.methods.closeDao()
        .accounts({
          zendao: daoPubkey,
        })
        .rpc()
      console.log({ txSolana })
    } catch (e) {
      console.error(`Delete error`, e);
      alert(`Delete error: ${(e as any).message}`);
    }
  }

  useEffect((): void => {
    if (!library) {
      setSigner(undefined);
      return;
    }

    setSigner(library.getSigner());
  }, [library]);

  return (
    <>
      <StyledGreetingDiv>
        <div>
          Solana
          <StyledButton
            onClick={createAccountInfoInsideCartesi}
          >
            Create Account
          </StyledButton>
          <StyledButton
            onClick={readDataFromCartesiRollups}
          >
            Read Account
          </StyledButton>
          <pre>{daoAccount}</pre>
          <StyledButton
            onClick={updateDataInsideCartesiRollups}
          >
            Update Account
          </StyledButton>
          <StyledButton
            onClick={deleteDataInsideCartesiRollups}
          >
            Delete Account
          </StyledButton>
          <StyledButton
            onClick={createEscrowWalletTokenAccount}
          >
            Create TokenAccount
          </StyledButton>
          <StyledButton
            onClick={readMint}
          >
            Read Mint
          </StyledButton>
          <StyledButton
            onClick={readTokenAccount}
          >
            Read TokenAccount
          </StyledButton>
          <pre>{tokenAccount}</pre>
        </div>
      </StyledGreetingDiv>
    </>
  );
}
