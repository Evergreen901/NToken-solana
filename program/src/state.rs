//! State transition types

use crate::instruction::MAX_SIGNERS;
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use num_enum::TryFromPrimitive;
use solana_program::{
    program_error::ProgramError,
    program_option::COption,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
    msg
};
use std::convert::TryInto;

/// Mint data.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Mint {
    /// Optional authority used to mint new tokens. The mint authority may only be provided during
    /// mint creation. If no mint authority is present then the mint has a fixed supply and no
    /// further tokens may be minted.
    pub mint_authority: COption<Pubkey>,
    /// Total supply of tokens.
    pub supply: u64,
    /// Number of base 10 digits to the right of the decimal place.
    pub decimals: u8,
    /// Is `true` if this structure has been initialized
    pub is_initialized: bool,
    /// Optional authority to freeze token accounts.
    pub freeze_authority: COption<Pubkey>,
    /// The mint id of the asset that will be hedged in the program.
    pub mint_id_asset:COption<Pubkey>,
    /// public key of swap .
    pub pubkey_swap:COption<Pubkey>,
}
impl Sealed for Mint {}
impl IsInitialized for Mint {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
impl Pack for Mint {
    const LEN: usize = 154;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 154];
        let (mint_authority, supply, decimals, is_initialized, freeze_authority,mint_id_asset, pubkey_swap) =
            array_refs![src, 36, 8, 1, 1, 36 , 36 , 36];
        let mint_authority = unpack_coption_key(mint_authority)?;
        let supply = u64::from_le_bytes(*supply);
        let decimals = decimals[0];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return  { 
                Err(ProgramError::InvalidAccountData)
            },
        };
        let freeze_authority = unpack_coption_key(freeze_authority)?;
        let mint_id_asset = unpack_coption_key(mint_id_asset)?;
        let pubkey_swap = unpack_coption_key(pubkey_swap)?;
        Ok(Mint {
            mint_authority,
            supply,
            decimals,
            is_initialized,
            freeze_authority,
            mint_id_asset,
            pubkey_swap,
        })
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 154];
        let (
            mint_authority_dst,
            supply_dst,
            decimals_dst,
            is_initialized_dst,
            freeze_authority_dst,
            mint_id_asset_dst,
            pubkey_swap_dst,
        ) = mut_array_refs![dst, 36, 8, 1, 1, 36,36,36];
        let &Mint {
            ref mint_authority,
            supply,
            decimals,
            is_initialized,
            ref freeze_authority,
            ref mint_id_asset,
            ref pubkey_swap,
        } = self;
        pack_coption_key(mint_authority, mint_authority_dst);
        *supply_dst = supply.to_le_bytes();
        decimals_dst[0] = decimals;
        is_initialized_dst[0] = is_initialized as u8;
        pack_coption_key(freeze_authority, freeze_authority_dst);
        pack_coption_key(mint_id_asset, mint_id_asset_dst);
        pack_coption_key(pubkey_swap, pubkey_swap_dst);
    }
}

