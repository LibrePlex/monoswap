mod create_swap;
mod swap;

pub use create_swap::*;
pub use swap::*;

use std::cmp::{max, min};

pub use crate::{
    assertions::{assert_same_pubkeys, assert_signer},
    asset_detection::detect_asset,
    state::{AssetType, SwapSeeds},
    transfer::{
        check_and_transfer_nifty, check_and_transfer_spl, TransferNiftyParams, TransferSplParams,
    },
    utils::unpack,
};
pub use borsh::BorshDeserialize;
pub use nifty_asset::{
    accounts::Asset as NiftyAsset, instructions::TransferCpi as NiftyTransferCpi,
};
pub use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    rent::Rent,
    system_instruction, system_program,
    sysvar::Sysvar,
};
pub use spl_token_2022::{
    instruction::transfer_checked,
    state::{Account as TokenAccount, Mint},
};

pub use crate::error::MonoswapError;
pub use crate::instruction::accounts::{CreateSwapAccounts, SwapAccounts};
pub use crate::instruction::{CreateSwapArgs, MonoswapInstruction};
pub use crate::state::SwapMarker;

#[macro_export]
macro_rules! require {
    ( $constraint:expr, $error:expr, $message:expr ) => {
        if !$constraint {
            solana_program::msg!("Constraint failed: {}", $message);
            return Err($error.into());
        }
    };
    ( $constraint:expr, $error:expr, $message:literal, $($args:tt)+ ) => {
        require!( $constraint, $error, format!($message, $($args)+) );
    };
}

pub fn process_instruction<'a>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction: MonoswapInstruction = MonoswapInstruction::try_from_slice(instruction_data)?;
    match instruction {
        MonoswapInstruction::CreateSwap(args) => {
            msg!("Instruction: Create Swap");
            process_create_swap(accounts, args)
        }
        MonoswapInstruction::Swap => {
            msg!("Instruction: Swap");
            process_swap(accounts)
        }
    }
}
