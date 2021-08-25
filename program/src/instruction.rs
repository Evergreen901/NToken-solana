//! Instruction types

use crate::error::TokenError;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    program_option::COption,
    pubkey::Pubkey,
    sysvar,
    msg
};
use std::convert::TryInto;
use std::mem::size_of;
 
/// Minimum number of multisignature signers (min N)
pub const MIN_SIGNERS: usize = 1;
/// Maximum number of multisignature signers (max N)
pub const MAX_SIGNERS: usize = 11;

/// Instructions supported by the token program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum TokenInstruction {
    /// Initializes a new mint and optionally deposits all the newly minted
    /// tokens in an account.
    ///
    /// The `InitializeMint` instruction requires no signers and MUST be
    /// included within the same Transaction as the system program's
    /// `CreateAccount` instruction that creates the account being initialized.
    /// Otherwise another party can acquire ownership of the uninitialized
    /// account.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` The mint to initialize.
    ///   1. `[]` Rent sysvar
    ///
    InitializeMint { 
        /// Number of base 10 digits to the right of the decimal place.
        decimals: u8,
        /// The authority/multisignature to mint tokens.
        mint_authority: Pubkey,
        /// The freeze authority/multisignature of the mint.
        freeze_authority: COption<Pubkey>,
        /// program id asset .
        mint_id_asset: COption<Pubkey>,
        /// program id swap.
        pubkey_swap: COption<Pubkey>
        },
    /// Initializes a new account to hold tokens.  If this account is associated
    /// with the native mint then the token balance of the initialized account
    /// will be equal to the amount of SOL in the account. If this account is
    /// associated with another mint, that mint must be initialized before this
    /// command can succeed.
    ///
    /// The `InitializeAccount` instruction requires no signers and MUST be
    /// included within the same Transaction as the system program's
    /// `CreateAccount` instruction that creates the account being initialized.
    /// Otherwise another party can acquire ownership of the uninitialized
    /// account.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]`  The account to initialize.
    ///   1. `[]` The mint this account will be associated with.
    ///   2. `[]` The new account's owner/multisignature.
    ///   3. `[]` Rent sysvar
    InitializeAccount,
    /// Initializes a multisignature account with N provided signers.
    ///
    /// Multisignature accounts can used in place of any single owner/delegate
    /// accounts in any token instruction that require an owner/delegate to be
    /// present.  The variant field represents the number of signers (M)
    /// required to validate this multisignature account.
    ///
    /// The `InitializeMultisig` instruction requires no signers and MUST be
    /// included within the same Transaction as the system program's
    /// `CreateAccount` instruction that creates the account being initialized.
    /// Otherwise another party can acquire ownership of the uninitialized
    /// account.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` The multisignature account to initialize.
    ///   1. `[]` Rent sysvar
    ///   2. ..2+N. `[]` The signer accounts, must equal to N where 1 <= N <=
    ///      11.
    InitializeMultisig {
        /// The number of signers (M) required to validate this multisignature
        /// account.
        m: u8 ,
    },
    /// Transfers tokens from one account to another either directly or via a
    /// delegate.  If this account is associated with the native mint then equal
    /// amounts of SOL and Tokens will be transferred to the destination
    /// account.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single owner/delegate
    ///   0. `[writable]` The source account.
    ///   1. `[writable]` The destination account.
    ///   2. `[signer]` The source account's owner/delegate.
    ///
    ///   * Multisignature owner/delegate
    ///   0. `[writable]` The source account.
    ///   1. `[writable]` The destination account.
    ///   2. `[]` The source account's multisignature owner/delegate.
    ///   3. ..3+M `[signer]` M signer accounts.
    Transfer {
        /// The amount of tokens to transfer.
        amount: u64,
    },
    /// Approves a delegate.  A delegate is given the authority over tokens on
    /// behalf of the source account's owner.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single owner
    ///   0. `[writable]` The source account.
    ///   1. `[]` The delegate.
    ///   2. `[signer]` The source account owner.
    ///
    ///   * Multisignature owner
    ///   0. `[writable]` The source account.
    ///   1. `[]` The delegate.
    ///   2. `[]` The source account's multisignature owner.
    ///   3. ..3+M `[signer]` M signer accounts
    Approve {
        /// The amount of tokens the delegate is approved for.
        amount: u64,
    },
    /// Revokes the delegate's authority.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single owner
    ///   0. `[writable]` The source account.
    ///   1. `[signer]` The source account owner.
    ///
    ///   * Multisignature owner
    ///   0. `[writable]` The source account.
    ///   1. `[]` The source account's multisignature owner.
    ///   2. ..2+M `[signer]` M signer accounts
    Revoke,
    /// Sets a new authority of a mint or account.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single authority
    ///   0. `[writable]` The mint or account to change the authority of.
    ///   1. `[signer]` The current authority of the mint or account.
    ///
    ///   * Multisignature authority
    ///   0. `[writable]` The mint or account to change the authority of.
    ///   1. `[]` The mint's or account's current multisignature authority.
    ///   2. ..2+M `[signer]` M signer accounts
    SetAuthority {
        /// The type of authority to update.
        authority_type: AuthorityType,
        /// The new authority
        new_authority: COption<Pubkey>,
    },
    /// Mints new tokens to an account.  The native mint does not support
    /// minting.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single authority
    ///   0. `[writable]` The mint.
    ///   1. `[writable]` The account to mint tokens to.
    ///   2. `[signer]` The mint's minting authority.
    ///
    ///   * Multisignature authority
    ///   0. `[writable]` The mint.
    ///   1. `[writable]` The account to mint tokens to.
    ///   2. `[]` The mint's multisignature mint-tokens authority.
    ///   3. ..3+M `[signer]` M signer accounts.
    MintTo {
        /// The amount of new tokens to mint.
        amount: u64,
    },
    /// Burns tokens by removing them from an account.  `Burn` does not support
    /// accounts associated with the native mint, use `CloseAccount` instead.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single owner/delegate
    ///   0. `[writable]` The account to burn from.
    ///   1. `[writable]` The token mint.
    ///   2. `[signer]` The account's owner/delegate.
    ///
    ///   * Multisignature owner/delegate
    ///   0. `[writable]` The account to burn from.
    ///   1. `[writable]` The token mint.
    ///   2. `[]` The account's multisignature owner/delegate.
    ///   3. ..3+M `[signer]` M signer accounts.
    Burn {
        /// The amount of tokens to burn.
        amount: u64,
    },
    /// Close an account by transferring all its SOL to the destination account.
    /// Non-native accounts may only be closed if its token amount is zero.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single owner
    ///   0. `[writable]` The account to close.
    ///   1. `[writable]` The destination account.
    ///   2. `[signer]` The account's owner.
    ///
    ///   * Multisignature owner
    ///   0. `[writable]` The account to close.
    ///   1. `[writable]` The destination account.
    ///   2. `[]` The account's multisignature owner.
    ///   3. ..3+M `[signer]` M signer accounts.
    CloseAccount,
    /// Freeze an Initialized account using the Mint's freeze_authority (if
    /// set).
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single owner
    ///   0. `[writable]` The account to freeze.
    ///   1. `[]` The token mint.
    ///   2. `[signer]` The mint freeze authority.
    ///
    ///   * Multisignature owner
    ///   0. `[writable]` The account to freeze.
    ///   1. `[]` The token mint.
    ///   2. `[]` The mint's multisignature freeze authority.
    ///   3. ..3+M `[signer]` M signer accounts.
    FreezeAccount,
    /// Thaw a Frozen account using the Mint's freeze_authority (if set).
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single owner
    ///   0. `[writable]` The account to freeze.
    ///   1. `[]` The token mint.
    ///   2. `[signer]` The mint freeze authority.
    ///
    ///   * Multisignature owner
    ///   0. `[writable]` The account to freeze.
    ///   1. `[]` The token mint.
    ///   2. `[]` The mint's multisignature freeze authority.
    ///   3. ..3+M `[signer]` M signer accounts.
    ThawAccount,

    /// Transfers tokens from one account to another either directly or via a
    /// delegate.  If this account is associated with the native mint then equal
    /// amounts of SOL and Tokens will be transferred to the destination
    /// account.
    ///
    /// This instruction differs from Transfer in that the token mint and
    /// decimals value is checked by the caller.  This may be useful when
    /// creating transactions offline or within a hardware wallet.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single owner/delegate
    ///   0. `[writable]` The source account.
    ///   1. `[]` The token mint.
    ///   2. `[writable]` The destination account.
    ///   3. `[signer]` The source account's owner/delegate.
    ///
    ///   * Multisignature owner/delegate
    ///   0. `[writable]` The source account.
    ///   1. `[]` The token mint.
    ///   2. `[writable]` The destination account.
    ///   3. `[]` The source account's multisignature owner/delegate.
    ///   4. ..4+M `[signer]` M signer accounts.
    TransferChecked {
        /// The amount of tokens to transfer.
        amount: u64,
        /// Expected number of base 10 digits to the right of the decimal place.
        decimals: u8,
    },
    /// Approves a delegate.  A delegate is given the authority over tokens on
    /// behalf of the source account's owner.
    ///
    /// This instruction differs from Approve in that the token mint and
    /// decimals value is checked by the caller.  This may be useful when
    /// creating transactions offline or within a hardware wallet.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single owner
    ///   0. `[writable]` The source account.
    ///   1. `[]` The token mint.
    ///   2. `[]` The delegate.
    ///   3. `[signer]` The source account owner.
    ///
    ///   * Multisignature owner
    ///   0. `[writable]` The source account.
    ///   1. `[]` The token mint.
    ///   2. `[]` The delegate.
    ///   3. `[]` The source account's multisignature owner.
    ///   4. ..4+M `[signer]` M signer accounts
    ApproveChecked {
        /// The amount of tokens the delegate is approved for.
        amount: u64,
        /// Expected number of base 10 digits to the right of the decimal place.
        decimals: u8,
    },
    /// Mints new tokens to an account.  The native mint does not support
    /// minting.
    ///
    /// This instruction differs from MintTo in that the decimals value is
    /// checked by the caller.  This may be useful when creating transactions
    /// offline or within a hardware wallet.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single authority
    ///   0. `[writable]` The mint.
    ///   1. `[writable]` The account to mint tokens to.
    ///   2. `[signer]` The mint's minting authority.
    ///
    ///   * Multisignature authority
    ///   0. `[writable]` The mint.
    ///   1. `[writable]` The account to mint tokens to.
    ///   2. `[]` The mint's multisignature mint-tokens authority.
    ///   3. ..3+M `[signer]` M signer accounts.
    MintToChecked {
        /// The amount of new tokens to mint.
        amount: u64,
        /// Expected number of base 10 digits to the right of the decimal place.
        decimals: u8,
    },
    /// Burns tokens by removing them from an account.  `BurnChecked` does not
    /// support accounts associated with the native mint, use `CloseAccount`
    /// instead.
    ///
    /// This instruction differs from Burn in that the decimals value is checked
    /// by the caller. This may be useful when creating transactions offline or
    /// within a hardware wallet.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single owner/delegate
    ///   0. `[writable]` The account to burn from.
    ///   1. `[writable]` The token mint.
    ///   2. `[signer]` The account's owner/delegate.
    ///
    ///   * Multisignature owner/delegate
    ///   0. `[writable]` The account to burn from.
    ///   1. `[writable]` The token mint.
    ///   2. `[]` The account's multisignature owner/delegate.
    ///   3. ..3+M `[signer]` M signer accounts.
    BurnChecked {
        /// The amount of tokens to burn.
        amount: u64,
        /// Expected number of base 10 digits to the right of the decimal place.
        decimals: u8,
    },
    /// Like InitializeAccount, but the owner pubkey is passed via instruction data
    /// rather than the accounts list. This variant may be preferable when using
    /// Cross Program Invocation from an instruction that does not need the owner's
    /// `AccountInfo` otherwise.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]`  The account to initialize.
    ///   1. `[]` The mint this account will be associated with.
    ///   3. `[]` Rent sysvar
    InitializeAccount2 {
        /// The new account's owner/multisignature.
        owner: Pubkey,
    },

    /// 17
    /// Deposit a an amount to hedge token
    Deposit {
        /// amount to deposit
       amount: u64,
       /// volatility
       volatility: u64,
        /// nonce used to create valid program address
        nonce: u8 
    },

    // 18
    /// withdraw funds after conversion
    Withdraw {
        /// amount to withdraw
        amount: u64,
    },

    //19
    /// Initialize Portfolio 
    InitializePortfolio {
        ///the data of the new portfolio
        metaDataUrl : Vec<u8>,
        ///Hash of dataUrl to insure the immuability of data
        metaDataHash : u16,
        ///pourcentage of first asset
        amountAsset1: u8,
        ///period of first asset
        periodAsset1 : u8,
        ///pourcentage of second asset
        amountAsset2 : u8,
        ///period of second asset
        periodAsset2 : u8,
        ///pourcentage of third asset
        amountAsset3 : u8,
        ///period of third asset
        periodAsset3 : u8,
        ///pourcentage of 4 asset
        amountAsset4 : u8,
        ///period of 4 asset
        periodAsset4 : u8,
        ///pourcentage of 5 asset
        amountAsset5 : u8,
        ///period of 5 asset
        periodAsset5 : u8,
        ///pourcentage of 6 asset
        amountAsset6 : u8,
        ///period of 6 asset
        periodAsset6 : u8,
        ///pourcentage of 7 asset
        amountAsset7 : u8,
        ///period of 7 asset
        periodAsset7 : u8,
        ///pourcentage of 8 asset
        amountAsset8 : u8,
        ///period of 8 asset
        periodAsset8 : u8,
        ///pourcentage of 9 asset
        amountAsset9 : u8,
        ///period of 9 asset
        periodAsset9 : u8,
       // ///pourcentage of 10 asset
        // amountAsset10 : u8,
        // ///period of 10 asset
        // periodAsset10 : u32,
    },

    //20
    /// create Init User Portfolio 
    createInitUserPortfolio {
        /// amount delegated
        delegated_amount: u64,
        ///user's amount of first asset
        valueAsset1: u64,
        ///user's amount  of second asset
        valueAsset2 : u64,
        ///user's amount  of third asset
        valueAsset3 : u64,
        ///user's amount  of 4 asset
        valueAsset4 : u64,
        ///user's amount  of 5 asset
        valueAsset5 : u64,
        ///user's amount  of 6 asset
        valueAsset6 : u64,
        ///user's amount  of 7 asset
        valueAsset7 : u64,
        ///user's amount  of 8 asset
        valueAsset8 : u64,
        ///user's amount  of 9 asset
        valueAsset9 : u64,
       // ///user's amount  of 10 asset
        // valueAsset10 : u64,

    }

    
}
impl TokenInstruction {
    /// Unpacks a byte buffer into a [TokenInstruction](enum.TokenInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        use TokenError::InvalidInstruction;
        