/// Account data.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Account {
    /// The mint associated with this account
    pub mint: Pubkey,
    /// The owner of this account.
    pub owner: Pubkey,
    /// The amount of tokens this account holds.
    pub amount: u64,
    /// If `delegate` is `Some` then `delegated_amount` represents
    /// the amount authorized by the delegate
    pub delegate: COption<Pubkey>,
    /// The account's state
    pub state: AccountState,
    /// If is_some, this is a native token, and the value logs the rent-exempt reserve. An Account
    /// is required to be rent-exempt, so the value is used by the Processor to ensure that wrapped
    /// SOL accounts do not drop below this threshold.
    pub is_native: COption<u64>,
    /// The amount delegated
    pub delegated_amount: u64,
    /// Optional authority to close the account.
    pub close_authority: COption<Pubkey>,
     /// the amount of token asset 
    pub asset: u64,
    /// the amount of token usdc
    pub usdc: u64,
}
impl Account {
    /// Checks if account is frozen
    pub fn is_frozen(&self) -> bool {
        self.state == AccountState::Frozen
    }
    /// Checks if account is native
    pub fn is_native(&self) -> bool {
        self.is_native.is_some()
    }
}
impl Sealed for Account {}
impl IsInitialized for Account {
    fn is_initialized(&self) -> bool {
        self.state != AccountState::Uninitialized
    }
}
/*
 Layout.publicKey('mint'), //  32
    Layout.publicKey('owner'), //32
    Layout.uint64('amount'), // 8
    Layout.uint64('usdc'), // 8
    Layout.uint64('asset'), // 8
    BufferLayout.u32('delegateOption'), 
    Layout.publicKey('delegate'),// 36
    BufferLayout.u8('state'), // 1
    BufferLayout.u32('isNativeOption'), 
    Layout.uint64('isNative'), //12
    Layout.uint64('delegatedAmount'),// 8
    BufferLayout.u32('closeAuthorityOption'),
    Layout.publicKey('closeAuthority'),//36


*/
impl Pack for Account {
    const LEN: usize = 181;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 181];
        let (mint, owner, amount, delegate, state, is_native, delegated_amount, close_authority,asset,usdc) =
            array_refs![src, 32, 32, 8, 36, 1, 12, 8, 36 , 8 , 8];
        Ok(Account {
            mint: Pubkey::new_from_array(*mint),
            owner: Pubkey::new_from_array(*owner),
            amount: u64::from_le_bytes(*amount),
            delegate: unpack_coption_key(delegate)?,
            state: AccountState::try_from_primitive(state[0])
                .or(Err(ProgramError::InvalidAccountData))?,
            is_native: unpack_coption_u64(is_native)?,
            delegated_amount: u64::from_le_bytes(*delegated_amount),
            close_authority: unpack_coption_key(close_authority)?,
            asset: u64::from_le_bytes(*asset),
            usdc: u64::from_le_bytes(*usdc),
        })
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 181];
        let (
            mint_dst,
            owner_dst,
            amount_dst,
            delegate_dst,
            state_dst,
            is_native_dst,
            delegated_amount_dst,
            close_authority_dst,
            asset_dst,
            usdc_dst,
        ) = mut_array_refs![dst, 32, 32, 8, 36, 1, 12, 8, 36,8,8];
        let &Account {
            ref mint,
            ref owner,
            amount,
            ref delegate,
            state,
            ref is_native,
            delegated_amount,
            ref close_authority,
             asset,
             usdc,
        } = self;
        mint_dst.copy_from_slice(mint.as_ref());
        owner_dst.copy_from_slice(owner.as_ref());
        *amount_dst = amount.to_le_bytes();
        pack_coption_key(delegate, delegate_dst);
        state_dst[0] = state as u8;
        pack_coption_u64(is_native, is_native_dst);
        *delegated_amount_dst = delegated_amount.to_le_bytes();
        pack_coption_key(close_authority, close_authority_dst);
        *asset_dst = asset.to_le_bytes();
        *usdc_dst = usdc.to_le_bytes();
        
    }
}

/// Account state.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, TryFromPrimitive)]
pub enum AccountState {
    /// Account is not yet initialized
    Uninitialized,
    /// Account is initialized; the account owner and/or delegate may perform permitted operations
    /// on this account
    Initialized,
    /// Account has been frozen by the mint freeze authority. Neither the account owner nor
    /// the delegate are able to perform operations on this account.
    Frozen,
}

impl Default for AccountState {
    fn default() -> Self {
        AccountState::Uninitialized
    }
}

