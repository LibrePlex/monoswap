use crate::{assertions::assert_program_owner, transfer::transfer_nifty};

use super::*;

use nifty_asset_types::state::Asset;

pub fn process_swap_nifty<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    let ctx = SwapNiftyAccounts::context(accounts)?;

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
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"swap_marker",
        &swap_seeds.namespace.to_bytes(),
        &min(asset1_bytes, asset2_bytes),
        &max(asset1_bytes, asset2_bytes),
        &[bump],
    ]];

    // Ensure incoming asset is a Nifty asset.

    // Onwed by the Nifty asset program.
    assert_program_owner("incoming asset", &incoming_asset_info, &nifty_asset::ID)?;

    let data = incoming_asset_info.try_borrow_data().unwrap();

    // Must have the expected amount of data and the correct discriminator and standard.
    if data.len() < Asset::LEN || data[2] != NiftyStandard::NonFungible as u8 {
        return Err(MonoswapError::InvalidNiftyAsset.into());
    }

    // Transfer Nifty asset from authority signer to the swap marker.
    let transfer_params = TransferNiftyParams {
        nifty_program_info: ctx.accounts.nifty_asset_program,
        asset_info: ctx.accounts.incoming_asset,
        signer_info: ctx.accounts.authority,
        recipient_info: ctx.accounts.swap_marker,
        group_asset_opt_info: ctx.accounts.incoming_asset_group,
        signer_seeds: &[],
    };

    drop(data); // Ensure data is dropped before invoking the CPI.

    msg!("Transferring Nifty asset into escrow.");
    transfer_nifty(transfer_params)?;

    // Transfer escrowed Nifty asset from the swap marker to the authority signer.
    let transfer_params = TransferNiftyParams {
        nifty_program_info: ctx.accounts.nifty_asset_program,
        asset_info: ctx.accounts.escrowed_asset,
        signer_info: ctx.accounts.swap_marker,
        recipient_info: ctx.accounts.authority,
        group_asset_opt_info: ctx.accounts.escrowed_asset_group,
        signer_seeds,
    };

    msg!("Transferring Nifty asset out of escrow.");
    transfer_nifty(transfer_params)?;

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