        let (&tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        msg!("1 ,{}",&tag);
        Ok(match tag {
            0 => {
                let (&decimals, rest) = rest.split_first().ok_or(InvalidInstruction)?;
                let (mint_authority, rest) = Self::unpack_pubkey(rest)?;
                let (freeze_authority, _rest) = Self::unpack_pubkey_option(rest)?;
                let (mint_id_asset, _rest2) = Self::unpack_pubkey_option(_rest)?;
                let (pubkey_swap, _rest3) = Self::unpack_pubkey_option(_rest2)?;
                Self::InitializeMint {
                    decimals,
                    mint_authority,
                    freeze_authority,
                    mint_id_asset,
                    pubkey_swap
                }
            }
            1 => Self::InitializeAccount,
            2 => {
                let &m = rest.get(0).ok_or(InvalidInstruction)?;
                Self::InitializeMultisig { m }
            }
            3 | 4 | 7 | 8 | 18 => {
                let amount = rest
                    .get(..8)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
                match tag {
                    3 => Self::Transfer { amount },
                    4 => Self::Approve { amount },
                    7 => Self::MintTo { amount },
                    8 => Self::Burn { amount },
                    18 => Self::Withdraw {amount},
                    _ => unreachable!(),
                }
            }
            17 => {
                  
                let (amount, rest) = rest.split_at(8);
               
                let amount = amount
                    .try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
                let (volatility, rest) = rest.split_at(8);
                let volatility = volatility.try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
                
                    let (&nonce, _rest) = rest.split_first().ok_or(InvalidInstruction)?;

                Self::Deposit { amount, volatility, nonce }
            }
            5 => Self::Revoke,
            6 => {
                let (authority_type, rest) = rest
                    .split_first()
                    .ok_or_else(|| ProgramError::from(InvalidInstruction))
                    .and_then(|(&t, rest)| Ok((AuthorityType::from(t)?, rest)))?;
                let (new_authority, _rest) = Self::unpack_pubkey_option(rest)?;

                Self::SetAuthority {
                    authority_type,
                    new_authority,
                }
            }
            9 => Self::CloseAccount,
            10 => Self::FreezeAccount,
            11 => Self::ThawAccount,
            12 => {
                let (amount, rest) = rest.split_at(8);
                let amount = amount
                    .try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
                let (&decimals, _rest) = rest.split_first().ok_or(InvalidInstruction)?;

                Self::TransferChecked { amount, decimals }
            }
            13 => {
                let (amount, rest) = rest.split_at(8);
                let amount = amount
                    .try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
                let (&decimals, _rest) = rest.split_first().ok_or(InvalidInstruction)?;

                Self::ApproveChecked { amount, decimals }
            }
            14 => {
                let (amount, rest) = rest.split_at(8);
                let amount = amount
                    .try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
                let (&decimals, _rest) = rest.split_first().ok_or(InvalidInstruction)?;

                Self::MintToChecked { amount, decimals }
            }
            15 => {
                let (amount, rest) = rest.split_at(8);
                let amount = amount
                    .try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
                let (&decimals, _rest) = rest.split_first().ok_or(InvalidInstruction)?;

                Self::BurnChecked { amount, decimals }
            }
            16 => {
                let (owner, _rest) = Self::unpack_pubkey(rest)?;
                Self::InitializeAccount2 { owner }
            }

            19 => {
                msg!("initial lecture {:?}",rest);
                let (metaDataUrl, rest) = rest.split_at(128);
                msg!("second error1 {:?}",rest);
                let metaDataUrl = metaDataUrl
                .try_into()
                .ok()
                .ok_or(InvalidInstruction)?;
                
                let (metaDataHash, rest) = rest.split_at(2);
                msg!("second error2 metadataHash {:?}", metaDataHash);
                msg!("second error2 rest {:?}", rest);
                let metaDataHash = metaDataHash
                .try_into()
                .ok()
                .map(u16::from_le_bytes)
                .ok_or(InvalidInstruction)?;
                
                let (amountAsset1, _rest) = rest.split_at(1);
                msg!("second error3 amountAsset1 {:?}", amountAsset1);
                msg!("second error3 rest {:?}", _rest);
                let amountAsset1 = amountAsset1
                .try_into()
                .ok()
                .map(u8::from_le_bytes)
                .ok_or(InvalidInstruction)?;
                
                let (periodAsset1, _rest2) = _rest.split_at(1);
                msg!("second error4 periodAsset1 {:?}", periodAsset1);
                msg!("second error4 rest {:?}", _rest2);
                let periodAsset1 = periodAsset1
                .try_into()
                .ok()
                .map(u8::from_le_bytes)
                .ok_or(InvalidInstruction)?;
                let (amountAsset2, _rest3) = _rest2.split_at(1);
                msg!("second error5 amountAsset2 {:?}", amountAsset2);
                msg!("second error5 rest {:?}", _rest3);
                let amountAsset2 = amountAsset2
                .try_into()
                .ok()
                .map(u8::from_le_bytes)
                .ok_or(InvalidInstruction)?;
                let (periodAsset2, _rest4) = _rest3.split_at(1);
                msg!("second error6");
                let periodAsset2 = periodAsset2
                .try_into()
                .ok()
                .map(u8::from_le_bytes)
                .ok_or(InvalidInstruction)?;
                let (amountAsset3, _rest5) = _rest4.split_at(1);
                msg!("second error7");
                let amountAsset3 = amountAsset3
                .try_into()
                .ok()
                .map(u8::from_le_bytes)
                .ok_or(InvalidInstruction)?;
                let (periodAsset3, _rest6) = _rest5.split_at(1);
                let periodAsset3 = periodAsset3
                .try_into()
                .ok()
                .map(u8::from_le_bytes)
                .ok_or(InvalidInstruction)?;
                let (amountAsset4, _rest7) = _rest6.split_at(1);
                let amountAsset4 = amountAsset4
                .try_into()
                .ok()
                .map(u8::from_le_bytes)
                .ok_or(InvalidInstruction)?;
                let (periodAsset4, _rest8) = _rest7.split_at(1);
                let periodAsset4 = periodAsset4
                .try_into()
                .ok()
                .map(u8::from_le_bytes)
                .ok_or(InvalidInstruction)?;
                let (amountAsset5, _rest9) = _rest8.split_at(1);
                let amountAsset5 = amountAsset5
                .try_into()
                .ok()
                .map(u8::from_le_bytes)
                .ok_or(InvalidInstruction)?;
                let (periodAsset5, _rest10) = _rest9.split_at(1);
                let periodAsset5 = periodAsset5
                .try_into()
                .ok()
                .map(u8::from_le_bytes)
                .ok_or(InvalidInstruction)?;
                let (amountAsset6, _rest11) = _rest10.split_at(1);
                let amountAsset6 = amountAsset6
                .try_into()
                .ok()
                .map(u8::from_le_bytes)
                .ok_or(InvalidInstruction)?;
                let (periodAsset6, _rest12) = _rest11.split_at(1);
                let periodAsset6 = periodAsset6
                .try_into()
                .ok()
                .map(u8::from_le_bytes)
                .ok_or(InvalidInstruction)?;
                let (amountAsset7, _rest13) = _rest12.split_at(1);
                let amountAsset7 = amountAsset7
                .try_into()
                .ok()
                .map(u8::from_le_bytes)
                .ok_or(InvalidInstruction)?;
                let (periodAsset7, _rest14) = _rest13.split_at(1);
                let periodAsset7 = periodAsset7
                .try_into()
                .ok()
                .map(u8::from_le_bytes)
                .ok_or(InvalidInstruction)?;
                let (amountAsset8, _rest15) = _rest14.split_at(1);
                let amountAsset8 = amountAsset8
                .try_into()
                .ok()
                .map(u8::from_le_bytes)
                .ok_or(InvalidInstruction)?;
                let (periodAsset8, _rest16) = _rest15.split_at(1);
                let periodAsset8 = periodAsset8
                .try_into()
                .ok()
                .map(u8::from_le_bytes)
                .ok_or(InvalidInstruction)?;
                let (amountAsset9, _rest17) = _rest16.split_at(1);
                msg!("second error777 {:?}", amountAsset9);
                msg!("second _rest17 {:?}", _rest17);
                let amountAsset9 = amountAsset9
                .try_into()
                .ok()
                .map(u8::from_le_bytes)
                .ok_or(InvalidInstruction)?;
                let (periodAsset9, _rest18) = _rest17.split_at(1);
                msg!("second error888 {:?}", periodAsset9);
                msg!("second _rest18 {:?}", _rest18);
                let periodAsset9 = periodAsset9
                .try_into()
                .ok()
                .map(u8::from_le_bytes)
                .ok_or(InvalidInstruction)?;
                // let (amountAsset10, _rest19) = _rest18.split_at(8);
                // let amountAsset10 = amountAsset10
                // .try_into()
                // .ok()
                // .map(u64::from_le_bytes)
                // .ok_or(InvalidInstruction)?;
                // let (periodAsset10, _rest20) = _rest19.split_at(32);
                // let periodAsset10 = periodAsset10
                // .try_into()
                // .ok()
                // .map(u64::from_le_bytes)
                // .ok_or(InvalidInstruction)?;
                Self::InitializePortfolio {
                    metaDataUrl,
                    metaDataHash,
                    amountAsset1,
                    periodAsset1,
                    amountAsset2,
                    periodAsset2,
                    amountAsset3,
                    periodAsset3,
                    amountAsset4,
                    periodAsset4,
                    amountAsset5,
                    periodAsset5,
                    amountAsset6,
                    periodAsset6,
                    amountAsset7,
                    periodAsset7,
                    amountAsset8,
                    periodAsset8,
                    amountAsset9,
                    periodAsset9,
                    // amountAsset10,
                    // periodAsset10,
                }
            }
            20 => {
                let (delegated_amount, _rest) = rest.split_at(8);
                msg!("delegated_amount : {:?}" , delegated_amount);
                let delegated_amount = delegated_amount
                    .try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
                let (valueAsset1, rest) = _rest.split_at(8);
                let valueAsset1 = valueAsset1
                    .try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
                let (valueAsset2, rest1) = rest.split_at(8);
                let valueAsset2 = valueAsset2
                    .try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
                let (valueAsset3, rest2) = rest1.split_at(8);
                let valueAsset3 = valueAsset3
                    .try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
                let (valueAsset4, rest3) = rest2.split_at(8);
                let valueAsset4 = valueAsset4
                    .try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
                let (valueAsset5, rest4) = rest3.split_at(8);
                let valueAsset5 = valueAsset5
                    .try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
                let (valueAsset6, rest5) = rest4.split_at(8);
                let valueAsset6 = valueAsset6
                    .try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
                let (valueAsset7, rest6) = rest5.split_at(8);
                let valueAsset7 = valueAsset7
                    .try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
                let (valueAsset8, rest7) = rest6.split_at(8);
                let valueAsset8 = valueAsset8
                    .try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
                let (valueAsset9, rest8) = rest7.split_at(8);
                let valueAsset9 = valueAsset9
                    .try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
                    msg!("valueAsset9 : {:?}" , valueAsset9);
                Self::createInitUserPortfolio { delegated_amount,valueAsset1, valueAsset2,valueAsset3,valueAsset4,valueAsset5,valueAsset6,valueAsset7,valueAsset8,valueAsset9 }
            }


            _ => return Err(TokenError::InvalidInstruction.into()),
        })
    }