/// Account data.
#[repr(C)]
#[derive(Clone, /*Copy,*/ Debug, Default, PartialEq)]
pub struct Portfolio {
      /// The account's creator
      pub portfolio_account: Pubkey,
    /// The owner of this account.
    pub creator_portfolio: Pubkey,
    /// The data of portfolio.
    pub metadataUrl: Vec<u8>,
    /// the hash of data
    pub metadataHash: u16,
    /// is initialize
    pub is_initialize: u8,
    /// the amount of first asset
    pub amountAsset1: u8,
    /// The first asset's address
    pub addressAsset1: Pubkey,
    /// First Asset's period
    pub periodAsset1: u8,
     /// the first asset to sold asset
    pub assetToSoldIntoAsset1: Pubkey,
    /// the amount of second asset
    pub amountAsset2: u8,
    /// The second asset's address
    pub addressAsset2: Pubkey,
    /// Second Asset's period
    pub periodAsset2: u8,
     /// the second asset to sold asset
    pub assetToSoldIntoAsset2: Pubkey,
    /// the amount of third asset
    pub amountAsset3: u8,
    /// The third asset's address
    pub addressAsset3: Pubkey,
    /// third Asset's period
    pub periodAsset3: u8,
     /// the third asset to sold asset
    pub assetToSoldIntoAsset3: Pubkey,
    /// the amount of firth asset
    pub amountAsset4: u8,
    /// The firth asset's address
    pub addressAsset4: Pubkey,
    /// firth Asset's period
    pub periodAsset4: u8,
     /// the firth asset to sold asset
    pub assetToSoldIntoAsset4: Pubkey,
    /// the amount of 5th asset
    pub amountAsset5: u8,
    /// The 5th asset's address
    pub addressAsset5: Pubkey,
    /// 5th Asset's period
    pub periodAsset5: u8,
     /// the 5th asset to sold asset
    pub assetToSoldIntoAsset5: Pubkey,
    /// the 6th amount of asset
    pub amountAsset6: u8,
    /// The 6th asset's address
    pub addressAsset6: Pubkey,
    /// 6th Asset's period
    pub periodAsset6: u8,
     /// the 6th asset to sold asset
    pub assetToSoldIntoAsset6: Pubkey,
    /// the 7th amount of asset
    pub amountAsset7: u8,
    /// The 7th asset's address
    pub addressAsset7: Pubkey,
    /// 7th Asset's period
    pub periodAsset7: u8,
     /// the 7th asset to sold asset
    pub assetToSoldIntoAsset7: Pubkey,
    /// the amount of 8th asset
    pub amountAsset8: u8,
    /// The 8th asset's address
    pub addressAsset8: Pubkey,
    /// 8th Asset's period
    pub periodAsset8: u8,
     /// the 8th asset to sold asset
    pub assetToSoldIntoAsset8: Pubkey,
    /// the amount of 9th asset
    pub amountAsset9: u8,
    /// The 9th asset's address
    pub addressAsset9: Pubkey,
    /// 9th Asset's period
    pub periodAsset9: u8,
     /// the 9th asset to sold asset
    pub assetToSoldIntoAsset9: Pubkey,
    // /// the amount of 10th asset
    // pub amountAsset10: u8,
    // /// The 10th asset's address
    // pub addressAsset10: Pubkey,
    // /// 10th Asset's period
    // pub periodAsset10: u32,
    //  /// the 10th asset to sold asset
    // pub assetToSoldIntoAsset10: Pubkey,
}

fn convert<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}
impl Sealed for Portfolio {}
impl IsInitialized for Portfolio {
    fn is_initialized(&self) -> bool {
  return true;
  
      // return self.is_initialized == 1;
}
}


impl Pack for Portfolio {
    const LEN: usize = 789;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
   
        let src = array_ref![src, 0, 789];
    
        let (portfolio_account,creator_portfolio , metadataUrl, metadataHash, is_initialize, amountAsset1, addressAsset1, periodAsset1,
            assetToSoldIntoAsset1, amountAsset2, addressAsset2, periodAsset2,assetToSoldIntoAsset2, amountAsset3, 
            addressAsset3, periodAsset3,assetToSoldIntoAsset3, amountAsset4, addressAsset4, periodAsset4,
            assetToSoldIntoAsset4, amountAsset5, addressAsset5, periodAsset5,assetToSoldIntoAsset5, amountAsset6, 
            addressAsset6, periodAsset6,assetToSoldIntoAsset6, amountAsset7, addressAsset7, periodAsset7,
            assetToSoldIntoAsset7, amountAsset8, addressAsset8, periodAsset8,assetToSoldIntoAsset8, amountAsset9, 
            addressAsset9, periodAsset9,assetToSoldIntoAsset9/*, amountAsset10, addressAsset10, periodAsset10,
            assetToSoldIntoAsset10*/) =
            array_refs![src,32, 32, 128, 2, 1, 1, 32 , 1, 32, 1, 32 , 1 , 32, 1, 32 , 1 , 32, 1, 32 , 
            1 , 32, 1, 32 , 1 , 32, 1, 32 , 1 , 32, 1, 32 , 1 , 32, 1, 32 , 1 , 32, 1, 32 , 1 , 32/*, 1, 32 
            , 1 , 32*/];
   
