// @flow

import fs from 'mz/fs';
import {
  Account,
  Connection,
  BpfLoader,
  PublicKey,
  BPF_LOADER_PROGRAM_ID,
} from '@solana/web3.js';
import {
  nToken,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  NATIVE_MINT,
} from '../client/nToken';
import {url} from '../url';
import {newAccountWithLamports} from '../client/util/new-account-with-lamports';
import {sleep} from '../client/util/sleep';
import {Store} from './store';
// Loaded token program's program id
let programId: PublicKey;
let associatedProgramId: PublicKey;

// Accounts setup in createMint and used by all subsequent tests
let testMintAuthority: Account;
let testToken: nToken;
let testTokenDecimals: number = 2;

let asset: nToken;
let USDC: nToken;
let managerNTokenWBTC:Account;
let managerNTokenUSDC:Account;



// Accounts setup in createAccount and used by all subsequent tests
let testAccountOwner: Account;
let testAccount: PublicKey;
let assetAccount: PublicKey;
let usdcAccount: PublicKey;
let accountKey : PublicKey;
let UserAccountAsset : Account ;
let UserAccountUsdc : Account ;

function assert(condition, message) {
  if (!condition) {
    console.log(Error().stack + ':token-test.js');
    throw message || 'Assertion failed';
  }
}

async function didThrow(obj, func, args): Promise<boolean> {
  try {
    await func.apply(testToken, args);
  } catch (e) {
    return true;
  }
  return false;
}

let connection;
async function getConnection(): Promise<Connection> {
  if (connection) return connection;

  connection = new Connection(url, 'recent');
  const version = await connection.getVersion();

  console.log('Connection to cluster established:', url, version);
  return connection;
}

async function loadProgram(
  connection: Connection,
  path: string,
): Promise<PublicKey> {
  const NUM_RETRIES = 500; /* allow some number of retries */
  const data = await fs.readFile(path);
  const {feeCalculator} = await connection.getRecentBlockhash();
  const balanceNeeded =
    feeCalculator.lamportsPerSignature *
      (BpfLoader.getMinNumSignatures(data.length) + NUM_RETRIES) +
    (await connection.getMinimumBalanceForRentExemption(data.length));

  const from = await newAccountWithLamports(connection, balanceNeeded);
  const program_account = new Account();
  console.log('Loading program:', path);
  await BpfLoader.load(
    connection,
    from,
    program_account,
    data,
    BPF_LOADER_PROGRAM_ID,
  );
  return program_account.publicKey;
}

async function GetPrograms(connection: Connection): Promise<void> {
  const programVersion = process.env.PROGRAM_VERSION;
  if (programVersion) {
    switch (programVersion) {
      case '2.0.4':
        programId = TOKEN_PROGRAM_ID;
        associatedProgramId = ASSOCIATED_TOKEN_PROGRAM_ID;
        return;
      default:
        throw new Error('Unknown program version');
    }
  }

  const store = new Store();
  try {
    const config = await store.load('config.json');
    console.log('Using pre-loaded nToken programs');
    console.log(
      `  Note: To reload program remove ${Store.getFilename('config.json')}`,
    );
    programId = new PublicKey(config.tokenProgramId);
    associatedProgramId = new PublicKey(config.associatedTokenProgramId);
    let info;
    info = await connection.getAccountInfo(programId);
    assert(info != null);
    info = await connection.getAccountInfo(associatedProgramId);
    assert(info != null);
  } catch (err) {
    console.log(
      'Checking pre-loaded nToken programs failed, will load new programs:',
    );
    console.log({err});

    programId = await loadProgram(
      connection,
      '../program/target/bpfel-unknown-unknown/release/spl_token.so',
    );
    associatedProgramId = programId;
    await store.save('config.json', {
      tokenProgramId: programId.toString(),
      associatedTokenProgramId: associatedProgramId.toString(),
    });
  }
}

export async function loadTokenProgram(): Promise<void> {
  const connection = await getConnection();
  await GetPrograms(connection);

  console.log('nToken Program ID', programId.toString());
  console.log('Associated nToken Program ID', associatedProgramId.toString());
}