    /// Packs a [TokenInstruction](enum.TokenInstruction.html) into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match self {
            &Self::InitializeMint {
                ref mint_authority,
                ref freeze_authority,
                decimals,
                ref mint_id_asset,
                ref pubkey_swap
            } => {
                buf.push(0);
                buf.push(decimals);
                buf.extend_from_slice(mint_authority.as_ref());
                Self::pack_pubkey_option(freeze_authority, &mut buf);
                Self::pack_pubkey_option(mint_id_asset, &mut buf);
                Self::pack_pubkey_option(pubkey_swap, &mut buf);
            }
            Self::InitializeAccount => buf.push(1),
            &Self::InitializeMultisig { m } => {
                buf.push(2);
                buf.push(m);
            }
            &Self::Transfer { amount } => {
                buf.push(3);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            &Self::Approve { amount } => {
                buf.push(4);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            &Self::MintTo { amount } => {
                buf.push(7);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            &Self::Burn { amount } => {
                buf.push(8);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::Revoke => buf.push(5),
            Self::SetAuthority {
                authority_type,
                ref new_authority,
            } => {
                buf.push(6);
                buf.push(authority_type.into());
                Self::pack_pubkey_option(new_authority, &mut buf);
            }
            Self::CloseAccount => buf.push(9),
            Self::FreezeAccount => buf.push(10),
            Self::ThawAccount => buf.push(11),
            &Self::TransferChecked { amount, decimals } => {
                buf.push(12);
                buf.extend_from_slice(&amount.to_le_bytes());
                buf.push(decimals);
            }
            &Self::ApproveChecked { amount, decimals } => {
                buf.push(13);
                buf.extend_from_slice(&amount.to_le_bytes());
                buf.push(decimals);
            }
            &Self::MintToChecked { amount, decimals } => {
                buf.push(14);
                buf.extend_from_slice(&amount.to_le_bytes());
                buf.push(decimals);
            }
            &Self::BurnChecked { amount, decimals } => {
                buf.push(15);
                buf.extend_from_slice(&amount.to_le_bytes());
                buf.push(decimals);
            }
            &Self::InitializeAccount2 { owner } => {
                buf.push(16);
                buf.extend_from_slice(owner.as_ref());
            }
            &Self::Deposit {amount , volatility, nonce} => {
                buf.push(17);
                buf.extend_from_slice(&amount.to_le_bytes());
                buf.extend_from_slice(&volatility.to_le_bytes());
                buf.push(nonce);
            },
            
            &Self::Withdraw {amount } => {
                buf.push(18);
                buf.extend_from_slice(&amount.to_le_bytes());
            },

            Self::InitializePortfolio {
                metaDataUrl,
                metaDataHash,
                amountAsset1,
                periodAsset1,
                amountAsset2,
                periodAsset2,
                amountAsset3,
                periodAsset3,
                amountAsset4,
                periodAsset4,
                amountAsset5,
                periodAsset5,
                amountAsset6,
                periodAsset6,
                amountAsset7,
                periodAsset7,
                amountAsset8,
                periodAsset8,
                amountAsset9,
                periodAsset9,
                // amountAsset10,
                // periodAsset10,
            } => {
                buf.push(19);
                buf.extend_from_slice(&metaDataUrl);
                buf.extend_from_slice(&metaDataHash.to_le_bytes());
                buf.extend_from_slice(&amountAsset1.to_le_bytes());
                buf.extend_from_slice(&periodAsset1.to_le_bytes());
                buf.extend_from_slice(&amountAsset2.to_le_bytes());
                buf.extend_from_slice(&periodAsset2.to_le_bytes());
                buf.extend_from_slice(&amountAsset3.to_le_bytes());
                buf.extend_from_slice(&periodAsset3.to_le_bytes());
                buf.extend_from_slice(&amountAsset4.to_le_bytes());
                buf.extend_from_slice(&periodAsset4.to_le_bytes());
                buf.extend_from_slice(&amountAsset5.to_le_bytes());
                buf.extend_from_slice(&periodAsset5.to_le_bytes());
                buf.extend_from_slice(&amountAsset6.to_le_bytes());
                buf.extend_from_slice(&periodAsset6.to_le_bytes());
                buf.extend_from_slice(&amountAsset7.to_le_bytes());
                buf.extend_from_slice(&periodAsset7.to_le_bytes());
                buf.extend_from_slice(&amountAsset8.to_le_bytes());
                buf.extend_from_slice(&periodAsset8.to_le_bytes());
                buf.extend_from_slice(&amountAsset9.to_le_bytes());
                buf.extend_from_slice(&periodAsset9.to_le_bytes());
                // buf.extend_from_slice(&amountAsset10.to_le_bytes());
                // buf.extend_from_slice(&periodAsset10.to_le_bytes());
               // buf.push(periodAsset10);
            },
            &Self::createInitUserPortfolio {delegated_amount ,valueAsset1 , valueAsset2, valueAsset3,valueAsset4,valueAsset5,valueAsset6,valueAsset7,valueAsset8,valueAsset9} => {
                buf.push(20);
                buf.extend_from_slice(&delegated_amount.to_le_bytes());
                buf.extend_from_slice(&valueAsset1.to_le_bytes());
                buf.extend_from_slice(&valueAsset2.to_le_bytes());
                buf.extend_from_slice(&valueAsset3.to_le_bytes());
                buf.extend_from_slice(&valueAsset4.to_le_bytes());
                buf.extend_from_slice(&valueAsset5.to_le_bytes());
                buf.extend_from_slice(&valueAsset6.to_le_bytes());
                buf.extend_from_slice(&valueAsset7.to_le_bytes());
                buf.extend_from_slice(&valueAsset8.to_le_bytes());
                buf.extend_from_slice(&valueAsset9.to_le_bytes());
            },

        };
        buf
    }

    fn unpack_pubkey(input: &[u8]) -> Result<(Pubkey, &[u8]), ProgramError> {
        if input.len() >= 32 {
            let (key, rest) = input.split_at(32);
            let pk = Pubkey::new(key);
            Ok((pk, rest))
        } else {
            Err(TokenError::InvalidInstruction.into())
        }
    }

    fn unpack_pubkey_option(input: &[u8]) -> Result<(COption<Pubkey>, &[u8]), ProgramError> {
        match input.split_first() {
            Option::Some((&0, rest)) => Ok((COption::None, rest)),
            Option::Some((&1, rest)) if rest.len() >= 32 => {
                let (key, rest) = rest.split_at(32);
                let pk = Pubkey::new(key);
                Ok((COption::Some(pk), rest))
            }
            _ => {
                Err(TokenError::InvalidInstruction.into()) 
            },
        }
    }

    fn pack_pubkey_option(value: &COption<Pubkey>, buf: &mut Vec<u8>) {
        match *value {
            COption::Some(ref key) => {
                buf.push(1);
                buf.extend_from_slice(&key.to_bytes());
            }
            COption::None => buf.push(0),
        }
    }
}

/// Specifies the authority type for SetAuthority instructions
#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum AuthorityType {
    /// Authority to mint new tokens
    MintTokens,
    /// Authority to freeze any account associated with the Mint
    FreezeAccount,
    /// Owner of a given token account
    AccountOwner,
    /// Authority to close a token account
    CloseAccount,
}

impl AuthorityType {
    fn into(&self) -> u8 {
        match self {
            AuthorityType::MintTokens => 0,
            AuthorityType::FreezeAccount => 1,
            AuthorityType::AccountOwner => 2,
            AuthorityType::CloseAccount => 3,
        }
    }