              Ok(Portfolio {
            portfolio_account: Pubkey::new_from_array(*portfolio_account),
            creator_portfolio: Pubkey::new_from_array(*creator_portfolio),
            metadataUrl: metadataUrl.to_vec(),
            metadataHash: u16::from_le_bytes(*metadataHash),
            
            /*is_initialized:  match is_initialized {
                [0] => 0,
                [1] => 1,
                _ => return  { 
                    Err(ProgramError::InvalidAccountData)
                },
            },*/
            is_initialize: u8::from_le_bytes(*is_initialize),
            amountAsset1: u8::from_le_bytes(*amountAsset1),
            addressAsset1: Pubkey::new_from_array(*addressAsset1),
            periodAsset1: u8::from_le_bytes(*periodAsset1),
            assetToSoldIntoAsset1: Pubkey::new_from_array(*assetToSoldIntoAsset1),
            amountAsset2: u8::from_le_bytes(*amountAsset2),
            addressAsset2: Pubkey::new_from_array(*addressAsset2),
            periodAsset2: u8::from_le_bytes(*periodAsset2),
            assetToSoldIntoAsset2: Pubkey::new_from_array(*assetToSoldIntoAsset2),
            amountAsset3: u8::from_le_bytes(*amountAsset3),
            addressAsset3: Pubkey::new_from_array(*addressAsset3),
            periodAsset3: u8::from_le_bytes(*periodAsset3),
            assetToSoldIntoAsset3: Pubkey::new_from_array(*assetToSoldIntoAsset3),
            amountAsset4: u8::from_le_bytes(*amountAsset4),
            addressAsset4: Pubkey::new_from_array(*addressAsset4),
            periodAsset4: u8::from_le_bytes(*periodAsset4),
            assetToSoldIntoAsset4: Pubkey::new_from_array(*assetToSoldIntoAsset4),
            amountAsset5: u8::from_le_bytes(*amountAsset5),
            addressAsset5: Pubkey::new_from_array(*addressAsset5),
            periodAsset5: u8::from_le_bytes(*periodAsset5),
            assetToSoldIntoAsset5: Pubkey::new_from_array(*assetToSoldIntoAsset5),
            amountAsset6: u8::from_le_bytes(*amountAsset6),
            addressAsset6: Pubkey::new_from_array(*addressAsset6),
            periodAsset6: u8::from_le_bytes(*periodAsset6),
            assetToSoldIntoAsset6: Pubkey::new_from_array(*assetToSoldIntoAsset6),
            amountAsset7: u8::from_le_bytes(*amountAsset7),
            addressAsset7: Pubkey::new_from_array(*addressAsset7),
            periodAsset7: u8::from_le_bytes(*periodAsset7),
            assetToSoldIntoAsset7: Pubkey::new_from_array(*assetToSoldIntoAsset7),
            amountAsset8: u8::from_le_bytes(*amountAsset8),
            addressAsset8: Pubkey::new_from_array(*addressAsset8),
            periodAsset8: u8::from_le_bytes(*periodAsset8),
            assetToSoldIntoAsset8: Pubkey::new_from_array(*assetToSoldIntoAsset8),
            amountAsset9: u8::from_le_bytes(*amountAsset9),
            addressAsset9: Pubkey::new_from_array(*addressAsset9),
            periodAsset9: u8::from_le_bytes(*periodAsset9),
            assetToSoldIntoAsset9: Pubkey::new_from_array(*assetToSoldIntoAsset9),
            // amountAsset10: u8::from_le_bytes(*amountAsset10),
            // addressAsset10: Pubkey::new_from_array(*addressAsset10),
            // periodAsset10: u8::from_le_bytes(*periodAsset10),
            // assetToSoldIntoAsset10: Pubkey::new_from_array(*assetToSoldIntoAsset10),
        })
  
    }



    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 789];
        let (
            portfolio_account_dst,
            creator_portfolio_dst,
            metadata_URL_dst,
            metadata_HASH_dst,
            is_initialize_dst,
            amount_Asset1_dst,
            address_Asset1_dst,
            period_Asset1_dst,
            asset_To_Sold_Into_Asset1_dst,
            amount_Asset2_dst,
            address_Asset2_dst,
            period_Asset2_dst,
            asset_To_Sold_Into_Asset2_dst,
            amount_Asset3_dst,
            address_Asset3_dst,
            period_Asset3_dst,
            asset_To_Sold_Into_Asset3_dst,
            amount_Asset4_dst,
            address_Asset4_dst,
            period_Asset4_dst,
            asset_To_Sold_Into_Asset4_dst,
            amount_Asset5_dst,
            address_Asset5_dst,
            period_Asset5_dst,
            asset_To_Sold_Into_Asset5_dst,
            amount_Asset6_dst,
            address_Asset6_dst,
            period_Asset6_dst,
            asset_To_Sold_Into_Asset6_dst,
            amount_Asset7_dst,
            address_Asset7_dst,
            period_Asset7_dst,
            asset_To_Sold_Into_Asset7_dst,
            amount_Asset8_dst,
            address_Asset8_dst,
            period_Asset8_dst,
            asset_To_Sold_Into_Asset8_dst,
            amount_Asset9_dst,
            address_Asset9_dst,
            period_Asset9_dst,
            asset_To_Sold_Into_Asset9_dst,
            // amount_Asset10_dst,
            // address_Asset10_dst,
            // period_Asset10_dst,
            // asset_To_Sold_Into_Asset10_dst,

        ) = mut_array_refs![dst, 32,32, 128, 2, 1, 1, 32 , 1 , 32, 1, 32 , 1 , 32, 1, 32 , 1 , 32, 1, 32 , 
        1 , 32, 1, 32 , 1 , 32, 1, 32 , 1 , 32, 1, 32 , 1 , 32, 1, 32 , 1 , 32, 1, 32 , 1 , 32/*, 1, 32 
        , 4 , 32*/];
        let Portfolio {
            ref portfolio_account,
            ref creator_portfolio,
            metadataUrl, 
            metadataHash,
            is_initialize,
            amountAsset1, 
            ref addressAsset1, 
            periodAsset1,
            ref assetToSoldIntoAsset1,
            amountAsset2, 
            ref addressAsset2, 
            periodAsset2,
            ref assetToSoldIntoAsset2, 
            amountAsset3, 
            ref addressAsset3, 
            periodAsset3,
            ref assetToSoldIntoAsset3, 
            amountAsset4, 
            ref addressAsset4, 
            periodAsset4,
            ref assetToSoldIntoAsset4, 
            amountAsset5, 
            ref addressAsset5, 
            periodAsset5,
            ref assetToSoldIntoAsset5, 
            amountAsset6, 
            ref addressAsset6, 
            periodAsset6,
            ref assetToSoldIntoAsset6, 
            amountAsset7, 
            ref  addressAsset7, 
            periodAsset7,
            ref assetToSoldIntoAsset7, 
            amountAsset8, 
            ref addressAsset8, 
            periodAsset8,
            ref assetToSoldIntoAsset8, 
            amountAsset9, 
            ref addressAsset9, 
            periodAsset9,
            ref assetToSoldIntoAsset9
            //, 
            // amountAsset10, 
            // ref addressAsset10, 
            // periodAsset10,
            // ref assetToSoldIntoAsset10
        } = self;
        portfolio_account_dst.copy_from_slice(portfolio_account.as_ref());
        //Pubkey(creatorAccount,creator_Account_dst);
        creator_portfolio_dst.copy_from_slice(creator_portfolio.as_ref());
        //*metadata_URL_dst = convert(metadataURL);
        *metadata_URL_dst = convert(metadataUrl.to_vec());
        // *metadata_URL_dst = metadataURL.borrow();
        *metadata_URL_dst= array_ref!( metadataUrl, 0, 128).clone();/*****/
        *metadata_HASH_dst = metadataHash.to_le_bytes();
        *is_initialize_dst = is_initialize.to_le_bytes();
     
        *amount_Asset1_dst = amountAsset1.to_le_bytes();
        address_Asset1_dst.copy_from_slice(addressAsset1.as_ref());
        //Pubkey(addressAsset1,address_Asset1_dst);
        *period_Asset1_dst = periodAsset1.to_le_bytes();
        asset_To_Sold_Into_Asset1_dst.copy_from_slice(assetToSoldIntoAsset1.as_ref());
        //Pubkey(assetToSoldIntoAsset1,asset_To_Sold_Into_Asset1_dst);
        *amount_Asset2_dst = amountAsset2.to_le_bytes();
        address_Asset2_dst.copy_from_slice(addressAsset2.as_ref());
        //Pubkey(addressAsset2,address_Asset2_dst);
        *period_Asset2_dst = periodAsset2.to_le_bytes();
        asset_To_Sold_Into_Asset2_dst.copy_from_slice(assetToSoldIntoAsset2.as_ref());
        //Pubkey(assetToSoldIntoAsset2,asset_To_Sold_Into_Asset2_dst);
        *amount_Asset3_dst = amountAsset3.to_le_bytes();
        address_Asset3_dst.copy_from_slice(addressAsset3.as_ref());
        //Pubkey(addressAsset3,address_Asset3_dst);
        *period_Asset3_dst = periodAsset3.to_le_bytes();
        asset_To_Sold_Into_Asset3_dst.copy_from_slice(assetToSoldIntoAsset3.as_ref());
        //Pubkey(assetToSoldIntoAsset3,asset_To_Sold_Into_Asset3_dst);
        *amount_Asset4_dst = amountAsset4.to_le_bytes();
        address_Asset4_dst.copy_from_slice(addressAsset4.as_ref());
        //Pubkey(addressAsset4,address_Asset4_dst);
        *period_Asset4_dst = periodAsset4.to_le_bytes();
        asset_To_Sold_Into_Asset4_dst.copy_from_slice(assetToSoldIntoAsset4.as_ref());
        //Pubkey(assetToSoldIntoAsset4,asset_To_Sold_Into_Asset4_dst);
        *amount_Asset5_dst = amountAsset5.to_le_bytes();
        address_Asset5_dst.copy_from_slice(addressAsset5.as_ref());
        //Pubkey(addressAsset5,address_Asset5_dst);
        *period_Asset5_dst = periodAsset5.to_le_bytes();
        asset_To_Sold_Into_Asset5_dst.copy_from_slice(assetToSoldIntoAsset5.as_ref());
        //Pubkey(assetToSoldIntoAsset5,asset_To_Sold_Into_Asset5_dst);
        *amount_Asset6_dst = amountAsset6.to_le_bytes();
        address_Asset6_dst.copy_from_slice(addressAsset6.as_ref());
        //Pubkey(addressAsset6,address_Asset6_dst);
        *period_Asset6_dst = periodAsset6.to_le_bytes();
        asset_To_Sold_Into_Asset6_dst.copy_from_slice(assetToSoldIntoAsset6.as_ref());
        //Pubkey(assetToSoldIntoAsset6,asset_To_Sold_Into_Asset6_dst);
        *amount_Asset7_dst = amountAsset7.to_le_bytes();
        address_Asset7_dst.copy_from_slice(addressAsset7.as_ref());
        //Pubkey(addressAsset7,address_Asset7_dst);
        *period_Asset7_dst = periodAsset7.to_le_bytes();
        asset_To_Sold_Into_Asset7_dst.copy_from_slice(assetToSoldIntoAsset7.as_ref());
        //Pubkey(assetToSoldIntoAsset7,asset_To_Sold_Into_Asset7_dst);
        *amount_Asset8_dst = amountAsset8.to_le_bytes();
        address_Asset8_dst.copy_from_slice(addressAsset8.as_ref());
        //Pubkey(addressAsset8,address_Asset8_dst);
        *period_Asset8_dst = periodAsset8.to_le_bytes();
        asset_To_Sold_Into_Asset8_dst.copy_from_slice(assetToSoldIntoAsset8.as_ref());
        //Pubkey(assetToSoldIntoAsset8,asset_To_Sold_Into_Asset8_dst);
        *amount_Asset9_dst = amountAsset9.to_le_bytes();
        address_Asset9_dst.copy_from_slice(addressAsset9.as_ref());
        //Pubkey(addressAsset9,address_Asset9_dst);
        *period_Asset9_dst = periodAsset9.to_le_bytes();
        asset_To_Sold_Into_Asset9_dst.copy_from_slice(assetToSoldIntoAsset9.as_ref());
        //Pubkey(assetToSoldIntoAsset9,asset_To_Sold_Into_Asset9_dst);
        // *amount_Asset10_dst = amountAsset10.to_le_bytes();
        // address_Asset10_dst.copy_from_slice(addressAsset10.as_ref());
        // //Pubkey(addressAsset10,address_Asset10_dst);
        // *period_Asset10_dst = periodAsset10.to_le_bytes();
        // asset_To_Sold_Into_Asset10_dst.copy_from_slice(assetToSoldIntoAsset10.as_ref());
        //Pubkey(assetToSoldIntoAsset10,asset_To_Sold_Into_Asset10_dst);
       
      
        
    }
}