export async function createMint(): Promise<void> {
  const connection = await getConnection();
  const payer = await newAccountWithLamports(connection, 1000000000 /* wag */);
  testMintAuthority = new Account();
  //nWBTC
  testToken = await nToken.createMint(
    connection,
    payer,
    testMintAuthority.publicKey,
    testMintAuthority.publicKey,
    testTokenDecimals,
    programId,
    new PublicKey("9ZFJWoBMQuYiBvbGpExs3smE59kQZbPnVmJp7F8iUsDG"),
    new PublicKey("4A3a33ozsqA6ihMXRAzYeNwZv4df9RfJoLPh6ycZJVhE")
  ); 
  testToken=new nToken(
    connection,
    //new PublicKey("887xCkc7KNUTQTJLHrrPAqHvcBCdbaBWFDqzXkXNjxkS"),
    new PublicKey("6ykyxd7bZFnvEHq61vnd69BkU3gabiDmKGEQb4sGiPQG"),
    programId,
    payer
  );
  
  console.log("createMint publickey asset -- "+testToken.publicKey)
  asset =new nToken(
    connection,
    new PublicKey("9ZFJWoBMQuYiBvbGpExs3smE59kQZbPnVmJp7F8iUsDG"),
    TOKEN_PROGRAM_ID,
    payer
   );
    USDC =new nToken(
     connection,
     new PublicKey("4A3a33ozsqA6ihMXRAzYeNwZv4df9RfJoLPh6ycZJVhE"),
     TOKEN_PROGRAM_ID,
     payer
    ); 

  // HACK: override hard-coded ASSOCIATED_TOKEN_PROGRAM_ID with corresponding
  // custom test fixture
  testToken.associatedProgramId = associatedProgramId;

}

export async function runApprove():Promise<void>{
  // testAccount ==>  nTokeen // 
   let testAccountInfo ;
  
   managerNTokenWBTC=await asset.createAccountNew(testToken.publicKey);
   managerNTokenUSDC=await USDC.createAccountNew(testToken.publicKey);

   console.log(" publickey mangerNtoken -- "+managerNTokenWBTC.publicKey);

   testAccountInfo=await asset.getAccountInfoNew(assetAccount);
   console.log("before approve info managerNToken mint --"+testAccountInfo.mint+" --owner --"+testAccountInfo.owner +" -amount --"+testAccountInfo.amount +"-- allownace --"+testAccountInfo.delegatedAmount.toNumber())
  
   console.log(" assetaccount is : " + assetAccount+" -- testAccount owner --"+testAccountOwner.publicKey);

   await asset.approveChecked(assetAccount,managerNTokenWBTC.publicKey ,testAccountOwner,[],2000000000,9);

   testAccountInfo=await asset.getAccountInfoNew(assetAccount);
   console.log("after approve info managerNToken mint --"+testAccountInfo.mint+" --owner --"+testAccountInfo.owner +" -amount --"+testAccountInfo.amount +"-- allownace --"+testAccountInfo.delegatedAmount.toNumber())
 
}
export async function runApproveChecked():Promise<void>{
  const delegate = new Account().publicKey;
  await testToken.approveChecked(testAccount, delegate, testAccountOwner, [], 9,2);
  let testAccountInfo = await testToken.getAccountInfo(testAccount);
  let allowance=testAccountInfo.delegatedAmount.toNumber()
  console.log("allowance : "+allowance)

}

