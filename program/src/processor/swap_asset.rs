use crate::interface::{AssetInterface, NiftyAsset, NiftyAssetInputs};

use super::*;

pub fn process_swap_asset<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    let ctx = SwapAssetAccounts::context(accounts)?;

    let mut swap_marker = SwapMarker::load(ctx.accounts.swap_marker)?;

    let escrowed_asset_info = ctx.accounts.escrowed_asset.clone();
    let incoming_asset_info = ctx.accounts.incoming_asset.clone();

    // Check signer.
    assert_signer("authority", ctx.accounts.authority)?;

    let asset1_pub = escrowed_asset_info.key;
    let asset1_bytes = asset1_pub.to_bytes();
    let asset2_pub = incoming_asset_info.key;
    let asset2_bytes = asset2_pub.to_bytes();

    // Check the swap marker account is derived from the correct seeds and owned by this program.
    let swap_seeds = SwapSeeds {
        namespace: &swap_marker.namespace,
        asset1: asset1_pub,
        asset2: asset2_pub,
    };

    let (swap_marker_pubkey, bump) = SwapMarker::find_pda(swap_seeds.clone());
    assert_same_pubkeys("swap_marker", ctx.accounts.swap_marker, &swap_marker_pubkey)?;

    // Swap marker signer seeds.
    let signers_seeds: &[&[&[u8]]] = &[&[
        b"swap_marker",
        &swap_seeds.namespace.to_bytes(),
        &min(asset1_bytes, asset2_bytes),
        &max(asset1_bytes, asset2_bytes),
        &[bump],
    ]];

    let incoming_asset_type = detect_asset(&incoming_asset_info)?;
    let escrowed_asset_type = detect_asset(&escrowed_asset_info)?;

    match incoming_asset_type {
        AssetType::NiftyAsset => {
            msg!("Incoming Nifty asset detected");

            let incoming_asset = NiftyAsset::new(NiftyAssetInputs {
                asset: ctx.accounts.incoming_asset,
                transfer_authority: ctx.accounts.authority,
                group: ctx.accounts.incoming_asset_group,
                program: ctx.accounts.asset_program,
                signers_seeds: &[],
            })?;

            // Transfer Nifty asset from owner to the swap marker.
            incoming_asset.transfer(ctx.accounts.swap_marker)?;
        }
        _ => {
            msg!("Invalid incoming asset detected");
            return Err(MonoswapError::InvalidNiftyAsset.into());
        }
    }

    match escrowed_asset_type {
        AssetType::NiftyAsset => {
            msg!("Escrowed Nifty asset detected");

            let escrowed_asset = NiftyAsset::new(NiftyAssetInputs {
                asset: ctx.accounts.escrowed_asset,
                transfer_authority: ctx.accounts.swap_marker,
                group: ctx.accounts.escrowed_asset_group,
                program: ctx.accounts.asset_program,
                signers_seeds,
            })?;

            // Transfer Nifty asset from the swap marker to the owner.
            escrowed_asset.transfer(ctx.accounts.authority)?;
        }
        _ => {
            msg!("Invalid escrowed asset detected");
            return Err(MonoswapError::InvalidNiftyAsset.into());
        }
    }

    // Update SwapMarker state.
    // Accounts have swapped, so update the escrowed and external assets.
    // This allows indexing to figure out what swaps are available for any given asset.
    std::mem::swap(
        &mut swap_marker.escrowed_asset,
        &mut swap_marker.external_asset,
    );
    std::mem::swap(
        &mut swap_marker.escrowed_amount,
        &mut swap_marker.external_amount,
    );

    swap_marker.save(ctx.accounts.swap_marker)
}