/// Account data.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UserPortfolio {
    /// The new account.
    pub user_portfolio_account: Pubkey,
    /// portfolio depends of new account
    pub portfolio_address: Pubkey,
    /// The owner of this account.
    pub owner: Pubkey,
    /// If `delegate` is `Some` then `delegated_amount` represents
    /// the amount authorized by the delegate
    pub delegate: Pubkey,
    /// The amount delegated
    pub delegated_amount: u64,
    /// The first asset's address
    pub splu_asset1: Pubkey,
    /// The second asset's address
    pub splu_asset2: Pubkey,
    /// The third asset's address
    pub splu_asset3: Pubkey,
    /// The firth asset's address
    pub splu_asset4: Pubkey,
    /// The 5th asset's address
    pub splu_asset5: Pubkey,
    /// The 6th asset's address
    pub splu_asset6: Pubkey,
    /// The 7th asset's address
    pub splu_asset7: Pubkey,
    /// The 8th asset's address
    pub splu_asset8: Pubkey,
    /// The 9th asset's address
    pub splu_asset9: Pubkey,
   
}

impl Sealed for UserPortfolio {}

impl IsInitialized for UserPortfolio {
    fn is_initialized(&self) -> bool {
  return true;
}
}
impl Pack for UserPortfolio {
    const LEN: usize = 424;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 424];
        let (user_portfolio_account,portfolio_address,owner ,delegate, delegated_amount, splu_asset1,  splu_asset2,
            splu_asset3, splu_asset4,  splu_asset5, splu_asset6,  splu_asset7, splu_asset8, splu_asset9) =
            array_refs![src,32,32, 32, 32, 8, 32, 32 , 32, 32 , 32, 32 , 32, 32 , 32 ];
        Ok(UserPortfolio {
            user_portfolio_account: Pubkey::new_from_array(*user_portfolio_account),
            portfolio_address: Pubkey::new_from_array(*portfolio_address),
            owner: Pubkey::new_from_array(*owner),
           // delegate: unpack_coption_key(delegate)?,
            delegate: Pubkey::new_from_array(*delegate),
            delegated_amount: u64::from_le_bytes(*delegated_amount),
            splu_asset1: Pubkey::new_from_array(*splu_asset1),
            splu_asset2: Pubkey::new_from_array(*splu_asset2),
            splu_asset3: Pubkey::new_from_array(*splu_asset3),
            splu_asset4: Pubkey::new_from_array(*splu_asset4),
            splu_asset5: Pubkey::new_from_array(*splu_asset5),
            splu_asset6: Pubkey::new_from_array(*splu_asset6),
            splu_asset7: Pubkey::new_from_array(*splu_asset7),
            splu_asset8: Pubkey::new_from_array(*splu_asset8),
            splu_asset9: Pubkey::new_from_array(*splu_asset9),

        })
    }



    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 424];
        let (
            user_portfolio_account_dst,
            portfolio_address_dst,
            owner_dst,
            delegate_dst,
            delegated_amount_dst,
            splu_asset1_dst,
            splu_asset2_dst,
            splu_asset3_dst,
            splu_asset4_dst,
            splu_asset5_dst,
            splu_asset6_dst,
            splu_asset7_dst,
            splu_asset8_dst,
            splu_asset9_dst,

        ) = mut_array_refs![dst,32,32, 32,32 , 8 ,  32, 32 ,  32, 32 , 32, 32 , 32,  32  , 32];
        let UserPortfolio {
            user_portfolio_account,
            portfolio_address,
            owner,
            delegate,
            delegated_amount,
            ref splu_asset1, 
            ref splu_asset2, 
            ref splu_asset3, 
            ref splu_asset4, 
            ref splu_asset5, 
            ref splu_asset6, 
            ref  splu_asset7, 
            ref splu_asset8, 
            ref splu_asset9, 

        } = self;
        user_portfolio_account_dst.copy_from_slice(user_portfolio_account.as_ref());
        portfolio_address_dst.copy_from_slice(portfolio_address.as_ref());
        owner_dst.copy_from_slice(owner.as_ref());
        //pack_coption_key(delegate, delegate_dst);
        delegate_dst.copy_from_slice(delegate.as_ref());
        *delegated_amount_dst = delegated_amount.to_le_bytes();
        splu_asset1_dst.copy_from_slice(splu_asset1.as_ref());
        splu_asset2_dst.copy_from_slice(splu_asset2.as_ref());
        splu_asset3_dst.copy_from_slice(splu_asset3.as_ref());
        splu_asset4_dst.copy_from_slice(splu_asset4.as_ref());
        splu_asset5_dst.copy_from_slice(splu_asset5.as_ref());
        splu_asset6_dst.copy_from_slice(splu_asset6.as_ref());
        splu_asset7_dst.copy_from_slice(splu_asset7.as_ref());
        splu_asset8_dst.copy_from_slice(splu_asset8.as_ref());
        splu_asset9_dst.copy_from_slice(splu_asset9.as_ref());
    }
}