export async function runDeposit(): Promise<void> {
  console.log("run test deposit");

  const connection = await getConnection();
  const payer = await newAccountWithLamports(connection, 10000000000 /* wag */);
 
  let infoMangerNToken;
  infoMangerNToken=await asset.getAccountInfoNew(managerNTokenWBTC.publicKey);
  console.log(assetAccount+"before transferFrom infoMangerNToken mint --"+infoMangerNToken.mint+" --owner --"+infoMangerNToken.owner +" -amount --"+infoMangerNToken.amount+"-- allownace --"+infoMangerNToken.delegatedAmount)
  infoMangerNToken= await asset.getAccountInfoNew(assetAccount);
  console.log("before transferFrom infoassetAccount mint --"+infoMangerNToken.mint+" --owner --"+infoMangerNToken.owner +" -amount --"+infoMangerNToken.amount+"-- allownace --"+infoMangerNToken.delegatedAmount)

  await asset.transfer(assetAccount, managerNTokenWBTC.publicKey, managerNTokenWBTC, [], 5000000);
  
  infoMangerNToken=await asset.getAccountInfoNew(assetAccount);
  console.log("after transferFrom infoassetAccount mint --"+infoMangerNToken.mint+" --owner --"+infoMangerNToken.owner +" -amount --"+infoMangerNToken.amount+"-- allownace --"+infoMangerNToken.delegatedAmount)


  infoMangerNToken=await asset.getAccountInfoNew(managerNTokenWBTC.publicKey);
 
  console.log("after transferFrom infoMangerNToken mint --"+infoMangerNToken.mint+" --owner --"+infoMangerNToken.owner +" -amount --"+infoMangerNToken.amount+"-- allownace --"+infoMangerNToken.delegatedAmount)

  let accountManagerNTokenWBTC = await asset.createAccountNew(managerNTokenWBTC.publicKey);

  const source  = await newAccountWithLamports(connection, 10000000000 /* wag */);

  let testAccount2 = await testToken.createAccount(source.publicKey);
  console.log("owner testAccount -- "+source.publicKey)
  console.log("created testaccount is : " + testAccount.toBase58());

  let accountInfo;
  accountInfo = await testToken.getAccountInfo(testAccount);

  console.log("**********Info nToken Account **************");
  console.log("mint nWBTC -- "+accountInfo.mint +" -- owner UserA --"+accountInfo.owner+" -- amount --"+accountInfo.amount+" -- amount wbtc --"+accountInfo.asset+" amount usdc --"+accountInfo.usdc)
  console.log("***end info nToken Account ******")



  await testToken.createDeposit(managerNTokenWBTC,managerNTokenUSDC,payer, 1000 , 10);
  //await testToken.createDeposit(accountManagerNTokenWBTC,managerNTokenUSDC,payer, 1000 , 10);

  //await transferAfterDeposit(accountKey,payer);
}

export async function withDraw(): Promise<void> {
  console.log("run test withdraw");
  const connection = await getConnection();
  const payer = await newAccountWithLamports(connection, 1000000000 /* wag */);
  accountKey = await testToken.createAccount(payer.publicKey);
  //runGetFullBalance(accountKey)
  await testToken.createWithDraw( accountKey ,10,payer);
  //
  runGetFullBalance(testAccount);
}


export async function createPortfolio() : Promise<void> {
  console.log ("start");
  testAccount = await testToken.createPortfolio(testAccountOwner.publicKey);




}


export async function infoAccountByPublicKey(): Promise<void> {
  const connection = await getConnection();
 let account = new PublicKey("6QJJ6VA4wm2bKXRGSJQPFStCas7mNreiHgbNUogKHAgJ");
  connection.getAccountInfo(account, 'confirmed')
.then(
  account => {
    console.log ("info"+account)
    if ((account)  && (account.owner)) {
      
      const data = Buffer.from(account.data);
      const accountInfo = AccountLayout.decode(data);
      if (accountInfo.owner) {
        console.log(  "Owner  => " + new PublicKey(accountInfo.owner).toBase58());
        console.log(  "Amount  => " + new PublicKey(accountInfo.amount).toBase58());
        console.log(  "USDC  => " + new PublicKey(accountInfo.usdc).toBase58());
        console.log(  "ASSET  => " + new PublicKey(accountInfo.asset).toBase58());
      }
    }
  }
).catch(a => {console.log("error info account")})
}


