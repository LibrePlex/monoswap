use mpl_token_metadata::accounts::MasterEdition;
use nifty_asset_types::state::{Asset, Standard as NiftyStandard};
use solana_program::{
    account_info::AccountInfo, msg, program_error::ProgramError, program_option::COption,
    program_pack::Pack,
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

            // Must have the expected amount of data and the correct standard.
            if data.len() >= Asset::LEN && data[0] == NiftyStandard::NonFungible as u8 {
                AssetType::Nifty
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

            // Check if it's a MPLX asset.
            let mplx_freeze_auth = MasterEdition::find_pda(asset_info.key).0;

            if let COption::Some(freeze_auth) = mint.freeze_authority {
                if freeze_auth == mplx_freeze_auth {
                    msg!("MPLX asset detected");
                    return Ok(AssetType::MplxLegacy);
                }
            }

            // Not a Metaplex legacy asset, so some type of SPL token.
            AssetType::SplToken
        }
        // Unknown program.
        _ => {
            msg!("Unknown program");
            AssetType::Invalid
        }
    };

    Ok(asset_type)
}
