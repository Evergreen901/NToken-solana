// @flow
import {expect} from 'chai';
import {Account, Connection, PublicKey,Transaction,SystemProgram} from '@solana/web3.js';
import {ASSOCIATED_TOKEN_PROGRAM_ID, Portfolio, TOKEN_PROGRAM_ID,PortfolioLayout} from '../client/Portfolio';
import { sendAndConfirmTransaction } from '../client/util/send-and-confirm-transaction';
//import { Store } from '../cli/store/store/config.json';
let config=require('../cli/store/store/config.json')
let portfolioAddress:Account;
let testToken:Portfolio;
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
  it('createInitPortfolio', async () => {
    let owner = new Account([253, 105, 193, 173, 55, 108, 145, 101, 186, 22, 187, 172, 156, 119, 173, 35, 25, 99, 80, 68, 92, 204, 232, 243, 67, 169, 199, 7, 218, 94, 225, 17, 173, 31, 39, 116, 250, 166, 211, 3, 213, 13, 179, 50, 47, 240, 7, 164, 48, 110, 143, 141, 244, 242, 74, 210, 185, 203, 0, 4, 138, 99, 110, 251]);
    let metaDataUrl = "aabbcc";
  var metaDataHash = new Uint16Array([789]);
  let amountAsset1 = 2;
  let splmAsset1 = new PublicKey("C16ua5YLDhDwEhdsfru1e1wBUhDMQ6jy4jofVntLCbZa");
  let periodAsset1 = 123;
  let assetToSoldIntoAsset1=new PublicKey("FAxFrLbWabNWgL1A9sLokNQbaBSq33iQHA2Y3zKk1g8x");
  let amountAsset2=3 ;
  let splmAsset2  = splmAsset1;
  let periodAsset2 = 4;
  let assetToSoldIntoAsset2  =new PublicKey("5DPmnnQHxMdf8NLYQ6m1C4D2E13pMLHym92SuSFTuQJJ");
 
  let amountAsset3=3 ;
  let splmAsset3  = splmAsset1;
  let periodAsset3 =3;
  let assetToSoldIntoAsset3  = splmAsset1;

  let amountAsset4 =3;
  let splmAsset4 = splmAsset1;
  let periodAsset4 = 3;
  let assetToSoldIntoAsset4  = splmAsset1;

  let amountAsset5 =3;
  let splmAsset5  = splmAsset1;
  let periodAsset5=3;
  let assetToSoldIntoAsset5  =splmAsset1;

  let amountAsset6 =3;
  let splmAsset6  = splmAsset1;
  let periodAsset6=3;
  let assetToSoldIntoAsset6  = splmAsset1;

  let amountAsset7=3 ;
  let splmAsset7  = splmAsset1;
  let periodAsset7=3;
  let assetToSoldIntoAsset7  = splmAsset1;

  let amountAsset8 =3;
  let splmAsset8  = splmAsset1;
  let periodAsset8=3;
  let assetToSoldIntoAsset8  = splmAsset1;

  let amountAsset9 =3;
  let splmAsset9  = splmAsset1;
  let periodAsset9 =3;
  let assetToSoldIntoAsset9  = splmAsset1;
const connection = new Connection("https://api.devnet.solana.com", "confirmed");
let programId = new PublicKey(config.tokenProgramId);
  let payer=new Account();
   testToken = new Portfolio(
    connection,
    new PublicKey("6ykyxd7bZFnvEHq61vnd69BkU3gabiDmKGEQb4sGiPQG"),
    programId,
    payer
); 
  portfolioAddress = await testToken.createPortfolio(owner , metaDataUrl , metaDataHash,
    amountAsset1 , splmAsset1 , periodAsset1 , assetToSoldIntoAsset1 ,
    amountAsset2 , splmAsset2 , periodAsset2 , assetToSoldIntoAsset2 ,
    amountAsset3 , splmAsset3 , periodAsset3 , assetToSoldIntoAsset3 ,
    amountAsset4 , splmAsset4 , periodAsset4 , assetToSoldIntoAsset4 ,
    amountAsset5 , splmAsset5 , periodAsset5 , assetToSoldIntoAsset5 ,
    amountAsset6 , splmAsset6 , periodAsset6 , assetToSoldIntoAsset6 ,
    amountAsset7 , splmAsset7 , periodAsset7 , assetToSoldIntoAsset7 ,
    amountAsset8 , splmAsset8 , periodAsset8 , assetToSoldIntoAsset8 ,
    amountAsset9 , splmAsset9 , periodAsset9 , assetToSoldIntoAsset9);

    let infoPortfolio=await testToken.getPortfolioInfo(portfolioAddress.publicKey);
   
    expect(infoPortfolio.portfolioAddress).to.eql(portfolioAddress.publicKey);
    expect(infoPortfolio.metadataHash.property).to.eql(metaDataHash[0]);
    expect(infoPortfolio.amountAsset1.property).to.eql(amountAsset1);
    expect(infoPortfolio.amountAsset2.property).to.eql(amountAsset2);
    expect(infoPortfolio.assetToSoldIntoAsset1).to.eql(assetToSoldIntoAsset1);
    expect(infoPortfolio.assetToSoldIntoAsset2).to.eql(assetToSoldIntoAsset2);
    expect(infoPortfolio.periodAsset4.property).to.eql(periodAsset4); 
    expect(infoPortfolio.addressAsset5).to.eql(splmAsset5);
  });
   it('createInitUserPortfolio',async () => {
     let ownerPortfolio = new Account([253, 105, 193, 173, 55, 108, 145, 101, 186, 22, 187, 172, 156, 119, 173, 35, 25, 99, 80, 68, 92, 204, 232, 243, 67, 169, 199, 7, 218, 94, 225, 17, 173, 31, 39, 116, 250, 166, 211, 3, 213, 13, 179, 50, 47, 240, 7, 164, 48, 110, 143, 141, 244, 242, 74, 210, 185, 203, 0, 4, 138, 99, 110, 251]);
     let splu_asset1 = new PublicKey ("GjMDerPggqZiELiqFd195nfMVLM6ypK8vHBVYcStSeGG");
    let  splu_asset2 = new PublicKey ("FGsWNma9oCE13NLcCvPrhomgA4eHFZmUo2M2uWroVpp7");
     let splu_asset3 = new PublicKey ("34mKewueSpsCHRJyyasu5Lca5oZWCMn7BBP3uFtfKwAZ");
     let splu_asset4 = new PublicKey ("7bfsMMK1YvyjDyqtaF7rtaWs8YghYbaUBHk4ct57WC3K");
     let splu_asset5 = new PublicKey ("4wwvysJ1XNN8RoaZvxwbPfgvXZ47gPCU3S33cSZLAipt");
     let splu_asset6 = new PublicKey ("3FUk2jBEoQn9ViaM3oa9MhXRNeiUQ5Sn4sNsWYBR4jFv");
     let splu_asset7 = new PublicKey ("35iFLMZYjUHtTQDWwAKPyC3VBj9cW2526TNgiZ6QBSY8");
     let splu_asset8 = new PublicKey ("8oJQ25rCvgYjA6p4v5NxHqLpc6LDrsXunticEEqLUxtH");
     let splu_asset9 =new PublicKey("Dfzx8QRHdr7oduQtfBcqpwU3fSvxHDTR9o47TKY9hNU8");
     let delegate =new PublicKey("SE3zVMvGX7kSHRK3rGH7K1bHZgnmHSGEvGNiYHrPhfF")
    let delegated_amount = 5;
    let  UserPortfolioAccount = await testToken.createUserPortfolio(ownerPortfolio,portfolioAddress.publicKey,delegate , delegated_amount,
      splu_asset1,
      splu_asset2,
      splu_asset3,
      splu_asset4,
      splu_asset5,
      splu_asset6,
      splu_asset7,
      splu_asset8,
      splu_asset9);

    let infoUserPortfolio =await testToken.getAccountUserPortfolioInfo(UserPortfolioAccount.publicKey);
    
    expect(infoUserPortfolio.portfolio_address).to.eql(portfolioAddress.publicKey);
    expect(infoUserPortfolio.user_portfolio_address.toBase58()).to.eql(UserPortfolioAccount.publicKey.toBase58());
  
    
  }); 
});