export async function createAccount(): Promise<void> {
  testAccountOwner = new Account([253,105,193,173,55,108,145,101,186,22,187,172,156,119,173,35,25,99,80,68,92,204,232,243,67,169,199,7,218,94,225,17,173,31,39,116,250,166,211,3,213,13,179,50,47,240,7,164,48,110,143,141,244,242,74,210,185,203,0,4,138,99,110,251]);

  
  //nToken Account nWBTC: 

  testAccount = await testToken.createAccount(testAccountOwner.publicKey);
 // testAccount=new PublicKey("6wyLxVejQGiUSzdNS7VvUM4ETBkpXYtSRgTqDtTVoXsX");

  console.log("owner testAccount -- "+testAccountOwner.publicKey)
  console.log("created testaccount is : " + testAccount.toBase58());

  let accountInfo;
  accountInfo = await testToken.getAccountInfo(testAccount);

  console.log("**********Info nToken Account **************");
  console.log("mint nWBTC -- "+accountInfo.mint +" -- owner UserA --"+accountInfo.owner+" -- amount --"+accountInfo.amount+" -- amount wbtc --"+accountInfo.asset+" amount usdc --"+accountInfo.usdc)
  console.log("***end info nToken Account ******")

  //Token Account WBTC:

  //assetAccount=await asset.createAccountNew(testAccountOwner.publicKey);
  assetAccount=new PublicKey("HhXqr6VokjdSZT1BJj7zn5fafJBvSbkrxVYkJX11UmAy");
  console.log("created assetaccount is : " + assetAccount.toBase58());
 // await asset.mintTo(assetAccount.publicKey,testAccountOwner,[],2000000000)

  let accountInfoAsset = await asset.getAccountInfoNew(assetAccount);
  console.log("**********Info Token Account wbtc**************");
  console.log("mint WBTC -- "+accountInfoAsset.mint +" -- owner UserA --"+accountInfoAsset.owner+" -- amount --"+accountInfoAsset.amount)
  console.log("***end info Token Account wbtc******")
 
   //Token Account USDC:
 // usdcAccount=await USDC.createAccountNew(testAccountOwner.publicKey);
  usdcAccount=new PublicKey("FY7nxSgM1HyAz9aiLbPkMnzgEqhmgN49VZn15SXJngnD")
  console.log("created usdcaccount is : " + usdcAccount.toBase58());

   let accountInfoUSDC = await USDC.getAccountInfoNew(usdcAccount);
  console.log("**********Info Token Account usdc **************");
  console.log("mint usdc -- "+accountInfoUSDC.mint +" -- owner UserA --"+accountInfoUSDC.owner+" -- amount --"+accountInfoUSDC.amount)
  console.log("***end info Token Account usdc ******")
 


  assert(accountInfo.mint.equals(testToken.publicKey));
  assert(accountInfo.owner.equals(testAccountOwner.publicKey));
  assert(accountInfo.amount.toNumber() === 0);
  assert(accountInfo.delegate === null);
  assert(accountInfo.delegatedAmount.toNumber() === 0);
  assert(accountInfo.isInitialized === true);
  assert(accountInfo.isFrozen === false);
  assert(accountInfo.isNative === false);
  assert(accountInfo.rentExemptReserve === null);
  assert(accountInfo.closeAuthority === null);

  // you can create as many accounts as with same owner
  const testAccount2 = await testToken.createAccount(
    testAccountOwner.publicKey,
  );
  assert(!testAccount2.equals(testAccount));
}

export async function createAssociatedAccount(): Promise<void> {
  let info;
  const connection = await getConnection();

  const owner = new Account();
  const associatedAddress = await nToken.getAssociatedTokenAddress(
    associatedProgramId,
    programId,
    testToken.publicKey,
    owner.publicKey,
  );

  // associated account shouldn't exist
  info = await connection.getAccountInfo(associatedAddress);
  assert(info == null);

  const createdAddress = await testToken.createAssociatedTokenAccount(
    owner.publicKey,
  );
  assert(createdAddress.equals(associatedAddress));

  // associated account should exist now
  info = await testToken.getAccountInfo(associatedAddress);
  assert(info != null);
  assert(info.mint.equals(testToken.publicKey));
  assert(info.owner.equals(owner.publicKey));
  assert(info.amount.toNumber() === 0);

  // creating again should cause TX error for the associated token account
  assert(
    await didThrow(testToken, testToken.createAssociatedTokenAccount, [
      owner.publicKey,
    ]),
  );
}

