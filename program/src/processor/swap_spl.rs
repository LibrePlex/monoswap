use super::*;

pub fn process_swap_spl<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    let ctx = SwapSPLAccounts::context(accounts)?;

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

    // Transfer fungible tokens from authority signer to the swap marker ata.
    let transfer_params = TransferSplParams {
        spl_program_info: ctx.accounts.incoming_asset_program,
        payer_info: ctx.accounts.payer,
        mint_info: ctx.accounts.incoming_asset,
        source_owner_info: ctx.accounts.authority,
        destination_owner_info: ctx.accounts.swap_marker,
        source_ata_info: ctx.accounts.incoming_asset_ata,
        destination_ata_info: ctx.accounts.swap_marker_incoming_ata,
        amount: swap_marker.external_amount,
        signer_seeds: &[],
    };

    msg!("Transferring SPL fungibles into escrow.");
    check_and_transfer_spl(transfer_params)?;

    // Transfer escrowed fungible tokens from the swap marker ata to the authority signer ata.
    let transfer_params = TransferSplParams {
        spl_program_info: ctx.accounts.escrowed_asset_program,
        payer_info: ctx.accounts.payer,
        mint_info: ctx.accounts.escrowed_asset,
        source_owner_info: ctx.accounts.swap_marker,
        destination_owner_info: ctx.accounts.authority,
        source_ata_info: ctx.accounts.swap_marker_escrowed_ata,
        destination_ata_info: ctx.accounts.outgoing_asset_ata,
        amount: swap_marker.escrowed_amount,
        signer_seeds,
    };

    msg!("Transferring SPL fungibles out of escrow.");
    check_and_transfer_spl(transfer_params)?;

    msg!("swap marker: {:?}", swap_marker);

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