    fn from(index: u8) -> Result<Self, ProgramError> {
        match index {
            0 => Ok(AuthorityType::MintTokens),
            1 => Ok(AuthorityType::FreezeAccount),
            2 => Ok(AuthorityType::AccountOwner),
            3 => Ok(AuthorityType::CloseAccount),
            _ => Err(TokenError::InvalidInstruction.into()),
        }
    }
}


/// Creates a `Deposit` instruction.
pub fn deposit(
    program_id: &Pubkey,
    swap_info: &Pubkey,
    owner_key: &Pubkey,
    account_key: &Pubkey,
    source_info: &Pubkey,
    swap_source_info: &Pubkey,
    swap_destination_info: &Pubkey,
    destination_info: &Pubkey,
    pool_mint_info: &Pubkey,
    pool_fee_account_info: &Pubkey,
    token_program_info: &Pubkey,
    host_fee_account: &Pubkey,
    prog_address: &Pubkey,
    pubkey_swap: &Pubkey,
    amount: u64,
    volatility: u64,
    nonce: u8,

) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::Deposit {
        amount,
        volatility,
        nonce,
     }.pack();


    let  accounts = vec![
    AccountMeta::new(*swap_info, false),
    AccountMeta::new(*owner_key, true),
    AccountMeta::new(*account_key, false),
    AccountMeta::new(*source_info, false),
    AccountMeta::new(*swap_source_info, false),
    AccountMeta::new(*swap_destination_info, false),
    AccountMeta::new(*destination_info, false),
    AccountMeta::new(*pool_mint_info, false),
    AccountMeta::new(*pool_fee_account_info, false),
    AccountMeta::new(*token_program_info, false),
    AccountMeta::new(*host_fee_account, false),
    AccountMeta::new(*prog_address, false),
    AccountMeta::new(*pubkey_swap, false),
    

       ];
  
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}