export async function mintTo(): Promise<void> {
  await testToken.mintTo(testAccount, testMintAuthority, [], 1000);
  let mintAuthorityAsset=new Account([253,105,193,173,55,108,145,101,186,22,187,172,156,119,173,35,25,99,80,68,92,204,232,243,67,169,199,7,218,94,225,17,173,31,39,116,250,166,211,3,213,13,179,50,47,240,7,164,48,110,143,141,244,242,74,210,185,203,0,4,138,99,110,251])
  
  await asset.mintTo(assetAccount, UserAccountAsset, [], 10000);
  const assetInfo = await asset.getAccountInfo(assetAccount);
  console.log("mintTo : min asset --"+assetInfo.mint +" -- owner --"+assetInfo.owner+" -- address Account --"+assetInfo.address +"-- amount before mintTo --"+assetInfo.amount)

  const mintInfo = await testToken.getMintInfo();
  assert(mintInfo.supply.toNumber() === 1000);

  const accountInfo = await testToken.getAccountInfo(testAccount);
  assert(accountInfo.amount.toNumber() === 1000);
  console.log(" usdc = " + accountInfo.usdc.toNumber());
  console.log(" asset = " + accountInfo.asset.toNumber());
}

export async function runGetFullBalance(account = testAccount): Promise<void> {
  console.log("run get full balance");
  const accountInfo = await testToken.getAccountInfo(account);
  console.log ("amount:"+accountInfo.amount.toNumber(),"usdc:"+accountInfo.usdc.toNumber(),"asset:"+accountInfo.asset.toNumber())
  return ({"amount":accountInfo.amount.toNumber(),"usdc":accountInfo.usdc.toNumber(),"asset":accountInfo.asset.toNumber()})
}

export async function mintToChecked(): Promise<void> {
  assert(
    await didThrow(testToken, testToken.mintToChecked, [
      testAccount,
      testMintAuthority,
      [],
      1000,
      1,
    ]),
  );

  await testToken.mintToChecked(testAccount, testMintAuthority, [], 1000, 2);

  const mintInfo = await testToken.getMintInfo();
  assert(mintInfo.supply.toNumber() === 2000);

  const accountInfo = await testToken.getAccountInfo(testAccount);
  assert(accountInfo.amount.toNumber() === 2000);
}

export async function transfer(): Promise<void> {
  const destOwner = new Account();
  const dest = await  testToken.createAccount(destOwner.publicKey);


  let accountInfo = await testToken.getAccountInfo(testAccount);
  console.log(" source amount befor transfer = " + accountInfo.amount);
  console.log(" dest is " + testAccountOwner.publicKey);
  await testToken.transfer(testAccount, dest, testAccountOwner, [], 100);

  const mintInfo = await testToken.getAccountInfo(testAccount);
  assert(mintInfo.amount.toNumber() === 900);
  console.log("Full balance of sender after transfer : ")
  console.log(" amount = " + mintInfo.amount.toNumber());
  console.log(" usdc = " + mintInfo.usdc.toNumber());
  console.log(" asset = " + mintInfo.asset.toNumber());

  
  let destAccountInfo = await testToken.getAccountInfo(dest);
  assert(destAccountInfo.amount.toNumber() === 100);
  console.log("Full balance of receipt after transfer : ")
  console.log(" amount = " + destAccountInfo.amount.toNumber());
  console.log(" usdc = " + destAccountInfo.usdc.toNumber());
  console.log(" asset = " + destAccountInfo.asset.toNumber());
 
}