/// Multisignature data.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Multisig {
    /// Number of signers required
    pub m: u8,
    /// Number of valid signers
    pub n: u8,
    /// Is `true` if this structure has been initialized
    pub is_initialized: bool,
    /// Signer public keys
    pub signers: [Pubkey; MAX_SIGNERS],
}
impl Sealed for Multisig {}
impl IsInitialized for Multisig {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
impl Pack for Multisig {
    const LEN: usize = 355;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 355];
        #[allow(clippy::ptr_offset_with_cast)]
        let (m, n, is_initialized, signers_flat) = array_refs![src, 1, 1, 1, 32 * MAX_SIGNERS];
        let mut result = Multisig {
            m: m[0],
            n: n[0],
            is_initialized: match is_initialized {
                [0] => false,
                [1] => true,
                _ =>  { 
                    return Err(ProgramError::InvalidAccountData)
                },
            },
            signers: [Pubkey::new_from_array([0u8; 32]); MAX_SIGNERS],
        };
        for (src, dst) in signers_flat.chunks(32).zip(result.signers.iter_mut()) {
            *dst = Pubkey::new(src);
        }
        Ok(result)
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 355];
        #[allow(clippy::ptr_offset_with_cast)]
        let (m, n, is_initialized, signers_flat) = mut_array_refs![dst, 1, 1, 1, 32 * MAX_SIGNERS];
        *m = [self.m];
        *n = [self.n];
        *is_initialized = [self.is_initialized as u8];
        for (i, src) in self.signers.iter().enumerate() {
            let dst_array = array_mut_ref![signers_flat, 32 * i, 32];
            dst_array.copy_from_slice(src.as_ref());
        }
    }
}

