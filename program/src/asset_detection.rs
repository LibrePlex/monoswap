use nifty_asset_types::state::{Asset, Standard as NiftyStandard};
use solana_program::{
    account_info::AccountInfo, msg, program_error::ProgramError, program_pack::Pack,
};
use spl_token_2022::state::Mint;

use crate::{
    processor::MonoswapError,
    state::{AssetType, LEGACY_TOKEN_PROGRAM_ID},
};

pub fn detect_asset(asset_info: &AccountInfo) -> Result<AssetType, ProgramError> {
    // Check program owner.
    let asset_type = match *asset_info.owner {
        // Nifty Program
        nifty_asset::ID => {
            let data = asset_info.try_borrow_data().unwrap();

            // Must have the expected amount of data and the correct discriminator and standard.
            if data.len() >= Asset::LEN && data[2] == NiftyStandard::NonFungible as u8 {
                AssetType::NiftyAsset
            }
            // Invalid account.
            else {
                AssetType::Invalid
            }
        }
        // A Token Program
        LEGACY_TOKEN_PROGRAM_ID | spl_token_2022::ID => {
            let data = asset_info.try_borrow_data().unwrap();
            let mint = Mint::unpack(&data[0..Mint::LEN])?;

            if !mint.is_initialized {
                return Err(MonoswapError::IninitalizedMint.into());
            }

            AssetType::SplToken
        }
        // Unknown program, so unsupported asset type.
        _ => {
            msg!("Unknown program");
            AssetType::Invalid
        }
    };

    Ok(asset_type)
}