export async function transferAfterDeposit(accountSource , accountSourceOwner): Promise<void> {
  const destOwner = new Account();
  const dest = await  testToken.createAccount(destOwner.publicKey);


  let accountInfo = await testToken.getAccountInfo(accountSource);
  console.log(" source amount befor transfer = " + accountInfo.amount);
  console.log(" accountSourceOwner is " + accountSourceOwner.publicKey);
  await testToken.transfer(accountSource, dest, accountSourceOwner, [], 100);

  const mintInfo = await testToken.getAccountInfo(accountSource);
  assert(mintInfo.amount.toNumber() === 900);
  console.log("Full balance of sender after transfer : ")
  console.log(" amount = " + mintInfo.amount.toNumber());
  console.log(" usdc = " + mintInfo.usdc.toNumber());
  console.log(" asset = " + mintInfo.asset.toNumber());

  
  let destAccountInfo = await testToken.getAccountInfo(dest);
  assert(destAccountInfo.amount.toNumber() === 100);
  console.log("Full balance of receipt after transfer : ")
  console.log(" amount = " + destAccountInfo.amount.toNumber());
  console.log(" usdc = " + destAccountInfo.usdc.toNumber());
  console.log(" asset = " + destAccountInfo.asset.toNumber());
 
}





export async function transferChecked(): Promise<void> {
  const destOwner = new Account();
  const dest = await testToken.createAccount(destOwner.publicKey);

  assert(
    await didThrow(testToken, testToken.transferChecked, [
      testAccount,
      dest,
      testAccountOwner,
      [],
      100,
      testTokenDecimals - 1,
    ]),
  );

  await testToken.transferChecked(
    testAccount,
    dest,
    testAccountOwner,
    [],
    100,
    testTokenDecimals,
  );

  const mintInfo = await testToken.getMintInfo();
  assert(mintInfo.supply.toNumber() === 2000);

  let destAccountInfo = await testToken.getAccountInfo(dest);
  assert(destAccountInfo.amount.toNumber() === 100);

  let testAccountInfo = await testToken.getAccountInfo(testAccount);
  assert(testAccountInfo.amount.toNumber() === 1800);
}

export async function transferCheckedAssociated(): Promise<void> {
  const dest = new Account().publicKey;
  let associatedAccount;

  associatedAccount = await testToken.getOrCreateAssociatedAccountInfo(dest);
  assert(associatedAccount.amount.toNumber() === 0);

  await testToken.transferChecked(
    testAccount,
    associatedAccount.address,
    testAccountOwner,
    [],
    123,
    testTokenDecimals,
  );

  associatedAccount = await testToken.getOrCreateAssociatedAccountInfo(dest);
  assert(associatedAccount.amount.toNumber() === 123);
}

export async function approveRevoke(): Promise<void> {
  const delegate = new Account().publicKey;

  await testToken.approve(testAccount, delegate, testAccountOwner, [], 42);

  let testAccountInfo = await testToken.getAccountInfo(testAccount);
  assert(testAccountInfo.delegatedAmount.toNumber() === 42);
  if (testAccountInfo.delegate === null) {
    throw new Error('delegate should not be null');
  } else {
    assert(testAccountInfo.delegate.equals(delegate));
  }

  await testToken.revoke(testAccount, testAccountOwner, []);

  testAccountInfo = await testToken.getAccountInfo(testAccount);
  assert(testAccountInfo.delegatedAmount.toNumber() === 0);
  if (testAccountInfo.delegate !== null) {
    throw new Error('delegate should be null');
  }
}

export async function failOnApproveOverspend(): Promise<void> {
  const owner = new Account();
  const account1 = await testToken.createAccount(owner.publicKey);
  const account2 = await testToken.createAccount(owner.publicKey);
  const delegate = new Account();

  await testToken.transfer(testAccount, account1, testAccountOwner, [], 10);

  await testToken.approve(account1, delegate.publicKey, owner, [], 2);

  let account1Info = await testToken.getAccountInfo(account1);
  assert(account1Info.amount.toNumber() == 10);
  assert(account1Info.delegatedAmount.toNumber() == 2);
  if (account1Info.delegate === null) {
    throw new Error('delegate should not be null');
  } else {
    assert(account1Info.delegate.equals(delegate.publicKey));
  }

  await testToken.transfer(account1, account2, delegate, [], 1);

  account1Info = await testToken.getAccountInfo(account1);
  assert(account1Info.amount.toNumber() == 9);
  assert(account1Info.delegatedAmount.toNumber() == 1);

  await testToken.transfer(account1, account2, delegate, [], 1);

  account1Info = await testToken.getAccountInfo(account1);
  assert(account1Info.amount.toNumber() == 8);
  assert(account1Info.delegate === null);
  assert(account1Info.delegatedAmount.toNumber() == 0);

  assert(
    await didThrow(testToken, testToken.transfer, [
      account1,
      account2,
      delegate,
      [],
      1,
    ]),
  );
}