// Helpers
fn pack_coption_key(src: &COption<Pubkey>, dst: &mut [u8; 36]) {
    let (tag, body) = mut_array_refs![dst, 4, 32];
    match src {
        COption::Some(key) => {
            *tag = [1, 0, 0, 0];
            body.copy_from_slice(key.as_ref());
        }
        COption::None => {
            *tag = [0; 4];
        }
    }
}

fn unpack_coption_key(src: &[u8; 36]) -> Result<COption<Pubkey>, ProgramError> {
    let (tag, body) = array_refs![src, 4, 32];
    match *tag {
        [0, 0, 0, 0] => Ok(COption::None),
        [1, 0, 0, 0] => Ok(COption::Some(Pubkey::new_from_array(*body))),
        _ =>  {
         
            Err(ProgramError::InvalidAccountData)
        },
    }
}
fn pack_coption_u64(src: &COption<u64>, dst: &mut [u8; 12]) {
    let (tag, body) = mut_array_refs![dst, 4, 8];
    match src {
        COption::Some(amount) => {
            *tag = [1, 0, 0, 0];
            *body = amount.to_le_bytes();
        }
        COption::None => {
            *tag = [0; 4];
        }
    }
}
fn unpack_coption_u64(src: &[u8; 12]) -> Result<COption<u64>, ProgramError> {
    let (tag, body) = array_refs![src, 4, 8];
    match *tag {
        [0, 0, 0, 0] => Ok(COption::None),
        [1, 0, 0, 0] => Ok(COption::Some(u64::from_le_bytes(*body))),
        _ => {
            Err(ProgramError::InvalidAccountData)
        },
    }
}