/// Creates a `Withdraw` instruction.
pub fn withdraw(
    program_id: &Pubkey,
    account: &Pubkey,
    owner: &Pubkey,
    amount: u64,
   

) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::Withdraw {
        amount,
     }.pack();


    let  accounts = vec![
    AccountMeta::new(*account, false),
    AccountMeta::new(*owner, true),
       ];
  
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}


fn convert<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

/// Creates a `InitializePortfolio` instruction.
pub fn initialize_portfolio(
    program_id: &Pubkey,
    creatorAccount: &Pubkey ,
    owner: &Pubkey ,
    metaDataUrl : &Vec<u8>,
    metaDataHash : &u16,
    amountAsset1 : &u8,
    addressAsset1: &Pubkey ,
    periodAsset1 : &u8,
    assetToSoldIntoAsset1: &Pubkey ,
    amountAsset2 : &u8,
    addressAsset2: &Pubkey ,
    periodAsset2 : &u8,
    assetToSoldIntoAsset2: &Pubkey ,
    amountAsset3 : &u8,
    addressAsset3: &Pubkey ,
    periodAsset3 : &u8,
    assetToSoldIntoAsset3: &Pubkey ,
    amountAsset4 : &u8,
    addressAsset4: &Pubkey ,
    periodAsset4 : &u8,
    assetToSoldIntoAsset4: &Pubkey ,
    amountAsset5 : &u8,
    addressAsset5: &Pubkey ,
    periodAsset5 : &u8,
    assetToSoldIntoAsset5: &Pubkey ,
    amountAsset6 : &u8,
    addressAsset6: &Pubkey ,
    periodAsset6 : &u8,
    assetToSoldIntoAsset6: &Pubkey ,
    amountAsset7 : &u8,
    addressAsset7: &Pubkey ,
    periodAsset7 : &u8,
    assetToSoldIntoAsset7: &Pubkey ,
    amountAsset8 : &u8,
    addressAsset8: &Pubkey ,
    periodAsset8 : &u8,
    assetToSoldIntoAsset8: &Pubkey ,
    amountAsset9 : &u8,
    addressAsset9: &Pubkey ,
    periodAsset9 : &u8,
    assetToSoldIntoAsset9: &Pubkey ,
    // addressAsset10: &Pubkey ,
    // assetToSoldIntoAsset10: &Pubkey ,
  
 
    
    
   


    // amountAsset10 : &u8,
    // periodAsset10 : &u32,

) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::InitializePortfolio {
        metaDataUrl: metaDataUrl.clone(),
        metaDataHash: *metaDataHash,
        amountAsset1: *amountAsset1,
        periodAsset1: *periodAsset1,
        amountAsset2: *amountAsset2,
        periodAsset2: *periodAsset2,
        amountAsset3: *amountAsset3,
        periodAsset3: *periodAsset3,
        amountAsset4: *amountAsset4,
        periodAsset4: *periodAsset4,
        amountAsset5: *amountAsset5,
        periodAsset5: *periodAsset5,
        amountAsset6: *amountAsset6,
        periodAsset6: *periodAsset6,
        amountAsset7: *amountAsset7,
        periodAsset7: *periodAsset7,
        amountAsset8: *amountAsset8,
        periodAsset8: *periodAsset8,
        amountAsset9: *amountAsset9,
        periodAsset9: *periodAsset9,
        // amountAsset10: *amountAsset10,
        // periodAsset10: *periodAsset10
     }.pack();


    let  accounts = vec![
        AccountMeta::new(*creatorAccount, true),
        AccountMeta::new(*addressAsset1, false),
        AccountMeta::new(*assetToSoldIntoAsset1, false),
        AccountMeta::new(*addressAsset2, false),
        AccountMeta::new(*assetToSoldIntoAsset2, false),
        AccountMeta::new(*addressAsset3, false),
        AccountMeta::new(*assetToSoldIntoAsset3, false),
        AccountMeta::new(*addressAsset4, false),
        AccountMeta::new(*assetToSoldIntoAsset4, false),
        AccountMeta::new(*addressAsset5, false),
        AccountMeta::new(*assetToSoldIntoAsset5, false),
        AccountMeta::new(*addressAsset6, false),
        AccountMeta::new(*assetToSoldIntoAsset6, false),
        AccountMeta::new(*addressAsset7, false),
        AccountMeta::new(*assetToSoldIntoAsset7, false),
        AccountMeta::new(*addressAsset8, false),
        AccountMeta::new(*assetToSoldIntoAsset8, false),
        AccountMeta::new(*addressAsset9, false),
        AccountMeta::new(*assetToSoldIntoAsset9, false),
        AccountMeta::new(*owner, true),
        // AccountMeta::new(*addressAsset10, false),
        // AccountMeta::new(*assetToSoldIntoAsset10, false),
     
       ];
  
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}
/// Creates a `createInitUserPortfolio` instruction.
pub fn create_Init_User_Portfolio