export async function setAuthority(): Promise<void> {
  const newOwner = new Account();
  await testToken.setAuthority(
    testAccount,
    newOwner.publicKey,
    'AccountOwner',
    testAccountOwner,
    [],
  );
  assert(
    await didThrow(testToken, testToken.setAuthority, [
      testAccount,
      newOwner.publicKey,
      'AccountOwner',
      testAccountOwner,
      [],
    ]),
  );
  await testToken.setAuthority(
    testAccount,
    testAccountOwner.publicKey,
    'AccountOwner',
    newOwner,
    [],
  );
}

export async function burn(): Promise<void> {
  let accountInfo = await testToken.getAccountInfo(testAccount);
  const amount = accountInfo.amount.toNumber();

  await testToken.burn(testAccount, testAccountOwner, [], 1);

  accountInfo = await testToken.getAccountInfo(testAccount);
  assert(accountInfo.amount.toNumber() == amount - 1);
}

export async function burnChecked(): Promise<void> {
  let accountInfo = await testToken.getAccountInfo(testAccount);
  const amount = accountInfo.amount.toNumber();

  assert(
    await didThrow(testToken, testToken.burnChecked, [
      testAccount,
      testAccountOwner,
      [],
      1,
      1,
    ]),
  );

  await testToken.burnChecked(testAccount, testAccountOwner, [], 1, 2);

  accountInfo = await testToken.getAccountInfo(testAccount);
  assert(accountInfo.amount.toNumber() == amount - 1);
}

export async function freezeThawAccount(): Promise<void> {
  let accountInfo = await testToken.getAccountInfo(testAccount);
  const amount = accountInfo.amount.toNumber();

  await testToken.freezeAccount(testAccount, testMintAuthority, []);

  const destOwner = new Account();
  const dest = await testToken.createAccount(destOwner.publicKey);

  assert(
    await didThrow(testToken, testToken.transfer, [
      testAccount,
      dest,
      testAccountOwner,
      [],
      100,
    ]),
  );

  await testToken.thawAccount(testAccount, testMintAuthority, []);

  await testToken.transfer(testAccount, dest, testAccountOwner, [], 100);

  let testAccountInfo = await testToken.getAccountInfo(testAccount);
  assert(testAccountInfo.amount.toNumber() === amount - 100);
}

export async function closeAccount(): Promise<void> {
  const closeAuthority = new Account();

  await testToken.setAuthority(
    testAccount,
    closeAuthority.publicKey,
    'CloseAccount',
    testAccountOwner,
    [],
  );
  let accountInfo = await testToken.getAccountInfo(testAccount);
  if (accountInfo.closeAuthority === null) {
    assert(accountInfo.closeAuthority !== null);
  } else {
    assert(accountInfo.closeAuthority.equals(closeAuthority.publicKey));
  }

  const dest = await testToken.createAccount(new Account().publicKey);
  const remaining = accountInfo.amount.toNumber();

  // Check that accounts with non-zero token balance cannot be closed
  assert(
    await didThrow(testToken, testToken.closeAccount, [
      testAccount,
      dest,
      closeAuthority,
      [],
    ]),
  );

  const connection = await getConnection();
  let tokenRentExemptAmount;
  let info = await connection.getAccountInfo(testAccount);
  if (info != null) {
    tokenRentExemptAmount = info.lamports;
  } else {
    throw new Error('Account not found');
  }

  // Transfer away all tokens
  await testToken.transfer(testAccount, dest, testAccountOwner, [], remaining);

  // Close for real
  await testToken.closeAccount(testAccount, dest, closeAuthority, []);

  info = await connection.getAccountInfo(testAccount);
  assert(info === null);

  let destInfo = await connection.getAccountInfo(dest);
  if (destInfo !== null) {
    assert(destInfo.lamports === 2 * tokenRentExemptAmount);
  } else {
    throw new Error('Account not found');
  }

  let destAccountInfo = await testToken.getAccountInfo(dest);
  assert(destAccountInfo.amount.toNumber() === remaining);
}

