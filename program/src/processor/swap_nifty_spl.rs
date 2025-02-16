use super::*;

pub fn process_swap_nifty_spl<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    let ctx = SwapNiftySPLAccounts::context(accounts)?;

    let mut swap_marker = SwapMarker::load(ctx.accounts.swap_marker)?;

    let escrowed_asset_info = ctx.accounts.escrowed_asset.clone();
    let incoming_asset_info = ctx.accounts.incoming_asset.clone();

    // Check signer.
    assert_signer("payer", ctx.accounts.payer)?;
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

    // Detect the incoming asset type to determine which way we are swapping.
    let incoming_asset_type = detect_asset(ctx.accounts.incoming_asset)?;

    match incoming_asset_type {
        AssetType::NiftyAsset => {
            msg!("Incoming Nifty asset detected");

            // Transfer Nifty asset from authority signer to the swap marker.
            let transfer_params = TransferNiftyParams {
                nifty_program_info: ctx.accounts.incoming_asset_program,
                asset_info: ctx.accounts.incoming_asset,
                signer_info: ctx.accounts.authority,
                recipient_info: ctx.accounts.swap_marker,
                group_asset_opt_info: ctx.accounts.nifty_asset_group,
                signer_seeds: &[],
            };

            msg!("Transferring Nifty asset into escrow.");
            check_and_transfer_nifty(transfer_params)?;

            // Transfer escrowed fungible tokens from the swap marker ata to the authority signer ata.
            let transfer_params = TransferSplParams {
                spl_program_info: ctx.accounts.escrowed_asset_program,
                payer_info: ctx.accounts.payer,
                mint_info: ctx.accounts.escrowed_asset,
                source_owner_info: ctx.accounts.swap_marker,
                destination_owner_info: ctx.accounts.authority,
                source_ata_info: ctx.accounts.swap_marker_ata,
                destination_ata_info: ctx.accounts.authority_ata,
                amount: swap_marker.escrowed_amount,
                signer_seeds,
            };

            msg!("Transferring SPL fungibles out of escrow.");
            check_and_transfer_spl(transfer_params)?;
        }
        AssetType::SplToken => {
            msg!("Incoming SPL token asset detected");

            // Transfer fungible tokens from authority signer to the swap marker ata.
            let transfer_params = TransferSplParams {
                spl_program_info: ctx.accounts.incoming_asset_program,
                payer_info: ctx.accounts.payer,
                mint_info: ctx.accounts.incoming_asset,
                source_owner_info: ctx.accounts.authority,
                destination_owner_info: ctx.accounts.swap_marker,
                source_ata_info: ctx.accounts.authority_ata,
                destination_ata_info: ctx.accounts.swap_marker_ata,
                amount: swap_marker.external_amount,
                signer_seeds: &[],
            };

            msg!("Transferring SPL fungibles into escrow.");
            check_and_transfer_spl(transfer_params)?;

            // Transfer escrowed Nifty asset from the swap marker to the authority signer.
            let transfer_params = TransferNiftyParams {
                nifty_program_info: ctx.accounts.escrowed_asset_program,
                asset_info: ctx.accounts.escrowed_asset,
                signer_info: ctx.accounts.swap_marker,
                recipient_info: ctx.accounts.authority,
                group_asset_opt_info: ctx.accounts.nifty_asset_group,
                signer_seeds,
            };

            msg!("Transferring Nifty asset out of escrow.");
            check_and_transfer_nifty(transfer_params)?;
        }
        _ => {
            return Err(MonoswapError::UnsupportedAssetType.into());
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
