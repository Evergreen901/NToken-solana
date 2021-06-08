// @flow
import {expect} from 'chai';
import {Account, PublicKey} from '@solana/web3.js';

import {ASSOCIATED_TOKEN_PROGRAM_ID, nToken, TOKEN_PROGRAM_ID} from '../client/nToken';

describe('Token', () => {
  it('createTransfer', () => {
    const ix = nToken.createTransferCheckedInstruction(
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
    const ix = nToken.createInitMintInstruction(
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
    const associatedPublicKey = await nToken.getAssociatedTokenAddress(
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
    const ix = nToken.createAssociatedTokenAccountInstruction(
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
