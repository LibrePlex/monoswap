use std::cmp::{max, min};

use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey,
    pubkey::Pubkey,
};

use crate::error::MonoswapError;

pub const LEGACY_TOKEN_PROGRAM_ID: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
pub const TOKEN_PROGRAM_IDS: [Pubkey; 2] = [spl_token_2022::ID, LEGACY_TOKEN_PROGRAM_ID];

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub enum Key {
    Uninitialized,
    SwapMarker,
}

/// Seeds: [b"swap_marker", namespace, smaller_asset, larger_asset, bump]
/// To achieve deterministic order of the seeds, the smaller asset always comes before the larger
/// asset, defined by comparing the first byte of the asset addresses.
#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
pub struct SwapMarker {
    pub namespace: Pubkey,
    pub escrowed_asset: Pubkey,
    pub external_asset: Pubkey,
    pub escrowed_amount: u64,
    pub external_amount: u64,
    pub bump: u8,
}

/// A helper struct to make it easier to compute the PDA for the swap marker account.
#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
pub struct SwapSeeds<'a> {
    pub namespace: &'a Pubkey,
    pub asset1: &'a Pubkey,
    pub asset2: &'a Pubkey,
}

impl SwapMarker {
    pub const LEN: usize = std::mem::size_of::<Pubkey>() * 3
        + std::mem::size_of::<u64>() * 2
        + std::mem::size_of::<u8>();

    pub fn load(account: &AccountInfo) -> Result<Self, ProgramError> {
        let mut bytes: &[u8] = &(*account.data).borrow();
        SwapMarker::deserialize(&mut bytes).map_err(|error| {
            msg!("Error: {}", error);
            MonoswapError::DeserializationError.into()
        })
    }

    pub fn save(&self, account: &AccountInfo) -> ProgramResult {
        borsh::to_writer(&mut account.data.borrow_mut()[..], self).map_err(|error| {
            msg!("Error: {}", error);
            MonoswapError::SerializationError.into()
        })
    }

    pub fn find_pda(seeds: SwapSeeds) -> (Pubkey, u8) {
        let SwapSeeds {
            namespace,
            asset1,
            asset2,
        } = seeds;

        // The account with the smaller first byte is the first seed, the allows the swap marker
        // to be reversible in that it has the same order regardless of which asset is incoming
        // or escrowed.

        Pubkey::find_program_address(
            &[
                b"swap_marker",
                &namespace.to_bytes(),
                min(asset1.to_bytes(), asset2.to_bytes()).as_ref(),
                max(asset1.to_bytes(), asset2.to_bytes()).as_ref(),
            ],
            &crate::ID,
        )
    }
}

#[repr(u8)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub enum AssetType {
    Invalid,
    Nifty,
    SplToken,
    MplxLegacy,
    MplxCore,
}
