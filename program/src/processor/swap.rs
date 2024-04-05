use super::*;

pub fn process_swap<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    let ctx = SwapAccounts::context(accounts)?;

    let swap_marker = SwapMarker::load(ctx.accounts.swap_marker)?;

    // Check signer.
    assert_signer("authority", ctx.accounts.authority)?;

    // Check the swap marker account is derived from the correct seeds and owned by this program.
    let swap_seeds = SwapSeeds {
        namespace: ctx.accounts.authority.key,
        asset1: ctx.accounts.escrowed_asset.key,
        asset2: ctx.accounts.incoming_asset.key,
    };
    let (swap_marker_pubkey, bump) = SwapMarker::find_pda(swap_seeds.clone());
    assert_same_pubkeys("swap_marker", ctx.accounts.swap_marker, &swap_marker_pubkey)?;

    // Detect various asset types to determine what kind of transfers need to happen.
    let escrowed_asset_type = detect_asset(ctx.accounts.escrowed_asset)?;
    let incoming_asset_type = detect_asset(ctx.accounts.incoming_asset)?;

    let binding = bump.to_le_bytes();
    let signer_seeds: &[&[u8]] = &SwapMarker::find_signer_seeds(swap_seeds, &binding);

    match incoming_asset_type {
        AssetType::Nifty => {
            let asset = NiftyAsset::from_bytes(&ctx.accounts.incoming_asset.try_borrow_data()?)?;

            // If a group is present on the asset, the group asset account must be the aux account.
            let group_asset = if let Some(group) = asset.group.to_option() {
                if ctx.accounts.incoming_asset_aux.is_none() {
                    msg!("Nifty group asset is missing");
                    return Err(MonoswapError::MissingNiftyGroup.into());
                }

                let group_asset = ctx.accounts.incoming_asset_aux.unwrap();

                assert_same_pubkeys("nifty group asset", group_asset, &group)?;

                Some(group_asset)
            } else {
                None
            };

            // Transfer Nifty asset from authority signer to the swap marker.
            NiftyTransferCpi {
                __program: ctx.accounts.incoming_asset_program,
                asset: ctx.accounts.incoming_asset,
                signer: ctx.accounts.authority,
                recipient: ctx.accounts.swap_marker,
                group_asset,
            }
            .invoke()?;
        }
        AssetType::SplToken => {
            let authority = ctx.accounts.authority.key;
            let source_mint = ctx.accounts.escrowed_asset.key;
            let source_ata = match ctx.accounts.escrowed_asset_aux {
                Some(account_info) => account_info.key,
                None => return Err(MonoswapError::MissingIncomingAssetAux.into()),
            };

            let source_mint_account =
                unpack::<Mint>(&ctx.accounts.escrowed_asset.try_borrow_data()?)?;
            let source_ata_account = unpack::<TokenAccount>(
                &ctx.accounts.escrowed_asset_aux.unwrap().try_borrow_data()?,
            )?;
            assert_same_pubkeys(
                "mint",
                ctx.accounts.escrowed_asset,
                &source_ata_account.mint,
            )?;

            let destination_ata = ctx.accounts.swap_marker_aux.key;

            let account_infos = &[
                ctx.accounts.escrowed_asset_program.clone(),
                ctx.accounts.authority.clone(),
                ctx.accounts.escrowed_asset.clone(),
                ctx.accounts.swap_marker_aux.clone(),
                ctx.accounts.escrowed_asset.clone(),
                ctx.accounts.escrowed_asset_aux.unwrap().clone(),
            ];

            // Transfer SPL token from authority signer to the swap marker ATA.
            invoke(
                &transfer_checked(
                    ctx.accounts.escrowed_asset_program.key,
                    source_ata,
                    source_mint,
                    destination_ata,
                    authority,
                    &[],
                    swap_marker.external_amount,
                    source_mint_account.decimals,
                )?,
                account_infos,
            )?;
        }
        _ => {
            // SPL token to Nifty swap.
        }
    }

    match escrowed_asset_type {
        AssetType::Nifty => {
            let asset = NiftyAsset::from_bytes(&ctx.accounts.escrowed_asset.try_borrow_data()?)?;

            // If a group is present on the asset, the group asset account must be the aux account.
            let group_asset = if let Some(group) = asset.group.to_option() {
                if ctx.accounts.escrowed_asset_aux.is_none() {
                    msg!("Nifty group asset is missing");
                    return Err(MonoswapError::MissingNiftyGroup.into());
                }

                let group_asset = ctx.accounts.escrowed_asset_aux.unwrap();

                assert_same_pubkeys("nifty group asset", group_asset, &group)?;

                Some(group_asset)
            } else {
                None
            };

            // Transfer Nifty asset from the swap marker to the authority signer.
            NiftyTransferCpi {
                __program: ctx.accounts.escrowed_asset_program,
                asset: ctx.accounts.escrowed_asset,
                signer: ctx.accounts.authority,
                recipient: ctx.accounts.swap_marker,
                group_asset,
            }
            .invoke_signed(&[signer_seeds])?;
        }
        AssetType::SplToken => {}
        _ => {
            // SPL token to Nifty swap.
        }
    }

    Ok(())
}
