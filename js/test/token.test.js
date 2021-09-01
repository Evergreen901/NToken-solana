// @flow
import {expect} from 'chai';
import {Account, PublicKey} from '@solana/web3.js';

import {ASSOCIATED_TOKEN_PROGRAM_ID, Portfolio, TOKEN_PROGRAM_ID} from '../client/Portfolio';

describe('Token', () => {
  it('createTransfer', () => {
    const ix = Portfolio.createTransferCheckedInstruction(
      TOKEN_PROGRAM_ID,
      new Account().publicKey,
      new Account().publicKey,
      new Account().publicKey,
      new Account().publicKey,
      [],
      1,
      9,
    );
    expect(ix.programId).to.eql(TOKEN_PROGRAM_ID);
    expect(ix.keys).to.have.length(4);
  });

  it('createInitMint', () => {
    const ix = Portfolio.createInitMintInstruction(
      TOKEN_PROGRAM_ID,
      new Account().publicKey,
      9,
      new Account().publicKey,
      null,
    );
    expect(ix.programId).to.eql(TOKEN_PROGRAM_ID);
    expect(ix.keys).to.have.length(3);
  });

  it('getAssociatedTokenAddress', async () => {
    const associatedPublicKey = await Portfolio.getAssociatedTokenAddress(
      ASSOCIATED_TOKEN_PROGRAM_ID,
      TOKEN_PROGRAM_ID,
      new PublicKey('7o36UsWR1JQLpZ9PE2gn9L4SQ69CNNiWAXd4Jt7rqz9Z'),
      new PublicKey('B8UwBUUnKwCyKuGMbFKWaG7exYdDk2ozZrPg72NyVbfj'),
    );
    expect(associatedPublicKey.toString()).to.eql(
      new PublicKey('DShWnroshVbeUp28oopA3Pu7oFPDBtC1DBmPECXXAQ9n').toString(),
    );
  });

  it('createAssociatedTokenAccount', () => {
    const ix = Portfolio.createAssociatedTokenAccountInstruction(
      ASSOCIATED_TOKEN_PROGRAM_ID,
      TOKEN_PROGRAM_ID,
      new Account().publicKey,
      new Account().publicKey,
      new Account().publicKey,
      new Account().publicKey,
    );
    expect(ix.programId).to.eql(ASSOCIATED_TOKEN_PROGRAM_ID);
    expect(ix.keys).to.have.length(7);
  });
});