(
    program_id: &Pubkey,
    userPortfolioAccount: &Pubkey ,
    portfolioAddress: &Pubkey ,
    owner: &Pubkey ,
    delegate: &Pubkey ,
    addressAsset1: &Pubkey ,
    addressAsset2: &Pubkey ,
    addressAsset3: &Pubkey ,
    addressAsset4: &Pubkey ,
    addressAsset5: &Pubkey ,
    addressAsset6: &Pubkey ,
    addressAsset7: &Pubkey ,
    addressAsset8: &Pubkey ,
    addressAsset9: &Pubkey ,
    // addressAsset10: &Pubkey ,
    delegated_amount: &u64,
    valueAsset1 : &u64,
    valueAsset2 : &u64,
    valueAsset3 : &u64,
    valueAsset4 : &u64,
    valueAsset5 : &u64,
    valueAsset6 : &u64,
    valueAsset7 : &u64,
    valueAsset8 : &u64,
    valueAsset9 : &u64,
    // valueAsset10 : &u64,

) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::createInitUserPortfolio {
        delegated_amount:*delegated_amount,
        valueAsset1: *valueAsset1,
        valueAsset2: *valueAsset2,
        valueAsset3: *valueAsset3,
        valueAsset4: *valueAsset4,
        valueAsset5: *valueAsset5,
        valueAsset6: *valueAsset6,
        valueAsset7: *valueAsset7,
        valueAsset8: *valueAsset8,
        valueAsset9: *valueAsset9,
        // valueAsset10: *valueAsset10,
     }.pack();


    let  accounts = vec![
        AccountMeta::new(*userPortfolioAccount, false),
        AccountMeta::new(*portfolioAddress, false),
        AccountMeta::new(*owner, true),
        AccountMeta::new(*delegate, false),
        AccountMeta::new(*addressAsset1, false),
        AccountMeta::new(*addressAsset2, false),
        AccountMeta::new(*addressAsset3, false),
        AccountMeta::new(*addressAsset4, false),
        AccountMeta::new(*addressAsset5, false),
        AccountMeta::new(*addressAsset6, false),
        AccountMeta::new(*addressAsset7, false),
        AccountMeta::new(*addressAsset8, false),
        AccountMeta::new(*addressAsset9, false),
        // AccountMeta::new(*addressAsset10, false),
       ];
  
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}



