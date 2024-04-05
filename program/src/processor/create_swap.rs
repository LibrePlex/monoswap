use spl_associated_token_account::instruction::create_associated_token_account;

use super::*;

use crate::{
    assertions::assert_same_pubkeys,
    asset_detection::detect_asset,
    state::{AssetType, SwapSeeds, LEGACY_TOKEN_PROGRAM_ID},
};

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

    // Check the swap marker account is derived from the correct seeds and owned by this program.
    let swap_seeds = SwapSeeds {
        namespace: ctx.accounts.namespace.key,
        asset1: ctx.accounts.incoming_asset.key,
        asset2: ctx.accounts.external_asset.key,
    };

    let (swap_marker_pub, bump) = SwapMarker::find_pda(swap_seeds.clone());
    assert_same_pubkeys("swap_marker", ctx.accounts.swap_marker, &swap_marker_pub)?;

    // Detect the incoming asset type and perform the transfer to escrow it.
    let incoming_asset_type = detect_asset(ctx.accounts.incoming_asset)?;

    match incoming_asset_type {
        AssetType::Nifty => {
            msg!("Nifty asset detected");
            // The incoming asset program is actually the Nifty program.
            assert_same_pubkeys(
                "incoming_asset_program",
                ctx.accounts.incoming_asset_program,
                &nifty_asset::ID,
            )?;

            // Decode the Nifty asset.
            let asset = NiftyAsset::from_bytes(&ctx.accounts.incoming_asset.try_borrow_data()?)?;

            // If a group is present on the asset, the group asset account must be the aux account.a
            let group_asset_info_opt = if let Some(group_pub) = asset.group.to_option() {
                if ctx.accounts.incoming_asset_aux.is_none() {
                    msg!("Nifty group asset is missing");
                    return Err(MonoswapError::MissingNiftyGroup.into());
                }

                // We need a group so get it and make sure it matches the group stored on the Nifty account.
                let group_asset_info = ctx.accounts.incoming_asset_aux.unwrap();
                assert_same_pubkeys("nifty group asset", group_asset_info, &group_pub)?;

                Some(group_asset_info)
            } else {
                None
            };

            // Transfer Nifty asset from authority signer to the swap marker.
            NiftyTransferCpi {
                __program: ctx.accounts.incoming_asset_program,
                asset: ctx.accounts.external_asset,
                signer: ctx.accounts.authority,
                recipient: ctx.accounts.swap_marker,
                group_asset: group_asset_info_opt,
            }
            .invoke()?;
        }
        AssetType::SplToken => {
            msg!("SPL mint detected");

            // Map accounts for clearer naming when building the transfer ix.

            // SPL token program.
            let spl_program_info = ctx.accounts.incoming_asset_program;
            // Payer of the transaction.
            let payer_info = ctx.accounts.payer;
            // Owner of the token(s).
            let authority_info = ctx.accounts.authority;
            let swap_marker_info = ctx.accounts.swap_marker;
            // Mint of the token.
            let mint_info = ctx.accounts.incoming_asset;
            // Source token account.
            let source_ata_info = match ctx.accounts.incoming_asset_aux {
                Some(account_info) => account_info,
                None => return Err(MonoswapError::MissingIncomingAssetAux.into()),
            };
            // Destination ATA is the swap marker ATA.
            let destination_ata_info = ctx.accounts.swap_marker_aux;

            // We need decimals from the mint account.
            let mint = unpack::<Mint>(&mint_info.try_borrow_data()?)?;
            // We need to ensure the ATA matches the mint.
            let source_ata = unpack::<TokenAccount>(&source_ata_info.try_borrow_data()?)?;

            // Checks.
            // The incoming asset program is actually one of the SPL token programs.
            assert_same_pubkeys(
                "incoming_asset_program",
                spl_program_info,
                &spl_token_2022::ID,
            )
            .or(assert_same_pubkeys(
                "incoming_asset_program",
                spl_program_info,
                &LEGACY_TOKEN_PROGRAM_ID,
            ))?;

            // Create destination ata, if necessary.
            if destination_ata_info.data_is_empty() {
                msg!("Creating swap marker ATA");
                // creating the associated token account
                invoke(
                    &create_associated_token_account(
                        payer_info.key,
                        swap_marker_info.key,
                        mint_info.key,
                        spl_program_info.key,
                    ),
                    &[
                        payer_info.clone(),
                        swap_marker_info.clone(),
                        mint_info.clone(),
                        destination_ata_info.clone(),
                    ],
                )?;
            }

            // ATA belongs to the mint.
            assert_same_pubkeys("mint", mint_info, &source_ata.mint)?;

            // Account infos for the transfer instruction.
            let account_infos = &[
                spl_program_info.clone(),
                authority_info.clone(),
                mint_info.clone(),
                source_ata_info.clone(),
                destination_ata_info.clone(),
            ];

            // Transfer SPL token from authority signer to the swap marker ATA.
            invoke(
                &transfer_checked(
                    spl_program_info.key,
                    source_ata_info.key,
                    mint_info.key,
                    destination_ata_info.key,
                    authority_info.key,
                    &[],
                    args.incoming_amount,
                    mint.decimals,
                )?,
                account_infos,
            )?;
        }
        _ => {
            return Err(MonoswapError::UnsupportedAssetType.into());
        }
    }

    // Fetch the space and minimum lamports required for rent exemption.
    let space: usize = SwapMarker::LEN;
    let lamports: u64 = rent.minimum_balance(space);

    // Swap marker signer seeds.
    let binding = bump.to_le_bytes();
    let signer_seeds: &[&[u8]] = &SwapMarker::find_signer_seeds(swap_seeds, &binding);

    // CPI to the System Program.
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
        &[signer_seeds],
    )?;

    let my_account = SwapMarker {
        namespace: *ctx.accounts.namespace.key,
        escrowed_asset: *ctx.accounts.incoming_asset.key,
        external_asset: *ctx.accounts.external_asset.key,
        escrowed_amount: args.incoming_amount,
        external_amount: args.external_amount,
        bump,
    };

    my_account.save(ctx.accounts.swap_marker)
}