export async function multisig(): Promise<void> {
  const m = 2;
  const n = 5;

  let signerAccounts = [];
  for (var i = 0; i < n; i++) {
    signerAccounts.push(new Account());
  }
  let signerPublicKeys = [];
  signerAccounts.forEach(account => signerPublicKeys.push(account.publicKey));
  const multisig = await testToken.createMultisig(m, signerPublicKeys);
  const multisigInfo = await testToken.getMultisigInfo(multisig);
  assert(multisigInfo.m === m);
  assert(multisigInfo.n === n);
  assert(multisigInfo.signer1.equals(signerPublicKeys[0]));
  assert(multisigInfo.signer2.equals(signerPublicKeys[1]));
  assert(multisigInfo.signer3.equals(signerPublicKeys[2]));
  assert(multisigInfo.signer4.equals(signerPublicKeys[3]));
  assert(multisigInfo.signer5.equals(signerPublicKeys[4]));

  const multisigOwnedAccount = await testToken.createAccount(multisig);
  const finalDest = await testToken.createAccount(multisig);

  await testToken.mintTo(multisigOwnedAccount, testMintAuthority, [], 1000);

  // Transfer via multisig
  await testToken.transfer(
    multisigOwnedAccount,
    finalDest,
    multisig,
    signerAccounts,
    1,
  );
  await sleep(500);
  let accountInfo = await testToken.getAccountInfo(finalDest);
  assert(accountInfo.amount.toNumber() == 1);

  // Approve via multisig
  {
    const delegate = new PublicKey(0);
    await testToken.approve(
      multisigOwnedAccount,
      delegate,
      multisig,
      signerAccounts,
      1,
    );
    const accountInfo = await testToken.getAccountInfo(multisigOwnedAccount);
    assert(accountInfo.delegate != null);
    if (accountInfo.delegate != null) {
      assert(accountInfo.delegate.equals(delegate));
      assert(accountInfo.delegatedAmount.toNumber() == 1);
    }
  }

  // SetAuthority of account via multisig
  {
    const newOwner = new PublicKey(0);
    await testToken.setAuthority(
      multisigOwnedAccount,
      newOwner,
      'AccountOwner',
      multisig,
      signerAccounts,
    );
    const accountInfo = await testToken.getAccountInfo(multisigOwnedAccount);
    assert(accountInfo.owner.equals(newOwner));
  }
}

export async function nativeToken(): Promise<void> {
  const connection = await getConnection();
  // this user both pays for the creation of the new token account
  // and provides the lamports to wrap
  const payer = await newAccountWithLamports(connection, 2000000000 /* wag */);
  const lamportsToWrap = 1000000000;

  const token = new nToken(connection, NATIVE_MINT, programId, payer);
  const owner = new Account();
  const native = await nToken.createWrappedNativeAccount(
    connection,
    programId,
    owner.publicKey,
    payer,
    lamportsToWrap,
  );
  let accountInfo = await token.getAccountInfo(native);
  assert(accountInfo.isNative);

  // check that the new account has wrapped native tokens.
  assert(accountInfo.amount.toNumber() === lamportsToWrap);

  let balance;
  let info = await connection.getAccountInfo(native);
  if (info != null) {
    balance = info.lamports;
  } else {
    throw new Error('Account not found');
  }

  const balanceNeeded = await connection.getMinimumBalanceForRentExemption(0);
  const dest = await newAccountWithLamports(connection, balanceNeeded);
  await token.closeAccount(native, dest.publicKey, owner, []);
  info = await connection.getAccountInfo(native);
  if (info != null) {
    throw new Error('Account not burned');
  }
  info = await connection.getAccountInfo(dest.publicKey);
  if (info != null) {
    assert(info.lamports == balanceNeeded + balance);
  } else {
    throw new Error('Account not found');
  }
}