/// Creates a `InitializeMint` instruction.
pub fn initialize_mint(
    token_program_id: &Pubkey,
    mint_pubkey: &Pubkey,
    decimals: u8,
    mint_authority_pubkey: &Pubkey,
    freeze_authority_pubkey: Option<&Pubkey>,
    cmint_id_asset: Option<&Pubkey>,
    cpubkey_swap: Option<&Pubkey>
) -> Result<Instruction, ProgramError> {
    let freeze_authority = freeze_authority_pubkey.cloned().into();
    let mint_id_asset = cmint_id_asset.cloned().into();
    let pubkey_swap = cpubkey_swap.cloned().into();
    let data = TokenInstruction::InitializeMint {
        mint_authority: *mint_authority_pubkey,
        freeze_authority,
        decimals,
        mint_id_asset,
        pubkey_swap
    }
    .pack();

    let accounts = vec![
        AccountMeta::new(*mint_pubkey, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}
 
/// Creates a `InitializeAccount` instruction.
pub fn initialize_account(
    token_program_id: &Pubkey,
    account_pubkey: &Pubkey,
    mint_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::InitializeAccount.pack();

    let accounts = vec![
        AccountMeta::new(*account_pubkey, false),
        AccountMeta::new_readonly(*mint_pubkey, false),
        AccountMeta::new_readonly(*owner_pubkey, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates a `InitializeAccount2` instruction.
pub fn initialize_account2(
    token_program_id: &Pubkey,
    account_pubkey: &Pubkey,
    mint_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::InitializeAccount2 {
        owner: *owner_pubkey,
    }
    .pack();

    let accounts = vec![
        AccountMeta::new(*account_pubkey, false),
        AccountMeta::new_readonly(*mint_pubkey, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates a `InitializeMultisig` instruction.
pub fn initialize_multisig(
    token_program_id: &Pubkey,
    multisig_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
    m: u8,
) -> Result<Instruction, ProgramError> {
    if !is_valid_signer_index(m as usize)
        || !is_valid_signer_index(signer_pubkeys.len())
        || m as usize > signer_pubkeys.len()
    {
        return Err(ProgramError::MissingRequiredSignature);
    }
    let data = TokenInstruction::InitializeMultisig { m }.pack();

    let mut accounts = Vec::with_capacity(1 + 1 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*multisig_pubkey, false));
    accounts.push(AccountMeta::new_readonly(sysvar::rent::id(), false));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new_readonly(**signer_pubkey, false));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates a `Transfer` instruction.
pub fn transfer(
    token_program_id: &Pubkey,
    source_pubkey: &Pubkey,
    destination_pubkey: &Pubkey,
    authority_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::Transfer { amount }.pack();

    let mut accounts = Vec::with_capacity(3 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*source_pubkey, false));
    accounts.push(AccountMeta::new(*destination_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *authority_pubkey,
        signer_pubkeys.is_empty(),
    ));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new_readonly(**signer_pubkey, true));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates an `Approve` instruction.
pub fn approve(
    token_program_id: &Pubkey,
    source_pubkey: &Pubkey,
    delegate_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::Approve { amount }.pack();

    let mut accounts = Vec::with_capacity(3 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*source_pubkey, false));
    accounts.push(AccountMeta::new_readonly(*delegate_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *owner_pubkey,
        signer_pubkeys.is_empty(),
    ));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new_readonly(**signer_pubkey, true));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates a `Revoke` instruction.
pub fn revoke(
    token_program_id: &Pubkey,
    source_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::Revoke.pack();

    let mut accounts = Vec::with_capacity(2 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*source_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *owner_pubkey,
        signer_pubkeys.is_empty(),
    ));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new_readonly(**signer_pubkey, true));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates a `SetAuthority` instruction.
pub fn set_authority(
    token_program_id: &Pubkey,
    owned_pubkey: &Pubkey,
    new_authority_pubkey: Option<&Pubkey>,
    authority_type: AuthorityType,
    owner_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
) -> Result<Instruction, ProgramError> {
    let new_authority = new_authority_pubkey.cloned().into();
    let data = TokenInstruction::SetAuthority {
        authority_type,
        new_authority,
    }
    .pack();

    let mut accounts = Vec::with_capacity(3 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*owned_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *owner_pubkey,
        signer_pubkeys.is_empty(),
    ));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new_readonly(**signer_pubkey, true));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates a `MintTo` instruction.
pub fn mint_to(
    token_program_id: &Pubkey,
    mint_pubkey: &Pubkey,
    account_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::MintTo { amount }.pack();

    let mut accounts = Vec::with_capacity(3 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*mint_pubkey, false));
    accounts.push(AccountMeta::new(*account_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *owner_pubkey,
        signer_pubkeys.is_empty(),
    ));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new_readonly(**signer_pubkey, true));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates a `Burn` instruction.
pub fn burn(
    token_program_id: &Pubkey,
    account_pubkey: &Pubkey,
    mint_pubkey: &Pubkey,
    authority_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::Burn { amount }.pack();

    let mut accounts = Vec::with_capacity(3 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*account_pubkey, false));
    accounts.push(AccountMeta::new(*mint_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *authority_pubkey,
        signer_pubkeys.is_empty(),
    ));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new_readonly(**signer_pubkey, true));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates a `CloseAccount` instruction.
pub fn close_account(
    token_program_id: &Pubkey,
    account_pubkey: &Pubkey,
    destination_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::CloseAccount.pack();

    let mut accounts = Vec::with_capacity(3 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*account_pubkey, false));
    accounts.push(AccountMeta::new(*destination_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *owner_pubkey,
        signer_pubkeys.is_empty(),
    ));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new_readonly(**signer_pubkey, true));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates a `FreezeAccount` instruction.
