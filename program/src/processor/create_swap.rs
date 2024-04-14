use super::*;

/// The `create_swap` function is responsible for creating a new swap marker account.
/// Incoming asset is the asset to be escrowed on the swap marker account.
/// External asset is the asset that is connected to the incoming asset and held by an external
/// account.
/// Auxiliary accounts are used with an extra account is required, such as a token account
/// or Nifty group account.
pub fn process_create_swap<'a>(
    accounts: &'a [AccountInfo<'a>],
    args: CreateSwapArgs,
) -> ProgramResult {
    // Accounts.
    let ctx = CreateSwapAccounts::context(accounts)?;
    let rent = Rent::get()?;

    // Guards.

    // Ensure the system program is the correct program.
    assert_same_pubkeys(
        "system_program",
        ctx.accounts.system_program,
        &system_program::ID,
    )?;

    // Check signers.
    assert_signer("payer", ctx.accounts.payer)?;
    assert_signer("namespace", ctx.accounts.namespace)?;
    assert_signer("authority", ctx.accounts.authority)?;

    let namespace_pub = *ctx.accounts.namespace.key;
    let asset1_pub = *ctx.accounts.incoming_asset.key;
    let asset2_pub = *ctx.accounts.external_asset.key;

    // Used to build the signer seeds.
    let asset1_bytes = asset1_pub.to_bytes();
    let asset2_bytes = asset2_pub.to_bytes();

    // Check the swap marker account is derived from the correct seeds and owned by this program.
    let swap_seeds = SwapSeeds {
        namespace: &namespace_pub,
        asset1: &asset1_pub,
        asset2: &asset2_pub,
    };

    let (swap_marker_pub, bump) = SwapMarker::find_pda(swap_seeds.clone());
    assert_same_pubkeys("swap_marker", ctx.accounts.swap_marker, &swap_marker_pub)?;

    // Detect the incoming asset type and perform the transfer to escrow it.
    let incoming_asset_type = detect_asset(ctx.accounts.incoming_asset)?;

    match incoming_asset_type {
        AssetType::Nifty => {
            msg!("Nifty asset detected");

            // Transfer Nifty asset from authority signer to the swap marker.
            let transfer_params = TransferNiftyParams {
                nifty_program_info: ctx.accounts.incoming_asset_program,
                asset_info: ctx.accounts.incoming_asset,
                signer_info: ctx.accounts.authority,
                recipient_info: ctx.accounts.swap_marker,
                group_asset_opt_info: ctx.accounts.incoming_asset_aux,
                signer_seeds: &[],
            };

            transfer_nifty(transfer_params)?;
        }
        AssetType::SplToken => {
            msg!("SPL mint detected");

            // Transfer fungible tokens from authority signer to the swap marker ata.
            let transfer_params = TransferSplParams {
                spl_program_info: ctx.accounts.incoming_asset_program,
                payer_info: ctx.accounts.payer,
                mint_info: ctx.accounts.incoming_asset,
                source_owner_info: ctx.accounts.authority,
                destination_owner_info: ctx.accounts.swap_marker,
                source_ata_info: match ctx.accounts.incoming_asset_aux {
                    Some(account_info) => account_info,
                    None => return Err(MonoswapError::MissingIncomingAssetAux.into()),
                },
                destination_ata_info: match ctx.accounts.swap_marker_aux {
                    Some(account_info) => account_info,
                    None => return Err(MonoswapError::MissingSwapMarkerAux.into()),
                },
                amount: args.incoming_amount,
                signer_seeds: &[],
            };

            check_and_transfer_spl(transfer_params)?;
        }
        _ => {
            return Err(MonoswapError::UnsupportedAssetType.into());
        }
    }

    // Fetch the space and minimum lamports required for rent exemption.
    let space: usize = SwapMarker::LEN;
    let lamports: u64 = rent.minimum_balance(space);

    // Swap marker signer seeds.
    let signer_seeds: &[&[u8]] = &[
        b"swap_marker",
        &swap_seeds.namespace.to_bytes(),
        &min(asset1_bytes, asset2_bytes),
        &max(asset1_bytes, asset2_bytes),
        &[bump],
    ];

    // CPI to the System Program to create the swap marker account.
    invoke_signed(
        &system_instruction::create_account(
            ctx.accounts.payer.key,
            ctx.accounts.swap_marker.key,
            lamports,
            space as u64,
            &crate::id(),
        ),
        &[
            ctx.accounts.payer.clone(),
            ctx.accounts.swap_marker.clone(),
            ctx.accounts.system_program.clone(),
        ],
        &[&signer_seeds],
    )?;

    let swap_marker = SwapMarker {
        namespace: *ctx.accounts.namespace.key,
        escrowed_asset: *ctx.accounts.incoming_asset.key,
        external_asset: *ctx.accounts.external_asset.key,
        escrowed_amount: args.incoming_amount,
        external_amount: args.external_amount,
        bump,
    };

    // Serialize and save the swap marker account.
    swap_marker.save(ctx.accounts.swap_marker)
}