pub fn freeze_account(
    token_program_id: &Pubkey,
    account_pubkey: &Pubkey,
    mint_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::FreezeAccount.pack();

    let mut accounts = Vec::with_capacity(3 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*account_pubkey, false));
    accounts.push(AccountMeta::new_readonly(*mint_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *owner_pubkey,
        signer_pubkeys.is_empty(),
    ));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new_readonly(**signer_pubkey, true));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates a `ThawAccount` instruction.
pub fn thaw_account(
    token_program_id: &Pubkey,
    account_pubkey: &Pubkey,
    mint_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::ThawAccount.pack();

    let mut accounts = Vec::with_capacity(3 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*account_pubkey, false));
    accounts.push(AccountMeta::new_readonly(*mint_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *owner_pubkey,
        signer_pubkeys.is_empty(),
    ));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new_readonly(**signer_pubkey, true));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates a `TransferChecked` instruction.
#[allow(clippy::too_many_arguments)]
pub fn transfer_checked(
    token_program_id: &Pubkey,
    source_pubkey: &Pubkey,
    mint_pubkey: &Pubkey,
    destination_pubkey: &Pubkey,
    authority_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
    amount: u64,
    decimals: u8,
) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::TransferChecked { amount, decimals }.pack();

    let mut accounts = Vec::with_capacity(4 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*source_pubkey, false));
    accounts.push(AccountMeta::new_readonly(*mint_pubkey, false));
    accounts.push(AccountMeta::new(*destination_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *authority_pubkey,
        signer_pubkeys.is_empty(),
    ));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new_readonly(**signer_pubkey, true));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates an `ApproveChecked` instruction.
#[allow(clippy::too_many_arguments)]
pub fn approve_checked(
    token_program_id: &Pubkey,
    source_pubkey: &Pubkey,
    mint_pubkey: &Pubkey,
    delegate_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
    amount: u64,
    decimals: u8,
) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::ApproveChecked { amount, decimals }.pack();

    let mut accounts = Vec::with_capacity(4 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*source_pubkey, false));
    accounts.push(AccountMeta::new_readonly(*mint_pubkey, false));
    accounts.push(AccountMeta::new_readonly(*delegate_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *owner_pubkey,
        signer_pubkeys.is_empty(),
    ));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new_readonly(**signer_pubkey, true));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates a `MintToChecked` instruction.
pub fn mint_to_checked(
    token_program_id: &Pubkey,
    mint_pubkey: &Pubkey,
    account_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
    amount: u64,
    decimals: u8,
) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::MintToChecked { amount, decimals }.pack();

    let mut accounts = Vec::with_capacity(3 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*mint_pubkey, false));
    accounts.push(AccountMeta::new(*account_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *owner_pubkey,
        signer_pubkeys.is_empty(),
    ));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new_readonly(**signer_pubkey, true));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Creates a `BurnChecked` instruction.
pub fn burn_checked(
    token_program_id: &Pubkey,
    account_pubkey: &Pubkey,
    mint_pubkey: &Pubkey,
    authority_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
    amount: u64,
    decimals: u8,
) -> Result<Instruction, ProgramError> {
    let data = TokenInstruction::BurnChecked { amount, decimals }.pack();

    let mut accounts = Vec::with_capacity(3 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*account_pubkey, false));
    accounts.push(AccountMeta::new(*mint_pubkey, false));
    accounts.push(AccountMeta::new_readonly(
        *authority_pubkey,
        signer_pubkeys.is_empty(),
    ));
    for signer_pubkey in signer_pubkeys.iter() {
        accounts.push(AccountMeta::new_readonly(**signer_pubkey, true));
    }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

/// Utility function that checks index is between MIN_SIGNERS and MAX_SIGNERS
pub fn is_valid_signer_index(index: usize) -> bool {
    (MIN_SIGNERS..=MAX_SIGNERS).contains(&index)
}


    #[test]
    fn test_instruction_packing() {
        let check = TokenInstruction::InitializeMint {
            decimals: 2,
            mint_authority: Pubkey::new(&[1u8; 32]),
            freeze_authority: COption::None,
            mint_id_asset:COption::None,
            pubkey_swap:COption::None
        };
        let packed = check.pack();
        let mut expect = Vec::from([0u8, 2]);
        expect.extend_from_slice(&[1u8; 32]);
        expect.extend_from_slice(&[0]);
        expect.extend_from_slice(&[0]);
        expect.extend_from_slice(&[0]);
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);
 

        let check = TokenInstruction::Revoke;
        let packed = check.pack();
        let expect = Vec::from([5u8]);
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = TokenInstruction::SetAuthority {
            authority_type: AuthorityType::FreezeAccount,
            new_authority: COption::Some(Pubkey::new(&[4u8; 32])),
        };
        let packed = check.pack();
        let mut expect = Vec::from([6u8, 1]);
        expect.extend_from_slice(&[1]);
        expect.extend_from_slice(&[4u8; 32]);
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = TokenInstruction::MintTo { amount: 1 };
        let packed = check.pack();
        let expect = Vec::from([7u8, 1, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = TokenInstruction::Burn { amount: 1 };
        let packed = check.pack();
        let expect = Vec::from([8u8, 1, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = TokenInstruction::CloseAccount;
        let packed = check.pack();
        let expect = Vec::from([9u8]);
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = TokenInstruction::FreezeAccount;
        let packed = check.pack();
        let expect = Vec::from([10u8]);
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = TokenInstruction::ThawAccount;
        let packed = check.pack();
        let expect = Vec::from([11u8]);
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = TokenInstruction::TransferChecked {
            amount: 1,
            decimals: 2,
        };
        let packed = check.pack();
        let expect = Vec::from([12u8, 1, 0, 0, 0, 0, 0, 0, 0, 2]);
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = TokenInstruction::ApproveChecked {
            amount: 1,
            decimals: 2,
        };
        let packed = check.pack();
        let expect = Vec::from([13u8, 1, 0, 0, 0, 0, 0, 0, 0, 2]);
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = TokenInstruction::MintToChecked {
            amount: 1,
            decimals: 2,
        };
        let packed = check.pack();
        let expect = Vec::from([14u8, 1, 0, 0, 0, 0, 0, 0, 0, 2]);
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = TokenInstruction::BurnChecked {
            amount: 1,
            decimals: 2,
        };
        let packed = check.pack();
        let expect = Vec::from([15u8, 1, 0, 0, 0, 0, 0, 0, 0, 2]);
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = TokenInstruction::InitializeAccount2 {
            owner: Pubkey::new(&[2u8; 32]),
        };
        let packed = check.pack();
        let mut expect = vec![16u8];
        expect.extend_from_slice(&[2u8; 32]);
        assert_eq!(packed, expect);
        let unpacked = TokenInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);
    }

